use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_catalog_closeout,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadAccessDeclarationPhaseTwoCloseout,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};

pub(crate) fn production_seed() -> WorthGraphReadAccessMilestoneSevenSeed {
    current_worth_graph_read_access_milestone_six_closeout_for_tests()
        .milestone_seven_seed()
        .clone()
}

pub(crate) fn phase_two_closeout_from_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadAccessDeclarationPhaseTwoCloseout {
    let phase_one = phase_one_closeout_from_milestone_seven_seed_for_tests(seed)
        .expect("Milestone 7 seed fixture should admit into Phase 1");
    current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect("Phase 1 fixture should build a declaration catalog")
}

pub(crate) fn only_requirement_record(
    closeout: &crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationCloseout,
) -> &crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationRecord {
    assert_eq!(closeout.requirement_records().len(), 1);
    &closeout.requirement_records()[0]
}

pub(crate) fn rust_sources_under(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut sources = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in std::fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        {
            let entry = entry.expect("directory entry should be readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}
