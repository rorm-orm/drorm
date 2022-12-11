//! is the other direction to a [foreign model](foreign_model::ForeignModel)

use crate::conditions::collections::CollectionOperator::Or;
use crate::conditions::{Binary, BinaryOperator, Column, Condition, DynamicCollection};
use futures::stream::TryStreamExt;
use rorm_db::row::RowIndex;
use rorm_db::{Database, Error, Row};
use std::collections::HashMap;

use crate::internal::field::as_db_type::AsDbType;
use crate::internal::field::foreign_model::ForeignModelByField;
use crate::internal::field::{
    foreign_model, AbstractField, Field, FieldProxy, FieldType, Pseudo, RawField,
};
use crate::model::{GetField, Model};
use crate::query;
#[allow(unused_imports)] // clion needs this import to access Patch::field on a Model
use crate::Patch;

/// A back reference is the other direction to a [foreign model](foreign_model::ForeignModel)
#[derive(Clone)]
pub struct BackRef<M: Model> {
    /// Cached list of models referencing this one.
    ///
    /// If there wasn't any query yet this field will be `None` instead of an empty vector.
    pub cached: Option<Vec<M>>,
}

impl<M: Model> FieldType for BackRef<M> {
    type Kind = Pseudo;
}

impl<T, BR, BRM, FM, FMM> AbstractField<Pseudo> for BR
where
    // `BRM` and `FMM` are two models
    BRM: Model,
    FMM: Model,

    // `BR` is a pseudo field on the model `BRM`.
    // It has the type of `BackRef<FMM>` and points to the related field `FM`
    BR: RawField<Kind = Pseudo, Model = BRM, RawType = BackRef<FMM>, RelatedField = FM>,

    // `FM` is a field on the model `FMM`.
    // It has the type of `ForeignModelByField<BRM, _>`.
    FM: Field<Model = FMM, Type = ForeignModelByField<BRM, T>>,

    // The field, `FM` points to, is on the model `BRM`.
    // Its type `T` matches the type `FM` stores.
    foreign_model::RelatedField<BRM, FM>: Field<Model = BRM, Type = T>,
{
    fn get_from_row(_row: &Row, _index: impl RowIndex) -> Result<Self::RawType, Error> {
        Ok(BackRef { cached: None })
    }
}

impl<T, BR, BRM, FM, FMM> FieldProxy<BR, BRM>
where
    T: AsDbType + 'static,

    BRM: Model,
    FMM: Model,

    BR: AbstractField<Model = BRM, RawType = BackRef<FMM>, RelatedField = FM>,
    FM: RawField<RawType = ForeignModelByField<BRM, T>>,
    foreign_model::RelatedField<BRM, FM>: RawField<RawType = T>,

    // needs to be a field to be able to be used as column in condition
    FM: Field,

    // obvious access to the models' fields
    FMM: GetField<FM>,
    BRM: GetField<BR>,
    BRM: GetField<foreign_model::RelatedField<BRM, FM>>,
{
    fn model_as_condition(model: &BRM) -> impl Condition {
        Binary {
            operator: BinaryOperator::Equals,
            fst_arg: Column::<FM, FMM>::new(),
            snd_arg: model
                .field::<foreign_model::RelatedField<BRM, FM>>()
                .as_primitive::<foreign_model::RelatedField<BRM, FM>>(),
        }
    }

    /// Populate the [BackRef]'s cached field.
    ///
    /// This method doesn't check whether it already has been populated.
    /// If it has, then it will be updated i.e. the cache overwritten.
    pub async fn populate(&self, db: &Database, model: &mut BRM) -> Result<(), Error> {
        let cached = Some(
            query!(db, FMM)
                .condition(Self::model_as_condition(model))
                .all()
                .await?,
        );
        model.field_mut::<BR>().cached = cached;
        Ok(())
    }

    /// Populate the [BackRef]'s cached field for a whole slice of models.
    ///
    /// This method doesn't check whether it already has been populated.
    /// If it has, then it will be updated i.e. the cache overwritten.
    ///
    /// This method doesn't check whether the slice contains a model twice.
    /// To avoid allocations only the first instance actually gets populated.
    pub async fn populate_bulk(&self, db: &Database, models: &mut [BRM]) -> Result<(), Error>
    where
        T: std::hash::Hash + Eq + Clone,
    {
        let mut cache: HashMap<T, Option<Vec<FMM>>> = HashMap::new();
        {
            let mut stream = query!(db, FMM)
                .condition(DynamicCollection {
                    operator: Or,
                    vector: models.iter().map(Self::model_as_condition).collect(),
                })
                .stream();

            while let Some(instance) = stream.try_next().await? {
                cache
                    .entry(
                        match instance.get_field() {
                            ForeignModelByField::Key(t) => t,
                            ForeignModelByField::Instance(brm) => {
                                brm.field::<foreign_model::RelatedField<BRM, FM>>()
                            }
                        }
                        .clone(),
                    )
                    .or_insert_with(|| Some(Vec::new()))
                    .as_mut()
                    .expect("the line 2 above should init missing keys with Some, never None")
                    .push(instance);
            }
        }

        for model in models {
            let cached = cache.get_mut(model.field::<foreign_model::RelatedField<BRM, FM>>());
            model.field_mut::<BR>().cached = cached.map(Option::take).unwrap_or(Some(Vec::new()));
        }

        Ok(())
    }
}
