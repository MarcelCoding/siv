use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

mod derive;

#[proc_macro_derive(Response)]
pub fn response(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  derive::response(input).into()
}
