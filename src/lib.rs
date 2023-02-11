use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod expand;

#[proc_macro_attribute]
pub fn serde_inline_default(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(s) => expand::expand_struct(s),
        _ => panic!("can only be used on structs"),
    }
}
