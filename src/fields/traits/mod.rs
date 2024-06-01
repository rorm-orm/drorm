//! Traits defining types which can be used as fields.

use crate::conditions::Value;
use crate::internal::field::decoder::FieldDecoder;
use crate::internal::field::modifier::{AnnotationsModifier, CheckModifier, ColumnsFromName};
use crate::internal::field::Field;
use crate::internal::imr;

pub mod cmp;

pub use cmp::*;

use crate::sealed;

/// Base trait for types which are allowed as fields in models
pub trait FieldType: 'static {
    /// Array with length specific to the field type
    type Columns: Columns;

    /// Construct an array of [`Value`] representing `self` in the database via ownership
    fn into_values(self) -> FieldColumns<Self, Value<'static>>;

    /// Construct an array of [`Value`] representing `self` in the database via borrowing
    fn as_values(&self) -> FieldColumns<Self, Value<'_>>;

    /// Construct an array of [`imr::Field`] representing this type
    fn get_imr<F: Field<Type = Self>>() -> FieldColumns<Self, imr::Field>;

    /// [`FieldDecoder`] to use for fields of this type
    type Decoder: FieldDecoder<Result = Self>;

    /// `const fn<F: Field>() -> Option<Annotations>`
    /// to allow modifying the a field's annotations which is of this type
    ///
    /// For example can be used to set `nullable` implicitly for `Option<_>`.
    type AnnotationsModifier<F: Field<Type = Self>>: AnnotationsModifier<F>;

    /// `const fn<F: Field>() -> Result<(), &'static str>`
    /// to allow custom compile time checks.
    ///
    /// For example can be used to ensure `String` has a `max_lenght`.
    type CheckModifier<F: Field<Type = Self>>: CheckModifier<F>;

    /// `const fn<F: Field>() -> Self::Columns<&'static str>`
    type ColumnsFromName<F: Field<Type = Self>>: ColumnsFromName<F>;
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
