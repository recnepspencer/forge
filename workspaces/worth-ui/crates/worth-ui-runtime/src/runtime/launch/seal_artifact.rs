use crate::runtime::active::WorthUiActiveArtifact;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigest, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
};

pub(crate) fn seal_launch_artifact(
    artifact: WorthUiArtifact,
) -> (WorthUiActiveArtifact, WorthUiArtifactDigest) {
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    (
        WorthUiActiveArtifact::new(artifact, artifact_digest),
        artifact_digest,
    )
}