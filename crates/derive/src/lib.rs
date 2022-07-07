extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod derives;
mod parse;

/// Generates the `Actor` impl.
#[proc_macro_derive(Actor, attributes(spawn))]
#[proc_macro_error]
pub fn actor(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_actor(&input).into()
}
