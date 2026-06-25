use super::super::{
    ShellViewDerivedProductExecutor, ShellViewMigrationCloseout, ShellViewMigrationError,
    ShellViewOldAuthorityResidue,
};
use super::support::{admitted_input, selected_shell_views_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn product_output_digest_rejects_tampered_shell_view_rows() {
    let plan = selected_shell_views_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 10, 3, 2, false, false)], 1);
    let executor = ShellViewDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let valid_output = executor.output().unwrap();
    let tampered_output = super::super::ShellViewDerivedProductOutput::from_rows(
        vec![super::super::ShellViewProductRow::from_source_row(
            &source_row(1, 10, 1, 1, true, false),
        )],
        valid_output.touched_closure_shell_view_bound(),
        valid_output.selected_source_row_count(),
        valid_output.available_source_row_count(),
        *valid_output.read_stage_counters(),
        valid_output.selected_plan_digest(),
        "forged-read-stage",
        valid_output.source_rows_digest(),
        valid_output.input_digest(),
    );

    let error = ShellViewMigrationCloseout::close(
        &receipt,
        &tampered_output,
        &ShellViewOldAuthorityResidue::current_source_scan(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::OutputDigestNotBoundToReceipt
    );
}
