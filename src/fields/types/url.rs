use std::borrow::Cow;

use rorm_db::sql::value::NullType;
use url::Url;

use crate::conditions::Value;
use crate::fields::traits::{Array, FieldColumns, FieldType};
use crate::fields::utils::check::string_check;
use crate::fields::utils::get_annotations::{forward_annotations, set_null_annotations};
use crate::fields::utils::get_names::single_column_name;
use crate::{impl_FieldEq, new_converting_decoder, Error};

impl_FieldEq!(impl<'rhs> FieldEq<'rhs, &'rhs Url> for Url {|url: &'rhs Url| Value::String(Cow::Borrowed(url.as_str()))});
impl_FieldEq!(impl<'rhs> FieldEq<'rhs, Url> for Url {|url: Url| Value::String(Cow::Owned(url.into()))});

impl FieldType for Url {
    type Columns = Array<1>;

    const NULL: FieldColumns<Self, NullType> = [NullType::String];

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        [Value::String(Cow::Owned(self.into()))]
    }

    #[inline(always)]
    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        [Value::String(Cow::Borrowed(self.as_str()))]
    }

    type Decoder = UrlDecoder;

    type GetAnnotations = forward_annotations<1>;

    type Check = string_check;

    type GetNames = single_column_name;
}
new_converting_decoder!(
    pub UrlDecoder,
    |value: String| -> Url {
        Url::parse(&value).map_err(|err| Error::DecodeError(format!("Couldn't parse url: {err}")))
    }
);

impl FieldType for Option<Url> {
    type Columns = Array<1>;

    const NULL: FieldColumns<Self, NullType> = [NullType::String];

    fn into_values(self) -> FieldColumns<Self, Value<'static>> {
        self.map(<Url>::into_values)
            .unwrap_or(Self::NULL.map(Value::Null))
    }

    fn as_values(&self) -> FieldColumns<Self, Value<'_>> {
        self.as_ref()
            .map(<Url>::as_values)
            .unwrap_or(Self::NULL.map(Value::Null))
    }

    type Decoder = OptionUrlDecoder;

    type GetAnnotations = set_null_annotations;

    type Check = string_check;

    type GetNames = single_column_name;
}
new_converting_decoder!(
    pub OptionUrlDecoder,
    |value: Option<String>| -> Option<Url> {
        value.map(|string| Url::parse(&string)).transpose().map_err(|err| Error::DecodeError(format!("Couldn't parse url: {err}")))
    }
);
