//! The [`Json<T>`] wrapper to store json data in the db

use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use rorm_db::sql::value::NullType;
use rorm_declaration::imr;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::conditions::Value;
use crate::fields::traits::{Array, FieldColumns, FieldType};
use crate::fields::utils::check::shared_linter_check;
use crate::fields::utils::get_annotations::{forward_annotations, set_null_annotations};
use crate::fields::utils::get_names::single_column_name;
use crate::internal::field::as_db_type::{get_single_imr, AsDbType};
use crate::internal::field::Field;
use crate::internal::hmr::db_type::{Binary, DbType};
use crate::new_converting_decoder;
use crate::Error::DecodeError;

/// Stores data by serializing it to json.
///
/// This is just a convenience wrapper around [serde_json] and `Vec<u8>`.
///
/// ```no_run
/// # use std::collections::HashMap;
/// use rorm::Model;
/// use rorm::fields::types::Json;
///
/// #[derive(Model)]
/// pub struct Session {
///     #[rorm(id)]
///     pub id: i64,
///
///     pub data: Json<HashMap<String, String>>,
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Json<T: Serialize + DeserializeOwned>(pub T);

impl<T: Serialize + DeserializeOwned> Json<T> {
    /// Unwrap into inner T value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

new_converting_decoder!(
    pub JsonDecoder<T: Serialize + DeserializeOwned>,
    |value: Vec<u8>| -> Json<T> {
        serde_json::from_slice(&value)
            .map(Json)
            .map_err(|err| DecodeError(format!("Couldn't decoder json: {err}")))
    }
);
impl<T: Serialize + DeserializeOwned + 'static> FieldType for Json<T> {
    type Columns = Array<1>;

    const NULL: FieldColumns<Self, NullType> = [NullType::Binary];

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        [Value::Binary(Cow::Owned(
            serde_json::to_vec(&self.0).unwrap(),
        ))] // TODO propagate error?
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        [Value::Binary(Cow::Owned(
            serde_json::to_vec(&self.0).unwrap(),
        ))] // TODO propagate error?
    }

    fn get_imr<F: Field<Type = Self>>() -> FieldColumns<Self, imr::Field> {
        get_single_imr::<F>(imr::DbType::Binary)
    }

    type Decoder = JsonDecoder<T>;

    type GetAnnotations = forward_annotations<1>;

    type Check = shared_linter_check<1>;

    type GetNames = single_column_name;
}
impl<T: Serialize + DeserializeOwned + 'static> AsDbType for Json<T> {
    type Primitive = Vec<u8>;
    type DbType = Binary;
}

new_converting_decoder!(
    pub OptionJsonDecoder<T: Serialize + DeserializeOwned>,
    |value: Option<Vec<u8>>| -> Option<Json<T>> {
        value
            .map(|value| {
                serde_json::from_slice(&value)
                    .map(Json)
                    .map_err(|err| DecodeError(format!("Couldn't decoder json: {err}")))
            })
            .transpose()
    }
);
impl<T: Serialize + DeserializeOwned + 'static> FieldType for Option<Json<T>> {
    type Columns = Array<1>;

    const NULL: FieldColumns<Self, NullType> = [NullType::Binary];

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        self.map(Json::into_values)
            .unwrap_or([Value::Null(Binary::NULL_TYPE)])
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        self.as_ref()
            .map(Json::as_values)
            .unwrap_or([Value::Null(Binary::NULL_TYPE)])
    }

    fn get_imr<F: Field<Type = Self>>() -> FieldColumns<Self, imr::Field> {
        get_single_imr::<F>(imr::DbType::Binary)
    }

    type Decoder = OptionJsonDecoder<T>;

    type GetAnnotations = set_null_annotations;

    type Check = shared_linter_check<1>;

    type GetNames = single_column_name;
}
impl<T: Serialize + DeserializeOwned + 'static> AsDbType for Option<Json<T>> {
    type Primitive = Option<Vec<u8>>;
    type DbType = Binary;
}

// From
impl<T: Serialize + DeserializeOwned> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

// Deref
impl<T: Serialize + DeserializeOwned> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Serialize + DeserializeOwned> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// AsRef
impl<T: Serialize + DeserializeOwned> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T: Serialize + DeserializeOwned> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
