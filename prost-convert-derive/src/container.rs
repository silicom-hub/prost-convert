use syn::{punctuated::Punctuated, spanned::Spanned, Token};

use crate::attributes::Attributes;

/// A source data structure annotated with `#[derive(ProstConvert)]`,
/// parsed into an internal representation.
pub struct Container<'a> {
    /// The struct or enum name (without generics).
    pub ident: syn::Ident,
    /// Attributes on the structure, parsed for `prost_derive`.
    pub attrs: Attributes,
    /// The contents of the struct or enum.
    pub data: Data<'a>,
    /// Original input.
    pub original: &'a syn::DeriveInput,
}

/// The fields of a struct or enum.
///
/// Analogous to `syn::Data`.
pub enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Vec<Field<'a>>),
}

/// A variant of an enum.
#[derive(Debug)]
pub struct Variant<'a> {
    pub ident: syn::Ident,
    pub style: Style,
    pub fields: Vec<Field<'a>>,
    pub original: &'a syn::Variant,
}

/// A field of a struct.
#[derive(Debug)]
pub struct Field<'a> {
    // TODO: check if unammed fields does't break the macro.
    pub name: Option<syn::Ident>,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
}

#[derive(Copy, Clone, Debug)]
pub enum Style {
    /// Named fields.
    Struct,
    /// Many unnamed fields.
    Tuple,
    /// One unnamed field.
    Newtype,
    /// No fields.
    Unit,
}

impl<'a> Container<'a> {
    pub fn from_ast(ast: &'a syn::DeriveInput) -> syn::Result<Self> {
        let data = match &ast.data {
            syn::Data::Struct(data) => Data::Struct(struct_from_ast(&data.fields).1),
            syn::Data::Enum(data) => Data::Enum(enum_from_ast(&data.variants)),
            syn::Data::Union(_) => {
                return Err(syn::Error::new(
                    ast.span(),
                    "ProstConvert does not support derive for unions",
                ));
            }
        };

        Ok(Container {
            ident: ast.ident.clone(),
            attrs: Attributes::from_ast(ast)?,
            data,
            original: ast,
        })
    }

    /// Return a boolean indicating if the container is an enum and its fields are [`Unit`].
    /// For intance
    /// ```rust
    /// enum Os {
    ///     Linux,
    ///     Windows,
    /// }
    /// ```
    /// return true.
    ///
    /// [`Unit`]: https://docs.rs/syn/latest/syn/enum.Fields.html
    pub fn unit_variants(&self) -> Option<&[Variant]> {
        if let Data::Enum(ref variants) = self.data {
            if let Some(variant) = variants.first() {
                if matches!(variant.style, Style::Unit) {
                    return Some(variants);
                }
            }
        }
        None
    }
}

fn struct_from_ast(fields: &syn::Fields) -> (Style, Vec<Field<'_>>) {
    match fields {
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(&fields.named)),
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            (Style::Newtype, fields_from_ast(&fields.unnamed))
        }
        syn::Fields::Unnamed(fields) => (Style::Tuple, fields_from_ast(&fields.unnamed)),
        syn::Fields::Unit => (Style::Unit, Vec::new()),
    }
}

fn enum_from_ast(variants: &Punctuated<syn::Variant, Token![,]>) -> Vec<Variant<'_>> {
    variants
        .iter()
        .map(|variant| {
            let (style, fields) = struct_from_ast(&variant.fields);
            Variant {
                ident: variant.ident.clone(),
                fields,
                style,
                original: variant,
            }
        })
        .collect()
}

fn fields_from_ast(fields: &Punctuated<syn::Field, Token![,]>) -> Vec<Field<'_>> {
    fields
        .iter()
        .map(|field| Field {
            name: field.ident.clone(),
            ty: &field.ty,
            original: field,
        })
        .collect()
}
