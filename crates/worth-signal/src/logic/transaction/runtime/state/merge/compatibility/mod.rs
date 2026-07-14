mod denial;
mod facts;
mod readmission;
mod witness;

pub use denial::{SignalMergeCompatibilityDenial, SignalMergeCompatibilityDenialKind};
pub use facts::SignalMergeCompatibilityFactInventory;
pub use witness::{
    bridge_signal_merge_compatibility_trust_boundary, bridged_compatibility_posture_kind,
    compatibility_posture_kind, BoundaryBridgedSignalMergeCompatibilityArtifact,
    SignalMergeCompatibilityArtifact, SignalMergeCompatibilityAuthority,
    SignalMergeCompatibilityBasis, SignalMergeCompatibilityPostureKind,
    SignalMergeCompatibilityReadmissionAuthority, SignalMergeCompatibilityReady,
    SignalMergeCompatibilityWitness, SIGNAL_MERGE_COMPATIBILITY_SCHEMA_VERSION,
};
