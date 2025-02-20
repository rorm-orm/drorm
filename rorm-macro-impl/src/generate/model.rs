use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{GenericParam, LitStr};

use crate::analyze::model::{AnalyzedField, AnalyzedModel, AnalyzedModelFieldAnnotations};
use crate::generate::patch::partially_generate_patch;
use crate::generate::utils::get_source;
use crate::generate::utils::phantom_data;
use crate::parse::annotations::{Index, NamedIndex, OnAction};

pub fn generate_model(model: &AnalyzedModel) -> TokenStream {
    let (fields_struct_ident, fields_struct) = generate_fields_struct(model);
    let value_space_impl = format_ident!("__{}_ValueSpaceImpl", model.ident);
    let field_declarations = generate_fields(model);
    let AnalyzedModel {
        vis,
        ident,
        table,
        fields,
        primary_key,
        experimental_unregistered,
        experimental_generics,
    } = model;
    let primary_struct = &fields[*primary_key].unit;
    let primary_ident = &fields[*primary_key].ident;
    let primary_type = &fields[*primary_key].ty;
    let impl_patch = partially_generate_patch(
        ident,
        ident,
        vis,
        experimental_generics,
        fields.iter().map(|field| &field.ident),
        fields.iter().map(|field| &field.ty),
    );
    let field_structs_1 = fields.iter().map(|field| &field.unit);
    let field_structs_2 = field_structs_1.clone();

    let source = get_source(ident.span());

    let (impl_generics, type_generics, where_clause) = experimental_generics.split_for_impl();
    let mut generics_with_path = model.experimental_generics.clone();
    generics_with_path
        .params
        .push(GenericParam::Type(syn::parse_quote!(P)));
    let (_, type_generics_with_path, _) = generics_with_path.split_for_impl();
    let type_generics_with_self = {
        let tokens = type_generics.to_token_stream();
        if tokens.is_empty() {
            quote! { <Self> }
        } else {
            let mut tokens: Vec<_> = tokens.into_iter().collect();
            tokens.pop();
            tokens.extend(quote! { , Self >});
            TokenStream::from_iter(tokens)
        }
    };

    let mut tokens = quote! {
        #field_declarations
        #fields_struct

        impl #impl_generics ::std::ops::Deref for #value_space_impl #type_generics #where_clause {
            type Target = <#ident #type_generics as ::rorm::Model>::Fields<#ident  #type_generics>;

            fn deref(&self) -> &Self::Target {
                ::rorm::model::ConstNew::REF
            }
        }
        impl #impl_generics ::rorm::model::Model for #ident #type_generics #where_clause {
            type Primary = #primary_struct #type_generics;

            type Fields<P: ::rorm::internal::relation_path::Path> = #fields_struct_ident #type_generics_with_path;
            const F: #fields_struct_ident #type_generics_with_self = ::rorm::model::ConstNew::NEW;
            const FIELDS: #fields_struct_ident #type_generics_with_self = ::rorm::model::ConstNew::NEW;

            const TABLE: &'static str = #table;
            const SOURCE: ::rorm::internal::hmr::Source = #source;

            fn push_fields_imr(fields: &mut Vec<::rorm::imr::Field>) {#(
                ::rorm::internal::field::push_imr::<#field_structs_1 #type_generics>(&mut *fields);
            )*}
        }

        #impl_patch
    };
    if !*experimental_unregistered {
        tokens.extend(quote! {
            const _: () = {
                #[::rorm::linkme::distributed_slice(::rorm::MODELS)]
                #[linkme(crate = ::rorm::linkme)]
                static __get_imr: fn() -> ::rorm::imr::Model = <#ident as ::rorm::model::Model>::get_imr;

                // Cross field checks
                let mut count_auto_increment = 0;
                #(
                    let mut annos_slice = <#field_structs_2 as ::rorm::internal::field::Field>::EFFECTIVE_ANNOTATIONS.as_slice();
                    while let [annos, tail @ ..] = annos_slice {
                        annos_slice = tail;
                        if annos.auto_increment.is_some() {
                            count_auto_increment += 1;
                        }
                    }
                )*
                assert!(count_auto_increment <= 1, "\"auto_increment\" can only be set once per model");
            };
        });
    }
    for (index, field) in fields.iter().enumerate() {
        let field_struct = &field.unit;
        let field_ident = &field.ident;
        let field_type = &field.ty;
        tokens.extend(quote! {
            impl #impl_generics ::rorm::model::FieldByIndex<{ #index }> for #ident #type_generics #where_clause {
                type Field = #field_struct #type_generics;
            }

            impl #impl_generics ::rorm::model::GetField<#field_struct #type_generics> for #ident #type_generics #where_clause {
                fn get_field(self) -> #field_type {
                    self.#field_ident
                }
                fn borrow_field(&self) -> &#field_type {
                    &self.#field_ident
                }
                fn borrow_field_mut(&mut self) -> &mut #field_type {
                    &mut self.#field_ident
                }
            }
        });
        if !field.annos.primary_key {
            tokens.extend(quote! {
                impl #impl_generics ::rorm::model::UpdateField<#field_struct #type_generics> for #ident #type_generics #where_clause {
                    fn update_field<'m, T>(
                        &'m mut self,
                        update: impl FnOnce(&'m #primary_type, &'m mut #field_type) -> T,
                    ) -> T {
                        update(&self.#primary_ident, &mut self.#field_ident)
                    }
                }
            });
        }
    }
    tokens
}

fn generate_fields(model: &AnalyzedModel) -> TokenStream {
    let mut tokens = TokenStream::new();
    let model_ident = &model.ident;
    for (index, field) in model.fields.iter().enumerate() {
        let AnalyzedField {
            vis: _,
            ident,
            column,
            unit,
            ty,
            annos,
        } = field;

        let source = get_source(ident.span());
        let vis = &model.vis;
        let doc = LitStr::new(
            &format!("rorm's representation of [`{model_ident}`]'s `{ident}` field",),
            ident.span(),
        );
        let annos = generate_field_annotations(annos);
        let (impl_generics, type_generics, where_clause) =
            model.experimental_generics.split_for_impl();
        let phantom_data = phantom_data(&model.experimental_generics);

        tokens.extend(quote_spanned! {ident.span()=>
            #[doc = #doc]
            #[allow(non_camel_case_types)]
            #vis struct #unit #impl_generics ( #phantom_data ) #where_clause;
            impl #impl_generics ::std::clone::Clone for #unit #type_generics #where_clause {
                fn clone(&self) -> Self {
                    *self
                }
            }
            impl #impl_generics ::std::marker::Copy for #unit #type_generics #where_clause {}
            impl #impl_generics ::rorm::internal::field::Field for #unit #type_generics #where_clause {
                type Type = #ty;
                type Model = #model_ident #type_generics;
                const INDEX: usize = #index;
                const NAME: &'static str = #column;
                const EXPLICIT_ANNOTATIONS: ::rorm::internal::hmr::annotations::Annotations = #annos;
                const SOURCE: ::rorm::internal::hmr::Source = #source;
                fn new() -> Self {
                    Self(::std::marker::PhantomData)
                }
            }
        });
        if !model.experimental_unregistered {
            tokens.extend(quote! {
                const _: () = {
                    if let Err(err) = ::rorm::internal::field::check::<#unit>() {
                        panic!("{}", err.as_str());
                    }
                };
            });
        }
    }
    tokens
}

fn generate_field_annotations(annos: &AnalyzedModelFieldAnnotations) -> TokenStream {
    let AnalyzedModelFieldAnnotations {
        auto_create_time,
        auto_update_time,
        auto_increment,
        primary_key,
        unique,
        on_delete,
        on_update,
        default,
        max_length,
        index,
    } = annos;

    // Convert every field into its "creation" expression
    let auto_create_time = auto_create_time.then(|| quote! {AutoCreateTime});
    let auto_update_time = auto_update_time.then(|| quote! {AutoUpdateTime});
    let auto_increment = auto_increment.then(|| quote! {AutoIncrement});
    let primary_key = primary_key.then(|| quote! {PrimaryKey});
    let unique = unique.then(|| quote! {Unique});
    let max_length = max_length.as_ref().map(|len| quote! {MaxLength(#len)});
    let default = default.as_ref().map(|default| {
        let variant = Ident::new(default.variant, default.literal.span());
        let literal = &default.literal;
        quote! {DefaultValue(::rorm::internal::hmr::annotations::DefaultValueData::#variant(#literal))}
    });
    let index = index.as_ref().map(|Index(index)| {
        match index {
            None => {
                quote! {Index(None)}
            }

            Some(NamedIndex {
                     name,
                     priority: None,
                 }) => {
                quote! { Index(Some(::rorm::internal::hmr::annotations::IndexData { name: #name, priority: None })) }
            }

            Some(NamedIndex {
                     name,
                     priority: Some(priority),
                 }) => {
                quote! { Index(Some(::rorm::internal::hmr::annotations::IndexData { name: #name, priority: Some(#priority) })) }
            }
        }
    });
    let on_delete = on_delete
        .as_ref()
        .map(|OnAction(token)| quote! {OnDelete::#token});
    let on_update = on_update
        .as_ref()
        .map(|OnAction(token)| quote! {OnUpdate::#token});

    // Unwrap all options
    // Add absolute path
    let finalize = |token: Option<TokenStream>| {
        if let Some(token) = token {
            quote! {Some(::rorm::internal::hmr::annotations::#token)}
        } else {
            quote! {None}
        }
    };
    let auto_create_time = finalize(auto_create_time);
    let auto_update_time = finalize(auto_update_time);
    let auto_increment = finalize(auto_increment);
    let default = finalize(default);
    let index = finalize(index);
    let max_length = finalize(max_length);
    let on_delete = finalize(on_delete);
    let on_update = finalize(on_update);
    let primary_key = finalize(primary_key);
    let unique = finalize(unique);

    quote! {
        ::rorm::internal::hmr::annotations::Annotations {
            auto_create_time: #auto_create_time,
            auto_update_time: #auto_update_time,
            auto_increment: #auto_increment,
            choices: None, // Set implicitly by type
            default: #default,
            index: #index,
            max_length: #max_length,
            on_delete: #on_delete,
            on_update: #on_update,
            primary_key: #primary_key,
            unique: #unique,
            nullable: false, // Set implicitly by type
            foreign: None,   //
        }
    }
}

fn generate_fields_struct(model: &AnalyzedModel) -> (Ident, TokenStream) {
    let vis = &model.vis;
    let ident = format_ident!("__{}_Fields_Struct", model.ident);
    let doc = LitStr::new(
        &format!(
            "[`{}`]'s [`Fields`](::rorm::model::Model::Fields) struct.",
            model.ident
        ),
        Span::call_site(),
    );

    let fields_vis = model.fields.iter().map(|field| &field.vis);
    let fields_doc = model.fields.iter().map(|field| {
        LitStr::new(
            &format!("[`{}`]'s `{}` field", model.ident, field.ident),
            field.ident.span(),
        )
    });
    let fields_ident_1 = model.fields.iter().map(|field| &field.ident);
    let fields_ident_2 = fields_ident_1.clone();
    let fields_type = model.fields.iter().map(|field| &field.unit);

    let mut generics = model.experimental_generics.clone();
    generics.params.push(GenericParam::Type(
        syn::parse_quote!(Path: ::rorm::internal::relation_path::Path),
    ));
    let (impl_generics_with_path, type_generics_with_path, _) = generics.split_for_impl();
    let (_, type_generics, where_clause) = model.experimental_generics.split_for_impl();

    let tokens = quote! {
        #[doc = #doc]
        #[allow(non_camel_case_types)]
        #vis struct #ident #impl_generics_with_path #where_clause {
            #(
                #[doc = #fields_doc]
                #fields_vis #fields_ident_1: ::rorm::fields::proxy::FieldProxy<(#fields_type #type_generics, Path)>,
            )*
        }
        impl #impl_generics_with_path ::rorm::model::ConstNew for #ident #type_generics_with_path #where_clause {
            const NEW: Self = Self {
                #(
                    #fields_ident_2: ::rorm::fields::proxy::new(),
                )*
            };
            const REF: &'static Self = &Self::NEW;
        }
    };
    (ident, tokens)
}
