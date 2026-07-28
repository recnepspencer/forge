#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualMountedNodeRef {
    node_receipt: u64,
    mounted_instance: u64,
    incarnation: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualGraphNodeRef(u64);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualDeclarationRef {
    diagnostic_value: u64,
    authored_semantic_name: Box<str>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualAuthoredProvenance {
    source_artifact: crate::UiSourceArtifactIdentity,
    source_generation: crate::UiSourceArtifactGeneration,
    declaration_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualEvidenceRef {
    family: crate::UiEvidenceFamily,
    authority_generation: u64,
    identity_digest: u64,
    handle_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiVisualIdentityTrace {
    mounted: UiVisualMountedNodeRef,
    graph: UiVisualGraphNodeRef,
    declaration: UiVisualDeclarationRef,
    provenance: UiVisualAuthoredProvenance,
    evidence: Box<[UiVisualEvidenceRef]>,
}

#[doc(hidden)]
pub struct UiVisualIdentityTraceInput {
    pub node_receipt: u64,
    pub mounted_instance: u64,
    pub incarnation: u64,
    pub graph_node: u64,
    pub declaration: u64,
    pub authored_semantic_name: Box<str>,
    pub source_artifact: crate::UiSourceArtifactIdentity,
    pub source_generation: crate::UiSourceArtifactGeneration,
    pub declaration_index: usize,
    pub evidence: Vec<UiVisualEvidenceRef>,
}

impl UiVisualIdentityTrace {
    #[doc(hidden)]
    pub fn from_runtime_projection(input: UiVisualIdentityTraceInput) -> Self {
        Self {
            mounted: UiVisualMountedNodeRef {
                node_receipt: input.node_receipt,
                mounted_instance: input.mounted_instance,
                incarnation: input.incarnation,
            },
            graph: UiVisualGraphNodeRef(input.graph_node),
            declaration: UiVisualDeclarationRef {
                diagnostic_value: input.declaration,
                authored_semantic_name: input.authored_semantic_name,
            },
            provenance: UiVisualAuthoredProvenance {
                source_artifact: input.source_artifact,
                source_generation: input.source_generation,
                declaration_index: input.declaration_index,
            },
            evidence: input.evidence.into_boxed_slice(),
        }
    }

    pub const fn mounted_node(&self) -> UiVisualMountedNodeRef {
        self.mounted
    }

    pub const fn graph_node(&self) -> UiVisualGraphNodeRef {
        self.graph
    }

    pub const fn declaration(&self) -> &UiVisualDeclarationRef {
        &self.declaration
    }

    pub const fn authored_provenance(&self) -> &UiVisualAuthoredProvenance {
        &self.provenance
    }

    pub fn evidence(&self) -> &[UiVisualEvidenceRef] {
        &self.evidence
    }
}

impl UiVisualMountedNodeRef {
    pub const fn node_receipt(self) -> u64 {
        self.node_receipt
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }

    pub const fn incarnation(self) -> u64 {
        self.incarnation
    }
}

impl UiVisualGraphNodeRef {
    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}

impl UiVisualDeclarationRef {
    pub const fn diagnostic_value(&self) -> u64 {
        self.diagnostic_value
    }

    pub fn authored_semantic_name(&self) -> &str {
        &self.authored_semantic_name
    }
}

impl UiVisualAuthoredProvenance {
    pub const fn source_artifact(&self) -> &crate::UiSourceArtifactIdentity {
        &self.source_artifact
    }

    pub const fn source_generation(&self) -> crate::UiSourceArtifactGeneration {
        self.source_generation
    }

    pub const fn declaration_index(&self) -> usize {
        self.declaration_index
    }
}

impl UiVisualEvidenceRef {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        family: crate::UiEvidenceFamily,
        authority_generation: u64,
        identity_digest: u64,
        handle_digest: u64,
    ) -> Self {
        Self {
            family,
            authority_generation,
            identity_digest,
            handle_digest,
        }
    }

    pub const fn family(self) -> crate::UiEvidenceFamily {
        self.family
    }

    pub const fn identity_digest(self) -> u64 {
        self.identity_digest
    }

    pub const fn handle_digest(self) -> u64 {
        self.handle_digest
    }

    pub const fn authority_generation(self) -> u64 {
        self.authority_generation
    }
}
