use super::{SnapshotLookupCounters, SnapshotLookupReport};

/// Index view for one frozen capability family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotFamilyIndex {
    family_name: &'static str,
    family_width: usize,
}

impl SnapshotFamilyIndex {
    pub(crate) fn new(family_name: &'static str, family_width: usize) -> Self {
        Self {
            family_name,
            family_width,
        }
    }

    pub fn family_name(self) -> &'static str {
        self.family_name
    }

    pub fn family_width(self) -> usize {
        self.family_width
    }

    pub(crate) fn lookup<T>(self, value: Option<T>) -> SnapshotLookupReport<T> {
        SnapshotLookupReport::new(
            value,
            SnapshotLookupCounters::index_backed(self.family_width),
        )
    }
}
