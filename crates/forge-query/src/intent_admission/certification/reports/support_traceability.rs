use crate::identity::hash_parts;

use super::super::fixtures::{
    certified_admitted_intent_fixture, certified_advisory_intent_fixture,
    certified_routing_intent_fixture, certified_violation_intent_fixture,
};
use super::super::fixtures::{
    certified_deferred_intent_fixture, certified_unsupported_intent_fixture,
};
use crate::intent_admission::forge_query_intent_admission_coverage_inventory;
use crate::intent_admission::forge_query_intent_admission_support_matrix;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionSupportTraceabilityRow {
    lane: &'static str,
    family: String,
    entrypoint: String,
    support_detail: String,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionSupportTraceabilityRow {
    pub fn lane(&self) -> &'static str {
        self.lane
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn entrypoint(&self) -> &str {
        &self.entrypoint
    }

    pub fn support_detail(&self) -> &str {
        &self.support_detail
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionSupportTraceabilityReport {
    rows: Vec<ForgeQueryIntentAdmissionSupportTraceabilityRow>,
    decision_support_traceability_digest: String,
}

impl ForgeQueryIntentAdmissionSupportTraceabilityReport {
    pub fn rows(&self) -> &[ForgeQueryIntentAdmissionSupportTraceabilityRow] {
        &self.rows
    }

    pub fn decision_support_traceability_digest(&self) -> &str {
        &self.decision_support_traceability_digest
    }
}

pub fn forge_query_intent_admission_support_traceability_report(
) -> ForgeQueryIntentAdmissionSupportTraceabilityReport {
    let support = forge_query_intent_admission_support_matrix();
    let admitted = certified_admitted_intent_fixture();
    let advisory = certified_advisory_intent_fixture();
    let violation = certified_violation_intent_fixture();
    let routing = certified_routing_intent_fixture();
    let deferred = certified_deferred_intent_fixture();
    let unsupported = certified_unsupported_intent_fixture();
    let rows = vec![
        traceability_row(
            "admitted",
            admitted.request.family().as_str(),
            admitted.request.entrypoint().as_str(),
            support_row_detail(
                &support,
                admitted.request.family(),
                admitted.request.entrypoint(),
            ),
        ),
        traceability_row(
            "advisory",
            advisory.request.family().as_str(),
            advisory.request.entrypoint().as_str(),
            support_row_detail(
                &support,
                advisory.request.family(),
                advisory.request.entrypoint(),
            ),
        ),
        traceability_row(
            "violation",
            violation.request.family().as_str(),
            violation.request.entrypoint().as_str(),
            support_row_detail(
                &support,
                violation.request.family(),
                violation.request.entrypoint(),
            ),
        ),
        traceability_row(
            "routing_admitted",
            routing.request.family().as_str(),
            routing.request.entrypoint().as_str(),
            support_row_detail(
                &support,
                routing.request.family(),
                routing.request.entrypoint(),
            ),
        ),
        traceability_row(
            "deferred",
            deferred.request.family().as_str(),
            deferred.request.entrypoint().as_str(),
            support_row_detail(
                &support,
                deferred.request.family(),
                deferred.request.entrypoint(),
            ),
        ),
        traceability_row(
            "unsupported",
            unsupported.request.family().as_str(),
            unsupported.request.entrypoint().as_str(),
            unsupported_coverage_detail(
                unsupported.request.family(),
                unsupported.request.entrypoint(),
            ),
        ),
    ];
    let decision_support_traceability_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionSupportTraceabilityReport {
        rows,
        decision_support_traceability_digest,
    }
}

fn traceability_row(
    lane: &'static str,
    family: &str,
    entrypoint: &str,
    support_detail: String,
) -> ForgeQueryIntentAdmissionSupportTraceabilityRow {
    ForgeQueryIntentAdmissionSupportTraceabilityRow {
        lane,
        family: family.to_string(),
        entrypoint: entrypoint.to_string(),
        support_detail: support_detail.clone(),
        row_digest: hash_parts(&[
            "forge_query_intent_admission_support_traceability_row_v1".to_string(),
            format!("lane:{lane}"),
            format!("family:{family}"),
            format!("entrypoint:{entrypoint}"),
            support_detail,
        ]),
    }
}

fn support_row_detail(
    matrix: &crate::intent_admission::ForgeQueryIntentAdmissionSupportMatrix,
    family: crate::intent_admission::ForgeQueryIntentAdmissionFamily,
    entrypoint: crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint,
) -> String {
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.family() == family && row.entrypoint() == entrypoint)
        .expect("support row should exist for certified lane");
    format!(
        "support:{}:{}:{}",
        row.posture().as_str(),
        row.execution_boundary().as_str(),
        row.detail().as_str()
    )
}

fn unsupported_coverage_detail(
    family: crate::intent_admission::ForgeQueryIntentAdmissionFamily,
    entrypoint: crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint,
) -> String {
    let row = forge_query_intent_admission_coverage_inventory()
        .rows()
        .iter()
        .find(|row| row.family() == family && row.entrypoint() == entrypoint)
        .expect("coverage row should exist for unsupported certification lane");
    format!(
        "unsupported:{}:{}:{}",
        row.execution_boundary().as_str(),
        row.advisory_decision_class().as_str(),
        row.violation_decision_class().as_str(),
    )
}
