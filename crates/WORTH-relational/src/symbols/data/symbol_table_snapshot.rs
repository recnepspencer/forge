use serde::{Deserialize, Serialize};

use super::Symbol;

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

#[cfg(test)]
mod tests {
    use super::SymbolTableSnapshot;
    use crate::symbols::data::Symbol;

    #[test]
    fn snapshot_merge_new_entries_preserves_order_without_recloning_existing_entries() {
        let mut snapshot = SymbolTableSnapshot {
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
