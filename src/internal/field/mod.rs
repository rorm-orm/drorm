//! The various field traits and their proxy.
//!
//! # Introduction
//! Rorm's main entry point is the [`Model`] trait and its derive macro.
//! It takes a struct and generates the code to represent this struct as a database table.
//! To do so each of the struct's fields need to be represented in some way.
//!
//! For each field the derive macro declares a unit struct (i.e. an empty struct) to represent it.
//! This empty struct is then "populated" with the field's information using various traits defined in this module.
//!
//! # Trait Implementation Flow
//! As stated in the introduction, the derive macro generates an unit struct per field.
//! It the proceeds to implement then [`Field`] trait on this empty struct.
//! Therefore, [`Field`] encapsulates all information the macro can gather.
//! This includes:
//! - the name (a db safe version of it, to be precise)
//! - its "raw type" ("raw" because the macro can't make any deductions about the type)
//! - the various annotations inside a `#[rorm(...)]` attribute
//!
//! #### Small illustration
//! ```text
//! #[derive(Model)]
//! struct User {
//!     id: i32,
//!     ...
//! }
//! ```
//! will produce something like
//! ```text
//! struct __User_id;
//! impl Field for __User_id {
//!     type RawType = i32;
//!     const NAME: &'static str = "id";
//!     ...
//! }
//! ```
//!
//! From there the various methods and associated type from [`FieldType`] take over.
//! TODO more docs

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use rorm_db::sql::value::NullType;
use rorm_declaration::imr;

use crate::conditions::Value;
use crate::internal::hmr::annotations::Annotations;
use crate::internal::hmr::{AsImr, Source};
use crate::internal::relation_path::{Path, PathField};
use crate::model::{ConstNew, Model};

pub mod access;
pub mod as_db_type;
pub mod decoder;
pub mod fake_field;
pub mod foreign_model;

use crate::fields::traits::{Array, FieldColumns, FieldType};
use crate::fields::utils::const_fn::{ConstFn, Contains};
use crate::internal::const_concat::ConstString;

/// This trait is implemented by the `#[derive(Model)]` macro on unique unit struct for each of a model's fields.
///
/// It contains all the information a model's author provides on the field.
///
/// This trait itself doesn't do much, but it forms the basis to implement the other traits.
pub trait Field: 'static + Copy {
    /// The type stored in the model's field
    type Type: FieldType;

    /// The model this field is part of
    type Model: Model;

    /// This field's position in the model
    const INDEX: usize;

    /// A db safe name of this field
    const NAME: &'static str;

    /// List of annotations which were set by the user
    const EXPLICIT_ANNOTATIONS: Annotations;

    /// List of annotations which are passed to db
    const EFFECTIVE_ANNOTATIONS: FieldColumns<Self::Type, Annotations> =
        <<<Self::Type as FieldType>::GetAnnotations as ConstFn<_, _>>::Body<(
            contains::ExplicitAnnotations<Self>,
        )> as Contains<_>>::ITEM;

    /// List of names which are passed to db
    const EFFECTIVE_NAMES: FieldColumns<Self::Type, &'static str> =
        <<<Self::Type as FieldType>::GetNames as ConstFn<_, _>>::Body<(contains::Name<Self>,)> as Contains<_>>::ITEM;

    /// Location of the field in the source code
    const SOURCE: Source;

    /// Create a new instance
    ///
    /// Since `Self` is always a zero sized type, this is a noop.
    /// It exists to enable accessing field method through [`FieldProxy`] without having to forward every one.
    fn new() -> Self;
}

/// Pushes a [`Field`]'s columns as [`imr`] onto a vector.
///
/// This function is called by the `#[derive(Model)]` macro to gather a list of all vectors.
pub fn push_imr<F: Field>(imr: &mut Vec<imr::Field>) {
    let names = F::EFFECTIVE_NAMES;
    let db_types = F::Type::NULL;
    let annotations = F::EFFECTIVE_ANNOTATIONS;
    let source_defined_at = F::SOURCE.as_imr();
    let is_option = F::Type::is_option::<()>();

    for ((name, mut annotations), null_type) in names
        .into_iter()
        .zip(annotations.into_iter())
        .zip(db_types.into_iter())
    {
        annotations.nullable |= is_option;
        imr.push(imr::Field {
            name: name.to_string(),
            db_type: match null_type {
                NullType::String => imr::DbType::VarChar,
                NullType::Choice => imr::DbType::Choices,
                NullType::I64 => imr::DbType::Int64,
                NullType::I32 => imr::DbType::Int32,
                NullType::I16 => imr::DbType::Int16,
                NullType::Bool => imr::DbType::Boolean,
                NullType::F64 => imr::DbType::Double,
                NullType::F32 => imr::DbType::Float,
                NullType::Binary => imr::DbType::Binary,
                NullType::ChronoNaiveTime => imr::DbType::Time,
                NullType::ChronoNaiveDate => imr::DbType::Date,
                NullType::ChronoNaiveDateTime => imr::DbType::DateTime,
                NullType::ChronoDateTime => imr::DbType::DateTime,
                NullType::TimeDate => imr::DbType::Date,
                NullType::TimeTime => imr::DbType::Time,
                NullType::TimeOffsetDateTime => imr::DbType::DateTime,
                NullType::TimePrimitiveDateTime => imr::DbType::DateTime,
                NullType::Uuid => imr::DbType::Uuid,
                NullType::UuidHyphenated => imr::DbType::Uuid,
                NullType::UuidSimple => imr::DbType::Uuid,
                NullType::JsonValue => imr::DbType::Binary,
                #[cfg(feature = "postgres-only")]
                NullType::MacAddress => imr::DbType::MacAddress,
                #[cfg(feature = "postgres-only")]
                NullType::IpNetwork => imr::DbType::IpNetwork,
                #[cfg(feature = "postgres-only")]
                NullType::BitVec => imr::DbType::BitVec,
            },
            annotations: annotations.as_imr(),
            source_defined_at: Some(source_defined_at.clone()),
        });
    }
}

/// Check a [`Field`] for correctness by evaluating its [`FieldType`]'s `Check`
///
/// This function is called and its error reported by the `#[derive(Model)]` macro.
pub const fn check<F: Field>() -> Result<(), ConstString<1024>> {
    <<<F::Type as FieldType>::Check as ConstFn<_, _>>::Body<(
        contains::ExplicitAnnotations<F>,
        contains::EffectiveAnnotations<F>,
    )> as Contains<_>>::ITEM
}

/// A field which is stored in db via a single column
pub trait SingleColumnField: Field {
    /// The annotations which are passed to db
    const EFFECTIVE_ANNOTATION: Annotations;

    /// Borrow an instance of the field's type as a [`Value`]
    fn type_as_value(field: &Self::Type) -> Value;

    /// Convert an instance of the field's type into a static [`Value`]
    fn type_into_value(field: Self::Type) -> Value<'static>;
}
impl<F> SingleColumnField for F
where
    F: Field,
    F::Type: FieldType<Columns = Array<1>>,
{
    const EFFECTIVE_ANNOTATION: Annotations = {
        let [annos] = Self::EFFECTIVE_ANNOTATIONS;
        annos
    };

    fn type_as_value(field: &Self::Type) -> Value {
        let [value] = field.as_values();
        value
    }

    fn type_into_value(field: Self::Type) -> Value<'static> {
        let [value] = field.into_values();
        value
    }
}

/// This struct acts as a proxy exposing type level information from the [`Field`] trait on the value level.
///
/// On top of that it can be used to keep track of the "path" this field is accessed through, when dealing with relations.
///
/// ## Type as Value
/// In other words [`FieldProxy`] allows access to things like [`Field::NAME`] without access to the concrete field type.
///
/// Pseudo code for illustration:
/// ```skip
/// // The following is a rough sketch of what the #[derive(Model)] will do:
/// pub struct Id;
/// impl Field for Id {
///     ...
/// }
///
/// pub struct Fields {
///     pub id: FieldProxy<Id>,
///     ...
/// }
///
/// pub struct User {
///     pub id: i32,
/// }
/// impl Model for User {
///     type Fields = Fields;
///     const FIELDS: Self::Fields = Fields {
///         id: Id,
///         ...
///     }
/// }
///
/// // To access Id::NAME from user code, we can't use the Field trait itself,
/// // because the type Id is not really accessible. (It's been generated from a macro.)
/// // Also User::FIELDS or User::F should have more of a struct like syntax.
/// //
/// // So, the Fields struct holds FieldProxy<Id> instead of Id, which implements simple methods
/// // forwarding varies data and behaviors from Id:
///
/// Id::NAME ~ User::F.id.name()
/// Id::Index ~ User::F.id.index()
/// ```
pub struct FieldProxy<Field, Path>(PhantomData<ManuallyDrop<(Field, Path)>>);

// SAFETY:
// struct contains no data
unsafe impl<F, P> Send for FieldProxy<F, P> {}
unsafe impl<F, P> Sync for FieldProxy<F, P> {}

impl<F: Field, P> FieldProxy<F, P> {
    /// Create a new instance
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// Get the field's position in the Model
    pub const fn index(_field: fn() -> Self) -> usize {
        F::INDEX
    }

    /// Change the path
    pub const fn through<NewP>(self) -> FieldProxy<F, NewP> {
        FieldProxy::new()
    }
}
impl<F: Field, P> FieldProxy<F, P> {
    /// Get the names of the columns which store the field
    pub const fn columns(_field: Self) -> FieldColumns<F::Type, &'static str> {
        <<<F::Type as FieldType>::GetNames as ConstFn<_, _>>::Body<(contains::Name<F>,)> as Contains<
            _,
        >>::ITEM
    }

    /// Get the underlying field to call its methods
    pub fn field(&self) -> F {
        F::new()
    }
}
impl<Field, Path> Clone for FieldProxy<Field, Path> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Field, Path> Copy for FieldProxy<Field, Path> {}

/// A field whose proxy should implement [`Deref`](std::ops::Deref) to some collection of fields.
///
/// Depending on the field, this collection might differ in meaning
/// - For [`BackRef`](crate::fields::types::BackRef) and [`ForeignModel`](crate::fields::types::ForeignModelByField),
///   its their related model's fields
/// - For multi-column fields, its their "contained" fields
pub trait ContainerField<T: FieldType, P: Path>: Field<Type = T> {
    /// Struct of contained fields
    type Target: ConstNew;
}

impl<T: FieldType, F: Field<Type = T>, P: Path> std::ops::Deref for FieldProxy<F, P>
where
    F: ContainerField<T, P>,
{
    type Target = F::Target;

    fn deref(&self) -> &'static Self::Target {
        ConstNew::REF
    }
}

impl<T, F, P> ContainerField<T, P> for F
where
    T: FieldType,
    F: Field<Type = T> + PathField<T>,
    P: Path<Current = <F::ParentField as Field>::Model>,
{
    type Target = <<F::ChildField as Field>::Model as Model>::Fields<P::Step<F>>;
}

/// Helper structs implementing [`Contains`] to expose
/// - [`Field::NAME`]
/// - [`Field::EXPLICIT_ANNOTATIONS`]
/// - [`Field::EFFECTIVE_ANNOTATIONS`]
mod contains {
    use std::marker::PhantomData;

    use crate::fields::traits::FieldColumns;
    use crate::fields::utils::const_fn::Contains;
    use crate::internal::field::Field;
    use crate::internal::hmr::annotations::Annotations;

    pub struct ExplicitAnnotations<F: Field>(PhantomData<F>);
    impl<F: Field> Contains<Annotations> for ExplicitAnnotations<F> {
        const ITEM: Annotations = F::EXPLICIT_ANNOTATIONS;
    }

    pub struct EffectiveAnnotations<F: Field>(PhantomData<F>);
    impl<F: Field> Contains<FieldColumns<F::Type, Annotations>> for EffectiveAnnotations<F> {
        const ITEM: FieldColumns<F::Type, Annotations> = F::EFFECTIVE_ANNOTATIONS;
    }

    pub struct Name<F: Field>(PhantomData<F>);
    impl<F: Field> Contains<&'static str> for Name<F> {
        const ITEM: &'static str = F::NAME;
    }
}
