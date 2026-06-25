use super::super::{
    RadialRingDerivedProductExecutor, RadialRingMigrationCloseout, RadialRingMigrationError,
    RadialRingOldAuthorityResidue,
};
use super::support::{admitted_input, selected_radial_rings_plan, source_row};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;

#[test]
fn source_firewall_classifies_reintroduced_direct_radial_ring_authority() {
    let token = super::super::old_authority_residue::forbidden_radial_ring_authority_token(
        "use crate::derived_topology::radial_rings::interpret_boundaries;",
    );

    assert_eq!(token, Some("derived_topology::radial_rings"));
}

#[test]
fn unknown_radial_ring_old_authority_cannot_close_without_required_cap() {
    let plan = selected_radial_rings_plan("loop-touch");
    let input = admitted_input(&plan, vec![source_row(1, 10, 3, 2, false, false)], 1);
    let executor = RadialRingDerivedProductExecutor::new(input);
    let receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(&plan, &executor)
            .unwrap();
    let output = executor.output().unwrap();

    let error = RadialRingMigrationCloseout::close(
        &receipt,
        &output,
        &RadialRingOldAuthorityResidue::unknown_old_authority_for_tests(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        RadialRingMigrationError::OldAuthorityResidueMissingRequiredCap
    );
}

#[test]
fn deleted_radial_ring_old_authority_leaves_no_residue_to_cap() {
    let residue = RadialRingOldAuthorityResidue::current_source_scan();

    assert!(residue.contains_required_caps());
    assert_eq!(
        residue.capped_direct_interpreter_count(),
        RadialRingOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(residue.capped_rows().is_empty());
}
