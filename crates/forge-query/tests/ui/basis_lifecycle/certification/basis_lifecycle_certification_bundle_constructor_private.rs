use forge_query::facade::{BasisEligibilityCounters, BasisLifecycleCertificationBundle};

fn main() {
    let _ = BasisLifecycleCertificationBundle {
        rows: Vec::new(),
        output_digests: Vec::new(),
        certification_bundle_digest: String::new(),
        counters: BasisEligibilityCounters::default(),
    };
}
