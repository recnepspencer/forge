use super::super::inventory_lane;
use super::super::phase_six_closeout::WorthGraphReadAccessPhaseSixCloseout;
use super::super::{
    WorthGraphReadAccessMilestoneSevenSeed, WorthGraphReadDeclarationCandidate,
    WorthGraphReadReadFamilyTarget, WorthGraphReadRequirementVocabulary,
};

pub(crate) fn uncapped_old_graph_read_folklore_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    let inventory = inventory_lane::current_worth_graph_read_access_surface_inventory_for_tests(
        inventory_lane::WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("test graph-read inventory should close");
    let honest_seed = WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory)
        .expect("test graph-read inventory should convert to Phase 6 closeout")
        .milestone_seven_seed();
    let old_graph_read_row = inventory
        .rows()
        .iter()
        .find(|row| row.source_path() == "crates/worth-kernel/src/query_adoption/graph_read_access")
        .expect("test inventory should include old graph-read adoption deletion row");
    let old_graph_read_candidate =
        WorthGraphReadDeclarationCandidate::for_inventory_row(old_graph_read_row)
            .read_family_target(
                WorthGraphReadReadFamilyTarget::TopologyHalfEdgeSharedVertexNeighborhood,
            )
            .touched_authority_input("old-graph-read-adoption-folklore")
            .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
            .milestone_seven_lowering_target("must-not-lower-old-graph-read-adoption")
            .build()
            .expect("test old graph-read row should build a synthetic bad candidate");
    WorthGraphReadAccessMilestoneSevenSeed::new(
        vec![old_graph_read_candidate],
        Vec::new(),
        honest_seed.deletion_items().to_vec(),
        *honest_seed.counters(),
    )
}
