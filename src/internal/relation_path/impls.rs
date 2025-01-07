use crate::internal::field::foreign_model::{ForeignModelField, ForeignModelTrait};
use crate::internal::field::{ModelField, SingleColumnField};
use crate::internal::query_context::QueryContext;
use crate::internal::relation_path::{Path, PathField};
use crate::new::Field;
use crate::prelude::{BackRef, ForeignModelByField};
use crate::{sealed, Model};

impl<M: Model> Path for M {
    sealed!(impl);

    type Origin = M;

    type Current = M;

    const IS_ORIGIN: bool = true;

    type Step<F>
        = (F, Self)
    where
        F: ModelField + PathField<<F as Field>::Type>,
        F::ParentField: ModelField<Model = Self::Current>;

    fn add_to_context<'ctx>(context: &'ctx mut QueryContext) -> &'ctx str {
        context.add_origin_path::<Self>()
    }
}

impl<F, P> Path for (F, P)
where
    F: ModelField + PathField<<F as Field>::Type>,
    P: Path<Current = <F::ParentField as ModelField>::Model>,
{
    sealed!(impl);

    type Origin = P::Origin;

    type Current = <<F as PathField<F::Type>>::ChildField as ModelField>::Model;

    type Step<F2>
        = (F2, Self)
    where
        F2: ModelField + PathField<<F2 as Field>::Type>,
        F2::ParentField: ModelField<Model = Self::Current>;

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
    F: ModelField<Type = BackRef<FMF>> + 'static,
{
    sealed!(impl);

    type ChildField = FMF;
    type ParentField = <<FMF as Field>::Type as ForeignModelTrait>::RelatedField;
}
