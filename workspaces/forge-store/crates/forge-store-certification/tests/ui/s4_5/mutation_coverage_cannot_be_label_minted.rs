use forge_store_physical_certification::{
    PhysicalMutationCoverageEvidence, Roadmap2HarnessSequence,
};

fn main() {
    let _coverage = PhysicalMutationCoverageEvidence::admitted_expected_failure(
        Roadmap2HarnessSequence::S45,
        "fake-mutant-label",
    );
}
