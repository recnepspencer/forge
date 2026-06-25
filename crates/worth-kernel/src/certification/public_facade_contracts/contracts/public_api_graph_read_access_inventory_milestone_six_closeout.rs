use worth_kernel::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout,
    WorthGraphReadAccessMilestoneSixCloseout, WorthGraphReadAccessMilestoneSixReadiness,
};
use worth_kernel::workload_composition::{
    QueryGraphObligationSelectionAuthorityKind, WorkloadCatalog,
    WorthQueryObligationSelectionMilestoneFiveCloseout, WorthQuerySelectorPrecisionPosture,
    WorthWorkload,
};
use worth_spatial::facade::workload_vocabulary::lower_spatial_touch_authority_to_query_descriptor;

use super::public_api_query_obligation_selection_real_spatial_authority_support::real_spatial_authority_case;
use super::public_api_query_obligation_selection_support::primitive_construction_birth_cases;

#[test]
fn public_milestone_six_closeout_accepts_real_milestone_five_query_obligation_seed() {
    let milestone_five = certified_milestone_five_closeout();
    let closeout = current_worth_graph_read_access_milestone_six_closeout(
        milestone_five.into_graph_read_inventory_seed(),
    )
    .expect("Milestone 6 graph-read closeout must accept real Milestone 5 seed");

    assert_public_closeout_is_ready_for_milestone_seven(&closeout);
    assert_public_seed_contains_declaration_handoff_payloads(&closeout);
    assert_public_seed_contains_capability_gap_handoff_payloads(&closeout);
}

fn assert_public_closeout_is_ready_for_milestone_seven(
    closeout: &WorthGraphReadAccessMilestoneSixCloseout,
) {
    assert_eq!(
        closeout.readiness(),
        WorthGraphReadAccessMilestoneSixReadiness::ReadyForMilestoneSeven
    );
    assert_eq!(closeout.counters().inventory_row_count(), 12);
    assert_eq!(closeout.counters().declaration_candidate_count(), 5);
    assert_eq!(closeout.counters().capability_gap_count(), 2);
    assert_eq!(closeout.counters().deletion_item_count(), 1);
    assert_eq!(
        closeout.counters().existing_deleted_source_count(),
        0,
        "deleted graph-read adoption source must not survive public closeout"
    );
}

fn assert_public_seed_contains_declaration_handoff_payloads(
    closeout: &WorthGraphReadAccessMilestoneSixCloseout,
) {
    assert_eq!(
        closeout
            .milestone_seven_seed()
            .declaration_candidates()
            .len(),
        closeout.counters().declaration_candidate_count()
    );
    assert!(closeout
        .milestone_seven_seed()
        .declaration_candidates()
        .iter()
        .all(|candidate| !candidate.touched_authority_input().is_empty()
            && !candidate
                .requirement_vocabulary()
                .requirement_kinds()
                .is_empty()
            && !candidate.milestone_seven_lowering_target().is_empty()));
}

fn assert_public_seed_contains_capability_gap_handoff_payloads(
    closeout: &WorthGraphReadAccessMilestoneSixCloseout,
) {
    assert!(closeout
        .milestone_seven_seed()
        .capability_gaps()
        .iter()
        .all(|gap| gap.must_not_exceed_count() > 0
            && !gap.blocker().is_empty()
            && !gap.removal_trigger().is_empty()));
}

fn certified_milestone_five_closeout() -> WorthQueryObligationSelectionMilestoneFiveCloseout {
    let workload = public_selection_workload();
    WorthQueryObligationSelectionMilestoneFiveCloseout::from_selected_closeouts([
        real_topology_selected_closeout(&workload),
        real_spatial_selected_closeout(),
    ])
    .expect("Milestone 5 must close from real selected Query obligations")
}

fn real_topology_selected_closeout(
    workload: &WorthWorkload,
) -> worth_kernel::workload_composition::WorthQuerySelectedGraphObligationCloseout {
    let case = primitive_construction_birth_cases()
        .into_iter()
        .next()
        .expect("primitive construction support should provide a topology case");
    let touched_basis = case.declared_touched_basis("phase8-public-graph-read-closeout");
    let selected = workload
        .select_query_graph_obligations(&touched_basis)
        .expect("topology touched basis should select Query obligations");
    let closeout = selected.closeout();

    assert_eq!(
        closeout.authority_kind(),
        QueryGraphObligationSelectionAuthorityKind::TopologyTouchedBasis
    );
    assert_eq!(
        closeout.selector_precision_report().posture(),
        WorthQuerySelectorPrecisionPosture::TouchedDescriptorBounded
    );
    assert!(closeout.local_ceremony_is_clean());
    assert!(!closeout.graph_read_access_planning_claimed());

    closeout
}

fn real_spatial_selected_closeout(
) -> worth_kernel::workload_composition::WorthQuerySelectedGraphObligationCloseout {
    let authority_case = real_spatial_authority_case("phase8-public-graph-read-spatial-closeout");
    let descriptor = lower_spatial_touch_authority_to_query_descriptor(
        authority_case.authority(),
        authority_case.lookup(),
    )
    .expect("real spatial authority must lower to Query descriptor");
    let selected = authority_case
        .workload()
        .select_query_graph_obligations(&descriptor)
        .expect("real spatial descriptor should select Query obligations");
    let closeout = selected.closeout();

    assert_eq!(
        closeout.authority_kind(),
        QueryGraphObligationSelectionAuthorityKind::SpatialQueryDescriptor
    );
    assert_eq!(
        closeout.selector_precision_report().posture(),
        WorthQuerySelectorPrecisionPosture::QueryExpressivenessGap
    );
    assert_eq!(
        closeout.spatial_query_gap_rows(),
        descriptor.gap_rows().len()
    );
    assert!(closeout.local_ceremony_is_clean());
    assert!(!closeout.graph_read_access_planning_claimed());

    closeout
}

fn public_selection_workload() -> WorthWorkload {
    WorkloadCatalog::cube()
        .with_retained_replay_artifacts()
        .build()
        .expect("catalog cube workload should build")
        .into_workload()
}
