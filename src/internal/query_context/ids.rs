//! Domain specific specializations of [`TypeId`]

use std::any::TypeId;

use crate::internal::relation_path::Path;
use crate::FieldAccess;

/// Globally unique id for a [`Path`]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PathId(TypeId);
impl PathId {
    /// Construct a `Path`'s id
    pub fn of<P: Path>() -> Self {
        Self(TypeId::of::<P>())
    }
}

/// Globally unique id for a [`FieldAccess`]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldAccessId(TypeId);
impl FieldAccessId {
    /// Construct a `FieldAccess`' id
    pub fn of<F: FieldAccess>() -> Self {
        Self(TypeId::of::<F>())
    }
}
