use proc_macro::TokenStream;
use proc_macro2::Span;
use syn;
use quote::quote;

#[proc_macro_derive(Hashable)]
pub fn hashable_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => {
            &fields.named
        },
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fields),
            ..
        }) => {
            &fields.unnamed
        },
        _ => panic!("Expected a struct"),
    };
    let fields: Vec<syn::Ident> = fields.iter().enumerate().map(|(i, f)| f.ident.clone().unwrap_or_else(|| syn::Ident::new(&i.to_string(), Span::call_site()))).collect();
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_code: Vec<_> = fields.into_iter().map(|f| {
        quote!{
            self.#f.update(h);
        }
    }).collect();
    let ret = quote! {
       impl #impl_generics simple_hash::Hashable for #name #ty_generics #where_clause {
           fn update<H: Hasher>(&self, h: &mut H) {
               #(#fields_code)*
           }
       }
    };

    ret.into()
}
