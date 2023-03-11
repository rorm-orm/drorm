//! A type level version of [`imr::DbType`](crate::imr::DbType) to be used in generic type bound checks

use rorm_db::sql::value::NullType;

use super::AsImr;
use crate::internal::hmr::annotations::AnnotationIndex;
use crate::{declare_type_option, imr, sealed};

/// Trait to associate the type-level db types with their runtime db types
pub trait DbType: 'static {
    sealed!();

    /// Equivalent runtime db type
    const IMR: imr::DbType;

    /// Type to pass to rorm-sql for null
    const NULL_TYPE: NullType;

    /// Annotations required by this type
    const REQUIRED: &'static [AnnotationIndex] = &[];
}

impl<T: DbType> AsImr for T {
    type Imr = imr::DbType;

    fn as_imr(&self) -> Self::Imr {
        T::IMR
    }
}

macro_rules! impl_db_types {
        ($(#[doc = $doc:literal] $type:ident, $variant:ident, $(requires $required:expr, )?)*) => {
            $(
                #[doc = $doc]
                pub struct $type;
                impl DbType for $type {
                    const IMR: imr::DbType = imr::DbType::$type;

                    const NULL_TYPE: NullType = NullType::$variant;

                    $(const REQUIRED: &'static [AnnotationIndex] = &$required;)?
                }
            )*
        };
    }

impl_db_types!(
    /// Type level version of [`imr::DbType::VarChar`]
    VarChar,
    String,
    requires[AnnotationIndex::MaxLength],
    /// Type level version of [`imr::DbType::VarBinary`]
    VarBinary,
    Binary,
    /// Type level version of [`imr::DbType::Int16`]
    Int16,
    I16,
    /// Type level version of [`imr::DbType::Int32`]
    Int32,
    I32,
    /// Type level version of [`imr::DbType::Int64`]
    Int64,
    I64,
    /// Type level version of [`imr::DbType::Float`]
    Float,
    F32,
    /// Type level version of [`imr::DbType::Double`]
    Double,
    F64,
    /// Type level version of [`imr::DbType::Boolean`]
    Boolean,
    Bool,
    /// Type level version of [`imr::DbType::Date`]
    Date,
    NaiveDate,
    /// Type level version of [`imr::DbType::DateTime`]
    DateTime,
    NaiveDateTime,
    /// Type level version of [`imr::DbType::Timestamp`]
    Timestamp,
    I64,
    /// Type level version of [`imr::DbType::Time`]
    Time,
    NaiveTime,
    /// Type level version of [`imr::DbType::Choices`]
    Choices,
    Choice, // TODO requires choices
);

declare_type_option!(OptionDbType, DbType);
