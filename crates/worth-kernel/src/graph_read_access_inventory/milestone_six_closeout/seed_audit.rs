use super::super::phase_six_closeout::WorthGraphReadAccessMilestoneSevenSeed;
use super::errors::{
    WorthGraphReadAccessMilestoneSixError, WorthGraphReadAccessMilestoneSixErrorKind,
};

pub(super) fn audit_milestone_seven_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> Result<(), WorthGraphReadAccessMilestoneSixError> {
    if seed.claims_execution_authority()
        || seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap()
        || seed_contains_old_graph_read_folklore_as_candidate_or_gap(seed)
    {
        return Err(WorthGraphReadAccessMilestoneSixError::new(
            WorthGraphReadAccessMilestoneSixErrorKind::OldGraphReadFolkloreInMilestoneSevenSeed,
        ));
    }
    Ok(())
}

fn seed_contains_old_graph_read_folklore_as_candidate_or_gap(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> bool {
    seed.declaration_candidates().iter().any(|candidate| {
        candidate
            .inventory_row_identity()
            .source_path()
            .ends_with(OLD_GRAPH_READ_ADOPTION_PATH)
    }) || seed.capability_gaps().iter().any(|gap| {
        gap.inventory_row_identity()
            .source_path()
            .ends_with(OLD_GRAPH_READ_ADOPTION_PATH)
    })
}

const OLD_GRAPH_READ_ADOPTION_PATH: &str =
    "crates/worth-kernel/src/query_adoption/graph_read_access";
