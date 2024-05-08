//! This module provides primitives used by the various builder.

use crate::conditions::Condition;
use crate::internal::query_context::QueryContext;
use crate::sealed;

/// Marker for the generic parameter storing an optional [`Condition`]
pub trait ConditionMarker<'a>: 'a + Send {
    sealed!(trait);

    /// Calls [`Condition::build`] if `Self: Condition`
    /// or returns `None` if `Self = ()`
    fn build(&self, context: &mut QueryContext<'a>) -> Option<usize>;
}

impl<'a> ConditionMarker<'a> for () {
    sealed!(impl);

    fn build(&self, _context: &mut QueryContext<'a>) -> Option<usize> {
        None
    }
}

impl<'a, T: Condition<'a>> ConditionMarker<'a> for T {
    sealed!(impl);

    fn build(&self, context: &mut QueryContext<'a>) -> Option<usize> {
        Some(context.add_condition(self))
    }
}
