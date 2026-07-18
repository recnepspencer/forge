use super::{UiSourceArtifactGeneration, UiSourceArtifactIdentity};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiInspectionDeclarationIdentity(u64);

impl UiInspectionDeclarationIdentity {
    pub const fn new(digest: u64) -> Self {
        Self(digest)
    }

    pub const fn digest(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAuthoredSourceProvenanceRef {
    source_artifact: UiSourceArtifactIdentity,
    source_generation: UiSourceArtifactGeneration,
    declaration_index: usize,
}

impl UiAuthoredSourceProvenanceRef {
    pub fn file_declaration(
        source_artifact: UiSourceArtifactIdentity,
        source_generation: UiSourceArtifactGeneration,
        declaration_index: usize,
    ) -> Self {
        Self {
            source_artifact,
            source_generation,
            declaration_index,
        }
    }

    pub fn source_artifact(&self) -> &UiSourceArtifactIdentity {
        &self.source_artifact
    }

    pub fn source_generation(&self) -> UiSourceArtifactGeneration {
        self.source_generation
    }

    pub fn declaration_index(&self) -> usize {
        self.declaration_index
    }
}

pub type UiInspectionAspectName = String;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionTarget {
    ProductRoot,
    DeclaredSurface {
        module_path: String,
        declaration_index: usize,
    },
    GraphNodeIdentity {
        graph_node_digest: u64,
    },
    PublishedAspect {
        aspect_name: UiInspectionAspectName,
    },
    ConsumedAspect {
        aspect_name: UiInspectionAspectName,
    },
    DeclarationIdentity {
        identity: UiInspectionDeclarationIdentity,
    },
    AuthoredSourceProvenance {
        provenance: UiAuthoredSourceProvenanceRef,
    },
    ObligationGraphNode {
        graph_node_digest: u64,
    },
    ObligationTouch {
        graph_node_digest: u64,
        touch_identity_digest: u64,
    },
    ObligationEvidenceHandle {
        handle_digest: u64,
    },
}

impl UiInspectionTarget {
    pub const fn product_root() -> Self {
        Self::ProductRoot
    }

    pub fn declared_surface(module_path: impl Into<String>, declaration_index: usize) -> Self {
        Self::DeclaredSurface {
            module_path: module_path.into(),
            declaration_index,
        }
    }

    pub const fn graph_node_identity(graph_node_digest: u64) -> Self {
        Self::GraphNodeIdentity { graph_node_digest }
    }

    pub fn published_aspect(aspect_name: impl Into<UiInspectionAspectName>) -> Self {
        Self::PublishedAspect {
            aspect_name: canonical_aspect_name(aspect_name),
        }
    }

    pub fn consumed_aspect(aspect_name: impl Into<UiInspectionAspectName>) -> Self {
        Self::ConsumedAspect {
            aspect_name: canonical_aspect_name(aspect_name),
        }
    }

    pub const fn declaration_identity(identity: UiInspectionDeclarationIdentity) -> Self {
        Self::DeclarationIdentity { identity }
    }

    pub fn authored_source_provenance(provenance: UiAuthoredSourceProvenanceRef) -> Self {
        Self::AuthoredSourceProvenance { provenance }
    }

    pub const fn obligation_graph_node(graph_node_digest: u64) -> Self {
        Self::ObligationGraphNode { graph_node_digest }
    }

    pub const fn obligation_touch(graph_node_digest: u64, touch_identity_digest: u64) -> Self {
        Self::ObligationTouch {
            graph_node_digest,
            touch_identity_digest,
        }
    }

    pub const fn obligation_evidence_handle(handle_digest: u64) -> Self {
        Self::ObligationEvidenceHandle { handle_digest }
    }
}

fn canonical_aspect_name(aspect_name: impl Into<UiInspectionAspectName>) -> UiInspectionAspectName {
    aspect_name.into().trim().to_ascii_lowercase()
}
