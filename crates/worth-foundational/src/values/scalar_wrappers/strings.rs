use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Symbol(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum InternedString {
    Raw(String),
    Symbol(Symbol),
}

impl InternedString {
    pub fn as_symbol(&self) -> Option<Symbol> {
        match self {
            Self::Raw(_) => None,
            Self::Symbol(symbol) => Some(*symbol),
        }
    }
}

impl From<&str> for InternedString {
    fn from(value: &str) -> Self {
        Self::Raw(value.to_string())
    }
}

impl From<String> for InternedString {
    fn from(value: String) -> Self {
        Self::Raw(value)
    }
}

pub type CanonicalString = InternedString;
