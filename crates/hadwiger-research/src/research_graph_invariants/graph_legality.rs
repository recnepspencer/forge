use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

use super::catalog::ResearchGraphInvariantFamily;
use super::graph_obligations::{
    executable_experiment_obligation, obligation_for_entity_kind, suppression_obligation,
    ResearchGraphInvariantObligationSet,
};
use super::runtime_projection::{
    ResearchGraphRuntimeEntityProjection, ResearchGraphRuntimeRelationProjection,
};
use super::runtime_vocabulary as vocab;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResearchGraphLegalityPosture {
    Enforced,
    Violated,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResearchGraphLegalityViolation {
    family: ResearchGraphInvariantFamily,
    subject_identity: String,
    missing_relation_kind: &'static str,
    detail: &'static str,
}

impl ResearchGraphLegalityViolation {
    pub(crate) fn new(
        family: ResearchGraphInvariantFamily,
        subject_identity: String,
        missing_relation_kind: &'static str,
        detail: &'static str,
    ) -> Self {
        Self {
            family,
            subject_identity,
            missing_relation_kind,
            detail,
        }
    }

    pub fn family(&self) -> ResearchGraphInvariantFamily {
        self.family
    }

    pub fn subject_identity(&self) -> &str {
        &self.subject_identity
    }

    pub fn missing_relation_kind(&self) -> &'static str {
        self.missing_relation_kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.family.as_str(),
            self.subject_identity,
            self.missing_relation_kind,
            self.detail
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResearchGraphLegalityReport {
    core: HadwigerArtifactCore,
    obligations: ResearchGraphInvariantObligationSet,
    violations: Vec<ResearchGraphLegalityViolation>,
    posture: ResearchGraphLegalityPosture,
}

impl ResearchGraphLegalityReport {
    pub(crate) fn new(
        parents: Vec<HadwigerArtifactReference>,
        entities: &[ResearchGraphRuntimeEntityProjection],
        relations: &[ResearchGraphRuntimeRelationProjection],
        rejected_evidence_requires_suppression: bool,
        query_recovery_requires_readiness: bool,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let obligations = derive_obligations(
            entities,
            rejected_evidence_requires_suppression,
            query_recovery_requires_readiness,
        );
        let mut violations = validate_shape(
            entities,
            relations,
            rejected_evidence_requires_suppression,
            query_recovery_requires_readiness,
        );
        violations.sort();
        violations.dedup();
        let posture = if violations.is_empty() {
            ResearchGraphLegalityPosture::Enforced
        } else {
            ResearchGraphLegalityPosture::Violated
        };
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphLegalityReport,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_legality_report".to_string(),
            },
            parents,
            report_payload(&obligations, &violations, posture),
        )?;
        Ok(Self {
            core,
            obligations,
            violations,
            posture,
        })
    }

    pub fn obligations(&self) -> &ResearchGraphInvariantObligationSet {
        &self.obligations
    }

    pub fn violations(&self) -> &[ResearchGraphLegalityViolation] {
        &self.violations
    }

    pub fn posture(&self) -> ResearchGraphLegalityPosture {
        self.posture
    }

    pub fn is_enforced(&self) -> bool {
        self.posture == ResearchGraphLegalityPosture::Enforced
    }
}

impl_hadwiger_artifact!(ResearchGraphLegalityReport, core);

fn derive_obligations(
    entities: &[ResearchGraphRuntimeEntityProjection],
    rejected_evidence_requires_suppression: bool,
    query_recovery_requires_readiness: bool,
) -> ResearchGraphInvariantObligationSet {
    let mut obligations = entities
        .iter()
        .filter_map(|entity| obligation_for_entity_kind(entity.kind_label()))
        .collect::<Vec<_>>();
    let has_plan = entities
        .iter()
        .any(|entity| entity.kind_label() == vocab::EXPERIMENT_PLAN.label());
    if has_plan && rejected_evidence_requires_suppression {
        obligations.push(suppression_obligation());
    }
    if has_plan && query_recovery_requires_readiness {
        obligations.push(executable_experiment_obligation());
    }
    ResearchGraphInvariantObligationSet::new(obligations)
}

fn validate_shape(
    entities: &[ResearchGraphRuntimeEntityProjection],
    relations: &[ResearchGraphRuntimeRelationProjection],
    rejected_evidence_requires_suppression: bool,
    query_recovery_requires_readiness: bool,
) -> Vec<ResearchGraphLegalityViolation> {
    let mut violations = Vec::new();
    for entity in entities {
        validate_entity(
            entity,
            relations,
            rejected_evidence_requires_suppression,
            query_recovery_requires_readiness,
            &mut violations,
        );
    }
    violations
}

fn validate_entity(
    entity: &ResearchGraphRuntimeEntityProjection,
    relations: &[ResearchGraphRuntimeRelationProjection],
    rejected_evidence_requires_suppression: bool,
    query_recovery_requires_readiness: bool,
    violations: &mut Vec<ResearchGraphLegalityViolation>,
) {
    match entity.kind_label() {
        label if label == vocab::FAILURE.label() => require_outgoing(
            entity,
            relations,
            ResearchGraphInvariantFamily::FailureResidency,
            &[
                vocab::FAILURE_HAS_NEGATIVE_EVIDENCE.label(),
                vocab::FAILURE_AFFECTS_ARTIFACT.label(),
                vocab::FAILURE_HAS_SCOPE.label(),
                vocab::FAILURE_HAS_REACTIVATION_HINT.label(),
            ],
            "failure nodes must retain negative evidence, affected artifact, scope, and reactivation hint",
            violations,
        ),
        label if label == vocab::EXPERIMENT_PLAN.label() => {
            if rejected_evidence_requires_suppression
                && !has_any_outgoing(
                    entity,
                    relations,
                    &[
                        vocab::PLAN_HAS_SUPPRESSION_PROOF.label(),
                        vocab::PLAN_HAS_REACTIVATION_CONDITION.label(),
                    ],
                )
            {
                violations.push(ResearchGraphLegalityViolation::new(
                    ResearchGraphInvariantFamily::SuppressionRelation,
                    entity.stable_identity().to_string(),
                    vocab::PLAN_HAS_SUPPRESSION_PROOF.label(),
                    "dead-end planning requires suppression proof or reactivation evidence",
                ));
            }
            if query_recovery_requires_readiness {
                require_outgoing(
                    entity,
                    relations,
                    ResearchGraphInvariantFamily::ExecutableExperimentAdmission,
                    &[vocab::PLAN_HAS_QUERY_READINESS_COUNTER.label()],
                    "query-owned recovery planning requires retained readiness evidence",
                    violations,
                );
            }
        }
        label if label == vocab::HYPOTHESIS.label() => require_outgoing(
            entity,
            relations,
            ResearchGraphInvariantFamily::HypothesisLifecycle,
            &[vocab::HYPOTHESIS_HAS_STATUS.label()],
            "hypothesis nodes must retain lifecycle status",
            violations,
        ),
        label if label == vocab::FRONTIER_STATE.label() => {
            require_outgoing(
                entity,
                relations,
                ResearchGraphInvariantFamily::BranchPromotion,
                &[vocab::FRONTIER_HAS_AUTHORITY_POSTURE.label()],
                "frontier state must retain authority posture",
                violations,
            );
            if !relations.iter().any(|relation| {
                relation.source_identity() == entity.stable_identity()
                    && relation.kind_label() == vocab::FRONTIER_HAS_AUTHORITY_POSTURE.label()
                    && relation.target_identity() == "authority:support_only"
            }) {
                violations.push(ResearchGraphLegalityViolation::new(
                    ResearchGraphInvariantFamily::BranchPromotion,
                    entity.stable_identity().to_string(),
                    vocab::FRONTIER_HAS_AUTHORITY_POSTURE.label(),
                    "frontier authority posture must remain support-only",
                ));
            }
        }
        _ => {}
    }
}

fn require_outgoing(
    entity: &ResearchGraphRuntimeEntityProjection,
    relations: &[ResearchGraphRuntimeRelationProjection],
    family: ResearchGraphInvariantFamily,
    required: &[&'static str],
    detail: &'static str,
    violations: &mut Vec<ResearchGraphLegalityViolation>,
) {
    for relation_kind in required {
        if !has_outgoing(entity, relations, relation_kind) {
            violations.push(ResearchGraphLegalityViolation::new(
                family,
                entity.stable_identity().to_string(),
                relation_kind,
                detail,
            ));
        }
    }
}

fn has_any_outgoing(
    entity: &ResearchGraphRuntimeEntityProjection,
    relations: &[ResearchGraphRuntimeRelationProjection],
    relation_kinds: &[&'static str],
) -> bool {
    relation_kinds
        .iter()
        .any(|relation_kind| has_outgoing(entity, relations, relation_kind))
}

fn has_outgoing(
    entity: &ResearchGraphRuntimeEntityProjection,
    relations: &[ResearchGraphRuntimeRelationProjection],
    relation_kind: &str,
) -> bool {
    relations.iter().any(|relation| {
        relation.source_identity() == entity.stable_identity()
            && relation.kind_label() == relation_kind
    })
}

fn report_payload(
    obligations: &ResearchGraphInvariantObligationSet,
    violations: &[ResearchGraphLegalityViolation],
    posture: ResearchGraphLegalityPosture,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.research_graph.legality.v1"),
        HadwigerArtifactPayloadEntry::text("posture", format!("{posture:?}")),
        HadwigerArtifactPayloadEntry::text("obligations", obligations.stable_token()),
    ];
    for violation in violations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "violation",
            violation.stable_token(),
        ));
    }
    payload
}
