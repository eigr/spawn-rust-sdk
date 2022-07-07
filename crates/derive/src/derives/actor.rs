use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn derive_actor(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;
    match input.data {
        Data::Struct(..) => {
            quote! {
                pub fn HelloWorld() {}
            }
        }
        _ => abort_call_site!("`#[derive(Actor)] not implemented yet"),
    }
}
