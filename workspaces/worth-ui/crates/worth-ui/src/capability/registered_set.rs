use super::SnapshotMetrics;

/// Builder-collected Worth UI capabilities that freeze into snapshot authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredCapabilitySet {
    registered_family_count: usize,
    total_width: usize,
}

impl RegisteredCapabilitySet {
    pub(crate) fn from_counts(registered_family_count: usize, total_width: usize) -> Self {
        Self {
            registered_family_count,
            total_width,
        }
    }

    /// Whether the set contains no registered capabilities.
    pub fn is_empty(&self) -> bool {
        self.registered_family_count == 0 && self.total_width == 0
    }

    /// Total registered capability count across all families.
    pub fn total_width(&self) -> usize {
        self.total_width
    }

    /// Count of capability families represented in the set.
    pub fn registered_family_count(&self) -> usize {
        self.registered_family_count
    }

    pub(crate) fn snapshot_metrics(&self) -> SnapshotMetrics {
        SnapshotMetrics::from_registered_capabilities(self)
    }
}
