/// Coordination metadata derived from the number of authoritative postings
/// currently attached to one account.
///
/// This value carries no monetary meaning. It exists only so a mutation can
/// retain and atomically compare the exact account-journal dependency used by
/// its invariant decision.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountJournalRevision(u64);

impl AccountJournalRevision {
    pub const fn from_posting_count(count: u64) -> Self {
        Self(count)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub const fn next(self) -> Option<Self> {
        self.advance_by(1)
    }

    pub const fn advance_by(self, posting_count: u64) -> Option<Self> {
        match self.0.checked_add(posting_count) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}
