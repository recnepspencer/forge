use super::super::{
    ShellViewDerivedProductExecutor, ShellViewMigrationCloseout, ShellViewMigrationError,
    ShellViewOldAuthorityResidue,
};
use super::support::{admitted_input, selected_shell_views_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn source_firewall_classifies_reintroduced_direct_shell_view_authority() {
    let token = super::super::old_authority_residue::forbidden_shell_view_authority_token(
        "use crate::derived_topology::shell_views::interpret_boundaries;",
    );

    assert_eq!(token, Some("derived_topology::shell_views"));
}

#[test]
fn unknown_shell_view_old_authority_cannot_close_without_required_cap() {
    let plan = selected_shell_views_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 10, 3, 2, false, false)], 1);
    let executor = ShellViewDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let output = executor.output().unwrap();

    let error = ShellViewMigrationCloseout::close(
        &receipt,
        &output,
        &ShellViewOldAuthorityResidue::unknown_old_authority_for_tests(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        ShellViewMigrationError::OldAuthorityResidueMissingRequiredCap
    );
}

#[test]
fn deleted_shell_view_old_authority_leaves_no_residue_to_cap() {
    let residue = ShellViewOldAuthorityResidue::current_source_scan();

    assert!(residue.contains_required_caps());
    assert_eq!(
        residue.capped_direct_interpreter_count(),
        ShellViewOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(residue.capped_rows().is_empty());
}
