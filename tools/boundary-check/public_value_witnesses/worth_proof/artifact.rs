//! Public artifact introductions through the ordinary carrier API.

pub(crate) struct WitnessPhase;
impl worth_proof::PhaseMarker for WitnessPhase {}

pub(crate) fn artifact() -> worth_proof::Artifact<WitnessPhase, u8> {
    worth_proof::Artifact::new(7_u8)
}

pub(crate) fn artifact_view(
    deliver: impl for<'a> FnOnce(worth_proof::ArtifactView<'a, WitnessPhase, u8, worth_proof::NoProofs, worth_proof::NoAssumptionBasis>),
) {
    let artifact = artifact();
    deliver(artifact.view());
}

pub(crate) fn artifact_parts(
) -> worth_proof::ArtifactParts<u8, worth_proof::NoProofs, worth_proof::NoAssumptionBasis> {
    artifact().into_parts()
}
