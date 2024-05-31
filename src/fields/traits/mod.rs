//! Traits defining types which can be used as fields.

use rorm_db::sql::value::NullType;

pub use self::aggregate::*;
pub use self::cmp::*;
use crate::conditions::Value;
use crate::fields::utils::const_fn::ConstFn;
use crate::internal::const_concat::ConstString;
use crate::internal::field::decoder::FieldDecoder;
use crate::internal::hmr::annotations::Annotations;
use crate::sealed;

pub mod aggregate;
pub mod cmp;

/// Base trait for types which are allowed as fields in models
pub trait FieldType: 'static {
    /// Array with length specific to the field type
    type Columns: Columns;

    /// The null types representing `Option<Self>` in the database
    ///
    /// This is used to implement `into_values` and `as_values` for `Option<Self>`,
    /// as well as provide the columns' database types to the migrator.
    const NULL: FieldColumns<Self, NullType>;

    /// Construct an array of [`Value`] representing `self` in the database via ownership
    fn into_values(self) -> FieldColumns<Self, Value<'static>>;

    /// Construct an array of [`Value`] representing `self` in the database via borrowing
    fn as_values(&self) -> FieldColumns<Self, Value<'_>>;

    /// [`FieldDecoder`] to use for fields of this type
    type Decoder: FieldDecoder<Result = Self>;

    /// Get the columns' names from the field's name
    type GetNames: ConstFn<(&'static str,), FieldColumns<Self, &'static str>>;

    /// Get the columns' annotations from the field's annotations
    type GetAnnotations: ConstFn<(Annotations,), FieldColumns<Self, Annotations>>;

    /// Check a field's annotations to be compatible with this type
    ///
    /// The function gets the annotations explicitly set by the model author
    /// as well as the result from [`FieldType::GetAnnotations`].
    type Check: ConstFn<
        (Annotations, FieldColumns<Self, Annotations>),
        Result<(), ConstString<1024>>,
    >;
}
/// Shorthand for constructing an array with the length for the [`FieldType`]'s columns
pub type FieldColumns<F, T> = <<F as FieldType>::Columns as Columns>::Array<T>;

/// The trait for the [`FieldType`]'s `Columns` associated type.
///
/// It is implemented by [`Array`] and is equivalent to a fixed length.
pub trait Columns {
    sealed!(trait);

    /// Array of length `NUM` to store columns' information in
    type Array<T>: IntoIterator<Item = T>;

    /// The number of columns
    const NUM: usize;
}

/// Implementor of [`Columns`] used to specify the number of a [`FieldType`]'s columns
pub struct Array<const N: usize>;

impl<const N: usize> Columns for Array<N> {
    sealed!(impl);

    type Array<T> = [T; N];

    const NUM: usize = N;
}
