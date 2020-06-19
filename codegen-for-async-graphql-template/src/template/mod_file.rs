use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::Save;

struct Mod;
impl Save for Mod {}

fn generate_token_stream(names: &Vec<String>) -> TokenStream {
    let mut src = quote!();
    names.iter().for_each(|f| {
        let name = Ident::new(f, Span::call_site());
        src = quote!(
          #src
          pub mod #name;
        )
    });
    src
}

pub fn generate_file(names: &Vec<String>) {
    let src = generate_token_stream(names);
    let name = "mod".to_string();
    Mod::save(&name, &src.to_string());
}