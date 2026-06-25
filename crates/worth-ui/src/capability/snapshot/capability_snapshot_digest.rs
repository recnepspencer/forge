use super::SnapshotMetrics;

/// Deterministic identity for a frozen capability snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilitySnapshotDigest {
    value: u64,
}

impl CapabilitySnapshotDigest {
    #[allow(dead_code)]
    pub(crate) fn from_metrics(metrics: SnapshotMetrics) -> Self {
        Self::from_metrics_and_registry_bases(
            metrics, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        )
    }

    pub(crate) fn from_metrics_and_registry_bases(
        metrics: SnapshotMetrics,
        command_basis: u64,
        command_projection_basis: u64,
        component_basis: u64,
        appearance_basis: u64,
        density_basis: u64,
        icon_basis: u64,
        image_asset_basis: u64,
        surface_basis: u64,
        mosaic_region_basis: u64,
        mosaic_placement_basis: u64,
        mosaic_sizing_basis: u64,
        mosaic_state_basis: u64,
        native_capability_basis: u64,
        plugin_slot_basis: u64,
        view_binding_basis: u64,
        runtime_outcome_projection_basis: u64,
        setting_basis: u64,
        task_presentation_basis: u64,
        theme_token_basis: u64,
    ) -> Self {
        Self {
            value: 0x9e37_79b9_7f4a_7c15
                ^ metrics.registered_family_count() as u64
                ^ ((metrics.total_width() as u64) << 32)
                ^ command_basis.rotate_left(17)
                ^ command_projection_basis.rotate_left(13)
                ^ component_basis.rotate_left(29)
                ^ appearance_basis.rotate_left(2)
                ^ density_basis.rotate_left(61)
                ^ icon_basis.rotate_left(41)
                ^ image_asset_basis.rotate_left(47)
                ^ surface_basis.rotate_left(53)
                ^ mosaic_region_basis.rotate_left(7)
                ^ mosaic_placement_basis.rotate_left(19)
                ^ mosaic_sizing_basis.rotate_left(31)
                ^ mosaic_state_basis.rotate_left(43)
                ^ native_capability_basis.rotate_left(59)
                ^ plugin_slot_basis.rotate_left(3)
                ^ view_binding_basis.rotate_left(5)
                ^ runtime_outcome_projection_basis.rotate_left(23)
                ^ setting_basis.rotate_left(37)
                ^ task_presentation_basis.rotate_left(47)
                ^ theme_token_basis.rotate_left(11),
        }
    }

    /// Stable numeric digest value for equality and later inspection.
    pub fn as_u64(self) -> u64 {
        self.value
    }
}
