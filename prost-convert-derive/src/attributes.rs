use crate::symbol::*;
use syn::spanned::Spanned;
use syn::Meta::NameValue;
use syn::NestedMeta::Meta;
const PROST_CONVERT: &str = "prost_convert";

/// Represents struct or enum attribute information.
#[derive(Debug)]
pub struct Attributes {
    /// The path of the protobuf enum or struct equivalent.
    src: syn::Path,
    /// The path of the wrapper struct if any.
    wrapper: Option<syn::Path>,
}

impl Attributes {
    pub fn from_ast(ast: &syn::DeriveInput) -> syn::Result<Self> {
        let mut wrapper = None;
        let mut src = None;

        for attr in ast
            .attrs
            .iter()
            .map(get_prost_convert_meta_item)
            .collect::<Result<Vec<_>, _>>()? // https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
            .into_iter()
            .flatten()
        {
            match attr {
                // Parse `#[prost_convert(src = "foo")]`
                Meta(NameValue(m)) if m.path == SRC => {
                    match &m.lit {
                        syn::Lit::Str(attr_value) => {
                            let path = attr_value.parse_with(syn::Path::parse_mod_style)?;
                            src = Some(path);
                        }
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "expected `prost_convert(src = \"...\")`",
                            ))
                        }
                    };
                }
                // Parse `#[prost_convert(wrapper = "foo")]`
                Meta(NameValue(m)) if m.path == WRAPPER => {
                    match &m.lit {
                        syn::Lit::Str(attr_value) => {
                            let path = attr_value.parse_with(syn::Path::parse_mod_style)?;
                            wrapper = Some(path);
                        }
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "expected `prost_convert(src = \"...\")`",
                            ))
                        }
                    };
                }
                Meta(other) => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "expected `prost_convert(src = \"...\")`",
                    ));
                }
                syn::NestedMeta::Lit(lit) => {
                    return Err(syn::Error::new_spanned(
                        lit,
                        "unexpected literal in prost_derive field attribute",
                    ));
                }
            }
        }

        Ok(Self {
            src: src.ok_or_else(|| {
                syn::Error::new(ast.span(), "expected `prost_convert(src = \"...\")`")
            })?,
            wrapper,
        })
    }

    pub fn src(&self) -> &syn::Path {
        &self.src
    }

    pub fn wrapper(&self) -> Option<&syn::Path> {
        self.wrapper.as_ref()
    }
}

/// Extract all attributes that are inside a `#[prost_convert(...)]` if the attribute is
/// "prost_convert", return an empty vec otherwise.
fn get_prost_convert_meta_item(attr: &syn::Attribute) -> syn::Result<Vec<syn::NestedMeta>> {
    if !attr.path.is_ident(PROST_CONVERT) {
        return Ok(Vec::new());
    }
    match attr.parse_meta()? {
        syn::Meta::List(meta) => Ok(meta.nested.into_iter().collect()),
        other => Err(syn::Error::new_spanned(
            other,
            "expected `prost_convert(src = \"...\")`",
        )),
    }
}
