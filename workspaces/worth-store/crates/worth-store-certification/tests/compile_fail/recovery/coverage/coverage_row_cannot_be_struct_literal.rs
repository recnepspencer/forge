use worth_store_physical_certification::{
    CoverageSurfaceKind, HarnessSubsystem, PhysicalCoverageMatrixRow, HarnessCoverageStage,
};

fn main() {
    let _row = PhysicalCoverageMatrixRow {
        sequence: HarnessCoverageStage::SimulationAdmission,
        subsystem: HarnessSubsystem::ScenarioDefinitions,
        surface: CoverageSurfaceKind::Scenario,
        source_identity: [0; 32],
        receipt: todo!(),
    };
}
