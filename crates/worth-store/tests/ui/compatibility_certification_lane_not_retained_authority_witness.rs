use worth_store::{Milestone12CertificationLaneOutcome, RetainedAuthorityCompatibilityWitness};

fn main() {
    retain_authority(lane_outcome());
}

fn retain_authority(_: RetainedAuthorityCompatibilityWitness) {}

fn lane_outcome() -> Milestone12CertificationLaneOutcome {
    panic!("compile-fail fixture")
}
