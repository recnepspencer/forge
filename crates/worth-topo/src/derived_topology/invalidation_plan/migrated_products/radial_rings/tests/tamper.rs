use super::super::{
    RadialRingDerivedProductExecutor, RadialRingMigrationCloseout, RadialRingMigrationError,
    RadialRingOldAuthorityResidue,
};
use super::support::{admitted_input, selected_radial_rings_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn product_output_digest_rejects_tampered_radial_ring_rows() {
    let plan = selected_radial_rings_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 10, 3, 2, false, false)], 1);
    let executor = RadialRingDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let valid_output = executor.output().unwrap();
    let tampered_output = super::super::RadialRingDerivedProductOutput::from_rows(
        vec![super::super::RadialRingProductRow::from_source_row(
            &source_row(1, 10, 1, 1, true, false),
        )],
        valid_output.touched_closure_radial_ring_bound(),
        valid_output.selected_source_row_count(),
        valid_output.available_source_row_count(),
        *valid_output.read_stage_counters(),
        valid_output.selected_plan_digest(),
        "forged-read-stage",
        valid_output.source_rows_digest(),
        valid_output.input_digest(),
    );

    let error = RadialRingMigrationCloseout::close(
        &receipt,
        &tampered_output,
        &RadialRingOldAuthorityResidue::current_source_scan(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::OutputDigestNotBoundToReceipt
    );
}
