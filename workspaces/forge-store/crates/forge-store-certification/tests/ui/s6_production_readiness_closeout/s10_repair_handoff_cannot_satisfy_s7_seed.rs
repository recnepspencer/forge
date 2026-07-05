use forge_store_certification::{S6ClosedS10RepairAdmissionSeed, S6ClosedS7PlacementAdmissionSeed};

fn requires_s7_seed(_: S6ClosedS7PlacementAdmissionSeed) {}

fn main() {
    let repair: S6ClosedS10RepairAdmissionSeed = todo!();
    requires_s7_seed(repair);
}
