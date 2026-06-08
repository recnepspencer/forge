use crate::identity::hash_parts;

use super::inventory::{CausalEvidenceFamily, CausalEvidenceOwner};
use super::receipt_types::{CausalInspectionReason, CausalObservationOutcome};
use super::request::{
    CausalInspectionExplanationFamily, CausalInspectionRequest, CausalInspectionRichness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionAdmissionDecisionKind {
    Success,
    Advisory,
    Violation,
}

impl CausalInspectionAdmissionDecisionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Advisory => "advisory",
            Self::Violation => "violation",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionAdvisoryKind {
    MaterializedDetailDeferredUntilBridgeEnvelope,
}

impl CausalInspectionAdvisoryKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MaterializedDetailDeferredUntilBridgeEnvelope => {
                "materialized_detail_deferred_until_bridge_envelope"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionViolationKind {
    UnsupportedExplanationFamily,
}

impl CausalInspectionViolationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedExplanationFamily => "unsupported_explanation_family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionAdmissionSubject {
    request_digest: String,
    anchor_digest: String,
    anchor_counter_snapshot: String,
    anchor_reference_family_count: usize,
    lower_runtime_evidence_family_count: usize,
    query_digest: String,
    query_observation_digest: String,
    inspection_reason: CausalInspectionReason,
    observation_outcome: CausalObservationOutcome,
    reference_set_digest: String,
    resolved_reference_count: usize,
    missing_reference_family_count: usize,
    resolved_evidence_families: Vec<CausalEvidenceFamily>,
    observation_target_digest: String,
    result_shape_context_digest: String,
    target_digest: String,
    explanation_family: CausalInspectionExplanationFamily,
    requested_richness: CausalInspectionRichness,
    requested_evidence_families: Vec<CausalEvidenceFamily>,
    subject_digest: String,
}

impl CausalInspectionAdmissionSubject {
    pub(super) fn from_request(request: &CausalInspectionRequest) -> Self {
        let requested_evidence_families = request.requested_evidence_families().to_vec();
        let family_part = requested_evidence_families
            .iter()
            .map(CausalEvidenceFamily::as_str)
            .collect::<Vec<_>>()
            .join("|");
        let anchor_digest = request
            .reference_set()
            .anchor()
            .anchor_digest()
            .as_str()
            .to_string();
        let observation_receipt = request.reference_set().anchor().observation_receipt();
        let inspection_reason = request.reference_set().anchor().inspection_reason();
        let observation_outcome = observation_receipt.outcome();
        let anchor_counter_snapshot = request
            .reference_set()
            .anchor()
            .counters()
            .counter_snapshot()
            .to_string();
        let anchor_reference_family_count = request
            .reference_set()
            .anchor()
            .counters()
            .reference_family_count();
        let lower_runtime_evidence_family_count = request
            .reference_set()
            .references()
            .iter()
            .filter(|reference| reference.owner() != CausalEvidenceOwner::Query)
            .map(|reference| reference.family())
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        let query_digest = observation_receipt.query_digest().to_string();
        let query_observation_digest = observation_receipt.observation_receipt_digest().to_string();
        let reference_set_digest = request
            .reference_set()
            .reference_set_digest()
            .as_str()
            .to_string();
        let resolved_reference_count = request.reference_set().receipt().resolved_reference_count();
        let missing_reference_family_count = request
            .reference_set()
            .receipt()
            .missing_reference_family_count();
        let resolved_evidence_families = request
            .reference_set()
            .references()
            .iter()
            .map(|reference| reference.family())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let resolved_family_part = resolved_evidence_families
            .iter()
            .map(CausalEvidenceFamily::as_str)
            .collect::<Vec<_>>()
            .join("|");
        let observation_target_digest = request.target().observation_target_digest().to_string();
        let result_shape_context_digest =
            request.target().result_shape_context_digest().to_string();
        let target_digest = request.target().target_digest().to_string();
        let subject_digest = hash_parts(&[
            "causal_inspection_admission_subject_v1".to_string(),
            format!("request:{}", request.request_digest()),
            format!("anchor:{anchor_digest}"),
            format!("anchor-counters:{anchor_counter_snapshot}"),
            format!("anchor-reference-families:{anchor_reference_family_count}"),
            format!("lower-runtime-families:{lower_runtime_evidence_family_count}"),
            format!("query:{query_digest}"),
            format!("query-observation:{query_observation_digest}"),
            format!("inspection-reason:{}", inspection_reason.as_str()),
            format!("observation-outcome:{}", observation_outcome.as_str()),
            format!("reference-set:{reference_set_digest}"),
            format!("resolved-references:{resolved_reference_count}"),
            format!("missing-reference-families:{missing_reference_family_count}"),
            format!("resolved-evidence-families:{resolved_family_part}"),
            format!("observation-target:{observation_target_digest}"),
            format!("result-shape:{result_shape_context_digest}"),
            format!("target:{target_digest}"),
            format!("family:{}", request.explanation_family().as_str()),
            format!("richness:{}", request.requested_richness().as_str()),
            format!("evidence-families:{family_part}"),
        ]);
        Self {
            request_digest: request.request_digest().to_string(),
            anchor_digest,
            anchor_counter_snapshot,
            anchor_reference_family_count,
            lower_runtime_evidence_family_count,
            query_digest,
            query_observation_digest,
            inspection_reason,
            observation_outcome,
            reference_set_digest,
            resolved_reference_count,
            missing_reference_family_count,
            resolved_evidence_families,
            observation_target_digest,
            result_shape_context_digest,
            target_digest,
            explanation_family: request.explanation_family(),
            requested_richness: request.requested_richness(),
            requested_evidence_families,
            subject_digest,
        }
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn anchor_digest(&self) -> &str {
        &self.anchor_digest
    }

    pub fn anchor_counter_snapshot(&self) -> &str {
        &self.anchor_counter_snapshot
    }

    pub fn anchor_reference_family_count(&self) -> usize {
        self.anchor_reference_family_count
    }

    pub fn lower_runtime_evidence_family_count(&self) -> usize {
        self.lower_runtime_evidence_family_count
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn query_observation_digest(&self) -> &str {
        &self.query_observation_digest
    }

    pub fn inspection_reason(&self) -> CausalInspectionReason {
        self.inspection_reason
    }

    pub fn observation_outcome(&self) -> CausalObservationOutcome {
        self.observation_outcome
    }

    pub fn reference_set_digest(&self) -> &str {
        &self.reference_set_digest
    }

    pub fn resolved_reference_count(&self) -> usize {
        self.resolved_reference_count
    }

    pub fn missing_reference_family_count(&self) -> usize {
        self.missing_reference_family_count
    }

    pub fn resolved_evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.resolved_evidence_families
    }

    pub fn observation_target_digest(&self) -> &str {
        &self.observation_target_digest
    }

    pub fn result_shape_context_digest(&self) -> &str {
        &self.result_shape_context_digest
    }

    pub fn target_digest(&self) -> &str {
        &self.target_digest
    }

    pub fn explanation_family(&self) -> CausalInspectionExplanationFamily {
        self.explanation_family
    }

    pub fn requested_richness(&self) -> CausalInspectionRichness {
        self.requested_richness
    }

    pub fn requested_evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.requested_evidence_families
    }

    pub fn subject_digest(&self) -> &str {
        &self.subject_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionAdmissionDecision {
    kind: CausalInspectionAdmissionDecisionKind,
    advisory_kind: Option<CausalInspectionAdvisoryKind>,
    violation_kind: Option<CausalInspectionViolationKind>,
    admitted_richness: CausalInspectionRichness,
    permitted_evidence_families: Vec<CausalEvidenceFamily>,
    decision_digest: String,
}

impl CausalInspectionAdmissionDecision {
    pub(super) fn success(request: &CausalInspectionRequest) -> Self {
        Self::new(
            CausalInspectionAdmissionDecisionKind::Success,
            None,
            None,
            request.requested_richness(),
            request.requested_evidence_families().to_vec(),
        )
    }

    pub(super) fn advisory(
        request: &CausalInspectionRequest,
        advisory_kind: CausalInspectionAdvisoryKind,
    ) -> Self {
        Self::new(
            CausalInspectionAdmissionDecisionKind::Advisory,
            Some(advisory_kind),
            None,
            CausalInspectionRichness::ReferenceOnly,
            request.requested_evidence_families().to_vec(),
        )
    }

    pub(super) fn violation(
        request: &CausalInspectionRequest,
        violation_kind: CausalInspectionViolationKind,
    ) -> Self {
        Self::new(
            CausalInspectionAdmissionDecisionKind::Violation,
            None,
            Some(violation_kind),
            CausalInspectionRichness::ReferenceOnly,
            request.requested_evidence_families().to_vec(),
        )
    }

    fn new(
        kind: CausalInspectionAdmissionDecisionKind,
        advisory_kind: Option<CausalInspectionAdvisoryKind>,
        violation_kind: Option<CausalInspectionViolationKind>,
        admitted_richness: CausalInspectionRichness,
        permitted_evidence_families: Vec<CausalEvidenceFamily>,
    ) -> Self {
        let family_part = permitted_evidence_families
            .iter()
            .map(CausalEvidenceFamily::as_str)
            .collect::<Vec<_>>()
            .join("|");
        let decision_digest = hash_parts(&[
            "causal_inspection_admission_decision_v1".to_string(),
            kind.as_str().to_string(),
            format!(
                "advisory:{}",
                advisory_kind.map_or("none", |kind| kind.as_str())
            ),
            format!(
                "violation:{}",
                violation_kind.map_or("none", |kind| kind.as_str())
            ),
            format!("richness:{}", admitted_richness.as_str()),
            format!("families:{family_part}"),
        ]);
        Self {
            kind,
            advisory_kind,
            violation_kind,
            admitted_richness,
            permitted_evidence_families,
            decision_digest,
        }
    }

    pub fn kind(&self) -> CausalInspectionAdmissionDecisionKind {
        self.kind
    }

    pub fn advisory_kind(&self) -> Option<CausalInspectionAdvisoryKind> {
        self.advisory_kind
    }

    pub fn violation_kind(&self) -> Option<CausalInspectionViolationKind> {
        self.violation_kind
    }

    pub fn admitted_richness(&self) -> CausalInspectionRichness {
        self.admitted_richness
    }

    pub fn permitted_evidence_families(&self) -> &[CausalEvidenceFamily] {
        &self.permitted_evidence_families
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }
}
