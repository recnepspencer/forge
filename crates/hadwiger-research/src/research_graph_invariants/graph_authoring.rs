use crate::discovery_loop::{DiscoveryFrontier, ExperimentBatch, ResearchEvidenceCorpus};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerArtifactShapeError};

use super::graph_legality::ResearchGraphLegalityReport;
use super::runtime_projection::{
    ResearchGraphRuntimeEntityProjection, ResearchGraphRuntimeRelationProjection,
};
use super::runtime_vocabulary as vocab;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResearchGraphProjectedShape {
    entities: Vec<ResearchGraphRuntimeEntityProjection>,
    relations: Vec<ResearchGraphRuntimeRelationProjection>,
}

impl ResearchGraphProjectedShape {
    pub(crate) fn from_frontier(
        corpus: &ResearchEvidenceCorpus,
        frontier: &DiscoveryFrontier,
    ) -> Self {
        let frontier_identity = format!("frontier:{}", frontier.reference().stable_token());
        Self::from_experiment_batch(corpus, frontier.experiment_batch(), frontier_identity)
    }

    pub(crate) fn from_experiment_batch(
        corpus: &ResearchEvidenceCorpus,
        experiment_batch: &ExperimentBatch,
        frontier_identity: String,
    ) -> Self {
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        project_reusable_negative_evidence(corpus, &mut entities, &mut relations);
        project_graph_resident_failures(corpus, &mut entities, &mut relations);
        project_frontier_like_batch(
            experiment_batch,
            frontier_identity,
            &mut entities,
            &mut relations,
        );
        entities.sort();
        entities.dedup();
        relations.sort();
        relations.dedup();
        Self {
            entities,
            relations,
        }
    }

    pub(crate) fn entities(&self) -> &[ResearchGraphRuntimeEntityProjection] {
        &self.entities
    }

    pub(crate) fn relations(&self) -> &[ResearchGraphRuntimeRelationProjection] {
        &self.relations
    }
}

pub(crate) fn legality_for_experiment_batch(
    corpus: &ResearchEvidenceCorpus,
    experiment_batch: &ExperimentBatch,
) -> Result<ResearchGraphLegalityReport, HadwigerArtifactShapeError> {
    let frontier_identity = format!(
        "frontier_batch:{}",
        experiment_batch.reference().stable_token()
    );
    let shape = ResearchGraphProjectedShape::from_experiment_batch(
        corpus,
        experiment_batch,
        frontier_identity,
    );
    ResearchGraphLegalityReport::new(
        vec![corpus.reference(), experiment_batch.reference()],
        shape.entities(),
        shape.relations(),
        corpus.rejected_evidence_available(),
        corpus.has_query_recovery_evidence(),
    )
}

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

fn project_frontier_like_batch(
    experiment_batch: &ExperimentBatch,
    frontier_identity: String,
    entities: &mut Vec<ResearchGraphRuntimeEntityProjection>,
    relations: &mut Vec<ResearchGraphRuntimeRelationProjection>,
) {
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

    let readiness = experiment_batch.query_readiness_checks();
    if readiness > 0 {
        let counter = format!("query_readiness:{readiness}");
        entities.push(ResearchGraphRuntimeEntityProjection::new(
            vocab::QUERY_READINESS_COUNTER.label(),
            counter.clone(),
        ));
        for plan in experiment_batch.experiment_plans() {
            let plan_identity = format!("plan:{}", plan.reference().stable_token());
            relations.push(ResearchGraphRuntimeRelationProjection::new(
                vocab::PLAN_HAS_QUERY_READINESS_COUNTER.label(),
                plan_identity,
                counter.clone(),
            ));
        }
    }

    for plan in experiment_batch.experiment_plans() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::research_graph_invariants::ResearchGraphInvariantFamily;

    #[test]
    fn malformed_failure_shape_reports_missing_edges() {
        let entities = vec![ResearchGraphRuntimeEntityProjection::new(
            vocab::FAILURE.label(),
            "failure:manual".to_string(),
        )];
        let report = ResearchGraphLegalityReport::new(Vec::new(), &entities, &[], true, false)
            .expect("legality report should build");

        assert!(!report.is_enforced());
        assert!(report
            .obligations()
            .contains_family(ResearchGraphInvariantFamily::FailureResidency));
        assert_eq!(report.violations().len(), 4);
    }
}
