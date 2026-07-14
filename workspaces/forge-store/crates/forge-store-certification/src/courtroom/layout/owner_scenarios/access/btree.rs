use forge_store_budgets::{PreExecutionBudgetEnvelope, PreExecutionBudgetScope};
use forge_store_layout_indexes::{
    layout_btree_recovery, layout_read_runtime, BTreeReplayLocation, BTreeReplayPhysicalSource,
    BTreeReplayRequest, ObserveOwnerCase, PageLookupRequest,
};
use forge_store_physical_format::PhysicalRecordSlot;
use forge_store_test_support::{
    admitted_layout_bootstrap_catalog, advanced_admitted_layout_bootstrap_catalog,
    baseline_btree_probe_slot, deterministic_baseline_btree_read_source,
    deterministic_btree_replay_world, deterministic_corrupt_leaf_btree_read_source,
    deterministic_cross_store_btree_read_source,
    deterministic_left_partition_violation_btree_read_source,
    deterministic_noncanonical_leaf_btree_read_source,
    deterministic_right_partition_violation_btree_read_source,
    deterministic_stale_child_btree_read_source, SecurityScopeFixtureAuthority,
};

use super::super::LayoutOwnerObservationLedger;
use super::execution::AccessScenarioEvidence;
use super::fixture_values::{page, page_security, record_slot, root_security, segment};

pub(super) fn execute_replay(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let page_security = page_security(SecurityScopeFixtureAuthority::Current);
    let root_security = root_security();
    let world = deterministic_btree_replay_world();
    let good_source = replay_source(&world, world.replay_artifact().store_identity().clone());
    let foreign_source = replay_source(
        &world,
        forge_store_test_support::foreign_layout_physical_store_identity(),
    );
    let zero_budget =
        PreExecutionBudgetEnvelope::new(PreExecutionBudgetScope::Maintenance, 0, 0, 0, 0, 0);

    for (security, budget, source) in [
        (
            page_security.witnesses(),
            PreExecutionBudgetEnvelope::maintenance_default(),
            good_source.clone(),
        ),
        (
            root_security.witnesses(),
            PreExecutionBudgetEnvelope::maintenance_default(),
            good_source.clone(),
        ),
        (page_security.witnesses(), zero_budget, good_source),
        (
            page_security.witnesses(),
            PreExecutionBudgetEnvelope::maintenance_default(),
            foreign_source,
        ),
    ] {
        let outcome = layout_btree_recovery().replay(BTreeReplayRequest::new(
            &catalog,
            security,
            BTreeReplayLocation::new(segment(7), page(9)),
            budget,
            source,
        ));
        ledger.record_btree_replay_execution(outcome.owner_case_observation());
    }
}

fn replay_source(
    world: &forge_store_test_support::DeterministicBTreeReplayWorld,
    expected_store: forge_store_physical_format::PhysicalStoreIdentity,
) -> BTreeReplayPhysicalSource {
    let root = world.root_reference();
    BTreeReplayPhysicalSource::new(
        world.readiness().clone(),
        root,
        world.replay_artifact().clone(),
        expected_store,
        forge_store_test_support::harness::recovery::redo_replay::
            checkpoint_plus_tail_source_for_root(20, 30, root),
    )
}

pub(super) fn execute_lookup(ledger: &mut LayoutOwnerObservationLedger) -> AccessScenarioEvidence {
    let catalog = admitted_layout_bootstrap_catalog();
    let advanced = advanced_admitted_layout_bootstrap_catalog();
    let security = page_security(SecurityScopeFixtureAuthority::Current);

    for request in [
        page_request(&catalog, &security, baseline_btree_probe_slot()),
        page_request(&catalog, &security, baseline_btree_probe_slot())
            .against_current_catalog(&advanced)
            .against_current_source(deterministic_cross_store_btree_read_source()),
    ] {
        let outcome = layout_read_runtime()
            .prepare_page_lookup(request)
            .expect("ordinary page lookup must reach readiness");
        ledger.record_btree_lookup_readiness(outcome.owner_case_observation());
    }

    let mut performance = None;
    let mut durable = None;
    for (source, slot) in [
        (
            deterministic_baseline_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (deterministic_baseline_btree_read_source(), record_slot(15)),
        (
            deterministic_corrupt_leaf_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_stale_child_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_noncanonical_leaf_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_left_partition_violation_btree_read_source(),
            baseline_btree_probe_slot(),
        ),
        (
            deterministic_right_partition_violation_btree_read_source(),
            record_slot(15),
        ),
    ] {
        let outcome = layout_read_runtime()
            .execute_page_lookup(page_request_with_source(&catalog, &security, slot, source))
            .expect("ordinary page lookup must execute");
        if performance.is_none()
            && matches!(
                outcome.view(),
                forge_store_layout_indexes::BTreeLookupExecutionView::Found(_)
            )
        {
            performance = outcome
                .counter_receipt()
                .map(forge_store_layout_indexes::LayoutAccessPerformanceReceipt::from_btree_lookup);
            ledger.record_btree_lookup_execution(outcome.owner_case_observation());
            durable = super::super::durable_observation::BTreeDurableObservationSource::
                from_found_execution(
                    outcome
                        .into_result()
                        .expect("found outcome retains stable execution evidence"),
                );
            continue;
        }
        ledger.record_btree_lookup_execution(outcome.owner_case_observation());
    }
    AccessScenarioEvidence {
        performance: performance
            .expect("ordinary owner scenarios include one successful B-tree lookup"),
        btree: durable.expect("successful B-tree lookup retains durable observation evidence"),
    }
}

fn page_request<'a>(
    catalog: &'a forge_store_layout_indexes::BootstrapCatalogReadAdmission,
    security: &'a forge_store_security::StoreAdmittedSecurityScope,
    slot: PhysicalRecordSlot,
) -> PageLookupRequest<'a> {
    page_request_with_source(
        catalog,
        security,
        slot,
        deterministic_baseline_btree_read_source(),
    )
}

fn page_request_with_source<'a>(
    catalog: &'a forge_store_layout_indexes::BootstrapCatalogReadAdmission,
    security: &'a forge_store_security::StoreAdmittedSecurityScope,
    slot: PhysicalRecordSlot,
    source: forge_store_layout_indexes::BaselineBTreeReadSource,
) -> PageLookupRequest<'a> {
    PageLookupRequest::new(
        catalog,
        security.witnesses(),
        segment(7),
        page(9),
        slot,
        PreExecutionBudgetEnvelope::foreground_default(),
        source,
    )
}
