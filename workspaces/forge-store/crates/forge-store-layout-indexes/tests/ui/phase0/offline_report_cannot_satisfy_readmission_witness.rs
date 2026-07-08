use forge_store_layout_indexes::S8LayoutReadmissionWitness;
use forge_store_offline_verifier::OfflineLayoutReport;

fn require_readmission(_: S8LayoutReadmissionWitness) {}

fn main() {
    require_readmission(OfflineLayoutReport::new(Vec::new()));
}
