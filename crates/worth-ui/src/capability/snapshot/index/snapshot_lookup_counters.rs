/// Structural lookup cost proof returned by snapshot index queries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotLookupCounters {
    family_width: usize,
    families_scanned: usize,
}

impl SnapshotLookupCounters {
    pub(crate) fn index_backed(family_width: usize) -> Self {
        Self {
            family_width,
            families_scanned: 0,
        }
    }

    pub fn family_width(self) -> usize {
        self.family_width
    }

    pub fn families_scanned(self) -> usize {
        self.families_scanned
    }
}
