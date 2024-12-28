use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parse::db_enum::ParsedDbEnum;

pub fn generate_db_enum(parsed: &ParsedDbEnum) -> TokenStream {
    let ParsedDbEnum {
        vis,
        ident,
        variants,
    } = parsed;
    let decoder = format_ident!("__{ident}_Decoder");

    quote! {
        const _: () = {
            const CHOICES: &'static [&'static str] = &[
                #(stringify!(#variants)),*
            ];

            impl ::rorm::fields::traits::FieldType for #ident {
                type Columns = ::rorm::fields::traits::Array<1>;

                const NULL: ::rorm::fields::traits::FieldColumns<Self, ::rorm::db::sql::value::NullType> = [
                    ::rorm::db::sql::value::NullType::String
                ];

                fn into_values<'a>(self) -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::conditions::Value<'a>> {
                    [::rorm::conditions::Value::Choice(::std::borrow::Cow::Borrowed(match self {
                        #(
                            Self::#variants => stringify!(#variants),
                        )*
                    }))]
                }

                fn as_values(&self) -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::conditions::Value<'_>> {
                    [::rorm::conditions::Value::Choice(::std::borrow::Cow::Borrowed(match self {
                        #(
                            Self::#variants => stringify!(#variants),
                        )*
                    }))]
                }

                type Decoder = #decoder;

                type GetAnnotations = get_db_enum_annotations;

                type Check = ::rorm::fields::utils::check::shared_linter_check<1>;

                type GetNames = ::rorm::fields::utils::get_names::single_column_name;
            }
            ::rorm::new_converting_decoder!(
                #[doc(hidden)]
                #vis #decoder,
                |value: ::rorm::db::choice::Choice| -> #ident {
                    let value: String = value.0;
                    match value.as_str() {
                        #(
                            stringify!(#variants) => Ok(#ident::#variants),
                        )*
                        _ => Err(format!("Invalid value '{}' for enum '{}'", value, stringify!(#ident))),
                    }
                }
            );
            ::rorm::impl_FieldEq!(impl<'rhs> FieldEq<'rhs, #ident> for #ident {
                |value: #ident| { let [value] = <#ident as ::rorm::fields::traits::FieldType>::into_values(value); value }
            });

            ::rorm::const_fn! {
                pub fn get_db_enum_annotations(
                    field: ::rorm::internal::hmr::annotations::Annotations
                ) -> [::rorm::internal::hmr::annotations::Annotations; 1] {
                    let mut field = field;
                    field.choices = Some(::rorm::internal::hmr::annotations::Choices(CHOICES));
                    [field]
                }
            }
        };
    }
}
