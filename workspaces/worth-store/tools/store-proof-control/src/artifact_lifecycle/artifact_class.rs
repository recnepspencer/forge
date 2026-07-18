use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum BuildArtifactClass {
    CurrentReusableObject,
    StaleHashedVariant,
    IncrementalState,
    Symbol,
    EvidenceBundle,
    UiExpectation,
    ProcessOutput,
    DiagnosticCapture,
    UnattributedBuildArtifact,
    DisposableRootMarker,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum BuildArtifactKind {
    File,
    Directory,
}
