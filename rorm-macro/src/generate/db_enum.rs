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

                fn into_values(self) -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::conditions::Value<'static>> {
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

            fn get_imr<F: ::rorm::internal::field::Field<Type = Self>>() -> ::rorm::fields::traits::FieldColumns<Self, ::rorm::internal::imr::Field> {
                ::rorm::internal::field::as_db_type::get_single_imr::<F>(
                    <::rorm::internal::hmr::db_type::Choices as ::rorm::internal::hmr::db_type::DbType>::IMR
                )
            }

            type GetAnnotations = ::rorm::fields::utils::get_annotations::forward_annotations<1>;

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
                        _ => Err(::rorm::Error::DecodeError(format!("Invalid value '{}' for enum '{}'", value, stringify!(#ident)))),
                    }
                }
            );
            impl ::rorm::internal::field::as_db_type::AsDbType for #ident {
                type Primitive = ::rorm::db::choice::Choice;
                type DbType = ::rorm::internal::hmr::db_type::Choices;

                const IMPLICIT: Option<::rorm::internal::hmr::annotations::Annotations> = Some({
                    let mut annos = ::rorm::internal::hmr::annotations::Annotations::empty();
                    annos.choices = Some(::rorm::internal::hmr::annotations::Choices(CHOICES));
                    annos
                });
            }
            ::rorm::impl_FieldEq!(impl<'rhs> FieldEq<'rhs, #ident> for #ident {
                |value: #ident| { let [value] = <#ident as ::rorm::fields::traits::FieldType>::into_values(value); value }
            });
        };
    }
}
