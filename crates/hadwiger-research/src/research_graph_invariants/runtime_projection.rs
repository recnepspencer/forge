use crate::discovery_loop::{DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::catalog::HadwigerResearchInvariantCatalog;
use super::graph_authoring::ResearchGraphProjectedShape;
use super::graph_legality::ResearchGraphLegalityReport;
use super::runtime_vocabulary as vocab;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResearchGraphRuntimeEntityProjection {
    kind_label: &'static str,
    stable_identity: String,
}

impl ResearchGraphRuntimeEntityProjection {
    pub(crate) fn new(kind_label: &'static str, stable_identity: String) -> Self {
        Self {
            kind_label,
            stable_identity,
        }
    }

    pub fn kind_label(&self) -> &'static str {
        self.kind_label
    }

    pub fn stable_identity(&self) -> &str {
        &self.stable_identity
    }

    pub(crate) fn stable_token(&self) -> String {
        format!("{}:{}", self.kind_label, self.stable_identity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResearchGraphRuntimeRelationProjection {
    kind_label: &'static str,
    source_identity: String,
    target_identity: String,
}

impl ResearchGraphRuntimeRelationProjection {
    pub(crate) fn new(
        kind_label: &'static str,
        source_identity: String,
        target_identity: String,
    ) -> Self {
        Self {
            kind_label,
            source_identity,
            target_identity,
        }
    }

    pub fn kind_label(&self) -> &'static str {
        self.kind_label
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}->{}",
            self.kind_label, self.source_identity, self.target_identity
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphInvariantRuntimeProjection {
    core: HadwigerArtifactCore,
    catalog: HadwigerResearchInvariantCatalog,
    legality_report: ResearchGraphLegalityReport,
    source_corpus_digest: String,
    entities: Vec<ResearchGraphRuntimeEntityProjection>,
    relations: Vec<ResearchGraphRuntimeRelationProjection>,
}

impl ResearchGraphInvariantRuntimeProjection {
    pub(crate) fn new(
        corpus: &ResearchEvidenceCorpus,
        frontier: &DiscoveryFrontier,
        catalog: HadwigerResearchInvariantCatalog,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let shape = ResearchGraphProjectedShape::from_frontier(corpus, frontier);
        let entities = shape.entities().to_vec();
        let relations = shape.relations().to_vec();
        let legality_report = catalog.legality_report().clone();
        let source_corpus_digest = corpus.corpus_digest().stable_token().to_string();
        let parents = vec![
            corpus.reference(),
            frontier.reference(),
            catalog.reference(),
            legality_report.reference(),
        ];
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantRuntimeProjection,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_runtime_projection".to_string(),
            },
            parents,
            projection_payload(
                &source_corpus_digest,
                &entities,
                &relations,
                &legality_report,
            ),
        )?;
        Ok(Self {
            core,
            catalog,
            legality_report,
            source_corpus_digest,
            entities,
            relations,
        })
    }

    pub fn catalog(&self) -> &HadwigerResearchInvariantCatalog {
        &self.catalog
    }

    pub fn source_corpus_digest(&self) -> &str {
        &self.source_corpus_digest
    }

    pub fn legality_report(&self) -> &ResearchGraphLegalityReport {
        &self.legality_report
    }

    pub fn entities(&self) -> &[ResearchGraphRuntimeEntityProjection] {
        &self.entities
    }

    pub fn relations(&self) -> &[ResearchGraphRuntimeRelationProjection] {
        &self.relations
    }

    pub fn contains_entity_kind(&self, kind_label: &str) -> bool {
        self.entities
            .iter()
            .any(|entity| entity.kind_label() == kind_label)
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(ResearchGraphInvariantRuntimeProjection, core);

fn projection_payload(
    source_corpus_digest: &str,
    entities: &[ResearchGraphRuntimeEntityProjection],
    relations: &[ResearchGraphRuntimeRelationProjection],
    legality_report: &ResearchGraphLegalityReport,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.research_graph.runtime_projection.v1",
        ),
        HadwigerArtifactPayloadEntry::text("source_corpus_digest", source_corpus_digest),
        HadwigerArtifactPayloadEntry::text(
            "legality_report",
            legality_report.artifact_digest().stable_token(),
        ),
    ];
    for kind in vocab::ENTITY_KINDS {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "entity_kind",
            kind.label(),
        ));
    }
    for kind in vocab::RELATION_KINDS {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "relation_kind",
            kind.label(),
        ));
    }
    for entity in entities {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "entity",
            entity.stable_token(),
        ));
    }
    for relation in relations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "relation",
            relation.stable_token(),
        ));
    }
    payload
}
