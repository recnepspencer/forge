use super::identity::{
    naming_row_identity, persistent_name_identity, selector_resolution_row_identity,
    subshape_signature_row_identity,
};
use super::query_evolution::PlanarBooleanSplitIdentityEvolutionRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlanarBooleanSplitNamedArtifactKind {
    SplitFragment,
    SplitVertex,
    OverlapChain,
    RetainedInterval,
    EventCause,
}

impl PlanarBooleanSplitNamedArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SplitFragment => "split_fragment",
            Self::SplitVertex => "split_vertex",
            Self::OverlapChain => "overlap_chain",
            Self::RetainedInterval => "retained_interval",
            Self::EventCause => "event_cause",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitPersistentNameRow {
    row_identity: String,
    source_edge_identity: String,
    artifact_kind: PlanarBooleanSplitNamedArtifactKind,
    artifact_identity: String,
    persistent_name_identity: String,
    identity_evolution_query_digest: String,
    identity_evolution_result_digest: String,
    identity_evolution_lineage_digest: String,
    event_cause_identities: Vec<String>,
    subshape_signature_identity: String,
}

impl PlanarBooleanSplitPersistentNameRow {
    pub(crate) fn new(
        source_edge_identity: &str,
        artifact_kind: PlanarBooleanSplitNamedArtifactKind,
        artifact_identity: &str,
        event_cause_identities: Vec<String>,
        evolution: &PlanarBooleanSplitIdentityEvolutionRow,
    ) -> Self {
        let persistent_name_identity = persistent_name_identity(
            source_edge_identity,
            artifact_kind.as_str(),
            artifact_identity,
            evolution.lineage_digest(),
        );
        let row_identity = naming_row_identity(
            source_edge_identity,
            artifact_kind.as_str(),
            artifact_identity,
            evolution.result_digest(),
        );
        let subshape_signature_identity =
            subshape_signature_row_identity(artifact_identity, evolution.lineage_digest());
        Self {
            row_identity,
            source_edge_identity: source_edge_identity.to_string(),
            artifact_kind,
            artifact_identity: artifact_identity.to_string(),
            persistent_name_identity,
            identity_evolution_query_digest: evolution.query_digest().to_string(),
            identity_evolution_result_digest: evolution.result_digest().to_string(),
            identity_evolution_lineage_digest: evolution.lineage_digest().to_string(),
            event_cause_identities,
            subshape_signature_identity,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn artifact_kind(&self) -> PlanarBooleanSplitNamedArtifactKind {
        self.artifact_kind
    }
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }
    pub fn persistent_name_identity(&self) -> &str {
        &self.persistent_name_identity
    }
    pub fn identity_evolution_query_digest(&self) -> &str {
        &self.identity_evolution_query_digest
    }
    pub fn identity_evolution_result_digest(&self) -> &str {
        &self.identity_evolution_result_digest
    }
    pub fn identity_evolution_lineage_digest(&self) -> &str {
        &self.identity_evolution_lineage_digest
    }
    pub fn event_cause_identities(&self) -> &[String] {
        &self.event_cause_identities
    }
    pub fn subshape_signature_identity(&self) -> &str {
        &self.subshape_signature_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitSelectorResolutionRow {
    row_identity: String,
    persistent_name_identity: String,
    artifact_identity: String,
    selector_basis_identity: String,
}

impl PlanarBooleanSplitSelectorResolutionRow {
    pub(crate) fn from_name_row(row: &PlanarBooleanSplitPersistentNameRow) -> Self {
        let selector_basis_identity = format!(
            "selector:{}:{}:{}",
            row.source_edge_identity(),
            row.artifact_kind().as_str(),
            row.identity_evolution_lineage_digest()
        );
        let row_identity = selector_resolution_row_identity(
            row.persistent_name_identity(),
            row.artifact_identity(),
            &selector_basis_identity,
        );
        Self {
            row_identity,
            persistent_name_identity: row.persistent_name_identity().to_string(),
            artifact_identity: row.artifact_identity().to_string(),
            selector_basis_identity,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn persistent_name_identity(&self) -> &str {
        &self.persistent_name_identity
    }
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }
    pub fn selector_basis_identity(&self) -> &str {
        &self.selector_basis_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitSubshapeSignatureRow {
    row_identity: String,
    artifact_identity: String,
    signature_basis_identity: String,
    correspondence_only: bool,
}

impl PlanarBooleanSplitSubshapeSignatureRow {
    pub(crate) fn from_name_row(row: &PlanarBooleanSplitPersistentNameRow) -> Self {
        let signature_basis_identity = format!(
            "subshape-correspondence:{}:{}:{}",
            row.source_edge_identity(),
            row.artifact_kind().as_str(),
            row.identity_evolution_result_digest()
        );
        let row_identity =
            subshape_signature_row_identity(row.artifact_identity(), &signature_basis_identity);
        Self {
            row_identity,
            artifact_identity: row.artifact_identity().to_string(),
            signature_basis_identity,
            correspondence_only: true,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }
    pub fn signature_basis_identity(&self) -> &str {
        &self.signature_basis_identity
    }
    pub fn is_correspondence_only(&self) -> bool {
        self.correspondence_only
    }
}
