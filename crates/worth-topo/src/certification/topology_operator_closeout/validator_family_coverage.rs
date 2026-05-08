use std::collections::BTreeSet;

use crate::certification::core::CertificationValidatorExpectation;
use crate::certification::error::TopologyCertificationError;

use super::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport, MilestoneThreeHostileSuiteReport,
    MilestoneThreeValidatorFamily, MilestoneThreeValidatorFamilyCoverageRow,
};
use super::{milestone_three_rejected_scenarios, milestone_three_required_scenarios};

const ALWAYS_REQUIRED_VALIDATOR_FAMILIES: &[MilestoneThreeValidatorFamily] = &[
    MilestoneThreeValidatorFamily::EditLocalContinuity,
    MilestoneThreeValidatorFamily::NamingContinuity,
];

pub(crate) fn milestone_three_validator_expectations() -> Vec<CertificationValidatorExpectation> {
    milestone_three_required_scenarios()
        .iter()
        .map(|scenario| CertificationValidatorExpectation {
            family: scenario.as_str().to_string(),
            validators: expected_validator_labels_for_scenario(*scenario),
        })
        .collect()
}

pub(super) fn build_validator_family_coverage_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeValidatorFamilyCoverageRow> {
    reports
        .iter()
        .flat_map(validator_family_rows_for_report)
        .collect()
}

pub(super) fn ensure_validator_family_coverage_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    for scenario in milestone_three_required_scenarios() {
        ensure_required_validator_families(report, *scenario)?;
    }
    for scenario in milestone_three_rejected_scenarios() {
        ensure_rejection_locality_validator_family(report, *scenario)?;
    }
    Ok(())
}

fn validator_family_rows_for_report(
    report: &MilestoneThreeHostileScenarioReport,
) -> Vec<MilestoneThreeValidatorFamilyCoverageRow> {
    expected_validator_families_for_report(report)
        .into_iter()
        .map(|validator_family| validator_family_coverage_row(report, validator_family))
        .collect()
}

fn expected_validator_families_for_report(
    report: &MilestoneThreeHostileScenarioReport,
) -> Vec<MilestoneThreeValidatorFamily> {
    let mut families = ALWAYS_REQUIRED_VALIDATOR_FAMILIES.to_vec();
    match report.outcome_class {
        MilestoneThreeHostileOutcomeClass::Accepted => {
            families.push(MilestoneThreeValidatorFamily::DerivedValidationInspection);
        }
        MilestoneThreeHostileOutcomeClass::Rejected => {
            families.push(MilestoneThreeValidatorFamily::RejectionLocality);
        }
    }
    families
}

fn validator_family_coverage_row(
    report: &MilestoneThreeHostileScenarioReport,
    validator_family: MilestoneThreeValidatorFamily,
) -> MilestoneThreeValidatorFamilyCoverageRow {
    let localized_rejection_boundary = localized_rejection_boundary(report);
    MilestoneThreeValidatorFamilyCoverageRow {
        scenario: report.scenario,
        validator_family,
        validator_names: validator_names_for_family(report, validator_family),
        edit_family_count: report.edit_families.len(),
        changed_scope_count: report.topology_edit_digest.changed_scope_count,
        naming_scope_count: report.topology_edit_digest.naming_scope_count,
        derived_region_count: report.topology_edit_digest.derived_region_count,
        derived_validation_row_count: report
            .derived_validation_report
            .as_ref()
            .map_or(0, |validation_report| validation_report.rows.len()),
        localized_rejection_boundary,
        row_digest: format!(
            "scenario={};validator_family={};validators={};edit_families={};changed_scopes={};naming_scopes={};derived_regions={};derived_validation_rows={};localized_rejection_boundary={}",
            report.scenario.as_str(),
            validator_family.as_str(),
            validator_names_for_family(report, validator_family).join("|"),
            report.edit_families.len(),
            report.topology_edit_digest.changed_scope_count,
            report.topology_edit_digest.naming_scope_count,
            report.topology_edit_digest.derived_region_count,
            report
                .derived_validation_report
                .as_ref()
                .map_or(0, |validation_report| validation_report.rows.len()),
            localized_rejection_boundary
        ),
    }
}

fn validator_names_for_family(
    report: &MilestoneThreeHostileScenarioReport,
    validator_family: MilestoneThreeValidatorFamily,
) -> Vec<String> {
    match validator_family {
        MilestoneThreeValidatorFamily::EditLocalContinuity => vec![
            "changed_scope_vocabulary".to_string(),
            "naming_scope_vocabulary".to_string(),
            "derived_region_vocabulary".to_string(),
        ],
        MilestoneThreeValidatorFamily::NamingContinuity => {
            vec!["naming_edit_continuity_matrix".to_string()]
        }
        MilestoneThreeValidatorFamily::DerivedValidationInspection => report
            .derived_validation_report
            .as_ref()
            .map(|validation_report| {
                validation_report
                    .rows
                    .iter()
                    .map(|row| row.validator.clone())
                    .collect()
            })
            .unwrap_or_default(),
        MilestoneThreeValidatorFamily::RejectionLocality => {
            vec!["rejected_edit_scope_report".to_string()]
        }
    }
}

fn localized_rejection_boundary(report: &MilestoneThreeHostileScenarioReport) -> bool {
    report
        .rejected_edit_scope_report
        .as_ref()
        .is_some_and(|scope_report| !scope_report.rows.is_empty())
}

fn ensure_required_validator_families(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    let present = validator_families_for_scenario(report, scenario);
    for validator_family in ALWAYS_REQUIRED_VALIDATOR_FAMILIES {
        if !present.contains(validator_family) {
            return Err(closeout_requirement_error(&format!(
                "missing validator-family coverage row for {} {}",
                scenario.as_str(),
                validator_family.as_str()
            )));
        }
    }
    ensure_outcome_specific_validator_family(report, scenario)?;
    Ok(())
}

fn ensure_outcome_specific_validator_family(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    if milestone_three_rejected_scenarios().contains(&scenario) {
        return ensure_rejection_locality_validator_family(report, scenario);
    }
    let verified = report.validator_family_coverage_rows.iter().any(|row| {
        row.scenario == scenario
            && row.validator_family == MilestoneThreeValidatorFamily::DerivedValidationInspection
            && row.derived_validation_row_count > 0
            && !row.validator_names.is_empty()
    });
    if verified {
        Ok(())
    } else {
        Err(closeout_requirement_error(&format!(
            "missing derived-validation validator-family coverage row for {}",
            scenario.as_str()
        )))
    }
}

fn ensure_rejection_locality_validator_family(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> Result<(), TopologyCertificationError> {
    let verified = report.validator_family_coverage_rows.iter().any(|row| {
        row.scenario == scenario
            && row.validator_family == MilestoneThreeValidatorFamily::RejectionLocality
            && row.localized_rejection_boundary
    });
    if verified {
        Ok(())
    } else {
        Err(closeout_requirement_error(&format!(
            "missing rejection-locality validator-family coverage row for {}",
            scenario.as_str()
        )))
    }
}

fn validator_families_for_scenario(
    report: &MilestoneThreeHostileSuiteReport,
    scenario: MilestoneThreeHostileScenario,
) -> BTreeSet<MilestoneThreeValidatorFamily> {
    report
        .validator_family_coverage_rows
        .iter()
        .filter(|row| row.scenario == scenario)
        .map(|row| row.validator_family)
        .collect()
}

fn expected_validator_labels_for_scenario(scenario: MilestoneThreeHostileScenario) -> Vec<String> {
    let mut validators = ALWAYS_REQUIRED_VALIDATOR_FAMILIES
        .iter()
        .map(|family| family.as_str().to_string())
        .collect::<Vec<_>>();
    if milestone_three_rejected_scenarios().contains(&scenario) {
        validators.push(
            MilestoneThreeValidatorFamily::RejectionLocality
                .as_str()
                .to_string(),
        );
    } else {
        validators.push(
            MilestoneThreeValidatorFamily::DerivedValidationInspection
                .as_str()
                .to_string(),
        );
    }
    validators
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}
