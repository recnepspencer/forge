use forge_store_certification::{
    S6ClosedS10RepairAdmissionSeed, S6ClosedS11SecureIoFoundationAdmissionSeed,
};

fn requires_s10_repair_seed(_: S6ClosedS10RepairAdmissionSeed) {}

fn main() {
    let secure_io: S6ClosedS11SecureIoFoundationAdmissionSeed = todo!();
    requires_s10_repair_seed(secure_io);
}
