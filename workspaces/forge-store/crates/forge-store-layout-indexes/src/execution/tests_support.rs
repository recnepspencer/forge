use crate::facade::{access_planning, deterministic_plan_selection, layout_execution_freshness};
use crate::strategy::tests_support::admit_phase_five_scope;
use crate::{
    access_lowering, S8AccessLoweringDenied, S8AccessLoweringOutcome, S8DegradedExactScanRequest,
};
use crate::execution::S8AdmittedExecutedCounters;
use forge_store_budgets::S8PreExecutionBudgetEnvelope;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

pub(super) fn admit_page_scope() -> (
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

pub(super) fn admit_wal_scope() -> (
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
    let lowered = match access_lowering().lower_selected(selected) {
        S8AccessLoweringOutcome::Lowered(lowered) => lowered,
        other => panic!("expected lowered outcome, got {other:?}"),
    };
    match access_lowering().admit_ready(lowered) {
        S8AccessLoweringOutcome::Ready(ready) => ready,
        other => panic!("expected ready outcome, got {other:?}"),
    }
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
    let lowered = match access_lowering().lower_selected(selected) {
        S8AccessLoweringOutcome::Lowered(lowered) => lowered,
        other => panic!("expected lowered outcome, got {other:?}"),
    };
    match access_lowering().admit_ready(lowered) {
        S8AccessLoweringOutcome::Ready(ready) => ready,
        other => panic!("expected ready outcome, got {other:?}"),
    }
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
    let lowered = match access_lowering().lower_selected(selected) {
        S8AccessLoweringOutcome::Lowered(lowered) => lowered,
        other => panic!("expected lowered outcome, got {other:?}"),
    };
    match access_lowering().admit_ready(lowered) {
        S8AccessLoweringOutcome::Ready(ready) => ready,
        other => panic!("expected ready outcome, got {other:?}"),
    }
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
    let stale =
        match access_lowering().admit_ready(match access_lowering().lower_selected(selected) {
            S8AccessLoweringOutcome::Lowered(lowered) => lowered,
            other => panic!("expected lowered outcome, got {other:?}"),
        }) {
            S8AccessLoweringOutcome::Stale(stale) => stale,
            other => panic!("expected stale outcome, got {other:?}"),
        };

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
