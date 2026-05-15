use crate::identity::hash_parts;

use super::super::fixtures::{
    certified_admitted_intent_fixture, certified_advisory_intent_fixture,
    certified_violation_intent_fixture,
};
use super::forge_query_intent_admission_legacy_parity_report;
use crate::intent_admission::{
    forge_query_intent_admission_coverage_inventory, forge_query_intent_admission_family_inventory,
    forge_query_intent_admission_support_matrix,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCertificationCounterSnapshot {
    intent_family_lookup_width: usize,
    covered_entrypoint_lookup_width: usize,
    decision_trace_width: usize,
    execution_provenance_width: usize,
    digest: String,
}

impl ForgeQueryIntentAdmissionCertificationCounterSnapshot {
    pub fn intent_family_lookup_width(&self) -> usize {
        self.intent_family_lookup_width
    }

    pub fn covered_entrypoint_lookup_width(&self) -> usize {
        self.covered_entrypoint_lookup_width
    }

    pub fn decision_trace_width(&self) -> usize {
        self.decision_trace_width
    }

    pub fn execution_provenance_width(&self) -> usize {
        self.execution_provenance_width
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionSlopeReport {
    counter_snapshot: ForgeQueryIntentAdmissionCertificationCounterSnapshot,
    admission_classification_slope_digest: String,
    decision_trace_assembly_slope_digest: String,
    decision_support_lookup_slope_digest: String,
    covered_entrypoint_inventory_slope_digest: String,
    execution_provenance_assembly_slope_digest: String,
    legacy_delegation_parity_slope_digest: String,
    decision_certification_coverage_slope_digest: String,
}

impl ForgeQueryIntentAdmissionSlopeReport {
    pub fn counter_snapshot(&self) -> &ForgeQueryIntentAdmissionCertificationCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn admission_classification_slope_digest(&self) -> &str {
        &self.admission_classification_slope_digest
    }

    pub fn decision_trace_assembly_slope_digest(&self) -> &str {
        &self.decision_trace_assembly_slope_digest
    }

    pub fn decision_support_lookup_slope_digest(&self) -> &str {
        &self.decision_support_lookup_slope_digest
    }

    pub fn covered_entrypoint_inventory_slope_digest(&self) -> &str {
        &self.covered_entrypoint_inventory_slope_digest
    }

    pub fn execution_provenance_assembly_slope_digest(&self) -> &str {
        &self.execution_provenance_assembly_slope_digest
    }

    pub fn legacy_delegation_parity_slope_digest(&self) -> &str {
        &self.legacy_delegation_parity_slope_digest
    }

    pub fn decision_certification_coverage_slope_digest(&self) -> &str {
        &self.decision_certification_coverage_slope_digest
    }
}

pub fn forge_query_intent_admission_slope_report() -> ForgeQueryIntentAdmissionSlopeReport {
    let admitted = certified_admitted_intent_fixture();
    let advisory = certified_advisory_intent_fixture();
    let violation = certified_violation_intent_fixture();
    let family_inventory = forge_query_intent_admission_family_inventory();
    let coverage_inventory = forge_query_intent_admission_coverage_inventory();
    let support_matrix = forge_query_intent_admission_support_matrix();
    let parity_report = forge_query_intent_admission_legacy_parity_report();
    let decision_trace_width = [
        admitted.trace.rows().len(),
        advisory.trace.rows().len(),
        violation.trace.rows().len(),
    ]
    .into_iter()
    .max()
    .expect("certified traces should exist");
    let execution_provenance_width = 6;
    let counter_snapshot = ForgeQueryIntentAdmissionCertificationCounterSnapshot {
        intent_family_lookup_width: family_inventory.rows().len(),
        covered_entrypoint_lookup_width: coverage_inventory.rows().len(),
        decision_trace_width,
        execution_provenance_width,
        digest: hash_parts(&[
            "forge_query_intent_admission_counter_snapshot_v1".to_string(),
            format!(
                "intent_family_lookup_width:{}",
                family_inventory.rows().len()
            ),
            format!(
                "covered_entrypoint_lookup_width:{}",
                coverage_inventory.rows().len()
            ),
            format!("decision_trace_width:{decision_trace_width}"),
            format!("execution_provenance_width:{execution_provenance_width}"),
        ]),
    };
    ForgeQueryIntentAdmissionSlopeReport {
        counter_snapshot,
        admission_classification_slope_digest: width_slope_digest(
            "admission_classification",
            family_inventory.rows().len(),
        ),
        decision_trace_assembly_slope_digest: width_slope_digest(
            "decision_trace_assembly",
            decision_trace_width,
        ),
        decision_support_lookup_slope_digest: width_slope_digest(
            "decision_support_lookup",
            support_matrix.rows().len(),
        ),
        covered_entrypoint_inventory_slope_digest: width_slope_digest(
            "covered_entrypoint_inventory",
            coverage_inventory.rows().len(),
        ),
        execution_provenance_assembly_slope_digest: width_slope_digest(
            "execution_provenance_assembly",
            execution_provenance_width,
        ),
        legacy_delegation_parity_slope_digest: width_slope_digest(
            "legacy_delegation_parity",
            parity_report.rows().len(),
        ),
        decision_certification_coverage_slope_digest: width_slope_digest(
            "decision_certification_coverage",
            family_inventory.rows().len()
                + coverage_inventory.rows().len()
                + support_matrix.rows().len()
                + parity_report.rows().len(),
        ),
    }
}

fn width_slope_digest(label: &'static str, width: usize) -> String {
    hash_parts(
        &(1..=width)
            .map(|current| format!("label:{label}:width:{current}"))
            .collect::<Vec<_>>(),
    )
}
