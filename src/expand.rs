use crate::utils::type_lifetimes_to_static;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_quote, ItemStruct};

pub(crate) fn expand_struct(mut item: ItemStruct) -> proc_macro::TokenStream {
    let mut inline_fns: Vec<TokenStream> = vec![];

    for (i, field) in item.fields.iter_mut().enumerate() {
        for (j, attr) in field.attrs.iter_mut().enumerate() {
            if !attr.path().is_ident("serde_inline_default") {
                continue;
            }

            let default: TokenStream = attr.parse_args().unwrap();

            // copy all the same #[cfg] conditional compilations flags for the field onto our built "constructor"
            // otherwise, it's possible to create a constructor for a type that may be filtered by the same #[cfg]'s, breaking compilation
            let cfg_attrs = field.attrs.iter().filter(|a| a.path().is_ident("cfg"));

            let fn_name_lit = format!("__serde_inline_default_{}_{}", item.ident, i);
            let fn_name_ident = Ident::new(&fn_name_lit, Span::call_site());
            let mut return_type = field.ty.clone();

            // replaces most lifetimes with 'static
            type_lifetimes_to_static(&mut return_type);

            inline_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #( #cfg_attrs )*
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
    }

    let expanded = quote! {
        #( #inline_fns )*

        #item
    };
    expanded.into()
}
