use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Field, GenericArgument, Ident, PathArguments, Type};

pub(crate) const ATTR_NAME: &str = "serde_inline_default";
pub(crate) const DEFAULT_FN_PREFIX: &str = "__serde_inline_default";

pub(crate) fn type_lifetimes_to_static(ty: &mut Type) {
    match ty {
        Type::Array(array) => type_lifetimes_to_static(array.elem.as_mut()),
        Type::Group(group) => type_lifetimes_to_static(&mut group.elem),
        Type::Path(path) => {
            for segment in &mut path.path.segments {
                match &mut segment.arguments {
                    PathArguments::None => (),
                    PathArguments::AngleBracketed(angle_bracketed) => {
                        for arg in &mut angle_bracketed.args {
                            match arg {
                                GenericArgument::Lifetime(lifetime) => {
                                    *lifetime = parse_quote!('static);
                                }
                                GenericArgument::Type(ty) => type_lifetimes_to_static(ty),
                                _ => (),
                            }
                        }
                    }
                    PathArguments::Parenthesized(parenthesized) => {
                        for input in &mut parenthesized.inputs {
                            type_lifetimes_to_static(input)
                        }
                    }
                }
            }
        }
        Type::Ptr(ptr) => type_lifetimes_to_static(&mut ptr.elem),
        Type::Reference(reference) => reference.lifetime = Some(parse_quote!('static)),
        Type::Slice(slice) => type_lifetimes_to_static(&mut slice.elem),
        Type::Tuple(tuple) => {
            for elem in &mut tuple.elems {
                type_lifetimes_to_static(elem)
            }
        }
        _ => (),
    }
}

pub(crate) fn check_field_for_default_expr(
    field: &mut Field,
    identifier_fn: impl FnOnce() -> String,
) -> Option<TokenStream> {
    for (i, attr) in field.attrs.iter_mut().enumerate() {
        if !attr.path().is_ident(ATTR_NAME) {
            continue;
        }

        let default_expr: TokenStream = attr.parse_args().unwrap();

        // copy all the same #[cfg] conditional compilations flags for the field onto our built
        // default function.
        // otherwise, it's possible to create a constructor for a type that may be filtered by
        // the same #[cfg]'s, breaking compilation
        let cfg_attrs = field.attrs.iter().filter(|a| a.path().is_ident("cfg"));

        let default_fn_lit = identifier_fn();
        let default_fn_ident = Ident::new(&default_fn_lit, Span::call_site());
        let mut return_type = field.ty.clone();

        // replace lifetimes with 'static.
        // the built default function / default values in general can only be static as they're
        // generated without reference to the parent struct
        type_lifetimes_to_static(&mut return_type);

        let default_fn_expr = quote! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            #( #cfg_attrs )*
            fn #default_fn_ident () -> #return_type {
                #default_expr
            }
        };

        field.attrs[i] = parse_quote!( #[serde(default = #default_fn_lit)] );
        return Some(default_fn_expr);
    }

    None
}
