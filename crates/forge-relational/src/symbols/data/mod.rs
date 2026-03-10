use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Symbol(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolPolicy {
    Disabled,
    PreferInterned,
    RequireInterned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SymbolTableSnapshot {
    pub entries: Vec<(Symbol, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringInterner {
    next_symbol: u32,
    by_value: BTreeMap<String, Symbol>,
    by_symbol: BTreeMap<Symbol, String>,
}

impl Default for StringInterner {
    fn default() -> Self {
        Self {
            next_symbol: 1,
            by_value: BTreeMap::new(),
            by_symbol: BTreeMap::new(),
        }
    }
}

impl StringInterner {
    pub fn intern(&mut self, value: &str) -> Symbol {
        if let Some(symbol) = self.by_value.get(value) {
            return *symbol;
        }
        let symbol = Symbol(self.next_symbol);
        self.next_symbol += 1;
        self.by_value.insert(value.to_string(), symbol);
        self.by_symbol.insert(symbol, value.to_string());
        symbol
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.by_symbol.get(&symbol).map(String::as_str)
    }

    pub fn normalize(&mut self, value: InternedString) -> InternedString {
        match value {
            InternedString::Raw(raw) => InternedString::Symbol(self.intern(&raw)),
            symbol => symbol,
        }
    }

    pub fn snapshot(&self) -> SymbolTableSnapshot {
        SymbolTableSnapshot {
            entries: self
                .by_symbol
                .iter()
                .map(|(symbol, value)| (*symbol, value.clone()))
                .collect(),
        }
    }
}
