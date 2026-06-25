use super::super::{
    close_materialized_graph_migration_slice, MaterializedGraphExecutionInput,
    MaterializedGraphMigrationCloseout, MaterializedGraphReadSource,
    MaterializedGraphReadStageExecutor, MaterializedGraphReadStageReceipt,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    legality_support_missing_selected_legality_plan, loop_cycles_touched_closure,
    query_support_missing_native_read, unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};

pub(super) fn selected_materialized_graph_plan() -> DerivedInvalidationSelectedPlan {
    selected_materialized_graph_plan_with_key("materialized-graph-touch")
}

pub(super) fn selected_materialized_graph_plan_with_key(
    semantic_family_key: &'static str,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure(semantic_family_key),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

#[allow(dead_code)]
pub(super) fn unrelated_geometry_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &unrelated_geometry_touched_closure(),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn materialized_graph_plan_missing_native_read() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("materialized-graph-missing-native-read"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn materialized_graph_plan_missing_legality() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("materialized-graph-missing-legality"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn selected_materialized_receipt(
    plan: &DerivedInvalidationSelectedPlan,
) -> MaterializedGraphReadStageReceipt {
    MaterializedGraphReadStageExecutor::execute(plan, selected_materialized_read_source()).unwrap()
}

pub(super) fn selected_materialized_read_source() -> MaterializedGraphReadSource {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    MaterializedGraphReadSource::from_topology_view_with_selected_prefix(&topology, 2, 1).unwrap()
}

pub(super) fn close_materialized_graph_slice(
    receipt: MaterializedGraphReadStageReceipt,
) -> MaterializedGraphMigrationCloseout {
    let plan = selected_materialized_graph_plan();
    let input =
        MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(&plan, receipt).unwrap();
    close_materialized_graph_migration_slice(&plan, input).unwrap()
}

pub(super) fn assert_plan_selects_materialized_graph(plan: &DerivedInvalidationSelectedPlan) {
    assert!(plan.selected_rows().iter().any(
        |row| row.family_identity() == DerivedTopologyProductFamilyIdentity::MaterializedGraph
    ));
}
