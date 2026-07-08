use super::{
    S8DeterministicSelectionRule, S8PlanningCapabilityGrant, S8SelectionCandidateEligibility,
};
use crate::facade::{access_planning, deterministic_plan_selection};
use crate::strategy::tests_support::admit_phase_five_scope;
use crate::{
    access_shapes, S8DegradedExactScanRequest, S8LayoutStrategyFamily, S8PlanSelectionDenied,
    S8SelectionCandidateOutcome,
};
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::LogSequenceNumber;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn deterministic_selection_keeps_btree_fingerprint_stable_for_exact_range_reads() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(7).unwrap(),
        )
        .unwrap();
    let access_shape = access_planning()
        .require_exact_range_access(coverage)
        .unwrap();

    let first = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_shape,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let replayed = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_shape,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();

    assert_eq!(first.fingerprint(), replayed.fingerprint());
    assert_eq!(
        first.selected_family(),
        S8LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(
        first.selection_rule(),
        S8DeterministicSelectionRule::SoleEligibleCandidate
    );
    assert_eq!(
        first.primary_candidate().outcome(),
        S8SelectionCandidateOutcome::Eligible(S8SelectionCandidateEligibility::RegistryAdmitted {
            granted_capability: S8PlanningCapabilityGrant::OrderedRange,
            planned_counter_envelope: first.planned_counter_envelope(),
        })
    );
}

#[test]
fn deterministic_selection_selects_lsm_for_exact_wal_point_paths() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            LogSequenceNumber::new(17),
        )
        .unwrap();
    let access_shape = access_planning()
        .require_exact_point_access(coverage)
        .unwrap();

    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_shape,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();

    assert_eq!(
        selected.selected_family(),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized
    );
    assert_eq!(
        selected.budget_receipt().scope(),
        forge_store_budgets::S8PreExecutionBudgetScope::Foreground
    );
    assert_eq!(
        selected.budget_receipt().plan_binding(),
        selected.fingerprint().plan_binding()
    );
}

#[test]
fn deterministic_selection_denies_when_budget_is_exceeded_before_execution() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(11).unwrap(),
        )
        .unwrap();
    let degraded = access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(coverage.require_exact().unwrap())
                .with_budget_rows(10_000),
        )
        .unwrap();

    let denial = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            degraded,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap_err();

    assert!(matches!(denial, S8PlanSelectionDenied::BudgetDenied(_)));
}

#[test]
fn degraded_exact_scan_uses_explicit_rule_and_plan_bound_budget_receipt() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(9).unwrap(),
        )
        .unwrap();
    let degraded = access_shapes()
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

    assert_eq!(
        selected.selection_rule(),
        S8DeterministicSelectionRule::ExplicitDegradedExactScan
    );
    assert_eq!(
        selected.primary_candidate().outcome(),
        S8SelectionCandidateOutcome::Eligible(
            S8SelectionCandidateEligibility::ExplicitDegradedExactScan {
                planned_counter_envelope: selected.planned_counter_envelope(),
                budget_rows: 8,
            },
        )
    );
    assert_eq!(
        selected.budget_receipt().plan_binding(),
        selected.fingerprint().plan_binding()
    );
}

#[test]
fn fingerprint_changes_when_selected_plan_basis_changes_within_same_family() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(13).unwrap(),
        )
        .unwrap();
    let point = access_planning()
        .require_exact_point_access(coverage)
        .unwrap();
    let range = access_planning()
        .require_exact_range_access(coverage)
        .unwrap();

    let point_plan = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            point,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let range_plan = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            range,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();

    assert_eq!(
        point_plan.selected_family(),
        S8LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(
        range_plan.selected_family(),
        S8LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_ne!(point_plan.fingerprint(), range_plan.fingerprint());
    assert_ne!(
        point_plan.budget_receipt().plan_binding(),
        range_plan.budget_receipt().plan_binding()
    );
}

#[test]
fn deterministic_selection_denies_when_no_strategy_is_eligible() {
    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalRootManifest,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(5).unwrap(),
        )
        .unwrap();
    let access_shape = access_planning()
        .require_exact_range_access(coverage)
        .unwrap();

    let denial = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_shape,
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap_err();

    assert_eq!(denial, S8PlanSelectionDenied::NoEligibleAlternative);
}

#[test]
fn exact_multi_range_and_grouped_prefix_paths_fail_closed_without_btree_counter_lane() {
    use crate::strategy::tests_support::admit_btree_page_strategy;
    use crate::{S8GroupedPrefixBasis, S8MultiRangeBasis};

    let (lifecycle, key_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(19).unwrap(),
        )
        .unwrap();
    let btree = admit_btree_page_strategy();

    assert_eq!(
        btree.planned_counter_envelope_for(crate::S8AccessShapeDetail::MultiRangeLookup(
            S8MultiRangeBasis::DeclaredDisjointRangeSet,
        )),
        None
    );
    assert_eq!(
        btree.planned_counter_envelope_for(crate::S8AccessShapeDetail::GroupedPrefixLookup(
            S8GroupedPrefixBasis::CanonicalGroupedPrefixes,
        )),
        None
    );
    assert_eq!(
        deterministic_plan_selection()
            .select_with_budget(
                lifecycle,
                key_domain,
                access_shapes()
                    .multi_range_lookup(coverage, S8MultiRangeBasis::DeclaredDisjointRangeSet)
                    .unwrap(),
                S8PreExecutionBudgetEnvelope::foreground_default(),
            )
            .unwrap_err(),
        S8PlanSelectionDenied::NoEligibleAlternative
    );
    assert_eq!(
        deterministic_plan_selection()
            .select_with_budget(
                lifecycle,
                key_domain,
                access_shapes()
                    .grouped_prefix_lookup(
                        coverage,
                        S8GroupedPrefixBasis::CanonicalGroupedPrefixes,
                    )
                    .unwrap(),
                S8PreExecutionBudgetEnvelope::foreground_default(),
            )
            .unwrap_err(),
        S8PlanSelectionDenied::NoEligibleAlternative
    );
}
