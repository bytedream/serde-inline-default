use crate::utils::type_lifetimes_to_static;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_quote, ItemStruct};



const SERDE_INLINE_PARENT: &str = "serde_inline_parent";
const SERDE_INLINE_DEFAULT: &str = "serde_inline_default";
pub(crate) fn expand_struct(mut item: ItemStruct) -> proc_macro::TokenStream {
    let mut inline_fns: Vec<TokenStream> = vec![];

    for (i, field) in item.fields.iter_mut().enumerate() {
        for (j, attr) in field.attrs.iter_mut().enumerate() {

           if attr.path().is_ident(SERDE_INLINE_DEFAULT) {
              let default: TokenStream = attr.parse_args().unwrap();

              let fn_name_lit = format!("__{}_{}_{}", SERDE_INLINE_DEFAULT, item.ident, i);
              let fn_name_ident = Ident::new(&fn_name_lit, Span::call_site());
              let mut return_type = field.ty.clone();

              // replaces most lifetimes with 'static
              type_lifetimes_to_static(&mut return_type);

              inline_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                fn #fn_name_ident () -> #return_type {
                    #default
                }
            });

              field.attrs.remove(j);
              field
                 .attrs
                 .insert(j, parse_quote!( #[serde(default = #fn_name_lit)] ));
              break;

           }
           else if attr.path().is_ident(SERDE_INLINE_PARENT) {

              let default: TokenStream = attr.parse_args().unwrap();

              let fn_name_lit = format!("__{}_{}_{}", SERDE_INLINE_PARENT, item.ident, i);
              let fn_name_ident = Ident::new(&fn_name_lit, Span::call_site());
              let mut return_type = field.ty.clone();

              // replaces most lifetimes with 'static
              type_lifetimes_to_static(&mut return_type);

              let parent_default_field_name = field.clone().ident.expect("missing field");
              let default_grab: TokenStream = format!("{}::default().{}", item.ident, parent_default_field_name).parse().expect("fail");

              inline_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                fn #fn_name_ident () -> #return_type {
                    #default_grab
                }
            });

              field.attrs.remove(j);
              field
                 .attrs
                 .insert(j, parse_quote!( #[serde(default = #fn_name_lit)] ));
              break;

           }
        }
    }

    let expanded = quote! {
        #( #inline_fns )*

        #item
    };
    expanded.into()
}
