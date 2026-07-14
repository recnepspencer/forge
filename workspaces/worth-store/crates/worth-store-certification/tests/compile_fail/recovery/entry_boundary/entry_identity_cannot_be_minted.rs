use worth_store_physical_certification::{
    SimulationHarnessRoadmapRequirementSet, SimulationHarnessEntryIdentity,
};

fn main() {
    let _ = SimulationHarnessEntryIdentity::new(
        "copied-root",
        "copied-source-decision-digest",
        &SimulationHarnessRoadmapRequirementSet::certification_required(),
    );
}
