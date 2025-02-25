//! is the other direction to a [foreign model](ForeignModelByField)

use std::collections::HashMap;
use std::fmt;
use std::future::poll_fn;
use std::pin::pin;

use futures_core::Stream;
use rorm_db::executor::Executor;
use rorm_db::sql::value::NullType;
use rorm_db::Error;

use crate::conditions::collections::CollectionOperator::Or;
use crate::conditions::{Binary, BinaryOperator, Column, Condition, DynamicCollection, Value};
use crate::crud::decoder::NoopDecoder;
use crate::crud::query::query;
use crate::fields::proxy;
use crate::fields::proxy::FieldProxy;
use crate::fields::traits::{Array, FieldColumns, FieldType};
use crate::fields::utils::check::disallow_annotations_check;
use crate::fields::utils::get_annotations::forward_annotations;
use crate::fields::utils::get_names::no_columns_names;
use crate::internal::field::foreign_model::{ForeignModelField, ForeignModelTrait};
use crate::internal::field::{foreign_model, Field, SingleColumnField};
use crate::model::GetField;
#[allow(unused_imports)] // clion needs this import to access Patch::field on a Model
use crate::Patch;

/// A back reference is the other direction to a [foreign model](crate::prelude::ForeignModelByField)
#[derive(Clone)]
pub struct BackRef<FMF: ForeignModelField> {
    /// Cached list of models referencing this one.
    ///
    /// If there wasn't any query yet this field will be `None` instead of an empty vector.
    pub cached: Option<Vec<FMF::Model>>,
}
impl<FMF: ForeignModelField> BackRef<FMF> {
    /// Access the cached instances or `None` if the cache wasn't populated yet.
    pub fn get(&self) -> Option<&Vec<FMF::Model>> {
        self.cached.as_ref()
    }

    /// Access the cached instances or `None` if the cache wasn't populated yet.
    pub fn get_mut(&mut self) -> Option<&mut Vec<FMF::Model>> {
        self.cached.as_mut()
    }
}

impl<FMF: ForeignModelField> FieldType for BackRef<FMF> {
    type Columns = Array<0>;

    const NULL: FieldColumns<Self, NullType> = [];

    fn into_values<'a>(self) -> FieldColumns<Self, Value<'a>> {
        []
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        []
    }

    type Decoder = NoopDecoder<Self>;

    type GetAnnotations = forward_annotations<0>;

    type Check = disallow_annotations_check<0>;

    type GetNames = no_columns_names;
}

impl<BRF, FMF> FieldProxy<(BRF, BRF::Model)>
where
    BRF: Field<Type = BackRef<FMF>>,

    FMF: ForeignModelField + SingleColumnField,
    FMF::Type: ForeignModelTrait,
    FMF::Model: GetField<FMF>, // always true
    foreign_model::RF<FMF>: SingleColumnField,
{
    fn model_as_condition<BRP>(patch: &BRP) -> impl Condition
    where
        BRP: Patch<Model = BRF::Model>,
        BRP: GetField<foreign_model::RF<FMF>>,
    {
        Binary {
            operator: BinaryOperator::Equals,
            fst_arg: Column(proxy::new::<(FMF, FMF::Model)>()),
            snd_arg: foreign_model::RF::<FMF>::type_as_value(patch.borrow_field()),
        }
    }

    /// Returns a reference to the [`BackRef`]'s cache after populating it if not done already.
    pub async fn get_or_query<'p, BRP>(
        &self,
        executor: impl Executor<'_>,
        patch: &'p mut BRP,
    ) -> Result<&'p mut [FMF::Model], Error>
    where
        BRP: Patch<Model = BRF::Model>,
        BRP: GetField<BRF>,
        BRP: GetField<foreign_model::RF<FMF>>,
    {
        if <BRP as GetField<BRF>>::borrow_field_mut(patch)
            .cached
            .is_none()
        {
            self.populate(executor, patch).await?;
        }
        Ok(<BRP as GetField<BRF>>::borrow_field_mut(patch)
            .cached
            .as_mut()
            .expect("The cache should have been populated"))
    }

    /// Takes the [`BackRef`]'s cache leaving it unpopulated again or just queries it.
    ///
    /// This function is similar to [`get_or_query`](Self::get_or_query) but returns ownership
    /// and therefore has to clear the cache.
    pub async fn take_or_query<BRP>(
        &self,
        executor: impl Executor<'_>,
        patch: &mut BRP,
    ) -> Result<Vec<FMF::Model>, Error>
    where
        BRP: Patch<Model = BRF::Model>,
        BRP: GetField<BRF>,
        BRP: GetField<foreign_model::RF<FMF>>,
    {
        if let Some(models) = <BRP as GetField<BRF>>::borrow_field_mut(patch)
            .cached
            .take()
        {
            Ok(models)
        } else {
            query(executor, <FMF::Model as Patch>::ValueSpaceImpl::default())
                .condition(Self::model_as_condition(patch))
                .all()
                .await
        }
    }

    /// Populate the [`BackRef`]'s cached field.
    ///
    /// This method doesn't check whether it already has been populated.
    /// If it has, then it will be updated i.e. the cache overwritten.
    pub async fn populate<BRP>(
        &self,
        executor: impl Executor<'_>,
        patch: &mut BRP,
    ) -> Result<(), Error>
    where
        BRP: Patch<Model = BRF::Model>,
        BRP: GetField<BRF>,
        BRP: GetField<foreign_model::RF<FMF>>,
    {
        let cached = Some(
            query(executor, <FMF::Model as Patch>::ValueSpaceImpl::default())
                .condition(Self::model_as_condition(patch))
                .all()
                .await?,
        );
        <BRP as GetField<BRF>>::borrow_field_mut(patch).cached = cached;
        Ok(())
    }

    /// Populate the [`BackRef`]'s cached field for a whole slice of models.
    ///
    /// This method doesn't check whether it already has been populated.
    /// If it has, then it will be updated i.e. the cache overwritten.
    ///
    /// This method doesn't check whether the slice contains a model twice.
    /// To avoid allocations only the first instance actually gets populated.
    pub async fn populate_bulk<BRP>(
        &self,
        executor: impl Executor<'_>,
        patches: &mut [BRP],
    ) -> Result<(), Error>
    where
        <foreign_model::RF<FMF> as Field>::Type: std::hash::Hash + Eq + Clone,
        BRP: Patch<Model = BRF::Model>,
        BRP: GetField<BRF>,
        BRP: GetField<foreign_model::RF<FMF>>,
    {
        if patches.is_empty() {
            return Ok(());
        }

        let mut cache: HashMap<<foreign_model::RF<FMF> as Field>::Type, Option<Vec<FMF::Model>>> =
            HashMap::new();
        {
            let mut stream = pin!(query(
                executor,
                <FMF::Model as Patch>::ValueSpaceImpl::default()
            )
            .condition(DynamicCollection {
                operator: Or,
                vector: patches.iter().map(Self::model_as_condition).collect(),
            })
            .stream());

            while let Some(instance) = poll_fn(|ctx| stream.as_mut().poll_next(ctx))
                .await
                .transpose()?
            {
                if let Some(key) = instance.borrow_field().as_key() {
                    cache
                        .entry(key.clone())
                        .or_insert_with(|| Some(Vec::new()))
                        .as_mut()
                        .expect("the line 2 above should init missing keys with Some, never None")
                        .push(instance);
                }
            }
        }

        for model in patches {
            let cached = cache.get_mut(<BRP as GetField<foreign_model::RF<FMF>>>::borrow_field(
                model,
            ));
            <BRP as GetField<BRF>>::borrow_field_mut(model).cached =
                cached.map(Option::take).unwrap_or(Some(Vec::new()));
        }

        Ok(())
    }
}

impl<FMF> fmt::Debug for BackRef<FMF>
where
    FMF: ForeignModelField,
    FMF::Model: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackRef")
            .field("cached", &self.cached)
            .finish()
    }
}

impl<FMF: ForeignModelField> Default for BackRef<FMF> {
    fn default() -> Self {
        Self { cached: None }
    }
}
