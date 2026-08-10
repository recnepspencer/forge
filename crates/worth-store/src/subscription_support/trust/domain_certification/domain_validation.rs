use super::batch_plan::SupportDomainCertificationBatchPlan;
use super::domain_counter::SupportDomainCertificationCounterSnapshot;
use super::domain_row::SupportDomainCertificationRow;
use super::scenario::{
    required_scenario_family, required_scenario_row_status, SupportDomainCertificationRowStatus,
    SupportDomainCertificationScenario,
};
use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use std::collections::BTreeSet;

pub(super) fn validate_required_domain_rows(
    rows: &[SupportDomainCertificationRow],
) -> Result<(), SupportTrustFailure> {
    let mut seen = BTreeSet::new();
    for row in rows {
        validate_required_domain_row(row, &mut seen)?;
    }
    validate_first_ship_scenarios(&seen)
}

fn validate_required_domain_row(
    row: &SupportDomainCertificationRow,
    seen: &mut BTreeSet<SupportDomainCertificationScenario>,
) -> Result<(), SupportTrustFailure> {
    validate_unique_scenario(row, seen)?;
    validate_row_status(row)?;
    validate_row_family(row)?;
    validate_row_debt_metadata(row)
}

fn validate_unique_scenario(
    row: &SupportDomainCertificationRow,
    seen: &mut BTreeSet<SupportDomainCertificationScenario>,
) -> Result<(), SupportTrustFailure> {
    if !seen.insert(row.scenario()) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification cannot contain duplicate scenario rows",
        ));
    }
    Ok(())
}

fn validate_row_status(row: &SupportDomainCertificationRow) -> Result<(), SupportTrustFailure> {
    if row.row_status() != required_scenario_row_status(row.scenario()) {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification row status must match the scenario's required certification posture",
        ));
    }
    Ok(())
}

fn validate_row_family(row: &SupportDomainCertificationRow) -> Result<(), SupportTrustFailure> {
    let expected = required_scenario_family(row.scenario());
    if row.family_kind() != expected.0
        || row.support_role() != expected.1
        || row.required_trust_strength() != expected.2
        || row.required_trust_class() != expected.3
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustRoleMismatch,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification row must preserve the scenario family, role, and required trust posture",
        ));
    }
    Ok(())
}

fn validate_row_debt_metadata(
    row: &SupportDomainCertificationRow,
) -> Result<(), SupportTrustFailure> {
    if row.row_status() == SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
        && (row.debt_reason().is_none() || row.required_future_milestone().is_none())
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "explicit domain support debt rows must name the debt reason and future owner",
        ));
    }
    if row.row_status() == SupportDomainCertificationRowStatus::CertifiedSemanticSupport
        && (row.debt_reason().is_some() || row.required_future_milestone().is_some())
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RerunCertification,
            "certified semantic support rows cannot carry future-debt metadata",
        ));
    }
    Ok(())
}

fn validate_first_ship_scenarios(
    seen: &BTreeSet<SupportDomainCertificationScenario>,
) -> Result<(), SupportTrustFailure> {
    for scenario in SupportDomainCertificationScenario::first_ship_required() {
        if !seen.contains(scenario) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification is missing a required first-ship scenario row",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_domain_counters(
    rows: &[SupportDomainCertificationRow],
    batch_plan: &SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
) -> Result<(), SupportTrustFailure> {
    let row_count = rows.len() as u64;
    let certified_count = rows
        .iter()
        .filter(|row| {
            row.row_status() == SupportDomainCertificationRowStatus::CertifiedSemanticSupport
        })
        .count() as u64;
    let debt_count = rows
        .iter()
        .filter(|row| {
            row.row_status() == SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
        })
        .count() as u64;
    let scope = batch_plan.batch_scope();
    if batch_plan.scenario_width() != row_count
        || batch_plan.family_role_row_width() != row_count
        || scope.row_count() != row_count
        || counter_snapshot.scenario_row_count() != row_count
        || counter_snapshot.certified_semantic_row_count() != certified_count
        || counter_snapshot.explicit_debt_row_count() != debt_count
        || counter_snapshot.index_probe_count() != scope.expected_index_probes()
        || counter_snapshot.receipt_reuse_count() != scope.expected_receipt_reuse_count()
        || counter_snapshot.allocation_count() != scope.expected_allocation_count()
        || counter_snapshot.physical_readiness_debt_count() != debt_count
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification counters must match declared scenario width and explicit debt rows",
        ));
    }
    Ok(())
}
