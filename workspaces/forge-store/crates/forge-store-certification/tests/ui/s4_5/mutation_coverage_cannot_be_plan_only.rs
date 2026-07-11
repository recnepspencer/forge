use forge_store_physical_certification::{
    FaultDeliveryAttempt, PhysicalMutationCoverageEvidence, PhysicalSimulationPlan,
    HarnessCoverageStage,
};

fn main() {
    let plan: PhysicalSimulationPlan = todo!();
    let _coverage = PhysicalMutationCoverageEvidence::from_private_mutation_denial(
        HarnessCoverageStage::SimulationAdmission,
        &plan,
        FaultDeliveryAttempt::private_mutation(),
    );
}
