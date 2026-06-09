use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{GraphIdentity, GraphVersion, HadwigerQueryDeclarationReference};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierGraphSeedArtifact {
    core: HadwigerArtifactCore,
    seed_id: String,
    source_family: String,
    source_url: String,
    source_digest: String,
    vertex_count: usize,
    edge_count: usize,
    algebraic_embedding_certificate: Option<String>,
}

impl FrontierGraphSeedArtifact {
    pub(crate) fn checked(
        graph_version_reference: HadwigerArtifactReference,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        seed_id: impl Into<String>,
        source_family: impl Into<String>,
        source_url: impl Into<String>,
        source_digest: impl Into<String>,
        vertex_count: usize,
        edge_count: usize,
        algebraic_embedding_certificate: Option<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let seed_id = require_non_empty(seed_id, "seed_id")?;
        let source_family = require_non_empty(source_family, "source_family")?;
        let source_url = require_non_empty(source_url, "source_url")?;
        let source_digest = require_non_empty(source_digest, "source_digest")?;
        let mut payload_entries = vec![
            HadwigerArtifactPayloadEntry::text("seed_id", seed_id.clone()),
            HadwigerArtifactPayloadEntry::text("source_family", source_family.clone()),
            HadwigerArtifactPayloadEntry::text("source_url", source_url.clone()),
            HadwigerArtifactPayloadEntry::text("source_digest", source_digest.clone()),
            HadwigerArtifactPayloadEntry::text(
                "query_declaration_reference",
                query_declaration_reference.stable_token(),
            ),
            HadwigerArtifactPayloadEntry::unsigned("vertex_count", vertex_count as u128),
            HadwigerArtifactPayloadEntry::unsigned("edge_count", edge_count as u128),
        ];
        if let Some(certificate) = &algebraic_embedding_certificate {
            payload_entries.push(HadwigerArtifactPayloadEntry::text(
                "algebraic_embedding_certificate",
                certificate,
            ));
        }
        let core = artifact_core(
            HadwigerArtifactKind::FrontierGraphSeedArtifact,
            HadwigerArtifactAuthorityOwner::QueryDeclaration,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            vec![graph_version_reference],
            payload_entries,
        )?;
        Ok(Self {
            core,
            seed_id,
            source_family,
            source_url,
            source_digest,
            vertex_count,
            edge_count,
            algebraic_embedding_certificate,
        })
    }

    pub fn seed_id(&self) -> &str {
        &self.seed_id
    }

    pub fn source_family(&self) -> &str {
        &self.source_family
    }

    pub fn source_url(&self) -> &str {
        &self.source_url
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn algebraic_embedding_certificate(&self) -> Option<&str> {
        self.algebraic_embedding_certificate.as_deref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(FrontierGraphSeedArtifact, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierGraphSeedImportReport {
    graph_identity: GraphIdentity,
    graph_version: GraphVersion,
    seed_artifact: FrontierGraphSeedArtifact,
}

impl FrontierGraphSeedImportReport {
    pub(crate) fn new(
        graph_identity: GraphIdentity,
        graph_version: GraphVersion,
        seed_artifact: FrontierGraphSeedArtifact,
    ) -> Self {
        Self {
            graph_identity,
            graph_version,
            seed_artifact,
        }
    }

    pub fn graph_identity(&self) -> &GraphIdentity {
        &self.graph_identity
    }

    pub fn graph_version(&self) -> &GraphVersion {
        &self.graph_version
    }

    pub fn seed_artifact(&self) -> &FrontierGraphSeedArtifact {
        &self.seed_artifact
    }
}
