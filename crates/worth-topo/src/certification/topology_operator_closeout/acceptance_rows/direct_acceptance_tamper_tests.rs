use crate::facade::certify_milestone_three_hostile_suite;
use crate::topology_operators::TopologyEditRejectionClass;
use crate::validation::reference_integrity::build_milestone_one_runtime;

use super::direct_acceptance::ensure_direct_acceptance_proof_rows;

#[test]
fn direct_acceptance_rejects_derived_fallback_policy_exceeded_rows() {
    let mut report = certified_hostile_suite_before_tamper(
        "m3.direct_acceptance.fallback_policy_exceeded_tamper",
    );

    let fallout_row = explicit_fallback_row(&mut report);
    fallout_row.fallback_policy_exceeded = true;
    fallout_row.fallback_rejection_class =
        Some(TopologyEditRejectionClass::DerivedFallbackExceeded);

    assert!(
        ensure_direct_acceptance_proof_rows(&report).is_err(),
        "closeout must reject fallback rows that exceed the declared edit fallback policy"
    );
}

#[test]
fn direct_acceptance_rejects_forged_derived_fallback_rejection_rows() {
    let mut report = certified_hostile_suite_before_tamper(
        "m3.direct_acceptance.forged_fallback_rejection_tamper",
    );

    let fallout_row = explicit_fallback_row(&mut report);
    fallout_row.fallback_policy_exceeded = false;
    fallout_row.fallback_rejection_class =
        Some(TopologyEditRejectionClass::DerivedFallbackExceeded);

    assert!(
        ensure_direct_acceptance_proof_rows(&report).is_err(),
        "closeout must reject a fallback rejection class that is not backed by an exceeded policy"
    );
}

#[test]
fn direct_acceptance_rejects_missing_derived_fallback_denial_rows() {
    let mut report =
        certified_hostile_suite_before_tamper("m3.direct_acceptance.missing_fallback_denial");

    report.derived_fallback_policy_denial_rows.clear();

    assert!(
        ensure_direct_acceptance_proof_rows(&report).is_err(),
        "closeout must require strict fallback policy denial rows"
    );
}

#[test]
fn direct_acceptance_rejects_weak_naming_continuity_breadth_rows() {
    let mut report =
        certified_hostile_suite_before_tamper("m3.direct_acceptance.weak_naming_breadth");

    let row = report
        .naming_continuity_breadth_rows
        .iter_mut()
        .next()
        .expect("hostile suite should include naming continuity breadth evidence");
    row.continuity_row_count = row.preserved_count + row.ambiguous_count + row.rejected_count + 1;

    assert!(
        ensure_direct_acceptance_proof_rows(&report).is_err(),
        "closeout must reject naming breadth rows whose totals do not match the continuity matrix"
    );
}

#[test]
fn direct_acceptance_rejects_weak_derived_fallback_denial_rows() {
    let mut report =
        certified_hostile_suite_before_tamper("m3.direct_acceptance.weak_fallback_denial");

    let row = report
        .derived_fallback_policy_denial_rows
        .iter_mut()
        .next()
        .expect("hostile suite should include strict fallback denial evidence");
    row.policy_exceeded = false;

    assert!(
        ensure_direct_acceptance_proof_rows(&report).is_err(),
        "closeout must reject denial rows that do not prove policy overflow"
    );
}

fn certified_hostile_suite_before_tamper(
    stem: &str,
) -> crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport {
    certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        stem,
    )
    .expect("milestone three hostile suite should certify before tampering")
}

fn explicit_fallback_row(
    report: &mut crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport,
) -> &mut crate::certification::topology_operator_closeout::MilestoneThreeEditFalloutBreadthRow {
    report
        .edit_fallout_breadth_rows
        .iter_mut()
        .find(|row| row.fallback_count > 0)
        .expect("hostile suite should include explicit fallback evidence")
}




