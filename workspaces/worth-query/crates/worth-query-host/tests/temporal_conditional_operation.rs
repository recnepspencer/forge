#[path = "temporal_conditional_operation/adapters.rs"]
mod adapters;
#[path = "temporal_conditional_operation/contract.rs"]
mod contract;
#[path = "temporal_conditional_operation/courtroom.rs"]
mod courtroom;
#[path = "temporal_conditional_operation/courtroom_lifecycle.rs"]
mod courtroom_lifecycle;
#[path = "temporal_conditional_operation/courtroom_settlement.rs"]
mod courtroom_settlement;
#[path = "temporal_conditional_operation/courtroom_support.rs"]
mod courtroom_support;
#[path = "temporal_conditional_operation/schema.rs"]
mod schema;
#[path = "temporal_conditional_operation/world.rs"]
mod world;

use courtroom_support::observe;
use world::CourtroomWorld;
use worth_query_host::facade::primary_graph;

#[test]
fn application_readiness_reports_current_query_basis_without_leaking_a_lease() {
    let world = CourtroomWorld::publish("ready");
    let observer = world.application.application_query_basis_observer();
    let before = observer.observe();

    let readiness = world
        .application
        .inspect_application_readiness()
        .expect("the published application basis should be inspectable");

    assert_eq!(
        readiness.schema_binding(),
        &world.application.installed_schema().binding_identity()
    );
    assert!(readiness
        .basis_token()
        .starts_with("basis:query-primary-graph-v2:"));
    let repeated = world
        .application
        .inspect_application_readiness()
        .expect("repeated readiness inspection should remain available");
    assert_eq!(
        repeated.basis_token(),
        readiness.basis_token(),
        "readiness inspection must not manufacture a new optimistic basis"
    );
    assert_eq!(
        world
            .application
            .application_basis_token(readiness.basis_identity())
            .expect("the observed query basis should render through its owning runtime"),
        readiness.basis_token(),
        "a result must retain its exact observed basis instead of rediscovering head"
    );
    let after = observer.observe();
    assert_eq!(after.active(), before.active());
    assert_eq!(after.acquisitions(), before.acquisitions() + 2);
}

#[test]
fn successor_generation_requires_fresh_typed_rebinding() {
    let mut world = CourtroomWorld::publish("ready");
    let successor = std::sync::Arc::new(world.installation.successor_generation());

    let denial = world
        .application
        .reinstall_conditional_runtime_for_installation(successor)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        primary_graph::WorthQueryConditionalRuntimeInstallationDenialKind::RebindRequired
    );
}

#[test]
fn reconstruction_panic_restores_runtime_owners_for_retry() {
    let mut world = CourtroomWorld::publish("ready");
    world.reconstruction_panic.set(true);

    assert!(world.application.reinstall_conditional_runtime().is_err());

    world.reconstruction_panic.set(false);
    assert!(world.application.reinstall_conditional_runtime().is_ok());
    let receipt = observe(&mut world);
    assert_eq!(receipt.committed_operation_count(), 1);
}

#[test]
fn host_installs_and_executes_a_due_temporal_application_operation() {
    courtroom::host_installs_and_executes_due_operation();
}

#[test]
fn temporal_wake_repairs_durable_settlement_before_exact_retirement() {
    courtroom_settlement::temporal_wake_repairs_durable_settlement_before_exact_retirement();
}

#[test]
fn temporal_wake_retries_query_publication_after_settlement_repair() {
    courtroom_settlement::temporal_wake_retries_query_publication_after_settlement_repair();
}

#[test]
fn temporal_identity_work_is_cold_or_fresh_admission_only() {
    courtroom::temporal_identity_work_is_cold_or_fresh_admission_only();
}

#[test]
fn future_temporal_operation_waits_until_its_due_coordinate() {
    courtroom::future_temporal_operation_waits_until_due();
}

#[test]
fn unrelated_rows_do_not_expand_conditional_observation_work() {
    courtroom::unrelated_rows_do_not_expand_conditional_observation_work();
}

#[test]
fn cancellation_after_publication_retires_the_stale_wake_without_effects() {
    courtroom::cancellation_after_publication_retires_stale_wake();
}

#[test]
fn active_successor_revision_replaces_the_predecessor_wake() {
    courtroom::active_successor_revision_replaces_predecessor_wake();
}

#[test]
fn reinstallation_reconstructs_active_authoritative_work() {
    courtroom_lifecycle::reinstallation_reconstructs_active_authoritative_work();
}

#[test]
fn reconstruction_work_scales_with_the_projection_not_unrelated_rows() {
    courtroom_lifecycle::reconstruction_work_ignores_unrelated_rows();
}

#[test]
fn reinstallation_restores_no_cancelled_or_completed_work() {
    courtroom_lifecycle::reinstallation_restores_no_terminal_work();
}

#[test]
fn reinstallation_after_eligibility_retries_freshly() {
    courtroom_lifecycle::reinstallation_after_eligibility_retries_freshly();
}

#[test]
fn reinstallation_after_commit_cannot_duplicate_the_effect() {
    courtroom_lifecycle::reinstallation_after_commit_cannot_duplicate_effect();
}

#[test]
fn reinstallation_revokes_captured_granular_batches() {
    courtroom_lifecycle::reinstallation_revokes_captured_granular_batches();
}

#[test]
fn closing_the_runtime_releases_conditional_inventory_and_revokes_handles() {
    courtroom_lifecycle::closing_runtime_releases_inventory_and_revokes_handles();
}

#[test]
fn dropping_the_runtime_releases_exact_conditional_inventory() {
    courtroom_lifecycle::dropping_runtime_releases_exact_inventory();
}

#[test]
fn suppressed_wake_is_reconsidered_after_authoritative_truth_changes() {
    courtroom::suppressed_wake_is_reconsidered_after_truth_change();
}

#[test]
fn precondition_panic_isolated_and_retry_succeeds() {
    courtroom::precondition_panic_isolated_and_retry_succeeds();
}

#[test]
fn predicate_panic_does_not_corrupt_runtime_owners() {
    courtroom::predicate_panic_does_not_corrupt_runtime_owners();
}

#[test]
fn duplicate_reordered_and_foreign_clocks_fail_closed() {
    courtroom::duplicate_reordered_and_foreign_clocks_fail_closed();
}

#[test]
fn provider_replacement_requires_fresh_runtime_publication() {
    courtroom::provider_replacement_requires_fresh_runtime_publication();
}
