mod append;
mod counters;
mod denials;
mod evidence;
mod execution_observation;
mod physical_access;
mod reopen;
mod replay_artifact;
mod reports;
mod requests;
mod root_publication;
mod runtime_receipt;
mod scan;
mod shortcut_rejection;
mod state;
pub(crate) mod storage;
mod storage_reference_index;
mod storage_segment_occupancy;
mod storage_support;
#[cfg(test)]
mod tests;
mod vocabulary;

pub use counters::PlatformPhysicalFacadeCounterSnapshot;
pub use denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
pub use evidence::PlatformPhysicalFacadeEvidence;
pub use execution_observation::{
    PlatformPhysicalDegradedExactScanReady, PlatformPhysicalDegradedExecutionObservation,
    PlatformPhysicalOperationAdmissionDenial, PlatformPhysicalRootPublicationObservation,
    PlatformPhysicalRootPublicationReady,
};
pub use replay_artifact::PlatformPhysicalReplayArtifact;
pub use reports::{
    PlatformPhysicalAppendReport, PlatformPhysicalFramedRecord,
    PlatformPhysicalRootPublicationReport, PlatformPhysicalRuntimeLayoutReport,
    PlatformPhysicalScanReport,
};
pub use requests::{
    PhysicalStoreIdentity, PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest,
    PlatformPhysicalRecordTarget,
};
pub use runtime_receipt::{
    PlatformPhysicalDegradedExactScanReceipt, PlatformPhysicalHiddenScanDenialReceipt,
    PlatformPhysicalLayoutAccessIntent, PlatformPhysicalLayoutAccessRequest,
    PlatformPhysicalRuntimeOperation, PlatformPhysicalRuntimeOutcome,
    PlatformPhysicalRuntimeReceipt, PlatformPhysicalRuntimeReceiptDenial,
    PlatformPhysicalRuntimeStrategy,
};
pub use state::PlatformPhysicalFacade;
pub use vocabulary::{PlatformPhysicalFacadeOperation, PlatformPhysicalFacadeVocabulary};
