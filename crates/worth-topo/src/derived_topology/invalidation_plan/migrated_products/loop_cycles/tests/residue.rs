use super::super::{
    LoopCycleDerivedProductExecutor, LoopCycleMigrationCloseout, LoopCycleMigrationError,
    LoopCycleOldAuthorityResidue,
};
use super::support::{admitted_input, selected_loop_cycles_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn source_firewall_classifies_reintroduced_direct_loop_cycle_authority() {
    let token = super::super::old_authority_residue::forbidden_loop_cycle_authority_token(
        "use crate::derived_topology::loop_cycles::interpret_boundaries;",
    );

    assert_eq!(token, Some("derived_topology::loop_cycles"));
}

#[test]
fn unknown_loop_cycle_old_authority_cannot_close_without_required_cap() {
    let plan = selected_loop_cycles_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 1, 3)], 1);
    let executor = LoopCycleDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let output = executor.output().unwrap();

    let error = LoopCycleMigrationCloseout::close(
        &receipt,
        &output,
        &LoopCycleOldAuthorityResidue::unknown_old_authority_for_tests(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        LoopCycleMigrationError::OldAuthorityResidueMissingRequiredCap
    );
}

#[test]
fn deleted_loop_cycle_old_authority_leaves_no_residue_to_cap() {
    let residue = LoopCycleOldAuthorityResidue::current_source_scan();

    assert!(residue.contains_required_caps());
    assert_eq!(
        residue.capped_direct_interpreter_count(),
        LoopCycleOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(residue.capped_rows().is_empty());
}
