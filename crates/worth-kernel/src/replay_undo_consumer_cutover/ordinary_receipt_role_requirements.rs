use crate::replay_undo_inventory::{
    ReplayUndoDeclaredInputRole, ReplayUndoInventorySourceIdentity,
};

pub(crate) const ORDINARY_RECEIPT_ROLE_REQUIREMENTS: &[(
    ReplayUndoInventorySourceIdentity,
    ReplayUndoDeclaredInputRole,
)] = &[
    (
        ReplayUndoInventorySourceIdentity::KernelWorthWorkloadRetainedReplay,
        ReplayUndoDeclaredInputRole::RetainedReplayWorkloadReceipt,
    ),
    (
        ReplayUndoInventorySourceIdentity::KernelWorthWorkloadDiagnostics,
        ReplayUndoDeclaredInputRole::DiagnosticsWorkloadReceipt,
    ),
    (
        ReplayUndoInventorySourceIdentity::KernelLookupConsumedWorkloadComposition,
        ReplayUndoDeclaredInputRole::LookupConsumedWorkloadHandoff,
    ),
    (
        ReplayUndoInventorySourceIdentity::SpatialEvidenceLookupConsumedWorkloadHandoff,
        ReplayUndoDeclaredInputRole::EvidenceLookupExecutionReceipt,
    ),
    (
        ReplayUndoInventorySourceIdentity::SpatialEvidenceLookupPublicCloseout,
        ReplayUndoDeclaredInputRole::EvidenceLookupPublicCloseout,
    ),
    (
        ReplayUndoInventorySourceIdentity::SpatialEvidenceLookupPublicCloseoutAssemblyInput,
        ReplayUndoDeclaredInputRole::EvidenceLookupPublicCloseoutAssemblyInput,
    ),
    (
        ReplayUndoInventorySourceIdentity::TopologyDerivedInvalidationSelectedPlan,
        ReplayUndoDeclaredInputRole::InvalidationSelectedPlan,
    ),
    (
        ReplayUndoInventorySourceIdentity::TopologyDerivedInvalidationExecutionReceipt,
        ReplayUndoDeclaredInputRole::InvalidationExecutionReceipt,
    ),
    (
        ReplayUndoInventorySourceIdentity::TopologyDerivedInvalidationMilestoneElevenSeed,
        ReplayUndoDeclaredInputRole::InvalidationMilestoneElevenSeed,
    ),
    (
        ReplayUndoInventorySourceIdentity::TopologyDerivedInvalidationMilestoneElevenProductReceiptRef,
        ReplayUndoDeclaredInputRole::InvalidationProductReceiptRef,
    ),
];
