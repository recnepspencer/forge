use worth_store_physical_certification::{
    FaultDeliveryAttempt, PhysicalMutationCoverageEvidence, PhysicalSimulationPlan,
    Roadmap2HarnessSequence,
};

fn main() {
    let plan: PhysicalSimulationPlan = todo!();
    let _coverage = PhysicalMutationCoverageEvidence::from_private_mutation_denial(
        Roadmap2HarnessSequence::S45,
        &plan,
        FaultDeliveryAttempt::private_mutation(),
    );
}
