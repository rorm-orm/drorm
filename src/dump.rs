use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use rorm_db::database::ColumnSelector;
use rorm_db::executor::Stream;
use rorm_db::row::Decode;
use rorm_db::sql::value::NullType;
use rorm_db::transaction::Transaction;
use rorm_db::{database, Row};
use rorm_declaration::imr;
use rorm_declaration::imr::DbType;

use crate::conditions::Value;
use crate::model::Unrestricted;
use crate::{query, Model};

async fn dump_generic_model<M: Model<QueryPermission = Unrestricted>>(
    tx: &mut Transaction,
) -> Result<Vec<HashMap<&'static str, Value<'static>>>, crate::Error> {
    // Reused tmp buffer
    let mut values = Vec::new();

    query!(tx, M)
        .stream()
        .map_ok(move |model| {
            model.push_values(&mut values);
            M::COLUMNS
                .iter()
                .copied()
                .zip(values.drain(..))
                .collect::<HashMap<_, _>>()
        })
        .try_collect()
        .await
}

async fn dump_imr_model(
    tx: &mut Transaction,
    model: &imr::Model,
) -> Result<Vec<HashMap<Arc<str>, Value<'static>>>, crate::Error> {
    fn get<'r, T>(
        row: &'r Row,
        column: &str,
        variant: impl FnOnce(T) -> Value<'static>,
        null: NullType,
    ) -> Result<Value<'static>, crate::Error>
    where
        Option<T>: Decode<'r>,
    {
        let option = row.get::<Option<T>, _>(column)?;
        let value = option.map(variant).unwrap_or(Value::Null(null));
        Ok(value)
    }

    let columns: Vec<_> = model
        .fields
        .iter()
        .map(|field| ColumnSelector {
            table_name: None,
            column_name: &field.name,
            select_alias: None,
            aggregation: None,
        })
        .collect();
    let row_stream =
        rorm_db::database::query::<Stream>(tx, &model.name, &columns, &[], None, &[], None);

    let mut values_prototype = model
        .fields
        .iter()
        .map(|field| (field.name.clone().into(), Value::Null(NullType::Bool)))
        .collect::<HashMap<Arc<str>, Value<'static>>>();
    values_prototype.shrink_to_fit();
    let values_stream = row_stream.map(|result| {
        let row = result?;

        let mut values = values_prototype.clone();
        for field in &model.fields {
            let value = match field.db_type {
                DbType::VarChar | DbType::Choices => get(
                    &row,
                    &field.name,
                    |string| Value::String(Cow::Owned(string)),
                    NullType::String,
                )?,
                DbType::Binary => get(
                    &row,
                    &field.name,
                    |string| Value::Binary(Cow::Owned(string)),
                    NullType::Binary,
                )?,
                DbType::Int8 => get(&row, &field.name, Value::I16, NullType::I16)?,
                DbType::Int16 => get(&row, &field.name, Value::I16, NullType::I16)?,
                DbType::Int32 => get(&row, &field.name, Value::I32, NullType::I32)?,
                DbType::Int64 | DbType::Timestamp => {
                    get(&row, &field.name, Value::I64, NullType::I64)?
                }
                DbType::Float => get(&row, &field.name, Value::F32, NullType::F32)?,
                DbType::Double => get(&row, &field.name, Value::F64, NullType::F64)?,
                DbType::Boolean => get(&row, &field.name, Value::Bool, NullType::Bool)?,
                #[cfg(feature = "time")]
                DbType::Date => get(&row, &field.name, Value::TimeDate, NullType::TimeDate)?,
                #[cfg(feature = "time")]
                DbType::DateTime => get(
                    &row,
                    &field.name,
                    Value::TimeOffsetDateTime,
                    NullType::TimeOffsetDateTime,
                )?,
                #[cfg(feature = "time")]
                DbType::Time => get(&row, &field.name, Value::TimeTime, NullType::TimeTime)?,
                #[cfg(all(feature = "chrono", not(feature = "time")))]
                DbType::Date => get(
                    &row,
                    &field.name,
                    Value::ChronoNaiveDate,
                    NullType::ChronoNaiveDate,
                )?,
                #[cfg(all(feature = "chrono", not(feature = "time")))]
                DbType::DateTime => get(
                    &row,
                    &field.name,
                    Value::ChronoDateTime,
                    NullType::ChronoDateTime,
                )?,
                #[cfg(all(feature = "chrono", not(feature = "time")))]
                DbType::Time => get(
                    &row,
                    &field.name,
                    Value::ChronoNaiveTime,
                    NullType::ChronoNaiveTime,
                )?,
                #[cfg(all(not(feature = "chrono"), not(feature = "time")))]
                DbType::Date | DbType::DateTime | DbType::Time => {
                    return Err(crate::Error::DecodeError(format!(
                        "Support for {:?} requires chrono or time",
                        field.db_type
                    )))
                }
                #[cfg(feature = "uuid")]
                DbType::Uuid => get(&row, &field.name, Value::Uuid, NullType::Uuid)?,
                #[cfg(not(feature = "uuid"))]
                DbType::Uuid => {
                    return Err(crate::Error::DecodeError(format!(
                        "Support for {:?} requires uuid",
                        field.db_type
                    )))
                }
                #[cfg(feature = "postgres-only")]
                DbType::MacAddress => {
                    get(&row, &field.name, Value::MacAddress, NullType::MacAddress)?
                }
                #[cfg(feature = "postgres-only")]
                DbType::IpNetwork => get(&row, &field.name, Value::IpNetwork, NullType::IpNetwork)?,
                #[cfg(feature = "postgres-only")]
                DbType::BitVec => get(&row, &field.name, Value::BitVec, NullType::BitVec)?,
                #[cfg(not(feature = "postgres-only"))]
                DbType::MacAddress | DbType::IpNetwork | DbType::BitVec => {
                    return Err(crate::Error::DecodeError(format!(
                        "Support for {:?} requires postgres-only",
                        field.db_type
                    )))
                }
            };
            *values.get_mut(field.name.as_str()).unwrap() = value;
        }

        Ok(values)
    });

    values_stream.try_collect().await
}

async fn load_generic_model<M: Model<InsertPermission = Unrestricted>>(
    tx: &mut Transaction,
    values: &[HashMap<impl Borrow<str> + Eq + Hash, Value<'_>>],
) -> Result<(), crate::Error> {
    let mut values_vec = Vec::with_capacity(values.len() * M::COLUMNS.len());

    for row in values {
        for &column in M::COLUMNS {
            values_vec.push(row.get(column).unwrap().as_sql());
        }
    }

    let values_slices: Vec<_> = values_vec.chunks(M::COLUMNS.len()).collect();
    database::insert_bulk(tx, M::TABLE, M::COLUMNS, &values_slices).await
}

async fn load_imr_model(
    tx: &mut Transaction,
    model: &imr::Model,
    values: &[HashMap<impl Borrow<str> + Eq + Hash, Value<'_>>],
) -> Result<(), crate::Error> {
    let columns = model
        .fields
        .iter()
        .map(|field| field.name.as_str())
        .collect::<Vec<_>>();

    let mut values_vec = Vec::with_capacity(values.len() * columns.len());

    for row in values {
        for &name in &columns {
            values_vec.push(row.get(name).unwrap().as_sql());
        }
    }

    let values_slices: Vec<_> = values_vec.chunks(columns.len()).collect();
    database::insert_bulk(tx, &model.name, &columns, &values_slices).await
}
