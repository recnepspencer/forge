#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReplayUndoDeclaredSourceIdentity {
    KernelWorthWorkloadRetainedReplay,
    KernelWorthWorkloadDiagnostics,
    KernelLookupConsumedWorkloadComposition,
    KernelBooleanSplitReplayUndoBoundaryAdmission,
    SpatialEvidenceLookupConsumedWorkloadHandoff,
    SpatialEvidenceLookupPublicCloseout,
    SpatialEvidenceLookupPublicCloseoutAssemblyInput,
    TopologyDerivedInvalidationSelectedPlan,
    TopologyDerivedInvalidationExecutionReceipt,
    TopologyDerivedInvalidationMilestoneElevenSeed,
    TopologyDerivedInvalidationMilestoneElevenProductReceiptRef,
}

impl ReplayUndoDeclaredSourceIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KernelWorthWorkloadRetainedReplay => "kernel.worth_workload.retained_replay",
            Self::KernelWorthWorkloadDiagnostics => "kernel.worth_workload.diagnostics",
            Self::KernelLookupConsumedWorkloadComposition => {
                "kernel.lookup_consumed_workload_composition"
            }
            Self::KernelBooleanSplitReplayUndoBoundaryAdmission => {
                "kernel.boolean_split_replay_undo_boundary_admission"
            }
            Self::SpatialEvidenceLookupConsumedWorkloadHandoff => {
                "spatial.evidence_lookup_consumed_workload_handoff"
            }
            Self::SpatialEvidenceLookupPublicCloseout => "spatial.evidence_lookup_public_closeout",
            Self::SpatialEvidenceLookupPublicCloseoutAssemblyInput => {
                "spatial.evidence_lookup_public_closeout_assembly_input"
            }
            Self::TopologyDerivedInvalidationSelectedPlan => {
                "topology.derived_invalidation_selected_plan"
            }
            Self::TopologyDerivedInvalidationExecutionReceipt => {
                "topology.derived_invalidation_execution_receipt"
            }
            Self::TopologyDerivedInvalidationMilestoneElevenSeed => {
                "topology.derived_invalidation_milestone_eleven_seed"
            }
            Self::TopologyDerivedInvalidationMilestoneElevenProductReceiptRef => {
                "topology.derived_invalidation_milestone_eleven_product_receipt_ref"
            }
        }
    }
}
