use worth_store_layout_indexes::{
    integrity::LayoutReadmissionWitness, OfflineVerifierLayoutProjection,
};

fn require_readmission(_: LayoutReadmissionWitness) {}

fn forged_readmission(report: OfflineVerifierLayoutProjection) {
    require_readmission(report);
}

fn main() {}
