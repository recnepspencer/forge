use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::projection_fact_test_support::{
    capability_report, display_field_projection_context, host_result_text_intrinsic_size,
    synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiConstraintAxisScope, UiConstraintHostIntrinsicKind,
    UiConstraintIntrinsicSourcePosture, UiConstraintPropagationDenialReason,
    UiConstraintPropagationEdgeFamily, UiMeasurementCoordinateSpace, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture,
};
use crate::graph::allocation_constraint_projection_tests::{
    control_app, graph_node_identity_for_provenance,
};
use crate::graph::allocation_neighborhood_test_support::snapshot_with_admitted_layout;

#[test]
fn query_and_host_intrinsic_edges_preserve_distinct_source_posture() {
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-constraint-intrinsic-parity");
    let app = control_app(world_profile.clone(), "operator:row");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let child_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, child_node]);

    let query_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-query"),
        root_node,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(71),
        &query_intrinsic_policy(),
        &[MeasurementEvidenceInput::child_query_projection_fact(
            child_node,
            &consume_declared_measurement_projection_facts(
                synthetic_declaration_identity("allocation-constraint-intrinsic-query"),
                UiEvidenceAuthorityGeneration::new(71),
                &query_intrinsic_policy(),
                prerequisites.clone(),
                &attempt,
            )
            .expect("query intrinsic receipt should admit"),
        )],
    );
    let query_constraints = query_basis
        .admit_allocation_constraint_set(
            &query_basis
                .admit_allocation_neighborhood_from_graph(&snapshot)
                .expect("query neighborhood should admit"),
        )
        .expect("query intrinsic constraints should admit");

    let report = capability_report(71);
    let host_basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-host"),
        root_node,
        world_profile.clone(),
        UiEvidenceAuthorityGeneration::new(71),
        &host_intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                child_node,
                &host_result_text_intrinsic_size(
                    301,
                    &report,
                    UiEvidenceAuthorityGeneration::new(71),
                ),
            ),
        ],
    );
    let host_constraints = host_basis
        .admit_allocation_constraint_set(
            &host_basis
                .admit_allocation_neighborhood_from_graph(&snapshot)
                .expect("host neighborhood should admit"),
        )
        .expect("host intrinsic constraints should admit");

    let query = intrinsic_edge(&query_constraints);
    let host = intrinsic_edge(&host_constraints);

    assert_eq!(query.axis_scope(), UiConstraintAxisScope::Primary);
    assert_eq!(query.primary_extent(), 240.0);
    assert_eq!(query.contributor_graph_node_identity(), child_node);
    assert_eq!(host.axis_scope(), UiConstraintAxisScope::Primary);
    assert_eq!(host.primary_extent(), 240.0);
    assert_eq!(host.contributor_graph_node_identity(), child_node);
    assert_eq!(query.unit_posture(), UiMeasurementUnitPosture::LogicalPx);
    assert_eq!(
        host.coordinate_space(),
        UiMeasurementCoordinateSpace::GraphNodeLocal
    );
    assert_eq!(
        host.rounding_posture(),
        UiMeasurementRoundingPosture::ExactFloat
    );
    assert_eq!(
        query.source_posture(),
        UiConstraintIntrinsicSourcePosture::QueryOnly
    );
    assert_eq!(
        host.source_posture(),
        UiConstraintIntrinsicSourcePosture::HostOnly
    );
    assert_eq!(host.host_kind(), UiConstraintHostIntrinsicKind::Text);
    assert_eq!(query.host_kind(), UiConstraintHostIntrinsicKind::None);
}

#[test]
fn combined_query_and_host_intrinsic_evidence_stays_generation_compatible() {
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-constraint-intrinsic-combined");
    let app = control_app(world_profile.clone(), "operator:row");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let child_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, child_node]);
    let query_receipt = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("allocation-constraint-intrinsic-combined"),
        UiEvidenceAuthorityGeneration::new(81),
        &query_intrinsic_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let report = capability_report(81);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-combined"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(81),
        &query_intrinsic_policy(),
        &[
            MeasurementEvidenceInput::child_query_projection_fact(child_node, &query_receipt),
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                child_node,
                &host_result_text_intrinsic_size(
                    302,
                    &report,
                    UiEvidenceAuthorityGeneration::new(81),
                ),
            ),
        ],
    );
    let constraints = basis
        .admit_allocation_constraint_set(
            &basis
                .admit_allocation_neighborhood_from_graph(&snapshot)
                .expect("combined neighborhood should admit"),
        )
        .expect("combined intrinsic constraints should admit");

    let intrinsic = intrinsic_edge(&constraints);
    assert_eq!(
        intrinsic.source_posture(),
        UiConstraintIntrinsicSourcePosture::QueryAndHost
    );
    assert_eq!(intrinsic.primary_extent(), 240.0);
}

#[test]
fn stale_query_intrinsic_evidence_denies_before_neighborhood_solve() {
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("allocation-constraint-intrinsic-stale-query");
    let app = control_app(world_profile.clone(), "operator:row");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let child_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, child_node]);
    let query_receipt = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("allocation-constraint-intrinsic-stale-query"),
        UiEvidenceAuthorityGeneration::new(91),
        &query_intrinsic_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-stale-query"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(92),
        &query_intrinsic_policy(),
        &[MeasurementEvidenceInput::child_query_projection_fact(
            child_node,
            &query_receipt,
        )],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("stale-query neighborhood should admit");

    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("stale query intrinsic evidence must deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
}

#[test]
fn stale_host_intrinsic_evidence_denies_before_neighborhood_solve() {
    let (_, _, world_profile) =
        display_field_projection_context("allocation-constraint-intrinsic-stale-host");
    let app = control_app(world_profile.clone(), "operator:row");
    let root_node = graph_node_identity_for_provenance(&app, 0);
    let child_node = graph_node_identity_for_provenance(&app, 1);
    let snapshot = snapshot_with_admitted_layout(&app, &[root_node, child_node]);
    let report = capability_report(101);
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("allocation-constraint-intrinsic-stale-host"),
        root_node,
        world_profile,
        UiEvidenceAuthorityGeneration::new(102),
        &host_intrinsic_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&report),
            MeasurementEvidenceInput::child_host_measurement_result(
                child_node,
                &host_result_text_intrinsic_size(
                    303,
                    &report,
                    UiEvidenceAuthorityGeneration::new(101),
                ),
            ),
        ],
    );
    let neighborhood = basis
        .admit_allocation_neighborhood_from_graph(&snapshot)
        .expect("stale-host neighborhood should admit");

    let denial = basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect_err("stale host intrinsic evidence must deny");

    assert_eq!(
        denial.reason(),
        UiConstraintPropagationDenialReason::IncompatibleMeasurementPosture
    );
}

fn query_intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent],
    )
    .expect("query intrinsic policy should admit")
}

fn host_intrinsic_policy() -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        None,
        None,
        None,
        vec![],
    )
    .expect("host intrinsic policy should admit")
}

fn intrinsic_edge(
    constraints: &crate::evidence::UiAllocationConstraintSet,
) -> crate::evidence::UiConstraintChildIntrinsicContribution {
    constraints
        .propagation_edges()
        .iter()
        .find(|edge| edge.family() == UiConstraintPropagationEdgeFamily::ChildIntrinsicContribution)
        .and_then(|edge| edge.payload().child_intrinsic_contribution())
        .expect("child intrinsic edge must preserve typed contribution payload")
}
