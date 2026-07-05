use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapCellClassificationDenialKind, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput,
};

use super::fixtures::{
    admitted_graph, ambiguous_hidden_right_winding_graph, cancelled_hidden_right_winding_graph,
    multi_boundary_graph,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn overlap_cell_winding_rejects_containment_from_a_different_arrangement_graph() {
    let canonical = admitted_graph(LoopFixtureEntryOrder::Canonical);
    let multi_boundary = multi_boundary_graph();
    let foreign_containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&multi_boundary),
    )
    .expect("fixture arrangement should admit containment classification");

    let denial = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(
            &canonical,
            &foreign_containment,
        ),
    )
    .expect_err(
        "winding should reject containment evidence admitted for a different arrangement graph",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapCellClassificationDenialKind::WindingFieldInputMismatchDenied
    );
}

#[test]
fn overlap_cell_winding_rejects_ambiguous_hidden_same_operand_support() {
    let graph = ambiguous_hidden_right_winding_graph();
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&graph),
    )
    .expect("fixture arrangement should admit containment classification");

    let denial = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(&graph, &containment),
    )
    .expect_err("winding should deny ambiguous hidden same-operand support without certified boundary topology");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapCellClassificationDenialKind::NoOperandLocalWindingEvidenceDenied
    );
}

#[test]
fn overlap_cell_containment_rejects_zero_net_hidden_same_operand_support() {
    let graph = cancelled_hidden_right_winding_graph();

    let denial = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(&graph),
    )
    .expect_err(
        "containment should deny hidden same-operand support whose signed winding cancels to zero",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapCellClassificationDenialKind::ContradictoryOperandContainmentEvidenceDenied
    );
}
