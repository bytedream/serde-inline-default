use syn::{parse_quote, GenericArgument, PathArguments, Type};

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
