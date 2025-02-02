//! The [ForeignModel] field type

use std::fmt;

use rorm_db::Executor;

use crate::conditions::{Binary, BinaryOperator, Column};
use crate::crud::query::query;
use crate::fields::proxy;
use crate::internal::field::SingleColumnField;
use crate::model::Model;
use crate::Patch;

/// Alias for [ForeignModelByField] which only takes a model uses to its primary key.
pub type ForeignModel<M> = ForeignModelByField<<M as Model>::Primary>;

/// Stores a link to another model in a field.
///
/// In database language, this is a many to one relation.
pub struct ForeignModelByField<FF: SingleColumnField>(pub FF::Type);

impl<FF: SingleColumnField> ForeignModelByField<FF> {
    /// Queries the associated model
    pub async fn query(self, executor: impl Executor<'_>) -> Result<FF::Model, crate::Error> {
        query(executor, <FF::Model as Patch>::ValueSpaceImpl::default())
            .condition(Binary {
                operator: BinaryOperator::Equals,
                fst_arg: Column(proxy::new::<(FF, FF::Model)>()),
                snd_arg: FF::type_into_value(self.0),
            })
            .one()
            .await
    }
}

impl<FF: SingleColumnField> fmt::Debug for ForeignModelByField<FF>
where
    FF::Type: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ForeignModelByField").field(&self.0).finish()
    }
}
impl<FF: SingleColumnField> Clone for ForeignModelByField<FF>
where
    FF::Type: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<FF: SingleColumnField> Copy for ForeignModelByField<FF> where FF::Type: Copy {}
