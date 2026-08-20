use worth_proof::TransitionOutcome;

use crate::canonicalization::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use crate::performance::policy::FoundationalPerformanceBudgetDecision;
use crate::performance::reports::FoundationalMaterializedPerformanceReport;
use crate::performance::{
    FoundationalPerformanceReportSectionDecision, FoundationalPerformanceWorkClass,
};

use super::canonical::{
    append_claim_surface_entries, append_contract_entries, append_counter_row_entries,
    append_counter_spec_entries, append_layout_entries, append_support_entries,
};
use super::support::{
    attachment_target_token, budget_kind_token, claim_bool_entry, claim_text_entry,
    counter_integer_entry, counter_text_entry, report_decision_cause_token,
    report_materialization_boundary_token, report_section_token, work_class_token,
};

pub fn prepare_materialized_performance_report_for_canonical_basis<Source>(
    version: CanonicalizationRuleVersion,
    report: &FoundationalMaterializedPerformanceReport<Source>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Performance,
        canonical_basis_for_materialized_report(report),
    )
}

fn canonical_basis_for_materialized_report<Source>(
    report: &FoundationalMaterializedPerformanceReport<Source>,
) -> Vec<crate::canonicalization::CanonicalBasisEntry> {
    let mut entries = vec![claim_text_entry("shape", "materialized-performance-report")];
    entries.push(claim_text_entry(
        "report.target",
        attachment_target_token(report.target()),
    ));
    entries.push(claim_text_entry(
        "report.materialization_boundary",
        report_materialization_boundary_token(report.materialization_boundary()),
    ));
    append_claim_surface_entries(
        report.boundary(),
        report.evidence_strength(),
        report.breadth_locality(),
        report.access_pattern(),
        report.execution_temperature(),
        report.freshness_retention(),
        report.fallback_debt(),
        report.included_work(),
        report.excluded_work(),
        report.observation_context(),
        &mut entries,
    );
    append_section_decisions(report.section_decisions(), &mut entries);
    append_layout_entries(report.layout_intent_claim(), &mut entries);
    append_contract_entries(report.contract_names(), &mut entries);
    append_counter_spec_entries(report.counter_specs(), &mut entries);
    append_counter_row_entries(report.counter_rows(), &mut entries);
    append_support_entries(report.supporting_evidence_rows(), &mut entries);
    append_budget_decisions(report.budget_decisions(), &mut entries);
    append_work_collection("report.denied_work", report.denied_work(), &mut entries);
    append_work_collection("report.widened_work", report.widened_work(), &mut entries);
    entries
}

fn append_section_decisions(
    decisions: &[FoundationalPerformanceReportSectionDecision],
    entries: &mut Vec<crate::canonicalization::CanonicalBasisEntry>,
) {
    for (ordinal, decision) in decisions.iter().enumerate() {
        let prefix = format!("report.section_decision.{ordinal}");
        entries.push(claim_text_entry(
            &format!("{prefix}.section"),
            report_section_token(decision.section()),
        ));
        entries.push(claim_bool_entry(
            &format!("{prefix}.included"),
            decision.is_included(),
        ));
        entries.push(claim_text_entry(
            &format!("{prefix}.cause"),
            report_decision_cause_token(decision.cause()),
        ));
    }
}

fn append_budget_decisions(
    decisions: &[FoundationalPerformanceBudgetDecision],
    entries: &mut Vec<crate::canonicalization::CanonicalBasisEntry>,
) {
    for (ordinal, decision) in decisions.iter().enumerate() {
        let prefix = format!("report.budget_decision.{ordinal}");
        entries.push(counter_text_entry(
            &format!("{prefix}.kind"),
            budget_kind_token(decision.kind()),
        ));
        entries.push(counter_integer_entry(
            &format!("{prefix}.requested_units"),
            u64::from(decision.requested_units()),
        ));
        entries.push(counter_integer_entry(
            &format!("{prefix}.admitted_units"),
            u64::from(decision.admitted_units()),
        ));
    }
}

fn append_work_collection(
    prefix: &str,
    values: &[FoundationalPerformanceWorkClass],
    entries: &mut Vec<crate::canonicalization::CanonicalBasisEntry>,
) {
    for (ordinal, value) in values.iter().enumerate() {
        entries.push(claim_text_entry(
            &format!("{prefix}.{ordinal}"),
            work_class_token(*value),
        ));
    }
}
