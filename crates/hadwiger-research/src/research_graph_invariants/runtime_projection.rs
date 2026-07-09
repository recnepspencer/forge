use crate::discovery_loop::{DiscoveryFrontier, ResearchEvidenceCorpus};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::catalog::HadwigerResearchInvariantCatalog;
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
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        project_reusable_negative_evidence(corpus, &mut entities, &mut relations);
        project_graph_resident_failures(corpus, &mut entities, &mut relations);
        project_frontier(frontier, &mut entities, &mut relations);
        entities.sort();
        entities.dedup();
        relations.sort();
        relations.dedup();
        let source_corpus_digest = corpus.corpus_digest().stable_token().to_string();
        let parents = vec![
            corpus.reference(),
            frontier.reference(),
            catalog.reference(),
        ];
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantRuntimeProjection,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_runtime_projection".to_string(),
            },
            parents,
            projection_payload(&source_corpus_digest, &entities, &relations),
        )?;
        Ok(Self {
            core,
            catalog,
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

fn project_reusable_negative_evidence(
    corpus: &ResearchEvidenceCorpus,
    entities: &mut Vec<ResearchGraphRuntimeEntityProjection>,
    relations: &mut Vec<ResearchGraphRuntimeRelationProjection>,
) {
    for evidence in corpus.reusable_negative_evidence() {
        let failure = format!("failure:{}", evidence.artifact_digest().stable_token());
        let negative = format!("negative:{}", evidence.reference().stable_token());
        let affected = evidence
            .parent_artifacts()
            .first()
            .map(HadwigerArtifactReference::stable_token)
            .unwrap_or_else(|| "affected:missing".to_string());
        let scope = format!("scope:{}", evidence.scope());
        let hint = format!("reactivation:{}", evidence.reactivation_or_repair_hint());
        push_failure_bundle(
            entities, relations, &failure, negative, affected, scope, hint,
        );
    }
}

fn project_graph_resident_failures(
    corpus: &ResearchEvidenceCorpus,
    entities: &mut Vec<ResearchGraphRuntimeEntityProjection>,
    relations: &mut Vec<ResearchGraphRuntimeRelationProjection>,
) {
    for failure in corpus.graph_resident_failures() {
        let failure_identity = format!("failure:{}", failure.reference().stable_token());
        let negative = format!(
            "negative:{}",
            failure.failure_basis_fingerprint().evidence_digest_token()
        );
        let affected = failure.failure_scope().affected_artifact().stable_token();
        let scope = format!("scope:{}", failure.failure_scope().stable_token());
        let hint = format!("reactivation:{}", failure.reactivation_hint());
        push_failure_bundle(
            entities,
            relations,
            &failure_identity,
            negative,
            affected,
            scope,
            hint,
        );
    }
}

fn project_frontier(
    frontier: &DiscoveryFrontier,
    entities: &mut Vec<ResearchGraphRuntimeEntityProjection>,
    relations: &mut Vec<ResearchGraphRuntimeRelationProjection>,
) {
    let frontier_identity = format!("frontier:{}", frontier.reference().stable_token());
    entities.push(ResearchGraphRuntimeEntityProjection::new(
        vocab::FRONTIER_STATE.label(),
        frontier_identity.clone(),
    ));
    let posture = "authority:support_only".to_string();
    entities.push(ResearchGraphRuntimeEntityProjection::new(
        "hadwiger.research_graph.authority_posture",
        posture.clone(),
    ));
    relations.push(ResearchGraphRuntimeRelationProjection::new(
        vocab::FRONTIER_HAS_AUTHORITY_POSTURE.label(),
        frontier_identity,
        posture,
    ));

    let readiness = frontier.scorecard().counters().query_readiness_checks();
    if readiness > 0 {
        let counter = format!("query_readiness:{readiness}");
        entities.push(ResearchGraphRuntimeEntityProjection::new(
            vocab::QUERY_READINESS_COUNTER.label(),
            counter.clone(),
        ));
        for plan in frontier.experiment_plans() {
            let plan_identity = format!("plan:{}", plan.reference().stable_token());
            entities.push(ResearchGraphRuntimeEntityProjection::new(
                vocab::EXPERIMENT_PLAN.label(),
                plan_identity.clone(),
            ));
            relations.push(ResearchGraphRuntimeRelationProjection::new(
                vocab::PLAN_HAS_QUERY_READINESS_COUNTER.label(),
                plan_identity,
                counter.clone(),
            ));
        }
    }

    for plan in frontier.experiment_plans() {
        let plan_identity = format!("plan:{}", plan.reference().stable_token());
        entities.push(ResearchGraphRuntimeEntityProjection::new(
            vocab::EXPERIMENT_PLAN.label(),
            plan_identity.clone(),
        ));
        if let Some(proof) = plan.suppression_proof() {
            let proof_identity = format!("suppression:{}", proof.reference().stable_token());
            entities.push(ResearchGraphRuntimeEntityProjection::new(
                vocab::SUPPRESSION_PROOF.label(),
                proof_identity.clone(),
            ));
            relations.push(ResearchGraphRuntimeRelationProjection::new(
                vocab::PLAN_HAS_SUPPRESSION_PROOF.label(),
                plan_identity,
                proof_identity,
            ));
        }
    }
}

fn push_failure_bundle(
    entities: &mut Vec<ResearchGraphRuntimeEntityProjection>,
    relations: &mut Vec<ResearchGraphRuntimeRelationProjection>,
    failure: &str,
    negative: String,
    affected: String,
    scope: String,
    hint: String,
) {
    entities.push(ResearchGraphRuntimeEntityProjection::new(
        vocab::FAILURE.label(),
        failure.to_string(),
    ));
    for (kind, identity, relation) in [
        (
            vocab::NEGATIVE_EVIDENCE.label(),
            negative,
            vocab::FAILURE_HAS_NEGATIVE_EVIDENCE.label(),
        ),
        (
            vocab::AFFECTED_ARTIFACT.label(),
            affected,
            vocab::FAILURE_AFFECTS_ARTIFACT.label(),
        ),
        (
            vocab::FAILURE_SCOPE.label(),
            scope,
            vocab::FAILURE_HAS_SCOPE.label(),
        ),
        (
            vocab::REACTIVATION_HINT.label(),
            hint,
            vocab::FAILURE_HAS_REACTIVATION_HINT.label(),
        ),
    ] {
        entities.push(ResearchGraphRuntimeEntityProjection::new(
            kind,
            identity.clone(),
        ));
        relations.push(ResearchGraphRuntimeRelationProjection::new(
            relation,
            failure.to_string(),
            identity,
        ));
    }
}

fn projection_payload(
    source_corpus_digest: &str,
    entities: &[ResearchGraphRuntimeEntityProjection],
    relations: &[ResearchGraphRuntimeRelationProjection],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "WORTH.hadwiger.research_graph.runtime_projection.v1",
        ),
        HadwigerArtifactPayloadEntry::text("source_corpus_digest", source_corpus_digest),
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
