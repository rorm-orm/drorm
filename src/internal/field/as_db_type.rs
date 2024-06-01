//! defines and implements the [`AsDbType`] trait.

use rorm_db::row::DecodeOwned;
use rorm_declaration::imr;

use crate::fields::traits::Array;
use crate::internal::field::{Field, FieldType};
use crate::internal::hmr::db_type::DbType;
use crate::internal::hmr::AsImr;

/// This trait maps rust types to database types
///
/// I.e. it specifies which datatypes are allowed on model's fields.
pub trait AsDbType: FieldType + Sized {
    /// A type which can be retrieved from the db and then converted into Self.
    type Primitive: DecodeOwned;

    /// The database type as defined in the Intermediate Model Representation
    type DbType: DbType;
}

/// Provides the "default" implementation of [`AsDbType`] and [`FieldType`] of kind `AsDbType`.
///
/// ## Usages
/// - `impl_as_db_type!(RustType, DbType, into_value, as_value);`
///     - `RustType` is the type to implement the traits on.
///     - `DbType` is the database type to associate with (must implement [`DbType`]).
///     - `into_value` is used to convert `RustType` into a [`Value<'static>`] (must implement `Fn(RustType) -> Value<'static>`).
///     - `as_value` is used to convert `&'a RustType` into a [`Value<'a>`] (must implement `Fn(&'_ RustType) -> Value<'_>`).
///       If `RustType` implements `Copy`, `as_value` can be omitted and will use `into_value` instead.
#[doc(hidden)]
#[allow(non_snake_case)] // makes it clearer that a trait and which trait is meant
#[macro_export]
macro_rules! impl_AsDbType {
    (Option<$type:ty>, $decoder:ty) => {
        impl $crate::fields::traits::FieldType for Option<$type> {
            type Columns = $crate::fields::traits::Array<1>;

            fn into_values(self) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'static>> {
                self.map(<$type>::into_values)
                    .unwrap_or([Value::Null(<<$type as $crate::internal::field::as_db_type::AsDbType>::DbType as $crate::internal::hmr::db_type::DbType>::NULL_TYPE)])
            }

            fn as_values(&self) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'_>> {
                self.as_ref()
                    .map(<$type>::as_values)
                    .unwrap_or([Value::Null(<<$type as $crate::internal::field::as_db_type::AsDbType>::DbType as $crate::internal::hmr::db_type::DbType>::NULL_TYPE)])
            }

            fn get_imr<F: $crate::internal::field::Field<Type = Self>>() -> $crate::fields::traits::FieldColumns<Self, $crate::internal::imr::Field> {
                $crate::internal::field::as_db_type::get_single_imr::<F>(
                    <<$type as $crate::internal::field::as_db_type::AsDbType>::DbType as $crate::internal::hmr::db_type::DbType>::IMR
                )
            }

            type Decoder = $decoder;

            type GetAnnotations = $crate::fields::utils::get_annotations::set_null_annotations;

            type Check = <$type as $crate::fields::traits::FieldType>::Check;

            type GetNames = $crate::fields::utils::get_names::single_column_name;
        }

        impl $crate::internal::field::as_db_type::AsDbType for Option<$type> {
            type Primitive = Option<<$type as $crate::internal::field::as_db_type::AsDbType>::Primitive>;
            type DbType = <$type as $crate::internal::field::as_db_type::AsDbType>::DbType;
        }
    };
    ($type:ty, $db_type:ty, $into_value:expr) => {
        impl_AsDbType!($type, $db_type, $into_value, |&value| $into_value(value));
    };
    ($type:ty, $db_type:ty, $into_value:expr, $as_value:expr) => {
        impl_AsDbType!($type, $db_type, $into_value, $as_value, $crate::fields::utils::check::shared_linter_check<1>);
    };
    ($type:ty, $db_type:ty, $into_value:expr, $as_value:expr, $Check:ty) => {
        impl $crate::fields::traits::FieldType for $type {
            type Columns = $crate::fields::traits::Array<1>;

            #[inline(always)]
            fn as_values(&self) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'_>> {
                #[allow(clippy::redundant_closure_call)] // clean way to pass code to a macro
                [$as_value(self)]
            }

            fn into_values(self) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'static>> {
                #[allow(clippy::redundant_closure_call)] // clean way to pass code to a macro
                [$into_value(self)]
            }

            fn get_imr<F: $crate::internal::field::Field<Type = Self>>() -> $crate::fields::traits::FieldColumns<Self, $crate::internal::imr::Field> {
                $crate::internal::field::as_db_type::get_single_imr::<F>(
                    <$db_type as $crate::internal::hmr::db_type::DbType>::IMR
                )
            }

            type Decoder = $crate::crud::decoder::DirectDecoder<Self>;

            type GetAnnotations = $crate::fields::utils::get_annotations::forward_annotations<1>;

            type Check = $Check;

            type GetNames = $crate::fields::utils::get_names::single_column_name;
        }

        impl $crate::internal::field::as_db_type::AsDbType for $type {
            type Primitive = Self;

            type DbType = $db_type;
        }

        impl_AsDbType!(Option<$type>, $crate::crud::decoder::DirectDecoder<Self>);
    };
}

/// Default implementation of [`FieldType::get_imr`] for field with a single column
pub fn get_single_imr<F>(db_type: imr::DbType) -> [imr::Field; 1]
where
    F: Field,
    F::Type: FieldType<Columns = Array<1>>,
{
    [imr::Field {
        name: F::NAME.to_string(),
        db_type,
        annotations: F::EFFECTIVE_ANNOTATIONS[0].as_imr(),
        source_defined_at: F::SOURCE.map(|s| s.as_imr()),
    }]
}
