use crate::facade::{access_planning, deterministic_plan_selection, layout_execution_freshness};
use crate::strategy::tests_support::admit_phase_five_scope;
use crate::{access_lowering, S8AccessLoweringDenied, S8DegradedExactScanRequest};
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::LogSequenceNumber;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn lowering_progression_defers_lsm_point_paths_until_runtime_lease() {
    let (lifecycle, key_domain) = admit_wal_scope();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            LogSequenceNumber::new(17),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_planning()
                .require_exact_point_access(coverage)
                .unwrap(),
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let lowered = access_lowering().lower_selected(selected).into_lowered();

    let deferred = access_lowering().admit_ready(lowered);
    let reason = deferred
        .into_deferred()
        .expect("LSM point path should defer");
    assert!(matches!(
        reason.spent_cost_receipt(),
        crate::S8AccessAttemptCostReceipt::NoExecutionCountersSpent { .. }
    ));
}

#[test]
fn lowering_progression_surfaces_stale_and_readmitted_outcomes_for_degraded_scan() {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(9).unwrap(),
        )
        .unwrap();
    let degraded = crate::access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(coverage.require_exact().unwrap()).with_budget_rows(8),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            degraded,
            S8PreExecutionBudgetEnvelope::terminal_default(),
        )
        .unwrap();
    let lowered = access_lowering().lower_selected(selected).into_lowered();
    let stale_outcome = access_lowering().admit_ready(lowered);
    assert_eq!(
        stale_outcome.production_transition().edge().to(),
        crate::production_transition::S8LayoutMachineState::Stale
    );
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::ExecutionReadiness,
        )
        .contains(stale_outcome.production_transition())
    );
    let stale = stale_outcome
        .into_stale()
        .expect("degraded scan should be stale");

    let witness = layout_execution_freshness()
        .admit_current_for_stale(&stale, lifecycle, key_domain, coverage)
        .unwrap();
    let readmitted = access_lowering().readmit_stale(stale, witness);
    assert_eq!(
        readmitted.production_transition().edge().to(),
        crate::production_transition::S8LayoutMachineState::Readmitted
    );
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::StaleRebindReadmission,
        )
        .contains(readmitted.production_transition())
    );
    assert!(readmitted.into_readmitted().is_ok());
}

#[test]
fn lowering_progression_denies_mismatched_readmission_witnesses() {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(11).unwrap(),
        )
        .unwrap();
    let first = crate::access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(coverage.require_exact().unwrap()).with_budget_rows(8),
        )
        .unwrap();
    let second = crate::access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(coverage.require_exact().unwrap()).with_budget_rows(9),
        )
        .unwrap();

    let first_selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            first,
            S8PreExecutionBudgetEnvelope::terminal_default(),
        )
        .unwrap();
    let second_selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            second,
            S8PreExecutionBudgetEnvelope::terminal_default(),
        )
        .unwrap();

    let stale = access_lowering()
        .admit_ready(
            access_lowering()
                .lower_selected(first_selected)
                .into_lowered(),
        )
        .into_stale()
        .expect("first degraded scan should be stale");
    let wrong_stale = access_lowering()
        .admit_ready(
            access_lowering()
                .lower_selected(second_selected)
                .into_lowered(),
        )
        .into_stale()
        .expect("second degraded scan should be stale");

    let wrong_witness = layout_execution_freshness()
        .admit_current_for_stale(&wrong_stale, lifecycle, key_domain, coverage)
        .unwrap();
    let denial = access_lowering()
        .readmit_stale(stale, wrong_witness)
        .into_denial()
        .expect("mismatched witness must deny");
    assert!(matches!(
        denial,
        S8AccessLoweringDenied::ReadmissionWitnessMismatch { .. }
    ));
}

#[test]
fn lowering_progression_supports_explicit_rebind_boundary() {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(13).unwrap(),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_planning()
                .require_exact_range_access(coverage)
                .unwrap(),
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let lowered = access_lowering().lower_selected(selected).into_lowered();
    let rebind_outcome = access_lowering().require_rebind(lowered);
    assert_eq!(
        rebind_outcome.production_transition().edge().to(),
        crate::production_transition::S8LayoutMachineState::RebindRequired
    );
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::StaleRebindReadmission,
        )
        .contains(rebind_outcome.production_transition())
    );
    let rebound = rebind_outcome
        .into_required()
        .expect("rebind should be required");
    let witness = layout_execution_freshness()
        .admit_rebind_for_execution(&rebound, lifecycle, key_domain, coverage)
        .unwrap();

    let rebound = access_lowering().rebind_for_execution(rebound, witness);
    assert_eq!(
        rebound.production_transition().edge().to(),
        crate::production_transition::S8LayoutMachineState::Lowered
    );
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::StaleRebindReadmission,
        )
        .contains(rebound.production_transition())
    );
    assert!(rebound.into_rebound().is_ok());
}

#[test]
fn lowering_progression_requires_exact_coverage_for_readmission_capability() {
    let denial = super::tests_support::expect_readmission_coverage_denial();
    assert!(matches!(
        denial,
        S8AccessLoweringDenied::CoverageDenied { .. }
    ));
}

#[test]
fn lowering_progression_denies_exact_but_wrong_current_coverage_for_readmission() {
    let (lifecycle, key_domain) = admit_page_scope();
    let selected_coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(21).unwrap(),
        )
        .unwrap();
    let wrong_exact_coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(22).unwrap(),
        )
        .unwrap();
    let degraded = crate::access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(selected_coverage.require_exact().unwrap())
                .with_budget_rows(10),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            degraded,
            S8PreExecutionBudgetEnvelope::terminal_default(),
        )
        .unwrap();
    let stale = access_lowering()
        .admit_ready(access_lowering().lower_selected(selected).into_lowered())
        .into_stale()
        .expect("degraded scan should be stale");

    assert!(matches!(
        layout_execution_freshness().admit_current_for_stale(
            &stale,
            lifecycle,
            key_domain,
            wrong_exact_coverage
        ),
        Err(S8AccessLoweringDenied::CurrentCoverageMismatch { .. })
    ));
}

#[test]
fn lowering_progression_denies_exact_but_wrong_current_coverage_for_rebind() {
    let (lifecycle, key_domain) = admit_page_scope();
    let selected_coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(23).unwrap(),
        )
        .unwrap();
    let wrong_exact_coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(24).unwrap(),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_planning()
                .require_exact_range_access(selected_coverage)
                .unwrap(),
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let rebind = access_lowering()
        .require_rebind(access_lowering().lower_selected(selected).into_lowered())
        .into_required()
        .expect("rebind should be required");

    assert!(matches!(
        layout_execution_freshness().admit_rebind_for_execution(
            &rebind,
            lifecycle,
            key_domain,
            wrong_exact_coverage
        ),
        Err(S8AccessLoweringDenied::CurrentCoverageMismatch { .. })
    ));
}

pub(crate) fn assert_owner_transition_handoff_equivalence() {
    lowering_progression_surfaces_stale_and_readmitted_outcomes_for_degraded_scan();
    lowering_progression_supports_explicit_rebind_boundary();
}

fn admit_page_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn admit_wal_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(crate) fn exercise_owner_outcome_cases() {
    lowering_progression_defers_lsm_point_paths_until_runtime_lease();
    lowering_progression_surfaces_stale_and_readmitted_outcomes_for_degraded_scan();
    lowering_progression_denies_mismatched_readmission_witnesses();
    lowering_progression_supports_explicit_rebind_boundary();
    lowering_progression_requires_exact_coverage_for_readmission_capability();
    lowering_progression_denies_exact_but_wrong_current_coverage_for_readmission();
    lowering_progression_denies_exact_but_wrong_current_coverage_for_rebind();
}
