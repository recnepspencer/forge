use super::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    catalog_closeout_with_loop_cycles_postures, empty_touched_closure,
    legality_support_missing_selected_legality_plan,
    legality_support_missing_selected_validator_receipt, loop_cycles_touched_closure,
    query_support_missing_native_read, query_support_missing_native_write,
    query_support_missing_projection_consumption, unrelated_geometry_touched_closure,
};
use super::{
    DerivedInvalidationDenialKind, DerivedInvalidationDensityPolicy,
    DerivedInvalidationExecutionAdmission, DerivedInvalidationPlannedDisposition,
    DerivedInvalidationSelectedPlan, DerivedInvalidationSelectionErrorKind,
};
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyQueryReceiptPosture,
};
use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;

#[test]
fn same_touched_closure_selects_same_plan_regardless_operator_family_name() {
    let catalog = catalog_closeout();
    let first = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("operator-family-a"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();
    let second = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("operator-family-b"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_eq!(first.selected_plan_digest(), second.selected_plan_digest());
    assert_eq!(first.counters().caller_owned_graph_work_count(), 0);
}

#[test]
fn unrelated_product_families_remain_unaffected_with_zero_execution_work() {
    let catalog = catalog_closeout();
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_selected_families(
        &plan,
        &[
            DerivedTopologyProductFamilyIdentity::MaterializedGraph,
            DerivedTopologyProductFamilyIdentity::TraversalViews,
            DerivedTopologyProductFamilyIdentity::LoopCycles,
            DerivedTopologyProductFamilyIdentity::ShellViews,
            DerivedTopologyProductFamilyIdentity::VertexDisks,
            DerivedTopologyProductFamilyIdentity::WireViews,
        ],
    );
    assert_unaffected_families(&plan, &[DerivedTopologyProductFamilyIdentity::RadialRings]);
    assert_eq!(plan.counters().candidate_product_count(), 7);
    assert_eq!(plan.counters().matched_product_count(), 6);
    assert_eq!(plan.counters().invalidated_product_count(), 6);
    assert_eq!(plan.counters().incremental_update_count(), 3);
    assert_eq!(plan.counters().bounded_rebuild_count(), 3);
    assert!(plan
        .unaffected_rows()
        .iter()
        .all(|row| row.execution_work_count() == 0));
    assert_eq!(plan.counters().whole_view_fallback_count(), 0);
}

#[test]
fn capped_phase_two_residue_is_projected_into_selected_plan() {
    let catalog = catalog_closeout();
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_eq!(plan.counters().residue_product_count(), 1);
    let residue = plan
        .residue_rows()
        .first()
        .expect("phase two capped residue must remain visible");
    assert_eq!(
        residue.residue_label(),
        "phase-two-certification-bootstrap-capped-residue"
    );
    assert_eq!(
        residue.capped_count(),
        catalog.catalog().phase_two_seed().capped_residue_count()
    );
}

#[test]
fn selected_plan_carries_shared_locality_routing_contract() {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_eq!(
        plan.routing_contract().overlap_identity().category(),
        ConflictOverlapCategory::Locality
    );
    assert_eq!(
        plan.routing_contract()
            .overlap_identity()
            .locality_identity()
            .expect("locality overlap carries locality")
            .authority_digest(),
        plan.touched_closure_digest()
    );
}

#[test]
fn missing_projection_consumption_support_denies_before_rebuild() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::ProjectionConsumptionRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &query_support_missing_projection_consumption(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_missing_query_denial(&plan);
}

#[test]
fn missing_native_read_support_denies_before_rebuild() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_missing_query_denial(&plan);
}

#[test]
fn missing_native_write_support_denies_before_rebuild() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::NativeWriteReceiptRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &query_support_missing_native_write(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_missing_query_denial(&plan);
}

#[test]
fn missing_selected_legality_plan_denies_before_rebuild() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_missing_legality_denial(&plan);
}

#[test]
fn missing_selected_validator_receipt_denies_before_rebuild() {
    let catalog = catalog_closeout_with_loop_cycles_postures(
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
        DerivedTopologyLegalityReceiptPosture::SelectedValidatorReceiptRequired,
    );
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &legality_support_missing_selected_validator_receipt(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_missing_legality_denial(&plan);
}

#[test]
fn untouched_geometry_closure_does_not_select_topology_products() {
    let catalog = catalog_closeout();
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog,
        &unrelated_geometry_touched_closure(),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();

    assert_eq!(plan.selected_rows().len(), 0);
    assert_eq!(
        plan.counters().candidate_product_count(),
        plan.counters().unaffected_product_count()
    );
    assert_eq!(plan.counters().invalidated_product_count(), 0);
    assert_eq!(plan.counters().touched_entity_count(), 1);
    assert_eq!(plan.counters().touched_relation_count(), 0);
    assert_eq!(plan.counters().touched_relation_kind_count(), 0);
    assert_eq!(plan.counters().touched_aspect_count(), 1);
    assert_eq!(plan.counters().touched_scope_count(), 1);
}

#[test]
fn empty_touched_closure_is_rejected_before_selection() {
    let error = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &empty_touched_closure(),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        DerivedInvalidationSelectionErrorKind::TouchedClosureEmpty
    );
}

fn assert_missing_query_denial(plan: &DerivedInvalidationSelectedPlan) {
    assert_denied_before_rebuild(plan);
    assert!(plan.denied_rows().iter().any(|row| {
        row.kind() == DerivedInvalidationDenialKind::MissingQuerySupport
            && row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles
            && row.required_query_posture().is_some()
    }));
}

fn assert_missing_legality_denial(plan: &DerivedInvalidationSelectedPlan) {
    assert_denied_before_rebuild(plan);
    assert!(plan.denied_rows().iter().any(|row| {
        row.kind() == DerivedInvalidationDenialKind::MissingLegalitySupport
            && row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles
            && row.required_legality_posture().is_some()
    }));
}

fn assert_denied_before_rebuild(plan: &DerivedInvalidationSelectedPlan) {
    assert_eq!(
        plan.execution_admission(),
        DerivedInvalidationExecutionAdmission::Denied
    );
    assert!(plan.selected_rows().is_empty());
    assert_eq!(plan.counters().caller_owned_graph_work_count(), 0);
    assert_eq!(plan.counters().whole_view_fallback_count(), 0);
}

fn assert_selected_families(
    plan: &DerivedInvalidationSelectedPlan,
    expected: &[DerivedTopologyProductFamilyIdentity],
) {
    let actual = plan
        .selected_rows()
        .iter()
        .map(|row| row.family_identity())
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert!(plan.selected_rows().iter().all(|row| {
        row.query_receipt_digest().is_some()
            && row.legality_receipt_digest().is_some()
            && matches!(
                row.planned_disposition(),
                DerivedInvalidationPlannedDisposition::IncrementalUpdate
                    | DerivedInvalidationPlannedDisposition::BoundedRebuild
            )
    }));
}

fn assert_unaffected_families(
    plan: &DerivedInvalidationSelectedPlan,
    expected: &[DerivedTopologyProductFamilyIdentity],
) {
    let actual = plan
        .unaffected_rows()
        .iter()
        .map(|row| row.family_identity())
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(actual, expected);
}
