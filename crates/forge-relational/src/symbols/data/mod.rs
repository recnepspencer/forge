use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Symbol(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolPolicy {
    Disabled,
    PreferInterned,
    RequireInterned,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SymbolTableSnapshot {
    pub entries: Vec<(Symbol, String)>,
}

impl SymbolTableSnapshot {
    pub fn merge_new_entries(&mut self, mut new_entries: Vec<(Symbol, String)>) {
        if new_entries.is_empty() {
            return;
        }

        new_entries.sort_by(|left, right| left.1.cmp(&right.1));
        let existing = std::mem::take(&mut self.entries);
        let mut existing = existing.into_iter().peekable();
        let mut incoming = new_entries.into_iter().peekable();
        let mut merged = Vec::new();

        while let (Some(current), Some(next)) = (existing.peek(), incoming.peek()) {
            if current.1 <= next.1 {
                merged.push(existing.next().expect("peeked existing entry"));
            } else {
                merged.push(incoming.next().expect("peeked incoming entry"));
            }
        }

        merged.extend(existing);
        merged.extend(incoming);
        self.entries = merged;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringInterner {
    next_symbol: u32,
    by_value: HashMap<String, Symbol>,
    by_symbol: BTreeMap<Symbol, String>,
}

impl Default for StringInterner {
    fn default() -> Self {
        Self {
            next_symbol: 1,
            by_value: HashMap::new(),
            by_symbol: BTreeMap::new(),
        }
    }
}

impl StringInterner {
    pub fn contains(&self, value: &str) -> bool {
        self.by_value.contains_key(value)
    }

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
        let mut entries = self
            .by_value
            .iter()
            .map(|(value, symbol)| (*symbol, value.clone()))
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| left.1.cmp(&right.1));
        SymbolTableSnapshot { entries }
    }

    pub fn restore_snapshot(&mut self, snapshot: SymbolTableSnapshot) {
        self.by_value.clear();
        self.by_symbol.clear();
        self.next_symbol = 1;
        for (symbol, value) in snapshot.entries {
            self.by_value.insert(value.clone(), symbol);
            self.by_symbol.insert(symbol, value);
            self.next_symbol = self.next_symbol.max(symbol.0 + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StringInterner, Symbol};

    #[test]
    fn snapshot_entries_are_ordered_by_string_value() {
        let mut left = StringInterner::default();
        left.intern("beta");
        left.intern("alpha");

        let mut right = StringInterner::default();
        right.intern("alpha");
        right.intern("beta");

        let left_snapshot = left.snapshot();
        let right_snapshot = right.snapshot();

        assert_eq!(
            left_snapshot
                .entries
                .iter()
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        assert_eq!(
            right_snapshot
                .entries
                .iter()
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
    }

    #[test]
    fn restore_snapshot_preserves_symbol_ids_after_canonical_snapshot_sort() {
        let mut interner = StringInterner::default();
        let beta = interner.intern("beta");
        let alpha = interner.intern("alpha");

        let snapshot = interner.snapshot();
        let mut restored = StringInterner::default();
        restored.restore_snapshot(snapshot);

        assert_eq!(restored.resolve(beta), Some("beta"));
        assert_eq!(restored.resolve(alpha), Some("alpha"));
        assert_eq!(restored.resolve(Symbol(9999)), None);
    }

    #[test]
    fn snapshot_merge_new_entries_preserves_order_without_recloning_existing_entries() {
        let mut snapshot = super::SymbolTableSnapshot {
            entries: vec![
                (Symbol(2), "beta".to_string()),
                (Symbol(4), "delta".to_string()),
            ],
        };

        snapshot.merge_new_entries(vec![
            (Symbol(1), "alpha".to_string()),
            (Symbol(3), "charlie".to_string()),
        ]);

        assert_eq!(
            snapshot
                .entries
                .iter()
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>(),
            vec!["alpha", "beta", "charlie", "delta"]
        );
    }
}
