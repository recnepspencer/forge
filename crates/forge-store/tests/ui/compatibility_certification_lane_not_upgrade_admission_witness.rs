use forge_store::{Milestone12CertificationLaneOutcome, UpgradeAdmissionWitness};

fn main() {
    admit_upgrade(lane_outcome());
}

fn admit_upgrade(_: UpgradeAdmissionWitness) {}

fn lane_outcome() -> Milestone12CertificationLaneOutcome {
    panic!("compile-fail fixture")
}
