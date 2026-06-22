use std::cmp::Ordering;

use super::identity::ForgeQueryJournalPosition;

impl Ord for ForgeQueryJournalPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.authority().cmp(&other.authority()).then_with(|| {
            self.ordinal_for_reporting()
                .cmp(&other.ordinal_for_reporting())
                .then_with(|| {
                    self.evidence_identity_ref()
                        .as_str()
                        .cmp(other.evidence_identity_ref().as_str())
                })
        })
    }
}

impl PartialOrd for ForgeQueryJournalPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
