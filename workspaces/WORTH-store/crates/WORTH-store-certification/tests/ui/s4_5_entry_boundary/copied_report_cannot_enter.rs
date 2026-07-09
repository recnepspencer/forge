use worth_store_physical_certification::{
    admit_s45_simulation_harness_entry, S45ExistingHarnessInventory,
    S45RoadmapHarnessRequirementSet,
};

fn main() {
    let copied_report = "copied S.4 report";
    let _ = admit_s45_simulation_harness_entry(
        copied_report,
        S45RoadmapHarnessRequirementSet::roadmap2_required(),
        S45ExistingHarnessInventory::dedicated_workspace_baseline(),
    );
}
