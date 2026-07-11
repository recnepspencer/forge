use forge_store_physical_certification::{
    PhysicalMutationCoverageEvidence, HarnessCoverageStage,
};

fn main() {
    let _coverage = PhysicalMutationCoverageEvidence::admitted_expected_failure(
        HarnessCoverageStage::SimulationAdmission,
        "fake-mutant-label",
    );
}
