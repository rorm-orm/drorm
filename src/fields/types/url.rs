use std::borrow::Cow;

use rorm_declaration::imr;
use url::Url;

use crate::conditions::Value;
use crate::fields::traits::{Array, FieldColumns, FieldType};
use crate::internal::field::as_db_type::{get_single_imr, AsDbType};
use crate::internal::field::modifier::{
    forward_annotations, set_null_annotations, single_column_name, string_check,
};
use crate::internal::field::Field;
use crate::internal::hmr;
use crate::{impl_FieldEq, new_converting_decoder, Error};

impl_FieldEq!(impl<'rhs> FieldEq<'rhs, &'rhs Url> for Url {|url: &'rhs Url| Value::String(Cow::Borrowed(url.as_str()))});
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Url> for Url {|url: Url| Value::String(Cow::Owned(url.into()))});

impl FieldType for Url {
    type Columns = Array<1>;

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        [Value::String(Cow::Owned(self.into()))]
    }

    #[inline(always)]
    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        [Value::String(Cow::Borrowed(self.as_str()))]
    }

    fn get_imr<F: Field<Type = Self>>() -> FieldColumns<Self, imr::Field> {
        get_single_imr::<F>(imr::DbType::VarChar)
    }

    type Decoder = UrlDecoder;

    type GetAnnotations = forward_annotations<1>;

    type Check = string_check;

    type GetNames = single_column_name;
}
impl AsDbType for Url {
    type Primitive = String;

    type DbType = hmr::db_type::VarChar;
}
new_converting_decoder!(
    pub UrlDecoder,
    |value: String| -> Url {
        Url::parse(&value).map_err(|err| Error::DecodeError(format!("Couldn't parse url: {err}")))
    }
);

impl FieldType for Option<Url> {
    type Columns = Array<1>;

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        self.map(<Url>::into_values).unwrap_or([Value::Null(
            <<Url as AsDbType>::DbType as hmr::db_type::DbType>::NULL_TYPE,
        )])
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        self.as_ref().map(<Url>::as_values).unwrap_or([Value::Null(
            <<Url as AsDbType>::DbType as hmr::db_type::DbType>::NULL_TYPE,
        )])
    }

    fn get_imr<F: Field<Type = Self>>() -> FieldColumns<Self, imr::Field> {
        get_single_imr::<F>(imr::DbType::VarChar)
    }

    type Decoder = OptionUrlDecoder;

    type GetAnnotations = set_null_annotations<1>;

    type Check = string_check;

    type GetNames = single_column_name;
}
impl AsDbType for Option<Url> {
    type Primitive = Option<<Url as AsDbType>::Primitive>;
    type DbType = <Url as AsDbType>::DbType;

    const IMPLICIT: Option<hmr::annotations::Annotations> = {
        let mut annos = if let Some(annos) = <Url as AsDbType>::IMPLICIT {
            annos
        } else {
            hmr::annotations::Annotations::empty()
        };
        annos.nullable = true;
        Some(annos)
    };
}
new_converting_decoder!(
    pub OptionUrlDecoder,
    |value: Option<String>| -> Option<Url> {
        value.map(|string| Url::parse(&string)).transpose().map_err(|err| Error::DecodeError(format!("Couldn't parse url: {err}")))
    }
);
