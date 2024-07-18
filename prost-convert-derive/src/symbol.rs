use std::fmt::{self, Display};
use syn::{Ident, Path};

// Directly from serde <https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals/symbol.rs>
// Allow to convenient comparaison to `Path` or `Ident`.

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const SRC: Symbol = Symbol("src");
pub const WRAPPER: Symbol = Symbol("wrapper");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
