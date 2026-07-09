use worth_store_certification::{
    S6ClosedS11SecureIoFoundationAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
};

fn requires_s11_secure_io_seed(_: S6ClosedS11SecureIoFoundationAdmissionSeed) {}

fn main() {
    let placement: S6ClosedS7PlacementAdmissionSeed = todo!();
    requires_s11_secure_io_seed(placement);
}
