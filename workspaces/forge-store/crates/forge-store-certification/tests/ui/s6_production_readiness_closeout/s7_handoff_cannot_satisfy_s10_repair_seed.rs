use forge_store_certification::{S6ClosedS10RepairAdmissionSeed, S6ClosedS7PlacementAdmissionSeed};

fn requires_s10_repair_seed(_: S6ClosedS10RepairAdmissionSeed) {}

fn main() {
    let placement: S6ClosedS7PlacementAdmissionSeed = todo!();
    requires_s10_repair_seed(placement);
}
