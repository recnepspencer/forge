use crate::access::execution::S8AdmittedExecutedCounters;
use crate::facade::{access_planning, deterministic_plan_selection, layout_execution_freshness};
use crate::strategy::tests_support::admit_strategy_scope;
use crate::{access_lowering, S8AccessLoweringDenied, S8DegradedExactScanRequest};
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

pub(crate) fn admit_page_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_strategy_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn admit_wal_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_strategy_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn ready_exact_point_plan(epoch: u64) -> crate::S8ExecutionReadyAccessReceipt {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(epoch).unwrap(),
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
    access_lowering()
        .admit_ready(lowered)
        .into_ready()
        .expect("point plan should be ready")
}

pub(super) fn ready_exact_prefix_plan(epoch: u64) -> crate::S8ExecutionReadyAccessReceipt {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(epoch).unwrap(),
        )
        .unwrap();
    let selected = deterministic_plan_selection()
        .select_with_budget(
            lifecycle,
            key_domain,
            access_planning()
                .require_exact_prefix_access(coverage)
                .unwrap(),
            S8PreExecutionBudgetEnvelope::foreground_default(),
        )
        .unwrap();
    let lowered = access_lowering().lower_selected(selected).into_lowered();
    access_lowering()
        .admit_ready(lowered)
        .into_ready()
        .expect("prefix plan should be ready")
}

pub(super) fn ready_exact_range_plan(epoch: u64) -> crate::S8ExecutionReadyAccessReceipt {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(epoch).unwrap(),
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
    access_lowering()
        .admit_ready(lowered)
        .into_ready()
        .expect("range plan should be ready")
}

pub(super) fn expect_readmission_coverage_denial() -> S8AccessLoweringDenied {
    let (lifecycle, key_domain) = admit_page_scope();
    let exact_coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(15).unwrap(),
        )
        .unwrap();
    let stale_coverage = access_planning()
        .stale_root_epoch_coverage(
            lifecycle.declaration(),
            PhysicalEpoch::from_raw(14).unwrap(),
        )
        .unwrap();
    let degraded = crate::access_shapes()
        .explicit_degraded_exact_scan(
            S8DegradedExactScanRequest::new(exact_coverage.require_exact().unwrap())
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
    let lowered = access_lowering().lower_selected(selected).into_lowered();
    let stale = access_lowering()
        .admit_ready(lowered)
        .into_stale()
        .expect("degraded scan should require readmission");

    layout_execution_freshness()
        .admit_current_for_stale(&stale, lifecycle, key_domain, stale_coverage)
        .unwrap_err()
}

pub(super) fn observed_from_snapshot(
    ready: &crate::S8ExecutionReadyAccessReceipt,
    snapshot: crate::S8AccessPathCounterSnapshot,
) -> S8AdmittedExecutedCounters {
    access_lowering()
        .admit_executed_counters(
            ready,
            &super::counter_witness::TestExecutedCounterWitness::new(
                ready.selected().budget_receipt().plan_binding(),
                ready.path_kind(),
                snapshot,
            ),
        )
        .expect("test snapshot should satisfy admitted executed counter evidence")
}

pub(crate) fn execute_budgeted_degraded_exact_scan() -> crate::S8ExecutedAccessReceipt {
    let (lifecycle, key_domain) = admit_page_scope();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                lifecycle.declaration().family(),
            ),
            PhysicalEpoch::from_raw(21).unwrap(),
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
    let stale = access_lowering()
        .admit_ready(lowered)
        .into_stale()
        .expect("degraded scan should require readmission");
    let witness = layout_execution_freshness()
        .admit_current_for_stale(&stale, lifecycle, key_domain, coverage)
        .expect("current exact coverage should admit the degraded scan");
    let ready = access_lowering()
        .readmit_stale(stale, witness)
        .into_readmitted()
        .expect("current witness should readmit degraded scan");
    let observed =
        observed_from_snapshot(&ready, ready.selected().planned_counter_envelope().lookup());
    access_lowering()
        .execute_ready(ready, observed)
        .expect("admitted degraded counters should execute")
}
