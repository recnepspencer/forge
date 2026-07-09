use worth_store_physical_certification::{
    CoverageSurfaceKind, HarnessSubsystem, PhysicalCoverageMatrixRow, Roadmap2HarnessSequence,
};

fn main() {
    let _row = PhysicalCoverageMatrixRow {
        sequence: Roadmap2HarnessSequence::S45,
        subsystem: HarnessSubsystem::ScenarioDefinitions,
        surface: CoverageSurfaceKind::Scenario,
        source_identity: [0; 32],
        receipt: todo!(),
    };
}
