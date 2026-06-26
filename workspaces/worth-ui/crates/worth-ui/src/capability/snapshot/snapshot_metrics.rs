use crate::capability::RegisteredCapabilitySet;

/// Structural counters for a frozen capability snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotMetrics {
    registered_family_count: usize,
    total_width: usize,
}

impl SnapshotMetrics {
    pub(crate) fn from_registered_capabilities(
        registered_capabilities: &RegisteredCapabilitySet,
    ) -> Self {
        Self {
            registered_family_count: registered_capabilities.registered_family_count(),
            total_width: registered_capabilities.total_width(),
        }
    }

    /// Count of capability families represented in the snapshot.
    pub fn registered_family_count(self) -> usize {
        self.registered_family_count
    }

    /// Total registered capability count across all families.
    pub fn total_width(self) -> usize {
        self.total_width
    }
}
