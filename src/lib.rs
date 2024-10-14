#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod expand;
mod utils;

/// The main macro of this crate.
/// Use it to define default values of fields in structs you [`Serialize`] or [`Deserialize`].
/// You do not need to create a extra function to provide the default value, as it is the case in serdes' implementation of default (`#[serde(default = "...")]`).
///
/// Set this macro on a struct where you use [`Serialize`] or [`Deserialize`] and use `#[serde_inline_default(...)]` on the field you want to have a inline default value.
/// Replace the `...` with the value you want and it will be set as default if serde needs it.
///
/// Note that you must set this macro _before_ `#[derive(Serialize)]` / `#[derive(Deserialize)]` as it wouldn't work properly if set after the derive.
///
/// # Examples
///
/// ```rust
/// #[serde_inline_default]
/// #[derive(Deserialize)]
/// struct Test {
///     #[serde_inline_default(42)]
///     value: u32
/// }
/// ```
///
/// [`Serialize`]: https://docs.rs/serde/*/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
#[proc_macro_attribute]
pub fn serde_inline_default(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(s) => expand::expand_struct(s),
        _ => panic!("can only be used on structs"),
    }
}
