use crate::source::{WorthUiArtifactHandle, WorthUiRuntimeDependencyHook, WorthUiSourceModuleId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactDependencyEdgeKind {
    ModuleImport,
    MosaicMount,
    RuntimeHook,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactDependencyTarget {
    Module(WorthUiSourceModuleId),
    Artifact(WorthUiArtifactHandle),
    RuntimeHook(WorthUiRuntimeDependencyHook),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactDependencyEdge {
    source: WorthUiArtifactHandle,
    target: WorthUiArtifactDependencyTarget,
    kind: WorthUiArtifactDependencyEdgeKind,
}

impl WorthUiArtifactDependencyEdge {
    pub(crate) fn new(
        source: WorthUiArtifactHandle,
        target: WorthUiArtifactDependencyTarget,
        kind: WorthUiArtifactDependencyEdgeKind,
    ) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    pub(crate) fn source(&self) -> &WorthUiArtifactHandle {
        &self.source
    }

    pub(crate) fn target(&self) -> &WorthUiArtifactDependencyTarget {
        &self.target
    }

    pub(crate) fn kind(&self) -> WorthUiArtifactDependencyEdgeKind {
        self.kind
    }
}
