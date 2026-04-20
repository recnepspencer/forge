use serde::Serialize;

pub trait EmbeddedCheckpointKindMarker: crate::modes::embedded::checkpoint_envelopes::sealed::Sealed {
    const CLASSIFICATION: crate::modes::embedded::checkpoint_envelopes::EmbeddedCheckpointClassification;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DerivedDurableCheckpointKind;

impl EmbeddedCheckpointKindMarker for DerivedDurableCheckpointKind {
    const CLASSIFICATION: crate::modes::embedded::checkpoint_envelopes::EmbeddedCheckpointClassification =
        crate::modes::embedded::checkpoint_envelopes::EmbeddedCheckpointClassification::DerivedDurable;
}

impl crate::modes::embedded::checkpoint_envelopes::sealed::Sealed for DerivedDurableCheckpointKind {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct EphemeralCheckpointKind;

impl EmbeddedCheckpointKindMarker for EphemeralCheckpointKind {
    const CLASSIFICATION: crate::modes::embedded::checkpoint_envelopes::EmbeddedCheckpointClassification =
        crate::modes::embedded::checkpoint_envelopes::EmbeddedCheckpointClassification::Ephemeral;
}

impl crate::modes::embedded::checkpoint_envelopes::sealed::Sealed for EphemeralCheckpointKind {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct NoContainedCommits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ContainsCanonicalCommits;
