use crate::capability::CapabilitySupportKind;

/// Frozen support-posture summary for capability registration evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SupportSnapshot {
    admitted_count: usize,
    deferred_count: usize,
    unsupported_count: usize,
    platform_internal_count: usize,
}

impl SupportSnapshot {
    pub fn from_support_kinds(
        support_kinds: impl IntoIterator<Item = CapabilitySupportKind>,
    ) -> Self {
        let mut snapshot = Self::empty();
        for support_kind in support_kinds {
            snapshot.record_support_kind(support_kind);
        }
        snapshot
    }

    pub fn empty() -> Self {
        Self {
            admitted_count: 0,
            deferred_count: 0,
            unsupported_count: 0,
            platform_internal_count: 0,
        }
    }

    pub fn admitted_count(self) -> usize {
        self.admitted_count
    }

    pub fn deferred_count(self) -> usize {
        self.deferred_count
    }

    pub fn unsupported_count(self) -> usize {
        self.unsupported_count
    }

    pub fn platform_internal_count(self) -> usize {
        self.platform_internal_count
    }

    pub fn total_width(self) -> usize {
        self.admitted_count
            + self.deferred_count
            + self.unsupported_count
            + self.platform_internal_count
    }

    fn record_support_kind(&mut self, support_kind: CapabilitySupportKind) {
        match support_kind {
            CapabilitySupportKind::Admitted => {
                self.admitted_count += 1;
            }
            CapabilitySupportKind::Deferred => {
                self.deferred_count += 1;
            }
            CapabilitySupportKind::Unsupported => {
                self.unsupported_count += 1;
            }
            CapabilitySupportKind::PlatformInternal => {
                self.platform_internal_count += 1;
            }
        }
    }
}
