use super::active_application_session_test_support::{
    admit_candidate_catalog, scaled_component_candidate_submission,
    source_backed_scaled_component_session,
};
use super::regional_activation_test_support::{
    regional_activation_inputs, RegionalActivationInputs,
};

#[test]
fn real_source_component_replacement_publishes_exact_complete_regional_truth() {
    let RegionalActivationInputs {
        mut runtime,
        pending,
        admitted_catalog,
    } = regional_activation_inputs();
    let candidate_generation = pending
        .candidate_application_authority()
        .generation_identity()
        .clone();
    let predecessor = runtime.active.active_plan();
    assert_ne!(
        runtime.inspect_active().generation_identity(),
        &candidate_generation,
        "fixture must cross an application-generation boundary"
    );
    let retired_identity = predecessor
        .exact_plan()
        .canonical_region_identities()
        .into_iter()
        .find(|identity| identity.exact_basis().contains("active_session_current"))
        .expect("active source component owns a regional identity");
    let stale = predecessor
        .exact_plan()
        .region_store()
        .handle_for(&retired_identity)
        .expect("active source component owns a regional slot")
        .clone();
    let predecessor_probe = predecessor
        .exact_plan()
        .region_storage_reclamation_probe_for_test();
    drop(predecessor);
    let boundary = runtime.safe_frame_boundary();

    let swap = runtime
        .activate_admitted_allocation_catalog_at_frame_boundary(
            pending,
            admitted_catalog,
            boundary,
            None,
        )
        .expect("real source-backed regional successor activates");
    let successor = runtime.active.active_plan();
    let plan = successor.exact_plan();
    let evidence = plan.regional_evidence();
    assert_eq!(swap.structural_reuse(), evidence);
    assert_eq!(
        runtime.inspect_active().generation_identity(),
        &candidate_generation,
        "the activation transaction publishes generation with artifact and plan"
    );
    let inserted_identity = plan
        .canonical_region_identities()
        .into_iter()
        .find(|identity| identity.exact_basis().contains("active_session_candidate"))
        .expect("candidate source component owns a regional identity");

    assert!(plan.region_count() > 0);
    assert!(
        !plan.has_reconstructive_flat_projection(),
        "regional publication does not rebuild a candidate-wide flat projection"
    );
    let construction = plan.construction_counters();
    assert_eq!(construction.handle_allocation().plan_node_input_count(), 0);
    assert_eq!(construction.lane_admission().plan_node_visit_count(), 0);
    assert_eq!(construction.topology().plan_node_input_count(), 0);
    assert_eq!(
        construction.full_candidate_node_visit_count(),
        construction.lowering().staged_node_input_count()
            + construction.lowering().query_binding_input_count()
            + construction.lowering().component_hook_input_count(),
        "the receipt leaves the remaining full-input reconstruction visible"
    );
    assert_eq!(
        evidence
            .transition_for_region(&retired_identity)
            .expect("retired region is indexed")
            .transition(),
        crate::runtime::planning::plan_topology::WorthUiPlanRegionTransition::Retired
    );
    assert_eq!(
        evidence
            .transition_for_region(&inserted_identity)
            .expect("inserted region is indexed")
            .transition(),
        crate::runtime::planning::plan_topology::WorthUiPlanRegionTransition::Inserted
    );
    assert!(!plan.region_store().resolves(&stale));
    assert!(plan.region_store().handle_for(&inserted_identity).is_some());
    assert!(plan.region_storage_counters().region_construction_count() > 0);
    assert!(plan.region_storage_counters().retirement_count() > 0);
    assert!(
        predecessor_probe.is_reclaimed(),
        "successful commit releases predecessor-only regional storage"
    );
}

#[test]
fn complete_successor_construction_cost_ignores_unrelated_predecessor_scale() {
    let small = activate_scaled_component_replacement(1);
    let large = activate_scaled_component_replacement(100);

    assert!(large.region_count > small.region_count * 10);
    assert_eq!(small.affected_region_count, large.affected_region_count);
    assert!(
        large
            .construction
            .lowering()
            .reconciliation_receipt_input_count()
            > small
                .construction
                .lowering()
                .reconciliation_receipt_input_count()
                * 10,
        "the receipt must retain the honest logical candidate cardinality"
    );
    assert_eq!(
        small.construction.full_candidate_node_visit_count(),
        large.construction.full_candidate_node_visit_count()
    );
    assert_eq!(
        small.construction.regional_storage(),
        large.construction.regional_storage()
    );
    assert_eq!(
        small.construction.handle_allocation(),
        large.construction.handle_allocation()
    );
    assert_eq!(small.construction.topology(), large.construction.topology());
    assert_eq!(
        small.construction.lane_admission().plan_node_visit_count(),
        large.construction.lane_admission().plan_node_visit_count()
    );
    assert_eq!(
        small.exact_region_comparison_count,
        large.exact_region_comparison_count
    );
    assert!(
        small.exact_region_comparison_count <= small.affected_region_count,
        "equivalence compares only the changed regional proof set"
    );
    assert_eq!(
        small
            .construction
            .lane_admission()
            .support_row_lookup_count(),
        large
            .construction
            .lane_admission()
            .support_row_lookup_count()
    );
    assert_eq!(
        small
            .construction
            .handle_allocation()
            .plan_node_input_count(),
        0
    );
    assert_eq!(
        small.construction.lane_admission().plan_node_visit_count(),
        0
    );
    assert_eq!(small.construction.topology().plan_node_input_count(), 0);
    assert!(small.construction.full_candidate_node_visit_count() > 0);
    assert!(
        small.construction.full_candidate_node_visit_count() <= small.affected_region_count,
        "candidate-row visits remain bounded by the changed regional set"
    );
}

struct ScaledActivationEvidence {
    region_count: usize,
    affected_region_count: usize,
    exact_region_comparison_count: usize,
    construction: crate::runtime::WorthUiPlanConstructionCounters,
}

fn activate_scaled_component_replacement(
    unrelated_component_count: usize,
) -> ScaledActivationEvidence {
    let mut session = source_backed_scaled_component_session(unrelated_component_count);
    let mut prepared = session
        .prepare_replacement(scaled_component_candidate_submission(
            &session,
            "scaled-regional-candidate",
            unrelated_component_count,
        ))
        .expect("scaled real source-backed candidate prepares");
    let admitted_catalog = admit_candidate_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("scaled source-backed candidate lowers");
    let pending_cutover = session
        .stage_prepared_replacement(lowered)
        .expect("scaled source-backed candidate stages");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("scaled boundary turn completes")
        .into_activation_boundary();
    let activation = session
        .activate_prepared_replacement(pending_cutover, admitted_catalog, boundary, None)
        .expect("scaled regional successor activates")
        .into_activation()
        .expect("scaled replacement changes executable meaning");
    let summary = activation
        .plan_decision()
        .summary()
        .expect("successful regional decision carries a summary");
    let observation = session.inspect_runtime();
    let construction = observation.cross_lane_bundle().construction_counters();
    ScaledActivationEvidence {
        region_count: observation
            .cross_lane_bundle()
            .plan_digest()
            .basis()
            .plan_node_count(),
        affected_region_count: activation.structural_reuse().affected_region_count(),
        exact_region_comparison_count: summary.exact_region_comparison_count(),
        construction,
    }
}
