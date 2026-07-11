mod backend_residue_rejection;
mod checkpoint_base_admission;
mod compaction_visibility;
mod decision_trace;
mod discovery_trace;
mod page_lsn_skip_apply;
mod physical_source;
mod source_admission;
mod source_admission_accumulator;
mod source_application_role;
mod source_candidate;
mod source_graph;
mod source_selection;
mod wal_tail_quarantine_handoff;
mod wal_tail_redo_source;

pub use backend_residue_rejection::{BackendResidueKind, BackendResidueRejection};
pub use checkpoint_base_admission::CheckpointBaseAdmission;
pub use compaction_visibility::{
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord,
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationIdentity, CompactionGenerationVisibility,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    RecoverableOldCompactionGeneration,
};
pub(crate) use decision_trace::RecoverySourceReplayBasis;
pub use decision_trace::{
    RecoverySourceDecisionKind, RecoverySourceDecisionOutcome, RecoverySourceDecisionRow,
    RecoverySourceDecisionTrace,
};
pub use discovery_trace::RecoveryCandidateDiscoveryTrace;
pub use page_lsn_skip_apply::PageLsnSkipApplyDecision;
pub use physical_source::PhysicalRecoverySource;
pub use source_admission::AdmittedRecoverySource;
pub use source_application_role::RecoverySourceApplicationRole;
pub use source_candidate::RecoverySourceCandidate;
#[cfg(feature = "certification-test-authority")]
pub use source_graph::RecoverySourcePrecedenceGraph;
#[cfg(not(feature = "certification-test-authority"))]
pub(crate) use source_graph::RecoverySourcePrecedenceGraph;
pub use wal_tail_quarantine_handoff::WalTailIntegrityQuarantineHandoff;
pub use wal_tail_redo_source::{WalOnlyTailProof, WalOnlyTailProofDenial, WalTailRedoSource};
