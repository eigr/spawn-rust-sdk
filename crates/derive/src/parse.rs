use proc_macro2::Ident;
use proc_macro_error::{abort, ResultExt};
use syn::{parse::Parse, punctuated::Punctuated, Attribute, Token};

pub fn parse_actor_attributes(attrs: &[Attribute]) -> Vec<ActorAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("spawn"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<ActorAttr, Token![,]>::parse_terminated)
                .unwrap_or_abort()
        })
        .collect()
}

pub enum ActorAttr {
    Name(Ident),
    Type(Ident),
}

impl Parse for ActorAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident_name: Ident = input.parse()?;
        let ident_name_str = ident_name.to_string();

        match ident_name_str.as_ref() {
            "name" => Ok(ActorAttr::Name(ident_name)),
            "type" => Ok(ActorAttr::Type(ident_name)),
            _ => abort!("unexpected attribute {}", ident_name_str),
        }
    }
}
