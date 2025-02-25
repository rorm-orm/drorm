use rorm_db::sql::value::NullType;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::conditions::Value;
use crate::{impl_FieldEq, impl_FieldMin_FieldMax, impl_FieldOrd, impl_FieldType};

impl_FieldType!(Time, TimeTime, Value::TimeTime);
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Time> for Time { Value::TimeTime });
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Option<Time>> for Option<Time> { |option: Self| option.map(Value::TimeTime).unwrap_or(Value::Null(NullType::TimeTime)) });
impl_FieldOrd!(Time, Time, Value::TimeTime);
impl_FieldOrd!(Option<Time>, Option<Time>, |option: Self| option
    .map(Value::TimeTime)
    .unwrap_or(Value::Null(NullType::TimeTime)));
impl_FieldMin_FieldMax!(Time);

impl_FieldType!(Date, TimeDate, Value::TimeDate);
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Date> for Date { Value::TimeDate });
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Option<Date>> for Option<Date> { |option: Self| option.map(Value::TimeDate).unwrap_or(Value::Null(NullType::TimeDate)) });
impl_FieldOrd!(Date, Date, Value::TimeDate);
impl_FieldOrd!(Option<Date>, Option<Date>, |option: Self| option
    .map(Value::TimeDate)
    .unwrap_or(Value::Null(NullType::TimeDate)));
impl_FieldMin_FieldMax!(Date);

impl_FieldType!(
    OffsetDateTime,
    TimeOffsetDateTime,
    Value::TimeOffsetDateTime
);
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, OffsetDateTime> for OffsetDateTime { Value::TimeOffsetDateTime });
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Option<OffsetDateTime>> for Option<OffsetDateTime> { |option: Self| option.map(Value::TimeOffsetDateTime).unwrap_or(Value::Null(NullType::TimeOffsetDateTime)) });
impl_FieldOrd!(OffsetDateTime, OffsetDateTime, Value::TimeOffsetDateTime);
impl_FieldOrd!(
    Option<OffsetDateTime>,
    Option<OffsetDateTime>,
    |option: Self| option
        .map(Value::TimeOffsetDateTime)
        .unwrap_or(Value::Null(NullType::TimeOffsetDateTime))
);
impl_FieldMin_FieldMax!(OffsetDateTime);

impl_FieldType!(
    PrimitiveDateTime,
    TimePrimitiveDateTime,
    Value::TimePrimitiveDateTime
);
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, PrimitiveDateTime> for PrimitiveDateTime { Value::TimePrimitiveDateTime });
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Option<PrimitiveDateTime>> for Option<PrimitiveDateTime> { |option: Self| option.map(Value::TimePrimitiveDateTime).unwrap_or(Value::Null(NullType::TimePrimitiveDateTime)) });
impl_FieldOrd!(
    PrimitiveDateTime,
    PrimitiveDateTime,
    Value::TimePrimitiveDateTime
);
impl_FieldOrd!(
    Option<PrimitiveDateTime>,
    Option<PrimitiveDateTime>,
    |option: Self| option
        .map(Value::TimePrimitiveDateTime)
        .unwrap_or(Value::Null(NullType::TimePrimitiveDateTime))
);
impl_FieldMin_FieldMax!(PrimitiveDateTime);
