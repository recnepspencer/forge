use crate::capability::CapabilitySnapshot;
use crate::source::{
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer, WorthUiBoundArtifactInput,
    WorthUiLegallyStructuredArtifactInput, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiStructuralLegalityLowerer,
};

use super::binding_app_fixture::standard_artifact_input;

pub(super) fn legally_structured_artifact_input(
    snapshot: &CapabilitySnapshot,
) -> WorthUiLegallyStructuredArtifactInput {
    legally_structured_artifact_input_for(snapshot, &standard_artifact_input())
}

pub(super) fn legally_structured_artifact_input_for(
    snapshot: &CapabilitySnapshot,
    artifact_input: &WorthUiRustAuthoredArtifactInput,
) -> WorthUiLegallyStructuredArtifactInput {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(artifact_input);
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 structural legality should succeed")
}

pub(super) fn bound_artifact_input(snapshot: &CapabilitySnapshot) -> WorthUiBoundArtifactInput {
    bound_artifact_input_for(snapshot, &standard_artifact_input())
}

pub(super) fn bound_artifact_input_for(
    snapshot: &CapabilitySnapshot,
    artifact_input: &WorthUiRustAuthoredArtifactInput,
) -> WorthUiBoundArtifactInput {
    let legally_structured = legally_structured_artifact_input_for(snapshot, artifact_input);
    WorthUiBindingSemanticsLowerer::lower(&legally_structured, snapshot)
        .expect("phase 6 binding semantics should succeed")
}
