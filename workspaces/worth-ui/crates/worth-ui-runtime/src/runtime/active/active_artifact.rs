use crate::source::{WorthUiArtifact, WorthUiArtifactDigest};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveArtifact {
    artifact: Rc<WorthUiArtifact>,
    digest: WorthUiArtifactDigest,
}

impl WorthUiActiveArtifact {
    pub(crate) fn new(artifact: Rc<WorthUiArtifact>, digest: WorthUiArtifactDigest) -> Self {
        Self { artifact, digest }
    }

    pub(crate) fn digest(&self) -> WorthUiArtifactDigest {
        self.digest
    }

    pub(crate) fn artifact(&self) -> &WorthUiArtifact {
        &self.artifact
    }
}
