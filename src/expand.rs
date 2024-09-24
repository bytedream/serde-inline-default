use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_quote, ItemStruct};

pub(crate) fn expand_struct(mut item: ItemStruct) -> proc_macro::TokenStream {
    let mut inline_fns: Vec<(String, TokenStream, TokenStream)> = vec![];

    for (i, field) in item.fields.iter_mut().enumerate() {
        for (j, attr) in field.attrs.iter_mut().enumerate() {
            if !attr.path().is_ident("serde_inline_default") {
                continue;
            }

            let default: TokenStream = attr.parse_args().unwrap();

            // we check here if a function with the exact same return value already exists. if so,
            // this function gets used.
            let fn_name_lit = if let Some((fn_name_lit, _, _)) = inline_fns
                .iter()
                .find(|(_, def, _)| def.to_string() == default.to_string())
            {
                fn_name_lit.clone()
            } else {
                let fn_name_lit = format!("__serde_inline_default_{}_{}", item.ident, i);
                let fn_name_ident = Ident::new(&fn_name_lit, Span::call_site());
                let return_type = &field.ty;

                let inline_fn = quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    fn #fn_name_ident () -> #return_type {
                        #default
                    }
                };
                inline_fns.push((fn_name_lit.clone(), default, inline_fn));
                fn_name_lit
            };
            field.attrs.remove(j);
            field
                .attrs
                .insert(j, parse_quote!( #[serde(default = #fn_name_lit)] ));
            break;
        }
    }

    let real_inline_fns: Vec<TokenStream> =
        inline_fns.into_iter().map(|(_, _, func)| func).collect();
    let expanded = quote! {
        #( #real_inline_fns )*

        #item
    };
    expanded.into()
}
