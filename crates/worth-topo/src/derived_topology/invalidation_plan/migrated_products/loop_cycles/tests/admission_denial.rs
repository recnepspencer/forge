use super::super::{
    LoopCycleMigrationError, LoopCycleReadSource, LoopCycleReadStageExecutor,
    LoopCycleTouchedBoundaryRows,
};
use super::support::{selected_loop_cycles_plan, source_row, unrelated_geometry_selected_plan};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::unrelated_geometry_touched_closure;

#[test]
fn unrelated_touched_closure_cannot_close_loop_cycle_migration() {
    let plan = unrelated_geometry_selected_plan();

    let read_source = LoopCycleReadSource::from_rows(vec![source_row(1, 1, 3)], 1).unwrap();

    let error = LoopCycleReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::SelectedPlanMissingLoopCycleRow
    );
}

#[test]
fn selected_plan_without_loop_cycle_row_cannot_admit_loop_cycle_input() {
    let plan = unrelated_geometry_selected_plan();

    let read_source = LoopCycleReadSource::from_rows(vec![source_row(1, 1, 3)], 1).unwrap();

    let error = LoopCycleReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::SelectedPlanMissingLoopCycleRow
    );
}

#[test]
fn read_source_rejects_touched_closure_from_different_selected_plan() {
    let plan = selected_loop_cycles_plan("loop-touch-a");
    let unrelated_closure = unrelated_geometry_touched_closure();
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);

    let error =
        LoopCycleReadSource::select_from_touched_closure(&plan, &unrelated_closure, &topology)
            .unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::ReadStageTouchedClosureNotBoundToSelectedPlan
    );
}

#[test]
fn read_stage_rejects_zero_selected_loop_cycle_rows() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let read_source = LoopCycleReadSource::from_rows(Vec::new(), 1).unwrap();

    let error = LoopCycleReadStageExecutor::execute(&plan, read_source).unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::ReadStageTouchedClosureSelectedNoLoopCycleRows
    );
}

#[test]
fn source_rows_cannot_claim_more_selected_than_available() {
    let error = LoopCycleTouchedBoundaryRows::from_selected_rows_with_available_count(
        vec![source_row(1, 1, 3), source_row(2, 1, 3)],
        1,
    )
    .unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::SelectedRowsExceedAvailableRows
    );
}
