use crate::identity::hash_parts;

use super::super::fixtures::{
    certified_basis_observation_intent_fixture, certified_deferred_intent_fixture,
    certified_inspection_advisory_redaction_fixture, certified_projection_consumption_admitted_fixture,
    certified_projection_consumption_warning_fixture, certified_unsupported_intent_fixture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionRepresentativeFamilyLane {
    BasisParity,
    ProjectionAdvisory,
    InspectionAdvisoryRedaction,
    RoutingFutureNeighbor,
}

impl ForgeQueryIntentAdmissionRepresentativeFamilyLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BasisParity => "basis_parity",
            Self::ProjectionAdvisory => "projection_advisory",
            Self::InspectionAdvisoryRedaction => "inspection_advisory_redaction",
            Self::RoutingFutureNeighbor => "routing_future_neighbor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionRepresentativeFamilyRow {
    lane: ForgeQueryIntentAdmissionRepresentativeFamilyLane,
    authority_surface: &'static str,
    evidence_digest: String,
    posture_detail: &'static str,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionRepresentativeFamilyRow {
    pub fn lane(&self) -> ForgeQueryIntentAdmissionRepresentativeFamilyLane {
        self.lane
    }

    pub fn authority_surface(&self) -> &'static str {
        self.authority_surface
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn posture_detail(&self) -> &'static str {
        &self.posture_detail
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionRepresentativeFamilyReport {
    rows: Vec<ForgeQueryIntentAdmissionRepresentativeFamilyRow>,
    representative_family_coverage_digest: String,
}

impl ForgeQueryIntentAdmissionRepresentativeFamilyReport {
    pub fn rows(&self) -> &[ForgeQueryIntentAdmissionRepresentativeFamilyRow] {
        &self.rows
    }

    pub fn representative_family_coverage_digest(&self) -> &str {
        &self.representative_family_coverage_digest
    }
}

pub fn forge_query_intent_admission_representative_family_report(
) -> ForgeQueryIntentAdmissionRepresentativeFamilyReport {
    let basis = certified_basis_observation_intent_fixture();
    let projection_admitted = certified_projection_consumption_admitted_fixture();
    let projection_warning = certified_projection_consumption_warning_fixture();
    let deferred = certified_deferred_intent_fixture();
    let unsupported = certified_unsupported_intent_fixture();
    let inspection = certified_inspection_advisory_redaction_fixture();

    let rows = vec![
        representative_row(
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::BasisParity,
            "intent_admission::forge_query_basis_observation_intent",
            hash_parts(&[
                basis.request.request_digest().to_string(),
                basis.eligibility.eligibility_digest().to_string(),
                basis.plan.request_digest().to_string(),
                basis.scoped_basis_digest.clone(),
            ]),
            "equivalent basis-use intents normalize through one intent-admission lattice path before scoping to lower-runtime evidence",
        ),
        representative_row(
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::ProjectionAdvisory,
            "intent_admission::forge_query_projection_consumption_intent",
            hash_parts(&[
                projection_admitted.request.request_digest().to_string(),
                projection_admitted.eligibility.eligibility_digest().to_string(),
                projection_admitted.plan.request_digest().to_string(),
                projection_admitted.contract_digest.clone(),
                projection_warning.request.request_digest().to_string(),
                projection_warning.eligibility.eligibility_digest().to_string(),
                projection_warning.contract_digest.clone(),
                projection_warning
                    .plan
                    .warning_kinds()
                    .map(|warnings| warnings.warning_digest().to_string())
                    .unwrap_or_else(|| "no-warnings".to_string()),
            ]),
            "warning-bearing and admitted projection consumption both resolve through one intent-admission lattice path before contract binding",
        ),
        representative_row(
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::InspectionAdvisoryRedaction,
            "runtime.inspect_intent(target).review()?.admit()?.execute()",
            hash_parts(&[
                inspection.request_digest.clone(),
                inspection.eligibility_digest.clone(),
                inspection.decision_trace_digest.clone(),
                inspection.execution_provenance_chain_digest.clone(),
                inspection.full_artifact_digest.clone(),
                inspection.redacted_artifact_digest.clone(),
                inspection.causal_identity_digest.clone(),
                inspection.boundary_audit_digest.clone(),
            ]),
            "inspection detail narrowing changes materialized detail while preserving one causal identity on the lattice-owned unified inspection path",
        ),
        representative_row(
            ForgeQueryIntentAdmissionRepresentativeFamilyLane::RoutingFutureNeighbor,
            "intent_admission::{admit_runtime_intent_request, ForgeQueryRawIntentAdmissionRequest::deferred_neighbor}",
            hash_parts(&[
                deferred.request.request_digest().to_string(),
                decision_digest(&deferred.decision),
                deferred.trace.trace_digest().to_string(),
                unsupported.request.request_digest().to_string(),
                decision_digest(&unsupported.decision),
                unsupported.trace.trace_digest().to_string(),
            ]),
            "lower-runtime capability routing preserves a typed deferred lane and a distinct typed unsupported lane before 9.3.6 execution semantics",
        ),
    ];
    let representative_family_coverage_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionRepresentativeFamilyReport {
        rows,
        representative_family_coverage_digest,
    }
}

fn representative_row(
    lane: ForgeQueryIntentAdmissionRepresentativeFamilyLane,
    authority_surface: &'static str,
    evidence_digest: String,
    posture_detail: &'static str,
) -> ForgeQueryIntentAdmissionRepresentativeFamilyRow {
    let row_digest = hash_parts(&[
        "forge_query_intent_admission_representative_family_row_v1".to_string(),
        format!("lane:{}", lane.as_str()),
        format!("authority:{authority_surface}"),
        format!("evidence:{evidence_digest}"),
        format!("posture:{posture_detail}"),
    ]);
    ForgeQueryIntentAdmissionRepresentativeFamilyRow {
        lane,
        authority_surface,
        evidence_digest,
        posture_detail,
        row_digest,
    }
}

fn decision_digest(decision: &crate::facade::runtime::ForgeQueryIntentAdmissionDecision) -> String {
    match decision {
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            plan.decision_digest().to_string()
        }
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            advisory.decision_digest().to_string()
        }
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            violation.decision_digest().to_string()
        }
    }
}
