use worth_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;
use worth_store_layout_indexes::{
    access_shapes, layout_rebuild_admission, layout_rebuild_execution,
    DerivedIndexRebuildSourceInput, ObserveOwnerCase,
};

use super::super::super::LayoutOwnerObservationLedger;
use super::fixture_inputs;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let strategy = fixture_inputs::btree_strategy();
    let source = fixture_inputs::root_source(11);
    let materialization = fixture_inputs::root_materialization(&strategy, &source);

    let admitted = layout_rebuild_admission().admit_plan(fixture_inputs::root_request(
        &strategy,
        materialization.clone(),
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: source.clone(),
        },
    ));
    ledger.record_derived_index_rebuild_admission(admitted.owner_case_observation());
    let plan = admitted
        .into_admitted()
        .expect("ordinary root rebuild must admit");
    ledger.record_corruption_classification(plan.corruption().owner_case_observation());
    let execution = layout_rebuild_execution().execute(plan);
    ledger.record_derived_index_rebuild_execution(execution.owner_case_observation());

    let cases = [
        (
            "denied.strategy",
            fixture_inputs::request_with(
                &strategy,
                LayoutStrategyFamily::BaselineLsmWriteOptimized,
                access_shapes()
                    .rebuild_read_declaration(
                        worth_store_layout_indexes::AccessLaneClassification::Maintenance,
                    )
                    .unwrap(),
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: source.clone(),
                },
            ),
        ),
        (
            "denied.shape",
            fixture_inputs::request_with(
                &strategy,
                strategy.admitted_strategy().family(),
                access_shapes().point_lookup_declaration(),
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: source.clone(),
                },
            ),
        ),
        (
            "denied.source_not_authority",
            fixture_inputs::root_request(
                &strategy,
                materialization.clone(),
                DerivedIndexRebuildSourceInput::DiagnosticReport,
            ),
        ),
        (
            "denied.source_strategy",
            fixture_inputs::root_request(
                &fixture_inputs::lsm_strategy(),
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: source.clone(),
                },
            ),
        ),
        (
            "denied.source_authority",
            fixture_inputs::root_request(
                &strategy,
                materialization.clone(),
                DerivedIndexRebuildSourceInput::PhysicalRootManifest {
                    source: fixture_inputs::root_source_for_store(
                        11,
                        &worth_store_test_support::foreign_layout_physical_store_identity(),
                    ),
                },
            ),
        ),
    ];
    for (expected, request) in cases {
        let outcome = layout_rebuild_admission().admit_plan(request);
        assert_eq!(outcome.case_id().as_str(), expected);
        ledger.record_derived_index_rebuild_admission(outcome.owner_case_observation());
    }

    let lsm_strategy = fixture_inputs::lsm_strategy();
    let lsm_materialization = fixture_inputs::lsm_materialization();
    let current_security = super::super::strategy::wal_security();
    let wrong_identity =
        fixture_inputs::wal_source_with_next_identity(&lsm_materialization, &current_security);
    let outcome = layout_rebuild_admission().admit_plan(fixture_inputs::root_request(
        &lsm_strategy,
        lsm_materialization.clone(),
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: wrong_identity,
        },
    ));
    assert_eq!(outcome.case_id().as_str(), "denied.source_identity");
    ledger.record_derived_index_rebuild_admission(outcome.owner_case_observation());

    let tenant_security = super::super::super::fixture_admission::security_scope(
        worth_store_test_support::SecurityScopeFixtureAuthority::Current,
        worth_store_security::StoreKeyScope::WalCheckpointEnvelope,
        worth_store_security::StoreTenantScope::TenantPhysicalBoundary,
        worth_store_security::StoreAuthenticityRequirement::required(
            worth_store_security::StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        worth_store_security::StoreCustodyPosture::InternalStoreCustody,
    );
    let mismatched_source =
        fixture_inputs::wal_source_for_materialization(&lsm_materialization, &tenant_security);
    let outcome = layout_rebuild_admission().admit_plan(fixture_inputs::root_request(
        &lsm_strategy,
        lsm_materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord {
            source_witness: mismatched_source,
        },
    ));
    assert_eq!(outcome.case_id().as_str(), "denied.source_security");
    ledger.record_derived_index_rebuild_admission(outcome.owner_case_observation());
}
