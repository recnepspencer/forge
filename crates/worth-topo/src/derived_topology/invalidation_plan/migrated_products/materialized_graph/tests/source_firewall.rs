use super::super::{
    MaterializedGraphDerivedProductExecutor, MaterializedGraphDiagnosticProjection,
    MaterializedGraphExecutionInput, MaterializedGraphMigrationCloseout,
    MaterializedGraphMigrationError, MaterializedGraphOldAuthorityResidue,
};
use super::support::{selected_materialized_graph_plan, selected_materialized_receipt};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn old_whole_view_materializer_authority_without_residue_cap_cannot_close() {
    let plan = selected_materialized_graph_plan();
    let read_stage_receipt = selected_materialized_receipt(&plan);
    let input = MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(
        &plan,
        read_stage_receipt.clone(),
    )
    .unwrap();
    let executor = MaterializedGraphDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let output = executor.output().unwrap();
    let diagnostic_projection = MaterializedGraphDiagnosticProjection::from_read_stage_and_output(
        &read_stage_receipt,
        &output,
    );

    let error = MaterializedGraphMigrationCloseout::close(
        &receipt,
        &output,
        &diagnostic_projection,
        &MaterializedGraphOldAuthorityResidue::uncapped_for_tests(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        MaterializedGraphMigrationError::OldAuthorityResidueNotCapped
    );
}

#[test]
fn residue_cap_names_owner_blocker_and_removal_trigger_for_old_materializer_paths() {
    let residue = MaterializedGraphOldAuthorityResidue::current_source_scan();

    assert!(residue.contains_required_caps());
    assert_eq!(
        residue.capped_whole_view_authority_count(),
        MaterializedGraphOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(residue.capped_rows().iter().all(|row| {
        !row.owner().is_empty()
            && !row.blocker().is_empty()
            && !row.removal_trigger().is_empty()
            && !row.row_digest().is_empty()
    }));
}
