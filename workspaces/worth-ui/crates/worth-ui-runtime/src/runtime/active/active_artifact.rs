use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDependencyGraph,
    WorthUiArtifactDependencyReport, WorthUiArtifactDigest,
};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveArtifact {
    artifact: Rc<WorthUiArtifact>,
    digest: WorthUiArtifactDigest,
    dependency_report: Rc<WorthUiArtifactDependencyReport>,
}

impl WorthUiActiveArtifact {
    pub(crate) fn new(artifact: Rc<WorthUiArtifact>, digest: WorthUiArtifactDigest) -> Self {
        let dependency_report = Rc::new(WorthUiArtifactDependencyDeriver::derive_with_report(
            &artifact,
        ));
        Self {
            artifact,
            digest,
            dependency_report,
        }
    }

    pub(crate) fn new_with_dependency_report(
        artifact: Rc<WorthUiArtifact>,
        digest: WorthUiArtifactDigest,
        dependency_report: Rc<WorthUiArtifactDependencyReport>,
    ) -> Self {
        Self {
            artifact,
            digest,
            dependency_report,
        }
    }

    pub(crate) fn digest(&self) -> WorthUiArtifactDigest {
        self.digest
    }

    pub(crate) fn artifact(&self) -> &WorthUiArtifact {
        &self.artifact
    }

    pub(crate) fn dependency_graph(&self) -> &WorthUiArtifactDependencyGraph {
        self.dependency_report.basis().dependency_graph()
    }
}
