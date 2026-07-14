use super::{DeterministicSelectionRule, PlanningCapabilityGrant, SelectionCandidateEligibility};
use crate::facade::{access_planning, deterministic_plan_selection};
use crate::strategy::tests_support::{
    admit_persisted_lsm_scope, admit_strategy_scope, persisted_lsm_materialization,
};
use crate::{
    access_shapes, AccessPlanSelectionDenied, DegradedExactScanRequest, LayoutStrategyFamily,
    SelectionCandidateOutcome,
};
use worth_store_budgets::PreExecutionBudgetEnvelope;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

fn root_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
    _epoch: u64,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    access_planning()
        .admit_current_catalog_root_materialization(family, &catalog)
        .expect("physical catalog must admit exact root materialization")
}

fn wal_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
    _lsn: u64,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    persisted_lsm_materialization(family, &catalog).0
}

#[test]
fn deterministic_selection_keeps_btree_fingerprint_stable_for_exact_range_reads() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let access_shape = access_planning().range_access();

    let first = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 7),
                    access_shape,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_btree_lookup()
        .expect("range request must issue B-tree lookup authority");
    let replayed = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 7),
                    access_shape,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_btree_lookup()
        .expect("replayed range request must issue B-tree lookup authority");

    assert_eq!(first.fingerprint(), replayed.fingerprint());
    assert_eq!(
        first.selected_family(),
        LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(
        first.selection_rule(),
        DeterministicSelectionRule::SoleEligibleCandidate
    );
    assert_eq!(
        first.primary_candidate().outcome(),
        &SelectionCandidateOutcome::Eligible(SelectionCandidateEligibility::RegistryAdmitted {
            granted_capability: PlanningCapabilityGrant::OrderedRange,
            planned_counter_envelope: first.planned_counter_envelope(),
        })
    );
    let admission = first.strategy_admission();
    assert_eq!(
        admission.request().exact_coverage(),
        Some(first.materialization().coverage())
    );
    assert_eq!(
        admission.request().requested_capability(),
        crate::strategy::registry::LayoutRequestedCapability::OrderedRange,
    );
    assert_eq!(
        admission.granted_capability(),
        crate::strategy::registry::LayoutStrategyCapability::OrderedRange,
    );
}

#[test]
fn deterministic_selection_selects_lsm_for_exact_wal_point_paths() {
    let (lifecycle, key_domain) = admit_persisted_lsm_scope();
    let access_shape = access_planning().point_access();

    let selected = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_wal_key(
                        key_domain,
                        worth_store_contracts::WalRecordFamily::DurableMutationIntent,
                        worth_store_wal::StoreWalRecordIdentity::new(1),
                    )
                    .expect("WAL identity must pass ordinary key admission"),
                    wal_materialization(lifecycle, 17),
                    access_shape,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_lsm_lookup()
        .expect("WAL point request must issue LSM lookup authority");

    assert_eq!(
        selected.selected_family(),
        LayoutStrategyFamily::BaselineLsmWriteOptimized
    );
    assert_eq!(
        selected.budget_receipt().scope(),
        worth_store_budgets::PreExecutionBudgetScope::Foreground
    );
}

#[test]
fn deterministic_selection_denies_when_budget_is_exceeded_before_execution() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let degraded = access_shapes()
        .explicit_degraded_exact_scan(DegradedExactScanRequest::new().with_budget_rows(10_000))
        .unwrap();

    let denial = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 11),
                    degraded,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap_err();

    assert!(matches!(denial, AccessPlanSelectionDenied::BudgetDenied(_)));
}

#[test]
fn degraded_exact_scan_uses_explicit_rule_and_plan_bound_budget_receipt() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let degraded = access_shapes()
        .explicit_degraded_exact_scan(DegradedExactScanRequest::new().with_budget_rows(8))
        .unwrap();

    let selected = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 9),
                    degraded,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::terminal_default(),
        )
        .into_degraded()
        .expect("explicit degraded request must issue degraded scan authority");

    assert_eq!(
        selected.selection_rule(),
        DeterministicSelectionRule::ExplicitDegradedExactScan
    );
    assert_eq!(
        selected.primary_candidate().outcome(),
        &SelectionCandidateOutcome::Eligible(
            SelectionCandidateEligibility::ExplicitDegradedExactScan {
                planned_counter_envelope: selected.planned_counter_envelope(),
                budget_rows: 8,
            },
        )
    );
}

#[test]
fn btree_lookup_selection_issues_the_exact_operation_capability() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let outcome = deterministic_plan_selection().select_admitted_with_budget(
        crate::planning::AccessPlanSelector
            .admit_read_request(
                lifecycle,
                crate::keyspace::admit_page_key(
                    key_domain,
                    worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                    worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                )
                .expect("page identity must pass ordinary key admission"),
                root_materialization(lifecycle, 29),
                access_planning().point_access(),
            )
            .expect("test request must pass ordinary admission"),
        PreExecutionBudgetEnvelope::foreground_default(),
    );
    let selected = outcome
        .into_btree_lookup()
        .expect("B-tree point lookup must issue exact B-tree lookup authority");
    assert_eq!(
        selected.selected_family(),
        LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(selected.operation(), crate::BTreeLookupOperation::Point);
    assert_eq!(
        selected.intent().shape(),
        crate::observation::AccessShape::PointLookup
    );
}

#[test]
fn fingerprint_changes_when_selected_plan_basis_changes_within_same_family() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let point = access_planning().point_access();
    let range = access_planning().range_access();

    let point_plan = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 13),
                    point,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_btree_lookup()
        .expect("point request must issue B-tree lookup authority");
    let range_plan = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_page_key(
                        key_domain,
                        worth_store_physical_format::PhysicalSegmentId::from_raw(1).unwrap(),
                        worth_store_physical_format::PhysicalPageId::from_raw(1).unwrap(),
                    )
                    .expect("page identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 13),
                    range,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_btree_lookup()
        .expect("range request must issue B-tree lookup authority");

    assert_eq!(
        point_plan.selected_family(),
        LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(
        range_plan.selected_family(),
        LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_ne!(point_plan.fingerprint(), range_plan.fingerprint());
    assert_ne!(point_plan.fingerprint(), range_plan.fingerprint());
    assert_eq!(point_plan.operation(), crate::BTreeLookupOperation::Point);
    assert_eq!(range_plan.operation(), crate::BTreeLookupOperation::Range);
}

#[test]
fn deterministic_selection_denies_when_no_strategy_is_eligible() {
    let (lifecycle, key_domain) = admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalRootManifest,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let access_shape = access_planning().range_access();

    let denial = deterministic_plan_selection()
        .select_admitted_with_budget(
            crate::planning::AccessPlanSelector
                .admit_read_request(
                    lifecycle,
                    crate::keyspace::admit_root_key(
                        key_domain,
                        worth_store_physical_format::PhysicalRootReference::from_raw(1).unwrap(),
                    )
                    .expect("root identity must pass ordinary key admission"),
                    root_materialization(lifecycle, 5),
                    access_shape,
                )
                .expect("test request must pass ordinary admission"),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap_err();

    assert_eq!(denial, AccessPlanSelectionDenied::NoEligibleAlternative);
}
