use super::{ValidatedOrderingEntry, ValidatedPredicateEntry};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidatedPredicateSet {
    entries: Vec<ValidatedPredicateEntry>,
}

impl ValidatedPredicateSet {
    pub fn entries(&self) -> &[ValidatedPredicateEntry] {
        &self.entries
    }

    pub fn digest_parts(&self) -> impl Iterator<Item = String> + '_ {
        self.entries
            .iter()
            .map(ValidatedPredicateEntry::digest_part)
    }

    pub fn from_entries(mut entries: Vec<ValidatedPredicateEntry>) -> Self {
        entries.sort();
        entries.dedup();
        Self { entries }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidatedOrderingSet {
    entries: Vec<ValidatedOrderingEntry>,
}

impl ValidatedOrderingSet {
    pub fn entries(&self) -> &[ValidatedOrderingEntry] {
        &self.entries
    }

    pub fn digest_parts(&self) -> impl Iterator<Item = String> + '_ {
        self.entries.iter().map(ValidatedOrderingEntry::digest_part)
    }

    pub fn from_entries(mut entries: Vec<ValidatedOrderingEntry>) -> Self {
        entries.sort();
        entries.dedup();
        Self { entries }
    }
}
