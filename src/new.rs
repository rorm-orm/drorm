use crate::fields::traits::{FieldColumns, FieldType};
use crate::fields::utils::const_fn::{ConstFn, Contains};
use crate::internal::hmr::annotations::Annotations;

pub trait Struct: Sized + 'static {}

pub trait Field: Sized + 'static {
    /* Core information */

    /// The struct this field is part of
    type Struct: Struct;

    /// The field's position in the struct
    const POSITION: usize;

    /// Annotations the field is declared with
    const ANNOTATIONS: Annotations;

    /// The field's identifier modified to be more db friendly
    const NAME: &'static str;

    /// The field's type
    type Type: FieldType;

    /* The field's location in the source code */

    /// The source file the field is declared in
    const FILE: &'static str;

    /// The source line the field is declared in
    const LINE: u32;

    /// The source column the field is declared in
    const COLUMN: u32;

    /* Values calculated by FieldType's const methods */

    const EFFECTIVE_ANNOTATIONS: FieldColumns<Self::Type, Annotations> =
        <<<Self::Type as FieldType>::GetAnnotations as ConstFn<_, _>>::Body<(
            contains::Annotations<Self>,
        )> as Contains<_>>::ITEM;

    const EFFECTIVE_NAMES: FieldColumns<Self::Type, &'static str> =
        <<<Self::Type as FieldType>::GetNames as ConstFn<_, _>>::Body<(contains::Name<Self>,)> as Contains<_>>::ITEM;
}

/// Helper structs implementing [`Contains`] to expose
/// - [`Field::IDENT`]
/// - [`Field::ANNOTATIONS`]
mod contains {
    use std::marker::PhantomData;

    use crate::fields::utils::const_fn::Contains;
    use crate::internal::hmr;
    use crate::new::Field;

    pub struct Annotations<F: Field>(PhantomData<F>);
    impl<F: Field> Contains<hmr::annotations::Annotations> for Annotations<F> {
        const ITEM: hmr::annotations::Annotations = F::ANNOTATIONS;
    }

    pub struct Name<F: Field>(PhantomData<F>);
    impl<F: Field> Contains<&'static str> for Name<F> {
        const ITEM: &'static str = F::NAME;
    }
}
