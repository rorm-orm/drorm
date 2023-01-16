//! The [ForeignModel] field type

use crate::conditions::Value;
use crate::internal::field::{kind, Field, FieldType, OptionField, RawField};
use crate::internal::hmr;
use crate::internal::hmr::annotations::Annotations;
use crate::model::Model;

/// Alias for [ForeignModelByField] which defaults the second generic parameter to use the primary key.
///
/// This default is only provided on this alias instead of the enum itself,
/// because this way internal code must provide the parameter while users can ignore it.
/// Forgetting to set it in internal code could lead to some bugs which are nasty to find.
pub type ForeignModel<M, T = <<M as Model>::Primary as RawField>::Type> = ForeignModelByField<M, T>;

/// Stores a link to another model in a field.
///
/// In database language, this is a many to one relation.
#[derive(Clone, Debug)]
pub enum ForeignModelByField<M: Model, T> {
    /// The other model's primary key which can be used to query it later.
    Key(T),
    /// The other model's queried instance.
    Instance(Box<M>),
}
impl<M: Model, T> ForeignModelByField<M, T> {
    /// Get the instance, if it is available
    pub fn instance(&self) -> Option<&M> {
        match self {
            Self::Key(_) => None,
            Self::Instance(instance) => Some(instance.as_ref()),
        }
    }
}
impl<M: Model, T> From<T> for ForeignModelByField<M, T> {
    fn from(key: T) -> Self {
        Self::Key(key)
    }
}

pub(crate) type RelatedField<M, F> =
    <<F as RawField>::RelatedField as OptionField>::UnwrapOr<<M as Model>::Primary>;

impl<M: Model, T> FieldType for ForeignModelByField<M, T> {
    type Kind = kind::ForeignModel;
}
impl<M, F> Field<kind::ForeignModel> for F
where
    M: Model,
    RelatedField<M, F>: Field,
    F: RawField<
        Type = ForeignModelByField<M, <RelatedField<M, F> as RawField>::Type>,
        Kind = kind::ForeignModel,
    >,
{
    type DbType = <RelatedField<M, F> as Field>::DbType;

    const ANNOTATIONS: Annotations = {
        let mut annos = Self::EXPLICIT_ANNOTATIONS;
        if annos.max_length.is_none() {
            annos.max_length = <RelatedField<M, F> as Field>::ANNOTATIONS.max_length;
        }
        annos.foreign = Some(hmr::annotations::ForeignKey {
            table_name: M::TABLE,
            column_name: RelatedField::<M, F>::NAME,
        });
        annos
    };

    type Primitive = <RelatedField<M, F> as Field>::Primitive;

    fn from_primitive(primitive: Self::Primitive) -> Self::Type {
        ForeignModelByField::Key(<RelatedField<M, F> as Field>::from_primitive(primitive))
    }

    fn as_condition_value(value: &Self::Type) -> Value {
        match value {
            ForeignModelByField::Key(value) => {
                <RelatedField<M, F> as Field>::as_condition_value(value)
            }
            ForeignModelByField::Instance(model) => {
                if let Some(value) = model.get_value(RelatedField::<M, F>::INDEX) {
                    value
                } else {
                    unreachable!("A model should contain its primary key");
                }
            }
        }
    }
}
