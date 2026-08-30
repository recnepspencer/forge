#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationPinnedBasisDenialKind {
    Cancelled,
    DeadlineExceeded,
    RuntimeSupportUnavailable,
    BasisUnavailable,
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    SnapshotIdentityExhausted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationPinnedBasisDenial {
    kind: WorthQueryApplicationPinnedBasisDenialKind,
    subject: String,
}

impl WorthQueryApplicationPinnedBasisDenial {
    pub(super) fn new(
        kind: WorthQueryApplicationPinnedBasisDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationPinnedBasisDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryApplicationPinnedBasisDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application-query pinned basis denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryApplicationPinnedBasisDenial {}
