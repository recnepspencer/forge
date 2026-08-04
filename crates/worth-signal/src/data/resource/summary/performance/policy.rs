use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn policy_compatibility(compared_width: u32, incompatible_width: u32) -> Self {
        Self::new(
            ResourceBoundaryKind::PolicyCompatibility,
            compared_width,
            0,
            compared_width.saturating_sub(incompatible_width),
            incompatible_width,
            0,
            0,
            0,
            0,
            0,
            1,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(18),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn diagnostics_expansion(
        runtime_summary_width: u32,
        replay_reconstruction_width: u32,
        branch_restore_width: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::DiagnosticsExpansion,
            runtime_summary_width
                .saturating_add(replay_reconstruction_width)
                .saturating_add(branch_restore_width),
            0,
            runtime_summary_width,
            0,
            replay_reconstruction_width,
            0,
            0,
            0,
            0,
            replay_reconstruction_width,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(16),
            ResourceCostPosture::Debt,
        )
    }

    pub(crate) fn diagnostics_expansion_denied(
        runtime_summary_width: u32,
        replay_reconstruction_width: u32,
        branch_restore_width: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::DiagnosticsExpansion,
            runtime_summary_width
                .saturating_add(replay_reconstruction_width)
                .saturating_add(branch_restore_width),
            0,
            0,
            1,
            replay_reconstruction_width,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(16),
            ResourceCostPosture::DeniedFallback,
        )
    }

    pub(crate) fn lifecycle_retention_compaction(
        selected_terminal_count: u32,
        reclaimed_in_flight_count: u32,
        retained_history_write_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::LifecycleRetentionCompaction,
            selected_terminal_count,
            0,
            reclaimed_in_flight_count,
            0,
            0,
            0,
            0,
            0,
            retained_history_write_count,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(17),
            ResourceCostPosture::Verified,
        )
    }
}
