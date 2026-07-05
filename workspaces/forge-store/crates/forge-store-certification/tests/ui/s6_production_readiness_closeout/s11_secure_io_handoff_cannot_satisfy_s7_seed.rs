use forge_store_certification::{
    S6ClosedS11SecureIoFoundationAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
};

fn requires_s7_seed(_: S6ClosedS7PlacementAdmissionSeed) {}

fn main() {
    let secure_io: S6ClosedS11SecureIoFoundationAdmissionSeed = todo!();
    requires_s7_seed(secure_io);
}
