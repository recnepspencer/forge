use std::collections::BTreeMap;

use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::{
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationRejectionClass,
};

use super::super::super::report::{
    MilestoneThreeHostileFamilyCoverageRow, MilestoneThreeHostileNamingDistributionRow,
    MilestoneThreeHostileRejectionDistributionRow, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
};

pub(in crate::certification::topology_operator_closeout) fn build_family_coverage_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileFamilyCoverageRow> {
    let mut rows = BTreeMap::<TopologyMutationFamily, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        for family in &report.mutation_families {
            rows.entry(*family).or_default().push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(family, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeHostileFamilyCoverageRow {
                family,
                scenario_count: scenarios.len(),
                row_digest: family_coverage_row_digest(family, &scenarios),
                scenarios,
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn build_rejection_distribution_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileRejectionDistributionRow> {
    let mut rows =
        BTreeMap::<TopologyMutationRejectionClass, Vec<MilestoneThreeHostileScenario>>::new();
    for rejection_class in TopologyMutationRejectionClass::ALL {
        rows.entry(rejection_class).or_default();
    }
    for report in reports {
        if let Some(rejection_class) = report.rejection_class {
            rows.entry(rejection_class)
                .or_default()
                .push(report.scenario);
        }
    }
    rows.into_iter()
        .map(|(rejection_class, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeHostileRejectionDistributionRow {
                rejection_class,
                case_count: scenarios.len(),
                row_digest: rejection_distribution_row_digest(rejection_class, &scenarios),
                scenarios,
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn build_naming_distribution_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeHostileNamingDistributionRow> {
    let mut rows =
        BTreeMap::<TopologyMutationNamingOutcome, Vec<MilestoneThreeHostileScenario>>::new();
    for report in reports {
        rows.entry(report.continuity_outcome_class)
            .or_default()
            .push(report.scenario);
    }
    rows.into_iter()
        .map(|(continuity_outcome_class, mut scenarios)| {
            scenarios.sort();
            scenarios.dedup();
            MilestoneThreeHostileNamingDistributionRow {
                continuity_outcome_class,
                case_count: scenarios.len(),
                row_digest: naming_distribution_row_digest(continuity_outcome_class, &scenarios),
                scenarios,
            }
        })
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_hostile_distribution_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    ensure_family_distribution_rows(report)?;
    ensure_rejection_distribution_rows(report)?;
    ensure_naming_distribution_rows(report)?;
    Ok(())
}

fn ensure_family_distribution_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let expected_rows = build_family_coverage_rows(&report.scenario_reports);
    if expected_rows.is_empty() || report.family_coverage_rows != expected_rows {
        return Err(distribution_error(
            "family coverage rows do not match scenario evidence",
        ));
    }
    Ok(())
}

fn ensure_rejection_distribution_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let expected_rows = build_rejection_distribution_rows(&report.scenario_reports);
    if report.rejection_distribution_rows != expected_rows {
        return Err(distribution_error(
            "rejection distribution rows do not match scenario evidence",
        ));
    }
    if report.rejection_distribution_rows.len() != TopologyMutationRejectionClass::ALL.len()
        || !TopologyMutationRejectionClass::ALL
            .into_iter()
            .all(|rejection_class| {
                report.rejection_distribution_rows.iter().any(|row| {
                    row.rejection_class == rejection_class
                        && row.case_count == row.scenarios.len()
                        && row.row_digest
                            == rejection_distribution_row_digest(
                                row.rejection_class,
                                &row.scenarios,
                            )
                })
            })
    {
        return Err(distribution_error(
            "rejection distribution rows do not cover the closed rejection taxonomy",
        ));
    }
    if !report.rejection_distribution_rows.iter().any(|row| {
        row.rejection_class == TopologyMutationRejectionClass::InvariantBlocked
            && row.case_count == row.scenarios.len()
            && row.case_count >= 2
            && row.row_digest
                == rejection_distribution_row_digest(row.rejection_class, &row.scenarios)
    }) {
        return Err(distribution_error(
            "missing proof-bearing invariant-blocked rejection distribution",
        ));
    }
    Ok(())
}

fn ensure_naming_distribution_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    let expected_rows = build_naming_distribution_rows(&report.scenario_reports);
    if report.naming_distribution_rows != expected_rows {
        return Err(distribution_error(
            "naming distribution rows do not match scenario evidence",
        ));
    }
    for outcome in [
        TopologyMutationNamingOutcome::Ambiguous,
        TopologyMutationNamingOutcome::Rejected,
    ] {
        let verified = report.naming_distribution_rows.iter().any(|row| {
            row.continuity_outcome_class == outcome
                && row.case_count == row.scenarios.len()
                && row.case_count > 0
                && row.row_digest == naming_distribution_row_digest(outcome, &row.scenarios)
        });
        if !verified {
            return Err(distribution_error(&format!(
                "missing proof-bearing naming distribution for {outcome:?}"
            )));
        }
    }
    Ok(())
}

fn family_coverage_row_digest(
    family: TopologyMutationFamily,
    scenarios: &[MilestoneThreeHostileScenario],
) -> String {
    format!(
        "family={family:?};scenario_count={};scenarios={}",
        scenarios.len(),
        scenario_set_digest(scenarios)
    )
}

fn rejection_distribution_row_digest(
    rejection_class: TopologyMutationRejectionClass,
    scenarios: &[MilestoneThreeHostileScenario],
) -> String {
    format!(
        "rejection_class={rejection_class:?};case_count={};scenarios={}",
        scenarios.len(),
        scenario_set_digest(scenarios)
    )
}

fn naming_distribution_row_digest(
    outcome: TopologyMutationNamingOutcome,
    scenarios: &[MilestoneThreeHostileScenario],
) -> String {
    format!(
        "naming_outcome={outcome:?};case_count={};scenarios={}",
        scenarios.len(),
        scenario_set_digest(scenarios)
    )
}

fn scenario_set_digest(scenarios: &[MilestoneThreeHostileScenario]) -> String {
    scenarios
        .iter()
        .map(|scenario| scenario.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

fn distribution_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three hostile distribution failed: {reason}"
    ))
}

#[cfg(test)]
mod tests {
    use crate::facade::certify_milestone_three_hostile_suite;
    use crate::topology_operators::TopologyMutationRejectionClass;
    use crate::validation::reference_integrity::build_milestone_one_runtime;

    use super::ensure_hostile_distribution_rows;

    #[test]
    fn hostile_distribution_verifier_rejects_tampered_scenario_digest() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect(" milestone one runtime builder"),
            "m3.hostile_suite.aggregate_distribution_tamper",
        )
        .expect("milestone three hostile suite should certify before tampering");

        report.rejection_distribution_rows[0]
            .row_digest
            .push_str("|forged_extra_scenario");

        assert!(
            ensure_hostile_distribution_rows(&report).is_err(),
            "aggregate distribution verifier must fail closed on scenario digest drift"
        );
    }

    #[test]
    fn hostile_distribution_verifier_rejects_missing_zero_count_taxonomy_row() {
        let mut report = certify_milestone_three_hostile_suite(
            || build_milestone_one_runtime().expect(" milestone one runtime builder"),
            "m3.hostile_suite.aggregate_distribution_missing_taxonomy_row",
        )
        .expect("milestone three hostile suite should certify before tampering");

        report.rejection_distribution_rows.retain(|row| {
            row.rejection_class != TopologyMutationRejectionClass::DerivedFallbackExceeded
        });

        assert!(
            ensure_hostile_distribution_rows(&report).is_err(),
            "aggregate distribution verifier must fail closed when a zero-count rejection taxonomy class is omitted"
        );
    }
}
