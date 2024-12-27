/// Provides the "default" implementation of [`FieldType`].
///
/// ## Usages
/// - `impl_FieldType!(RustType, NullType, into_value, as_value);`
///     - `RustType` is the type to implement the traits on.
///     - `NullType` is the database type to associate with (variant of [`NullType`](crate::db::sql::value::NullType)).
///     - `into_value` is used to convert `RustType` into a [`Value<'static>`] (must implement `Fn(RustType) -> Value<'static>`).
///     - `as_value` is used to convert `&'a RustType` into a [`Value<'a>`] (must implement `Fn(&'_ RustType) -> Value<'_>`).
///       If `RustType` implements `Copy`, `as_value` can be omitted and will use `into_value` instead.
#[doc(hidden)]
#[allow(non_snake_case)] // makes it clearer that a trait and which trait is meant
#[macro_export]
macro_rules! impl_FieldType {
    ($type:ty, $null_type:ident, $into_value:expr) => {
        impl_FieldType!($type, $null_type, $into_value, |&value| $into_value(value));
    };
    ($type:ty, $null_type:ident, $into_value:expr, $as_value:expr) => {
        impl_FieldType!(
            $type,
            $null_type,
            $into_value,
            $as_value,
            $crate::fields::utils::check::shared_linter_check<1>
        );
    };
    ($type:ty, $null_type:ident, $into_value:expr, $as_value:expr, $Check:ty) => {
        impl $crate::fields::traits::FieldType for $type {
            type Columns = $crate::fields::traits::Array<1>;

            const NULL: $crate::fields::traits::FieldColumns<
                Self,
                $crate::db::sql::value::NullType,
            > = [$crate::db::sql::value::NullType::$null_type];

            #[inline(always)]
            fn as_values(
                &self,
            ) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'_>> {
                #[allow(clippy::redundant_closure_call)] // clean way to pass code to a macro
                [$as_value(self)]
            }

            fn into_values<'a>(
                self,
            ) -> $crate::fields::traits::FieldColumns<Self, $crate::conditions::Value<'a>> {
                #[allow(clippy::redundant_closure_call)] // clean way to pass code to a macro
                [$into_value(self)]
            }

            type Decoder = $crate::crud::decoder::DirectDecoder<Self>;

            type GetAnnotations = $crate::fields::utils::get_annotations::forward_annotations<1>;

            type Check = $Check;

            type GetNames = $crate::fields::utils::get_names::single_column_name;
        }
    };
}
