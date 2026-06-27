use crate::source::{WorthUiArtifactDependencyGraph, WorthUiArtifactImpactMetadata};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiIncrementalInvalidationBasis {
    dependency_graph: WorthUiArtifactDependencyGraph,
    impact_metadata: WorthUiArtifactImpactMetadata,
}

impl WorthUiIncrementalInvalidationBasis {
    pub(crate) fn new(
        dependency_graph: WorthUiArtifactDependencyGraph,
        impact_metadata: WorthUiArtifactImpactMetadata,
    ) -> Self {
        Self {
            dependency_graph,
            impact_metadata,
        }
    }

    pub(crate) fn dependency_graph(&self) -> &WorthUiArtifactDependencyGraph {
        &self.dependency_graph
    }

    pub(crate) fn impact_metadata(&self) -> &WorthUiArtifactImpactMetadata {
        &self.impact_metadata
    }
}
