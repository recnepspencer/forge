use std::collections::BTreeSet;

use crate::certification::error::TopologyCertificationError;
use crate::certification::ReplayParityStatus;
use crate::topology_operators::TopologyMutationFamily;

use super::super::operator_family_proof::{
    operator_family_closure_labels, primitive_family_closure_labels,
};
use super::super::query_traversal_proof::required_mutation_query_traversal_views;
use super::super::report::{MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileSuiteReport};
use super::super::scale_pressure_proof::scale_pressure_labels;
use super::hostile_category_requirements::{
    partial_status_allowed, required_hostile_certification_categories,
};
use super::hostile_category_types::{
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus,
};

pub(in crate::certification::topology_operator_closeout) fn build_hostile_certification_category_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<MilestoneThreeHostileCertificationCategoryRow> {
    vec![
        mutation_pipeline_integrity_row(report),
        primitive_topology_family_closure_row(report),
        operator_brutality_row(report),
        query_traversal_brutality_row(report),
        non_manifold_radial_brutality_row(report),
        degeneracy_corruption_localization_row(report),
        determinism_order_assault_row(report),
        diagnostics_failure_taxonomy_row(report),
        scale_depth_sustained_pressure_row(report),
    ]
}

pub(in crate::certification::topology_operator_closeout) fn ensure_hostile_certification_category_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for category in required_hostile_certification_categories() {
        let row = report
            .hostile_certification_category_rows
            .iter()
            .find(|row| row.category == *category)
            .ok_or_else(|| missing_category_error(*category))?;
        if row.evidence_count == 0 || row.scenario_count == 0 || row.replay_verified_count == 0 {
            return Err(category_error(*category, "missing direct hostile evidence"));
        }
        if row.evidence_count != row.evidence_labels.len() {
            return Err(category_error(
                *category,
                "evidence labels do not match evidence count",
            ));
        }
        if row.status != MilestoneThreeHostileCertificationStatus::Certified
            && !partial_status_allowed(*category)
        {
            return Err(category_error(*category, "category is not certified"));
        }
        if row.status == MilestoneThreeHostileCertificationStatus::Partial
            && row.gap_labels.is_empty()
        {
            return Err(category_error(
                *category,
                "partial category has no named gaps",
            ));
        }
        if row.status == MilestoneThreeHostileCertificationStatus::Certified
            && !row.gap_labels.is_empty()
        {
            return Err(category_error(
                *category,
                "certified category still has named gaps",
            ));
        }
    }
    Ok(())
}

fn mutation_pipeline_integrity_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    category_row(
        MilestoneThreeHostileCertificationCategory::MutationPipelineIntegrity,
        MilestoneThreeHostileCertificationStatus::Certified,
        report.coverage_rows.len(),
        vec![
            format!("coverage_rows={}", report.coverage_rows.len()),
            format!(
                "topology_mutation_digest_rows={}",
                report.topology_mutation_digest_rows.len()
            ),
            format!(
                "naming_continuity_rows={}",
                report.naming_mutation_continuity_matrix_rows.len()
            ),
        ],
        Vec::new(),
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn primitive_topology_family_closure_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let (scenario_count, evidence_labels, gap_labels) = primitive_family_closure_labels(report);
    category_row(
        MilestoneThreeHostileCertificationCategory::PrimitiveTopologyFamilyClosure,
        category_status_from_gaps(&gap_labels),
        scenario_count,
        evidence_labels,
        gap_labels,
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn operator_brutality_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let (scenario_count, evidence_labels, gap_labels) = operator_family_closure_labels(report);
    category_row(
        MilestoneThreeHostileCertificationCategory::OperatorBrutality,
        category_status_from_gaps(&gap_labels),
        scenario_count,
        evidence_labels,
        gap_labels,
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn query_traversal_brutality_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let required_views = required_mutation_query_traversal_views();
    let gap_labels = required_views
        .iter()
        .filter(|view| {
            !report
                .mutation_query_traversal_rows
                .iter()
                .any(|row| row.view == **view && row.parity_verified)
        })
        .map(|view| {
            format!(
                "missing_mutation_topology_query_traversal={}",
                view.as_str()
            )
        })
        .collect::<Vec<_>>();
    let edited_evidence_labels = report
        .mutation_query_traversal_rows
        .iter()
        .filter(|row| row.parity_verified)
        .map(|row| format!("mutation_topology_query_traversal={}", row.view.as_str()));
    category_row(
        MilestoneThreeHostileCertificationCategory::QueryTraversalBrutality,
        category_status_from_gaps(&gap_labels),
        report.mutation_query_traversal_rows.len(),
        vec![
            format!(
                "domain_read_requests={}",
                report.side_quest_closeout_report.domain_read_request_count
            ),
            format!(
                "domain_read_parity={}",
                report.side_quest_closeout_report.domain_read_parity_count
            ),
            format!(
                "side_quest_replay_verified={}",
                report.side_quest_closeout_report.replay_verified_count
            ),
        ]
        .into_iter()
        .chain(edited_evidence_labels)
        .collect(),
        gap_labels,
        report.side_quest_closeout_report.replay_verified_count
            + report
                .mutation_query_traversal_rows
                .iter()
                .filter(|row| row.parity_verified)
                .count(),
        report.validator_family_coverage_rows.len(),
    )
}

fn non_manifold_radial_brutality_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let radial_scenarios = report
        .scenario_reports
        .iter()
        .filter(|scenario| {
            scenario
                .mutation_families
                .contains(&TopologyMutationFamily::SpliceRadialAdjacency)
        })
        .count();
    category_row(
        MilestoneThreeHostileCertificationCategory::NonManifoldRadialBrutality,
        MilestoneThreeHostileCertificationStatus::Certified,
        radial_scenarios,
        report
            .scenario_reports
            .iter()
            .filter(|scenario| {
                scenario
                    .mutation_families
                    .contains(&TopologyMutationFamily::SpliceRadialAdjacency)
            })
            .map(|scenario| format!("radial_scenario={}", scenario.scenario.as_str()))
            .collect(),
        Vec::new(),
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn degeneracy_corruption_localization_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    category_row(
        MilestoneThreeHostileCertificationCategory::DegeneracyCorruptionLocalization,
        MilestoneThreeHostileCertificationStatus::Certified,
        rejected_scenario_count(report),
        report
            .failure_locality_rows
            .iter()
            .map(|row| format!("localized_rejection={}", row.scenario.as_str()))
            .collect(),
        Vec::new(),
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn determinism_order_assault_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    category_row(
        MilestoneThreeHostileCertificationCategory::DeterminismOrderAssault,
        MilestoneThreeHostileCertificationStatus::Certified,
        report.scenario_reports.len(),
        report
            .determinism_rule_rows
            .iter()
            .map(|row| format!("determinism_rule={}", row.rule_kind.as_str()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        Vec::new(),
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn diagnostics_failure_taxonomy_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    category_row(
        MilestoneThreeHostileCertificationCategory::DiagnosticsFailureTaxonomy,
        MilestoneThreeHostileCertificationStatus::Certified,
        report.rejection_distribution_rows.len() + report.naming_distribution_rows.len(),
        report
            .rejection_distribution_rows
            .iter()
            .map(|row| format!("rejection_class={:?}", row.rejection_class))
            .chain(
                report
                    .naming_distribution_rows
                    .iter()
                    .map(|row| format!("naming_outcome={:?}", row.continuity_outcome_class)),
            )
            .collect(),
        Vec::new(),
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn scale_depth_sustained_pressure_row(
    report: &MilestoneThreeHostileSuiteReport,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let (scenario_count, evidence_labels, gap_labels) = scale_pressure_labels(report);
    category_row(
        MilestoneThreeHostileCertificationCategory::ScaleDepthSustainedPressure,
        category_status_from_gaps(&gap_labels),
        scenario_count,
        evidence_labels,
        gap_labels,
        replay_verified_count(report),
        report.failure_locality_rows.len(),
    )
}

fn category_row(
    category: MilestoneThreeHostileCertificationCategory,
    status: MilestoneThreeHostileCertificationStatus,
    scenario_count: usize,
    evidence_labels: Vec<String>,
    gap_labels: Vec<String>,
    replay_verified_count: usize,
    diagnostic_locality_count: usize,
) -> MilestoneThreeHostileCertificationCategoryRow {
    let evidence_count = evidence_labels.len();
    let gap_count = gap_labels.len();
    MilestoneThreeHostileCertificationCategoryRow {
        category,
        status,
        scenario_count,
        evidence_count,
        replay_verified_count,
        diagnostic_locality_count,
        evidence_labels,
        gap_labels,
        row_digest: format!(
            "category={};status={};scenarios={scenario_count};evidence={evidence_count};gaps={gap_count};replay={replay_verified_count};diagnostics={diagnostic_locality_count}",
            category.as_str(),
            status.as_str()
        ),
    }
}

fn category_status_from_gaps(gap_labels: &[String]) -> MilestoneThreeHostileCertificationStatus {
    if gap_labels.is_empty() {
        MilestoneThreeHostileCertificationStatus::Certified
    } else {
        MilestoneThreeHostileCertificationStatus::Partial
    }
}

fn replay_verified_count(report: &MilestoneThreeHostileSuiteReport) -> usize {
    report
        .mutation_replay_parity_rows
        .iter()
        .filter(|row| row.replay_checked && row.parity_status == ReplayParityStatus::Match)
        .count()
}

fn rejected_scenario_count(report: &MilestoneThreeHostileSuiteReport) -> usize {
    report
        .coverage_rows
        .iter()
        .filter(|row| row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected)
        .count()
}

fn missing_category_error(
    category: MilestoneThreeHostileCertificationCategory,
) -> TopologyCertificationError {
    category_error(category, "missing hostile certification category row")
}

fn category_error(
    category: MilestoneThreeHostileCertificationCategory,
    reason: &str,
) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three hostile certification category `{}` failed: {reason}",
        category.as_str()
    ))
}
