mod admission;
mod backend_residue_rejection;
mod candidate;
mod checkpoint_base;
mod checkpoint_base_admission;
mod compaction_product;
mod compaction_visibility;
mod contiguous_wal_tail;
mod current_previous_root;
mod decision_trace;
mod discovery_trace;
mod page_facts;
mod page_lsn_skip_apply;
mod physical_source;
mod residue;
mod selection;
mod source_admission;
mod source_admission_accumulator;
mod source_application_role;
mod source_candidate;
mod source_graph;
mod source_selection;
mod wal_artifacts;
mod wal_tail;
mod wal_tail_quarantine_handoff;
mod wal_tail_redo_source;

pub use admission::admit_physical_root_slot;
pub use backend_residue_rejection::{BackendResidueKind, BackendResidueRejection};
pub use candidate::{
    PhysicalRootCandidateDenial, PhysicalRootSlotObservation, PhysicalRootSourceCandidate,
};
pub use checkpoint_base::{PhysicalCheckpointBase, PhysicalCheckpointBaseDenial};
pub use checkpoint_base_admission::CheckpointBaseAdmission;
pub use compaction_product::SelectedCompactionProduct;
pub use compaction_visibility::{
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord,
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationIdentity, CompactionGenerationVisibility,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    RecoverableOldCompactionGeneration,
};
pub use contiguous_wal_tail::ContiguousWalTailProof;
pub use current_previous_root::{
    select_current_previous_root, PhysicalRootSelectionDenial, SelectedPhysicalRoot,
    SelectedPhysicalRootRole,
};
pub(crate) use decision_trace::RecoverySourceReplayBasis;
pub use decision_trace::{
    RecoverySourceDecisionKind, RecoverySourceDecisionOutcome, RecoverySourceDecisionRow,
    RecoverySourceDecisionTrace,
};
pub use discovery_trace::RecoveryCandidateDiscoveryTrace;
pub use page_facts::{
    admit_physical_page_facts, PhysicalManifestBlockCandidate, PhysicalPageFactDenial,
    SelectedPhysicalPageFacts,
};
pub use page_lsn_skip_apply::PageLsnSkipApplyDecision;
pub use physical_source::PhysicalRecoverySource;
pub use residue::{PhysicalRecoveryResidue, PhysicalRecoveryResidueKind};
pub use selection::{
    select_physical_recovery_sources, PhysicalSourceSelection, PhysicalSourceSelectionDenial,
    PhysicalSourceSelectionTrace,
};
pub use source_admission::AdmittedRecoverySource;
pub use source_application_role::RecoverySourceApplicationRole;
pub use source_candidate::RecoverySourceCandidate;
#[cfg(feature = "certification-test-authority")]
pub use source_graph::RecoverySourcePrecedenceGraph;
#[cfg(not(feature = "certification-test-authority"))]
pub(crate) use source_graph::RecoverySourcePrecedenceGraph;
pub use wal_artifacts::{
    inspect_physical_wal_artifacts, InspectedPhysicalWalArtifacts, PhysicalWalArtifactCorruption,
    PhysicalWalArtifactInspectionDenial,
};
pub use wal_tail::{
    admit_physical_wal_tail, PhysicalWalSegmentCandidate, SelectedPhysicalWalTail,
    SelectedPhysicalWalTailDenial,
};
pub use wal_tail_quarantine_handoff::WalTailIntegrityQuarantineHandoff;
pub use wal_tail_redo_source::{WalOnlyTailProof, WalOnlyTailProofDenial, WalTailRedoSource};
