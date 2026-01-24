use quote::quote;
use syn::{spanned::Spanned, Error, Fields, ItemEnum, ItemStruct};

use crate::utils::{check_field_for_default_expr, ATTR_NAME, DEFAULT_FN_PREFIX};

pub(crate) fn expand_struct(mut item: ItemStruct) -> proc_macro::TokenStream {
    let mut default_fns = vec![];

    for (i, field) in item.fields.iter_mut().enumerate() {
        default_fns.extend(check_field_for_default_expr(field, || {
            format!("{}_{}_Field{}", DEFAULT_FN_PREFIX, item.ident, i)
        }));
    }

    quote! {
        #( #default_fns )*

        #item
    }
    .into()
}

pub(crate) fn expand_enum(mut item: ItemEnum) -> proc_macro::TokenStream {
    let mut default_fns = vec![];

    for (i, variant) in item.variants.iter_mut().enumerate() {
        if variant.attrs.iter().any(|a| a.path().is_ident(ATTR_NAME)) {
            return Error::new(
                variant.span(),
                format!(
                    "#[{}] can only be used on named enum variant fields",
                    ATTR_NAME
                ),
            )
            .to_compile_error()
            .into();
        }

        let fields = match &mut variant.fields {
            Fields::Named(fields) => fields,
            _ => {
                return Error::new(
                    variant.span(),
                    format!(
                        "#[{}] can only be used on named enum variant fields",
                        ATTR_NAME
                    ),
                )
                .to_compile_error()
                .into()
            }
        };

        for (j, field) in fields.named.iter_mut().enumerate() {
            default_fns.extend(check_field_for_default_expr(field, || {
                format!(
                    "{}_{}_Variant{}_Field{}",
                    DEFAULT_FN_PREFIX, item.ident, i, j
                )
            }));
        }
    }

    quote! {
        #( #default_fns )*

        #item
    }
    .into()
}
