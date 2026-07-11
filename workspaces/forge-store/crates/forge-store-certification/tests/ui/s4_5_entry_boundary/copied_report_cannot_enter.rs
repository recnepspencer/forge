use forge_store_physical_certification::{
    admit_simulation_harness_entry, ExistingSimulationHarnessInventory,
    SimulationHarnessRoadmapRequirementSet,
};

fn main() {
    let copied_report = "copied S.4 report";
    let _ = admit_simulation_harness_entry(
        copied_report,
        SimulationHarnessRoadmapRequirementSet::certification_required(),
        ExistingSimulationHarnessInventory::dedicated_workspace_baseline(),
    );
}
