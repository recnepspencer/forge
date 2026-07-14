mod builder;
mod checkpoint_envelopes;
mod checkpoint_kinds;
mod checkpoint_types;
mod persistence;

pub use builder::{EmbeddedModeBuilder, EmbeddedStoreHandle};
pub use checkpoint_envelopes::{
    EmbeddedCheckpointClassification, ExternalRuntimeCheckpointEnvelope,
    ExternalRuntimeCommitEnvelope,
};
pub use checkpoint_kinds::{
    ContainsCanonicalCommits, DerivedDurableCheckpointKind, EphemeralCheckpointKind,
    NoContainedCommits,
};
pub use checkpoint_types::{
    BasisBoundCheckpoint, BasisBoundCheckpointWitness, BasisFreeCheckpoint,
    VerifiedEmbeddedCheckpoint,
};
pub use persistence::EmbeddedCheckpointPersistenceReceipt;
