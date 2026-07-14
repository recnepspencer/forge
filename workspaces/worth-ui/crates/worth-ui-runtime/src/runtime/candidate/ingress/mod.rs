mod worth_ui_file_authored_candidate_ingress;
mod worth_ui_rust_authored_candidate_ingress;

pub(crate) use worth_ui_file_authored_candidate_ingress::file_authored_replacement_candidate;
pub(crate) use worth_ui_rust_authored_candidate_ingress::rust_authored_replacement_candidate;

use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::candidate::{
    WorthUiCandidateArtifactBundle, WorthUiCandidateAuthoringLane, WorthUiReplacementCandidate,
    WorthUiReplacementCandidateDenial, WorthUiReplacementCause,
};
use crate::source::WorthUiArtifact;

fn replacement_candidate_from_ingress(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    cause: WorthUiReplacementCause,
    authoring_lane: WorthUiCandidateAuthoringLane,
) -> Result<WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial> {
    let bundle = WorthUiCandidateArtifactBundle::derive_and_seal(artifact, snapshot_digest)?;
    WorthUiReplacementCandidate::from_artifact_bundle(bundle, cause, authoring_lane)
}
