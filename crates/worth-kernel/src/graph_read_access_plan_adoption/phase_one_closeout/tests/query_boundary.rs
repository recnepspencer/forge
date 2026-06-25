use super::super::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use super::source_firewall_support::production_adoption_lane_sources;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;

#[test]
fn phase_one_query_anchors_name_plan_surfaces_without_executing_them() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");
    let anchors = closeout.query_surface_anchors();

    assert!(anchors
        .access_admission_type()
        .contains("ForgeQueryGraphReadAccessAdmission"));
    assert!(anchors
        .admission_posture_type()
        .contains("ForgeQueryGraphReadAccessAdmissionPosture"));
    assert!(anchors
        .admitted_plan_type()
        .contains("ForgeQueryAdmittedGraphReadAccessPlan"));
    assert!(anchors
        .plan_consumption_type()
        .contains("ForgeQueryGraphReadAccessPlanConsumption"));
    assert!(anchors
        .receipt_fields()
        .contains(&"graph_read_access_plan_consumption"));
}

#[test]
fn phase_one_does_not_call_query_plan_admission_or_execution() {
    let phase_one_sources = [
        include_str!("../closeout.rs"),
        include_str!("../counters.rs"),
        include_str!("../errors.rs"),
        include_str!("../../seed_admission/admitted_seed.rs"),
        include_str!("../../execution_folklore_inventory/inventory.rs"),
        include_str!("../../execution_folklore_inventory/inventory_row.rs"),
        include_str!("../../execution_folklore_inventory/inventory_counters.rs"),
    ];
    let forbidden_fragments = [
        "admit_graph_read_access_for_family",
        "plan_admitted_graph_read_access_for_family",
        ".execute(",
        "consume_plan",
        "from_admission(",
        "ForgeQueryGraphReadAccessExecutionRecorder",
    ];

    for source in phase_one_sources {
        for forbidden in forbidden_fragments {
            assert!(
                !source.contains(forbidden),
                "Phase 1 must not admit, plan, or execute graph reads early: {forbidden}"
            );
        }
    }
}

#[test]
fn phase_one_production_lane_rejects_execution_and_local_helper_residue() {
    let production_sources = production_adoption_lane_sources();
    let forbidden_fragments = [
        "admit_graph_read_access_for_family",
        "plan_admitted_graph_read_access_for_family",
        ".execute(",
        "consume_plan",
        "from_admission(",
        "ForgeQueryGraphReadAccessExecutionRecorder",
        "local_adjacency",
        "local graph read",
        "manual_plan_hint",
        "strategy_switch",
        "fabricated_receipt",
        "compatibility_wrapper",
    ];

    for source in production_sources {
        for forbidden in forbidden_fragments {
            assert!(
                !source.contents().contains(forbidden),
                "Phase 1 production source must not carry execution/local-helper residue: {} contains {forbidden}",
                source.path().display()
            );
        }
    }
}

#[test]
fn phase_one_public_surface_does_not_expose_execution_vocabulary() {
    let public_sources = [
        include_str!("../../mod.rs"),
        include_str!("../closeout.rs"),
        include_str!("../../seed_admission/mod.rs"),
    ];
    let forbidden_public_fragments = [
        "pub fn execute",
        "pub fn graph_read_access_plan_consumption",
        "pub fn graph_read_receipt",
        "pub fn graph_read_streaming_receipt",
        "pub fn ephemeral_graph_index_receipt",
        "pub fn plan_admitted",
        "pub fn consume_plan",
    ];

    for source in public_sources {
        for forbidden in forbidden_public_fragments {
            assert!(
                !source.contains(forbidden),
                "Phase 1 public surface must not expose execution vocabulary: {forbidden}"
            );
        }
    }
}
