use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{admit_measurement_basis, MeasurementEvidenceInput};
use crate::graph::allocation_constraint_sibling_support_test_support::{
    graph_node_identity_for_provenance, mosaic_alternate_runtime_sizing_contract_id,
    mosaic_peer_app, mosaic_peer_app_with_contracts, mosaic_runtime_sizing_contract_id,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn mosaic_capability_resize_support_alone_no_longer_admits_on_the_ordinary_sibling_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-sibling-mosaic-support");
    let app = mosaic_peer_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(71);
    let resize_support = MeasurementEvidenceInput::mosaic_sibling_resize_support(
        app.capabilities(),
        root_node,
        &mosaic_runtime_sizing_contract_id(),
    )
    .expect("mosaic sizing capability should admit sibling resize support");
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling-mosaic-support"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(71),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    731,
                    &report,
                    UiEvidenceAuthorityGeneration::new(71),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    732,
                    &report,
                    UiEvidenceAuthorityGeneration::new(71),
                ),
            ),
            resize_support,
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("mosaic neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err(
            "capability-backed resize support alone must not authorize ordinary durable resize",
        );

    assert_eq!(
        denial.reason(),
        crate::evidence::UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}

#[test]
fn mosaic_support_for_a_different_root_denies_on_this_sibling_lane() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-sibling-mosaic-support-mismatch");
    let app = mosaic_peer_app(world_profile.clone());
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(72);
    let resize_support = MeasurementEvidenceInput::mosaic_sibling_resize_support(
        app.capabilities(),
        crate::graph::UiGraphNodeIdentity::new(peer_a.digest().wrapping_add(10_000)),
        &mosaic_runtime_sizing_contract_id(),
    )
    .expect("mosaic sizing capability should admit a targeted sibling resize support witness");
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-sibling-mosaic-support-mismatch"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(72),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    733,
                    &report,
                    UiEvidenceAuthorityGeneration::new(72),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    734,
                    &report,
                    UiEvidenceAuthorityGeneration::new(72),
                ),
            ),
            resize_support,
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("mosaic neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("a support witness for a different root must not authorize this sibling group");

    assert_eq!(
        denial.reason(),
        crate::evidence::UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}

fn intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        vec![],
    )
    .expect("intrinsic policy should admit")
}

#[test]
fn mosaic_support_for_the_same_root_but_the_wrong_contract_denies_on_this_sibling_lane() {
    let (_, _, world_profile) = display_field_projection_context(
        "allocation-constraint-sibling-mosaic-support-contract-mismatch",
    );
    let app = mosaic_peer_app_with_contracts(
        world_profile.clone(),
        "worth-ui.runtime.graph.allocation-constraint-sibling-support.contract-mismatch",
        [
            mosaic_runtime_sizing_contract_id().as_str(),
            mosaic_runtime_sizing_contract_id().as_str(),
            mosaic_runtime_sizing_contract_id().as_str(),
        ],
        true,
    );
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let peer_a = graph_node_identity_for_provenance(&app, 1);
    let peer_b = graph_node_identity_for_provenance(&app, 2);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, peer_a, peer_b]);
    let report = capability_report(73);
    let resize_support = MeasurementEvidenceInput::mosaic_sibling_resize_support(
        app.capabilities(),
        root_node,
        &mosaic_alternate_runtime_sizing_contract_id(),
    )
    .expect("an alternate resizable contract should still admit a witness");
    let basis = admit_measurement_basis(
        synthetic_declaration_identity(
            "allocation-constraint-sibling-mosaic-support-contract-mismatch",
        ),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(73),
        &intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_a,
                &host_result_text_intrinsic_size(
                    735,
                    &report,
                    UiEvidenceAuthorityGeneration::new(73),
                ),
            ),
            MeasurementEvidenceInput::child_host_measurement_result(
                peer_b,
                &host_result_text_intrinsic_size(
                    736,
                    &report,
                    UiEvidenceAuthorityGeneration::new(73),
                ),
            ),
            resize_support,
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("mosaic neighborhood should admit");
    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err(
            "a same-root witness for the wrong contract must not authorize this sibling group",
        );

    assert_eq!(
        denial.reason(),
        crate::evidence::UiConstraintPropagationDenialReason::UnsupportedSiblingFixedPoint
    );
    assert_eq!(
        denial.family(),
        Some(crate::evidence::UiConstraintPropagationEdgeFamily::SiblingNegotiation)
    );
}
