use worth_store_physical_certification::{
    S45RoadmapHarnessRequirementSet, S45SimulationHarnessEntryIdentity,
};

fn main() {
    let _ = S45SimulationHarnessEntryIdentity::new(
        "copied-root",
        "copied-source-decision-digest",
        &S45RoadmapHarnessRequirementSet::roadmap2_required(),
    );
}
