//! This module provides primitives used by the various builder.

use rorm_db::conditional as sql;

use crate::conditions::Condition;
use crate::internal::query_context::{QueryContext, QueryContextBuilder};
use crate::sealed;

/// Marker for the generic parameter storing an optional [`Condition`]
pub trait ConditionMarker<'a>: 'a {
    sealed!();

    /// Prepare a query context to be able to handle this condition by registering all implicit joins.
    fn add_to_builder(&self, builder: &mut QueryContextBuilder);

    /// Convert the condition into rorm-sql's format using a query context's registered joins.
    fn into_option<'c>(self, context: &'c QueryContext) -> Option<sql::Condition<'c>>
    where
        'a: 'c;
}

impl<'a> ConditionMarker<'a> for () {
    fn add_to_builder(&self, _builder: &mut QueryContextBuilder) {}

    fn into_option<'c>(self, _context: &'c QueryContext) -> Option<sql::Condition<'c>>
    where
        'a: 'c,
    {
        None
    }
}

impl<'a, T: Condition<'a>> ConditionMarker<'a> for T {
    fn add_to_builder(&self, builder: &mut QueryContextBuilder) {
        Condition::add_to_builder(self, builder);
    }

    fn into_option<'c>(self, context: &'c QueryContext) -> Option<sql::Condition<'c>>
    where
        'a: 'c,
        Self: 'c,
    {
        Some(self.as_sql(context))
    }
}
