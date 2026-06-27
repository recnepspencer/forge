use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::candidate::{
    WorthUiCandidateAuthoringLane, WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial,
    WorthUiReplacementCause,
};
use crate::source::WorthUiArtifact;

use super::replacement_candidate_from_ingress;

pub(crate) fn rust_authored_replacement_candidate(
    artifact: WorthUiArtifact,
    snapshot_digest: CapabilitySnapshotDigest,
    cause: WorthUiReplacementCause,
) -> Result<WorthUiReplacementCandidate, WorthUiReplacementCandidateDenial> {
    replacement_candidate_from_ingress(
        artifact,
        snapshot_digest,
        cause,
        WorthUiCandidateAuthoringLane::rust_authored(),
    )
}
