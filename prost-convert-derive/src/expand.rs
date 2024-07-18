use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::{
    attributes::Attributes,
    container::{Container, Data, Field, Style, Variant},
};

pub fn expand_derive_prost_convert(ast: syn::DeriveInput) -> syn::Result<TokenStream> {
    let container = Container::from_ast(&ast)?;

    let native = &container.ident;
    let proto = &container.attrs.src();
    let from_native_body = from_native_body(&container);
    let try_from_proto_body = try_from_proto_body(&container);

    let mut expanded = quote!(

        impl prost_convert::FromNative<#native> for #proto {
            fn from_native(value: #native) -> Self {
                #from_native_body
            }
        }

        impl prost_convert::TryFromProto<#proto> for #native {
            fn try_from_proto(value: #proto) -> std::result::Result<Self, prost_convert::ProstConvertError> {
                #try_from_proto_body
            }
        }
    );

    if let Some(wrapper) = container.attrs.wrapper() {
        expanded.extend(wrapper_struct_impl(wrapper, &container.ident));
    }

    if let Some(variants) = container.unit_variants() {
        expanded.extend(enum_i32_convertion(variants, &container));
    }

    Ok(expanded)
}

/// If the type wrap another type (i.e. `Option<T>` or `Vec<T>`) return the inner type.
fn inner_ty<'a>(wrapper: &str, field: &'a syn::Type) -> Option<&'a syn::Type> {
    // Faudrait créer tout les cas, genre std::option::Option<T>, option::Option<T>, Option<T>
    if let syn::Type::Path(ref path) = field {
        if let Some(last_segment) = path.path.segments.last() {
            if last_segment.ident != wrapper {
                return None;
            }
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = last_segment.arguments {
                // Option must have only one argument.
                if inner_ty.args.len() != 1 {
                    return None;
                }
                let inner_ty = &inner_ty.args[0];
                if let syn::GenericArgument::Type(inner_ty) = inner_ty {
                    return Some(inner_ty);
                }
            }
        }
    }
    None
}

/// return the body of the `from_native` function.
fn from_native_body(cont: &Container) -> TokenStream {
    match &cont.data {
        Data::Enum(data) => from_native_enum(data, &cont.ident),
        Data::Struct(data) => from_native_struct(data),
    }
}

fn from_native_struct(data: &[Field]) -> TokenStream {
    let fields = data.iter().map(|field| {
        let name = &field.name;
        // If the native field is an option we don't to flat the proto one.
        if inner_ty("Option", field.ty).is_some() {
            quote!(#name: value.#name.map(|field| field.into_proto()))
        } else {
            quote!(#name: value.#name.into_proto())
        }
    });

    quote!(
        use prost_convert::IntoProto;
        Self {
            #(#fields),*
        }
    )
}

fn from_native_enum(data: &[Variant], native: &syn::Ident) -> TokenStream {
    let arm = data.iter().map(|variant| {
        let variant_ident = &variant.ident;
        // TODO: Tuple and struct might be unreachable state.
        match variant.style {
            Style::Unit => {
                quote! {
                    #native::#variant_ident => Self::#variant_ident
                }
            }
            Style::Newtype => {
                quote! {
                    #native::#variant_ident(__field0) => Self::#variant_ident(__field0.into_proto())
                }
            }
            Style::Tuple => {
                let field_names = (0..variant.fields.len())
                    .map(|i| syn::Ident::new(&format!("__field{}", i), Span::call_site()));
                let field_names2 = field_names.clone();
                quote! {
                    #native::#variant_ident(#(#field_names),*) => Self::#variant_ident(#(#field_names2),*)
                }
            }
            Style::Struct => {
                let members = variant.fields.iter().map(|f| &f.name);
                let members2 = members.clone();
                quote! {
                    #native::#variant_ident { #(#members),* } => Self::#variant_ident { #(#members2),* }
                }
            }
        }

    });

    quote!(
        use prost_convert::IntoProto;
        match value {
            #(#arm),*
        }
    )
}

/// return the body of the `try_from_proto` function.
fn try_from_proto_body(container: &Container) -> TokenStream {
    match &container.data {
        Data::Enum(data) => try_from_proto_body_enum(data, &container.attrs),
        Data::Struct(data) => try_from_proto_body_struct(data),
    }
}

fn try_from_proto_body_struct(data: &[Field]) -> TokenStream {
    let fields = data.iter().map(|field| {
        let name = &field.name;
        // If the native field is an option we don't to flat the proto one.
        if inner_ty("Option", field.ty).is_some() {
            quote!(
                #name: value
                        .#name
                        .map(|field| field.try_into_native())
                        .transpose()?
            )
        } else {
            quote!(#name: value.#name.try_into_native()?)
        }
    });

    quote!(
        use prost_convert::TryIntoNative;
        std::result::Result::Ok(Self {
            #(#fields),*
        })
    )
}

fn try_from_proto_body_enum(data: &[Variant], attrs: &Attributes) -> TokenStream {
    let variants = data.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let proto = attrs.src();

        match variant.style {
            Style::Unit => {
                quote! {
                    #proto::#variant_ident => Self::#variant_ident
                }
            }
            Style::Newtype => {
                // In case the proto field is an () and native one unit, this won't work. Native must also be ()
                quote! {
                    #proto::#variant_ident(__field0) => Self::#variant_ident(__field0.try_into_native()?)
                }
            }
            Style::Tuple => {
                let field_names = (0..variant.fields.len())
                    .map(|i| syn::Ident::new(&format!("__field{}", i), Span::call_site()));
                let field_names2 = field_names.clone();
                quote! {
                    #proto::#variant_ident(#(#field_names),*) => Self::#variant_ident(#(#field_names2),*)
                }
            }
            // Style::struct can't be generated from Prost. If the native variant is `Style::struct` we make the asumption
            // that the prost one is a `Style::newtype`. 
            Style::Struct => {
                let members = variant.fields.iter().map(|f| &f.name);
                let members2 = members.clone();
                quote! {
                    #proto::#variant_ident { #(#members),* } => Self::#variant_ident { #(#members2),* }
                }
            }
        }
    });

    quote!(

        use prost_convert::TryIntoNative;
        std::result::Result::Ok(
            match value {
                #(#variants),*
            }
        )
    )
}

// we assume that the inner struct got the same name as the Wrapped one in lower case.
fn wrapper_struct_impl(wrapper: &syn::Path, native: &syn::Ident) -> TokenStream {
    let wrapper_inner_field = ident_to_snake_case(native);
    quote!(
        impl prost_convert::TryFromProto<#wrapper> for #native {
            fn try_from_proto(value: #wrapper) -> std::result::Result<Self, prost_convert::ProstConvertError> {
                use prost_convert::TryIntoNative;
                value.#wrapper_inner_field.try_into_native()
            }
        }

        impl prost_convert::FromNative<#native> for #wrapper {
            fn from_native(value: #native) -> Self {
                use prost_convert::IntoProto;
                Self {
                    #wrapper_inner_field: value.into_proto(),
                }
            }
        }
    )
}

fn ident_to_snake_case(ident: &syn::Ident) -> syn::Ident {
    let snake_case = to_snake_case(ident.to_string().as_str());
    syn::Ident::new(&snake_case, ident.span())
}

fn to_snake_case(value: &str) -> String {
    let mut acc = String::new();
    let mut prev = '_';
    for ch in value.to_string().chars() {
        if ch.is_uppercase() && prev != '_' {
            acc.push('_');
        }
        acc.push(ch);
        prev = ch;
    }
    acc.to_lowercase()
}

fn enum_i32_convertion(variants: &[Variant], container: &Container) -> syn::Result<TokenStream> {
    let native = &container.ident;

    let from_native_arm = variants
        .iter()
        .map(|variant| &variant.ident)
        .enumerate()
        .map(|(n, ident)| (n as i32, ident))
        .map(|(n, ident)| quote!(#n => std::result::Result::Ok(#native::#ident)));

    let try_from_proto_arm = variants
        .iter()
        .map(|variant| &variant.ident)
        .enumerate()
        .map(|(n, ident)| (n as i32, ident))
        .map(|(n, ident)| quote!(#native::#ident => #n));

    // FIXME: wrong error variant. panic or add a variant to the error?
    Ok(quote!(

        impl prost_convert::FromNative<#native> for i32 {
            fn from_native(value: #native) -> Self {
                match value {
                    #(#try_from_proto_arm,)*
                }
            }
        }

        impl prost_convert::TryFromProto<i32> for #native {
            fn try_from_proto(value: i32) -> std::result::Result<Self, prost_convert::ProstConvertError> {
                match value {
                    #(#from_native_arm,)*
                    _ => std::result::Result::Err(prost_convert::ProstConvertError::MissingRequiredField),
                }
            }
        }

    ))
}

#[cfg(test)]
mod test {
    use super::to_snake_case;
    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Foo"), "foo");
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
        assert_eq!(to_snake_case("FooBarBaz"), "foo_bar_baz");
        assert_eq!(to_snake_case("FOO"), "f_o_o");
    }
}
