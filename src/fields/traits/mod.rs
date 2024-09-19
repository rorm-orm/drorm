//! Traits defining types which can be used as fields.

use rorm_db::row::RowError;
use rorm_db::sql::value::NullType;
use rorm_db::Row;

pub use self::aggregate::*;
pub use self::cmp::*;
use crate::conditions::Value;
use crate::crud::decoder::Decoder;
use crate::fields::utils::const_fn::ConstFn;
use crate::internal::const_concat::ConstString;
use crate::internal::field::decoder::FieldDecoder;
use crate::internal::field::fake_field::FakeField;
use crate::internal::field::{Field, FieldProxy};
use crate::internal::hmr::annotations::Annotations;
use crate::internal::query_context::QueryContext;
use crate::internal::relation_path::Path;
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

    #[doc(hidden)]
    fn is_option<Private: crate::private::Private>() -> bool {
        false
    }
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

    /// Calls [`array::map`] on the generic array types produced by [`Self::Array<T>`](Self::Array)
    fn map<T, U>(array: Self::Array<T>, f: impl FnMut(T) -> U) -> Self::Array<U>;

    /// The number of columns
    const NUM: usize;
}

/// Implementor of [`Columns`] used to specify the number of a [`FieldType`]'s columns
pub struct Array<const N: usize>;

impl<const N: usize> Columns for Array<N> {
    sealed!(impl);

    type Array<T> = [T; N];

    fn map<T, U>(array: Self::Array<T>, f: impl FnMut(T) -> U) -> Self::Array<U> {
        array.map(f)
    }

    const NUM: usize = N;
}

impl<T: FieldType> FieldType for Option<T> {
    type Columns = T::Columns;

    const NULL: FieldColumns<Self, NullType> = T::NULL;

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        self.map(T::into_values)
            .unwrap_or(T::Columns::map(T::NULL, Value::Null))
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        self.as_ref()
            .map(T::as_values)
            .unwrap_or(T::Columns::map(T::NULL, Value::Null))
    }

    type Decoder = OptionDecoder<T>;
    type GetNames = T::GetNames;
    // Sadly we can't iterate over the array returned by T::GetAnnotations
    // in a const context in order to set nullable.
    // Therefore, we have to resort to "fixing" it at runtime in the `push_imr` function.
    type GetAnnotations = T::GetAnnotations;
    type Check = T::Check;

    fn is_option<Private: crate::private::Private>() -> bool {
        true
    }
}

/// [`FieldDecoder`] for [`Option<T>`]
pub struct OptionDecoder<T: FieldType>(T::Decoder);
impl<T: FieldType> FieldDecoder for OptionDecoder<T> {
    fn new<F, P>(ctx: &mut QueryContext, _: FieldProxy<F, P>) -> Self
    where
        F: Field<Type = Self::Result>,
        P: Path,
    {
        Self(T::Decoder::new::<FakeField<T, F>, P>(
            ctx,
            FieldProxy::new(),
        ))
    }
}
impl<T: FieldType> Decoder for OptionDecoder<T> {
    type Result = Option<T>;

    fn by_name<'index>(&'index self, row: &'_ Row) -> Result<Self::Result, RowError<'index>> {
        self.0.by_name(row).map(Some).or_else(|error| match error {
            RowError::UnexpectedNull { .. } => Ok(None),
            _ => Err(error),
        })
    }

    fn by_index<'index>(&'index self, row: &'_ Row) -> Result<Self::Result, RowError<'index>> {
        self.0.by_name(row).map(Some).or_else(|error| match error {
            RowError::UnexpectedNull { .. } => Ok(None),
            _ => Err(error),
        })
    }
}
