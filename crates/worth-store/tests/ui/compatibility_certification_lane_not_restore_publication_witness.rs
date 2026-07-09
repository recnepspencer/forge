use worth_store::{Milestone12CertificationLaneOutcome, RestorePublicationWitness};

fn main() {
    publish(lane_outcome());
}

fn publish(_: RestorePublicationWitness) {}

fn lane_outcome() -> Milestone12CertificationLaneOutcome {
    panic!("compile-fail fixture")
}
