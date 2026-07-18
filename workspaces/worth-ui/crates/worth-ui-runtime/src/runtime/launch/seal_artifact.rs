use crate::runtime::active::WorthUiActiveArtifact;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigest, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalenceBasis,
};
use std::rc::Rc;

pub(crate) fn seal_launch_artifact(
    artifact: Rc<WorthUiArtifact>,
) -> (WorthUiActiveArtifact, WorthUiArtifactDigest) {
    let artifact_digest = WorthUiArtifactDigestor::digest(
        artifact.as_ref(),
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    (
        WorthUiActiveArtifact::new(artifact, artifact_digest),
        artifact_digest,
    )
}
