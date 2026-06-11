use super::SnapshotMetrics;

/// Deterministic identity for a frozen capability snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilitySnapshotDigest {
    value: u64,
}

impl CapabilitySnapshotDigest {
    #[allow(dead_code)]
    pub(crate) fn from_metrics(metrics: SnapshotMetrics) -> Self {
        Self::from_metrics_and_registry_bases(metrics, 0, 0, 0, 0)
    }

    pub(crate) fn from_metrics_and_registry_bases(
        metrics: SnapshotMetrics,
        command_basis: u64,
        component_basis: u64,
        surface_basis: u64,
        mosaic_region_basis: u64,
    ) -> Self {
        Self {
            value: 0x9e37_79b9_7f4a_7c15
                ^ metrics.registered_family_count() as u64
                ^ ((metrics.total_width() as u64) << 32)
                ^ command_basis.rotate_left(17)
                ^ component_basis.rotate_left(29)
                ^ surface_basis.rotate_left(41)
                ^ mosaic_region_basis.rotate_left(53),
        }
    }

    /// Stable numeric digest value for equality and later inspection.
    pub fn as_u64(self) -> u64 {
        self.value
    }
}
