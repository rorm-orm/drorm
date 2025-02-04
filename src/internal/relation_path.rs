//! Implicit join prototypes

use crate::internal::field::foreign_model::{ForeignModelField, ForeignModelTrait};
use crate::internal::field::{Field, SingleColumnField};
use crate::internal::query_context::QueryContext;
use crate::prelude::{BackRef, ForeignModelByField};
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

impl<M: Model> Path for M {
    sealed!(impl);

    type Origin = M;

    type Current = M;

    const IS_ORIGIN: bool = true;

    type Step<F>
        = (F, Self)
    where
        F: Field + PathField<<F as Field>::Type>,
        F::ParentField: Field<Model = Self::Current>;

    fn add_to_context<'ctx>(context: &'ctx mut QueryContext) -> &'ctx str {
        context.add_origin_path::<Self>()
    }
}

impl<F, P> Path for (F, P)
where
    F: Field + PathField<<F as Field>::Type>,
    P: Path<Current = <F::ParentField as Field>::Model>,
{
    sealed!(impl);

    type Origin = P::Origin;

    type Current = <<F as PathField<F::Type>>::ChildField as Field>::Model;

    type Step<F2>
        = (F2, Self)
    where
        F2: Field + PathField<<F2 as Field>::Type>,
        F2::ParentField: Field<Model = Self::Current>;

    fn add_to_context<'ctx>(context: &'ctx mut QueryContext) -> &'ctx str {
        context.add_relation_path::<F, P>()
    }
}

impl<FF, F> PathField<ForeignModelByField<FF>> for F
where
    FF: SingleColumnField,
    F: ForeignModelField<Type = ForeignModelByField<FF>>,
{
    sealed!(impl);

    type ChildField = FF;
    type ParentField = F;
}
impl<FF, F> PathField<Option<ForeignModelByField<FF>>> for F
where
    FF: SingleColumnField,
    F: ForeignModelField<Type = Option<ForeignModelByField<FF>>>,
{
    sealed!(impl);

    type ChildField = FF;
    type ParentField = F;
}
impl<FMF, F> PathField<BackRef<FMF>> for F
where
    FMF: ForeignModelField,
    FMF::Type: ForeignModelTrait,
    F: Field<Type = BackRef<FMF>> + 'static,
{
    sealed!(impl);

    type ChildField = FMF;
    type ParentField = <<FMF as Field>::Type as ForeignModelTrait>::RelatedField;
}
