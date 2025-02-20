use crate::conditions::collections::CollectionOperator;
use crate::conditions::{BinaryOperator, Condition, Value};
use crate::internal::query_context::flat_conditions::FlatCondition;
use crate::internal::query_context::QueryContext;

/// An "IN" expression
///
/// The implementation will definitely change,
/// but it's better to have some form of "IN" than none.
#[derive(Clone, Debug)]
pub struct In<A, B> {
    /// SQL operator to use
    pub operator: InOperator,
    /// The left side of the operator
    pub fst_arg: A,
    /// The mulidple values on the operator's right side
    pub snd_arg: Vec<B>,
}

/// The operator of an "IN" expression
#[derive(Copy, Clone, Debug)]
pub enum InOperator {
    /// Representation of "{} IN ({}, ...)" in SQL
    In,
    /// Representation of "{} NOT IN ({}, ...)" in SQL
    NotIn,
}

impl<'a, A, B> Condition<'a> for In<A, B>
where
    A: Condition<'a>,
    B: Condition<'a>,
{
    fn build(&self, context: &mut QueryContext<'a>) {
        if self.snd_arg.is_empty() {
            Value::Bool(false).build(context);
        } else {
            context
                .conditions
                .push(FlatCondition::StartCollection(CollectionOperator::Or));
            for snd_arg in self.snd_arg.iter() {
                context
                    .conditions
                    .push(FlatCondition::BinaryCondition(BinaryOperator::Equals));
                self.fst_arg.build(context);
                snd_arg.build(context);
            }
            context.conditions.push(FlatCondition::EndCollection);
        }
    }
}
