use super::super::{
    LoopCycleDerivedProductExecutor, LoopCycleMigrationCloseout, LoopCycleMigrationError,
    LoopCycleOldAuthorityResidue,
};
use super::support::{admitted_input, selected_loop_cycles_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn product_output_digest_rejects_tampered_loop_cycle_rows() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 1, 3)], 1);
    let executor = LoopCycleDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let valid_output = executor.output().unwrap();
    let tampered_output = super::super::LoopCycleDerivedProductOutput::from_rows(
        vec![super::super::LoopCycleProductRow::from_source_row(
            &source_row(1, 0, 0),
        )],
        valid_output.touched_closure_loop_cycle_bound(),
        valid_output.selected_source_row_count(),
        valid_output.available_source_row_count(),
        *valid_output.read_stage_counters(),
        valid_output.selected_plan_digest(),
        "forged-read-stage",
        valid_output.source_rows_digest(),
        valid_output.input_digest(),
    );

    let error = LoopCycleMigrationCloseout::close(
        &receipt,
        &tampered_output,
        &LoopCycleOldAuthorityResidue::current_source_scan(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::OutputDigestNotBoundToReceipt
    );
}
