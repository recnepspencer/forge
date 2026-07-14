use worth_query::facade::foundation::BasisEligibilityCounters;
use worth_query::facade::certification::BasisLifecycleCertificationBundle;

fn main() {
    let _ = BasisLifecycleCertificationBundle {
        rows: Vec::new(),
        output_digests: Vec::new(),
        certification_bundle_digest: String::new(),
        counters: BasisEligibilityCounters::default(),
    };
}
