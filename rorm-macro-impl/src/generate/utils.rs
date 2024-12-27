use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

pub fn phantom_data(generics: &Generics) -> TokenStream {
    let mut tokens = TokenStream::new();
    tokens.extend(generics.lifetimes().map(|lifetime| {
        let lifetime = &lifetime.lifetime;
        quote! { #lifetime, }
    }));
    tokens.extend(generics.type_params().map(|parameter| {
        let parameter = &parameter.ident;
        quote! { #parameter, }
    }));
    quote! {
        ::std::marker::PhantomData<( #tokens )>
    }
}
