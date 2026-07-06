use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture};
use crate::evidence::projection_fact_test_support::{
    capability_report, display_field_projection_context, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiConstraintCycleParticipationPosture,
};
use crate::graph::allocation_constraint_projection_tests::{
    control_app, graph_node_identity_for_provenance,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn emitted_constraint_edges_preserve_cycle_posture_on_production_path() {
    let (_, _, world_profile) = display_field_projection_context("allocation-constraint-cycle");
    let app = control_app(world_profile.clone(), "operator:grid");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_node]);
    let report = capability_report(31);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-cycle"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(31),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_node,
                &host_result_text_intrinsic_size(
                    304,
                    &report,
                    UiEvidenceAuthorityGeneration::new(31),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("grid neighborhood should admit");
    let constraints = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("grid constraints should admit");

    assert!(
        constraints
            .propagation_edges()
            .iter()
            .any(|edge| edge.cycle_participation_posture()
                == UiConstraintCycleParticipationPosture::AdmittedFixedPoint),
        "grid constraints should preserve admitted fixed-point edges"
    );
}

fn intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![],
    )
    .expect("intrinsic policy should admit")
}
