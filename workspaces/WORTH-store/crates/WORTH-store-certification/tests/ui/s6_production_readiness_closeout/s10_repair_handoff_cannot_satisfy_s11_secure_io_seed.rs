use worth_store_certification::{
    S6ClosedS10RepairAdmissionSeed, S6ClosedS11SecureIoFoundationAdmissionSeed,
};

fn requires_s11_secure_io_seed(_: S6ClosedS11SecureIoFoundationAdmissionSeed) {}

fn main() {
    let repair: S6ClosedS10RepairAdmissionSeed = todo!();
    requires_s11_secure_io_seed(repair);
}
