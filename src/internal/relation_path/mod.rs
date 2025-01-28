//! Implicit join prototypes

mod impls;

use crate::internal::field::{Field, SingleColumnField};
use crate::internal::query_context::QueryContext;
use crate::{sealed, Model};

/// Trait to store a relation path in generics
///
/// They represent the "path" a field is access through:
/// ```
/// # use rorm::internal::field::FieldProxy;
/// # use rorm::prelude::*;
/// # #[derive(Model)]
/// # struct Group {
/// #     #[rorm(id)]
/// #     id: i64,
/// #     #[rorm(max_length = 255)]
/// #     name: String,
/// # }
/// # #[derive(Model)]
/// # struct User {
/// #     #[rorm(id)]
/// #     id: i64,
/// #     group: ForeignModel<Group>,
/// # }
/// # #[derive(Model)]
/// # struct Comment {
/// #     #[rorm(id)]
/// #     id: i64,
/// #     user: ForeignModel<User>,
/// # }
/// // Direct access
/// let _: FieldProxy<__Group_name, Group>
///     = Group.name;
///
/// // Access through a single relation
/// let _: FieldProxy<__Group_name, (__User_group, User)>
///     = User.group.name;
///
/// // Access through two relation steps
/// let _: FieldProxy<__Group_name, (__User_group, (__Comment_user, Comment))>
///     = Comment.user.group.name;
/// ```
///
/// Paths start at a model (`Origin`), step through relational fields ([`PathField`]) and end on a model (`Current`).
///
/// - To start a path, simply use the origin (every `Model` also implements `Path`).
/// - To change the path's current model, use the "method" `Step` or `Join`.
///
/// As the example above showed, single path steps are represented as tuples.
/// However, this should be treated as implementation detail and not depended on outside of this module.
pub trait Path: 'static {
    sealed!(trait);

    /// The model this path originates from
    ///
    /// (In the context of sql,
    /// this would be the table selected from)
    type Origin: Model;

    /// The model this path currently points to
    ///
    /// (In the context of sql,
    /// this would be the table whose columns can be selected through this path's alias)
    type Current: Model;

    /// Is `Self = Self::Origin`?
    const IS_ORIGIN: bool = false;

    /// "Function" which constructs a new path by taking a step through `F`
    type Step<F>: Path
    where
        F: Field + PathField<<F as Field>::Type>,
        F::ParentField: Field<Model = Self::Current>;

    //type Join<SubPath>: Path
    //where
    //    SubPath: Path<Origin = Self::Current>;

    /// Add all joins required to use this path to the query context
    fn add_to_context<'ctx>(context: &'ctx mut QueryContext) -> &'ctx str;
}

/// A field representing a db relation which can be used to construct paths.
///
/// When applied to a path (using [`Path::Step`])
/// this field will change the path's [current](Path::Current) model
/// from [`ParentField::Model`](PathField::ParentField)
/// to [`ChildField::Model`](PathField::ChildField).
///
/// Implementors are fields with type [`ForeignModel`] or [`BackRef`].
///
/// `ChildField` and `ParentField` are not necessarily the same as `Self`
/// but they both have to represent a single column, one of them has to
/// be a foreign key and `ParentField` has to exist on the same model as `Self`.
///
/// The generic parameter `FieldType` is a workaround
/// to be able to have 3 different `impl<F> PathField for F`
/// without rust complaining about overlapping implementations.
/// Its value will always be `<Self as Field>::Type`.
pub trait PathField<FieldType>: Field {
    sealed!(trait);

    /// Field existing on the path's new current model relating to `ParentField`
    type ChildField: SingleColumnField;

    /// Field existing on the path's old current model relating to `ChildField`
    type ParentField: SingleColumnField;
}
