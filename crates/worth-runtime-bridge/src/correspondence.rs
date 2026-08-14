pub(crate) mod admission;
mod admission_outcome;
mod admitted_truth_identity;
mod batch_admission;
mod delivery;
mod delivery_outcome;
mod delivery_preflight;
mod installed_witness;
mod mapping_admission;
mod rebuild_report;
mod registration;
mod resolution;
mod semantic_delivery_match;
mod semantic_dependency_candidate;
mod semantic_dependency_registry;
mod signal_admission;
mod signal_graph_binding;
mod signal_target_declaration;
mod slot_allocation;
mod target_allocation;
mod target_mapping;

pub(crate) use batch_admission::{
    isolate_allocation_state, prepare_registered_correspondence_batch,
};
pub(crate) use mapping_admission::unique_mapping_id_for_dependency;
pub(crate) use semantic_dependency_registry::{
    AdmittedSemanticDependencyExtension, AdmittedSemanticDependencyRegistry,
};
pub(crate) use slot_allocation::SharedCorrespondenceAllocationRegistry;

pub use admission::CorrespondenceAdmissionOutcome;
pub use admission_outcome::{
    BridgeCorrespondenceAdmissionFailure, BridgeCorrespondenceDeferred, BridgeCorrespondenceDenial,
    BridgeCorrespondenceDenialKind, BridgeCorrespondenceRebindRequired, BridgeCorrespondenceStale,
    CorrespondenceAdmissionCounters,
};
pub use admitted_truth_identity::{
    BridgeAdmittedTruthCommitIdentity, BridgeAdmittedTruthRecordIdentity,
    BridgeAdmittedTruthSnapshotIdentity,
};
pub use delivery::CorrespondenceDeliveryOutcome;
pub use delivery_outcome::{
    BridgeCorrespondenceDeliveryDenial, BridgeCorrespondenceDeliveryReceipt,
    BridgeDeliveredCorrespondenceChange, BridgeDeliveredCorrespondenceChangeSet,
    CorrespondenceDeliveryCounters,
};
pub use installed_witness::{
    BridgeCorrespondenceBasis, BridgeCorrespondencePrecision, BridgeInstalledSemanticCorrespondence,
};
pub(crate) use installed_witness::{InstalledCorrespondenceTarget, ProvenCorrespondenceTargets};
pub use rebuild_report::BridgeCorrespondenceRebuildReport;
pub use registration::BridgeSemanticCorrespondenceRegistration;
pub use semantic_dependency_candidate::{
    BridgeSemanticDependencyCandidate, BridgeSemanticDependencyCandidateParts,
    BridgeSemanticLocality,
};
pub use signal_graph_binding::BridgeSignalGraphBinding;
pub use signal_target_declaration::BridgeSignalAspectTargetDeclaration;
pub(crate) use signal_target_declaration::BridgeSignalSlotRequest;
