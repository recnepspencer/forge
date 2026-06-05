use std::collections::BTreeSet;

use super::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use super::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use super::query_references::HadwigerQueryDeclarationReference;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphIdentity {
    core: HadwigerArtifactCore,
    graph_id: String,
}

impl GraphIdentity {
    pub fn from_query_declaration(
        graph_id: impl Into<String>,
        source: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let graph_id = require_non_empty(graph_id, "graph_id")?;
        let core = artifact_core(
            HadwigerArtifactKind::GraphIdentity,
            HadwigerArtifactAuthorityOwner::QueryDeclaration,
            HadwigerArtifactSourceReference::QueryDeclaration(source),
            Vec::new(),
            vec![HadwigerArtifactPayloadEntry::text(
                "graph_id",
                graph_id.clone(),
            )],
        )?;
        Ok(Self { core, graph_id })
    }

    pub fn graph_id(&self) -> &str {
        &self.graph_id
    }
}

impl_hadwiger_artifact!(GraphIdentity, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexIdentity {
    core: HadwigerArtifactCore,
    vertex_label: String,
}

impl VertexIdentity {
    fn new(
        graph_reference: HadwigerArtifactReference,
        vertex_label: String,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::VertexIdentity,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "graph_version_vertex".to_string(),
            },
            vec![graph_reference],
            vec![HadwigerArtifactPayloadEntry::text(
                "vertex_label",
                vertex_label.clone(),
            )],
        )?;
        Ok(Self { core, vertex_label })
    }

    pub fn vertex_label(&self) -> &str {
        &self.vertex_label
    }
}

impl_hadwiger_artifact!(VertexIdentity, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeIdentity {
    core: HadwigerArtifactCore,
    left_vertex_label: String,
    right_vertex_label: String,
}

impl EdgeIdentity {
    fn new(
        graph_reference: HadwigerArtifactReference,
        left_vertex_label: String,
        right_vertex_label: String,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let core = artifact_core(
            HadwigerArtifactKind::EdgeIdentity,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "graph_version_edge".to_string(),
            },
            vec![graph_reference],
            vec![
                HadwigerArtifactPayloadEntry::text("left_vertex_label", left_vertex_label.clone()),
                HadwigerArtifactPayloadEntry::text(
                    "right_vertex_label",
                    right_vertex_label.clone(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            left_vertex_label,
            right_vertex_label,
        })
    }

    pub fn endpoints(&self) -> (&str, &str) {
        (&self.left_vertex_label, &self.right_vertex_label)
    }
}

impl_hadwiger_artifact!(EdgeIdentity, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphVersion {
    core: HadwigerArtifactCore,
    graph_reference: HadwigerArtifactReference,
    version_id: String,
    vertices: Vec<VertexIdentity>,
    edges: Vec<EdgeIdentity>,
}

impl GraphVersion {
    pub fn builder(
        graph_reference: HadwigerArtifactReference,
        version_id: impl Into<String>,
    ) -> GraphVersionBuilder {
        GraphVersionBuilder::new(graph_reference, version_id)
    }

    pub fn version_id(&self) -> &str {
        &self.version_id
    }

    pub fn vertices(&self) -> &[VertexIdentity] {
        &self.vertices
    }

    pub fn edges(&self) -> &[EdgeIdentity] {
        &self.edges
    }
}

impl_hadwiger_artifact!(GraphVersion, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphVersionBuilder {
    graph_reference: HadwigerArtifactReference,
    version_id: String,
    vertices: BTreeSet<String>,
    edges: BTreeSet<(String, String)>,
}

impl GraphVersionBuilder {
    fn new(graph_reference: HadwigerArtifactReference, version_id: impl Into<String>) -> Self {
        Self {
            graph_reference,
            version_id: version_id.into(),
            vertices: BTreeSet::new(),
            edges: BTreeSet::new(),
        }
    }

    pub fn with_vertex(
        mut self,
        vertex_label: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let vertex_label = require_non_empty(vertex_label, "vertex_label")?;
        if !self.vertices.insert(vertex_label.clone()) {
            return Err(HadwigerArtifactShapeError::DuplicateVertex { vertex_label });
        }
        Ok(self)
    }

    pub fn with_undirected_edge(
        mut self,
        left_vertex_label: impl Into<String>,
        right_vertex_label: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let left_vertex_label = require_non_empty(left_vertex_label, "left_vertex_label")?;
        let right_vertex_label = require_non_empty(right_vertex_label, "right_vertex_label")?;
        if left_vertex_label == right_vertex_label {
            return Err(HadwigerArtifactShapeError::SelfEdge {
                vertex_label: left_vertex_label,
            });
        }
        self.require_declared_vertex(&left_vertex_label)?;
        self.require_declared_vertex(&right_vertex_label)?;
        let edge = normalized_edge(left_vertex_label, right_vertex_label);
        self.edges.insert(edge);
        Ok(self)
    }

    pub fn finish(self) -> Result<GraphVersion, HadwigerArtifactShapeError> {
        let version_id = require_non_empty(self.version_id, "version_id")?;
        let vertices = self
            .vertices
            .iter()
            .cloned()
            .map(|vertex| VertexIdentity::new(self.graph_reference.clone(), vertex))
            .collect::<Result<Vec<_>, _>>()?;
        let edges = self
            .edges
            .iter()
            .cloned()
            .map(|(left, right)| EdgeIdentity::new(self.graph_reference.clone(), left, right))
            .collect::<Result<Vec<_>, _>>()?;
        let mut payload_entries = vec![HadwigerArtifactPayloadEntry::text(
            "version_id",
            version_id.clone(),
        )];
        payload_entries.extend(
            self.vertices
                .iter()
                .cloned()
                .map(|vertex| HadwigerArtifactPayloadEntry::text("vertex_label", vertex)),
        );
        payload_entries.extend(self.edges.iter().cloned().flat_map(|(left, right)| {
            [
                HadwigerArtifactPayloadEntry::text("edge_left_vertex_label", left),
                HadwigerArtifactPayloadEntry::text("edge_right_vertex_label", right),
            ]
        }));
        let core = artifact_core(
            HadwigerArtifactKind::GraphVersion,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "graph_version".to_string(),
            },
            vec![self.graph_reference.clone()],
            payload_entries,
        )?;
        Ok(GraphVersion {
            core,
            graph_reference: self.graph_reference,
            version_id,
            vertices,
            edges,
        })
    }

    fn require_declared_vertex(
        &self,
        vertex_label: &str,
    ) -> Result<(), HadwigerArtifactShapeError> {
        if self.vertices.contains(vertex_label) {
            Ok(())
        } else {
            Err(HadwigerArtifactShapeError::MissingEdgeEndpoint {
                vertex_label: vertex_label.to_string(),
            })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddingCandidate {
    core: HadwigerArtifactCore,
    embedding_id: String,
}

impl EmbeddingCandidate {
    pub fn new(
        graph_version_reference: HadwigerArtifactReference,
        embedding_id: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let embedding_id = require_non_empty(embedding_id, "embedding_id")?;
        let core = artifact_core(
            HadwigerArtifactKind::EmbeddingCandidate,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "embedding_candidate".to_string(),
            },
            vec![graph_version_reference],
            vec![HadwigerArtifactPayloadEntry::text(
                "embedding_id",
                embedding_id.clone(),
            )],
        )?;
        Ok(Self { core, embedding_id })
    }

    pub fn embedding_id(&self) -> &str {
        &self.embedding_id
    }
}

impl_hadwiger_artifact!(EmbeddingCandidate, core);

macro_rules! simple_parent_artifact {
    ($name:ident, $kind:ident, $operation:literal, $field:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            core: HadwigerArtifactCore,
            artifact_id: String,
        }

        impl $name {
            pub fn new(
                parent_reference: HadwigerArtifactReference,
                artifact_id: impl Into<String>,
            ) -> Result<Self, HadwigerArtifactShapeError> {
                let artifact_id = require_non_empty(artifact_id, $field)?;
                let core = artifact_core(
                    HadwigerArtifactKind::$kind,
                    HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
                    HadwigerArtifactSourceReference::ArtifactConstruction {
                        operation: $operation.to_string(),
                    },
                    vec![parent_reference],
                    vec![HadwigerArtifactPayloadEntry::text(
                        $field,
                        artifact_id.clone(),
                    )],
                )?;
                Ok(Self { core, artifact_id })
            }

            pub fn artifact_id(&self) -> &str {
                &self.artifact_id
            }
        }

        impl_hadwiger_artifact!($name, core);
    };
}

simple_parent_artifact!(
    GadgetDefinition,
    GadgetDefinition,
    "gadget_definition",
    "gadget_id"
);
simple_parent_artifact!(
    GadgetContract,
    GadgetContract,
    "gadget_contract",
    "contract_id"
);
simple_parent_artifact!(
    ReductionTrace,
    ReductionTrace,
    "reduction_trace",
    "reduction_id"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphComposition {
    core: HadwigerArtifactCore,
    composition_id: String,
}

impl GraphComposition {
    pub fn new(
        composition_id: impl Into<String>,
        parent_references: Vec<HadwigerArtifactReference>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let composition_id = require_non_empty(composition_id, "composition_id")?;
        if parent_references.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyParentArtifacts);
        }
        let core = artifact_core(
            HadwigerArtifactKind::GraphComposition,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "graph_composition".to_string(),
            },
            parent_references,
            vec![HadwigerArtifactPayloadEntry::text(
                "composition_id",
                composition_id.clone(),
            )],
        )?;
        Ok(Self {
            core,
            composition_id,
        })
    }

    pub fn composition_id(&self) -> &str {
        &self.composition_id
    }
}

impl_hadwiger_artifact!(GraphComposition, core);

fn normalized_edge(left: String, right: String) -> (String, String) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}
