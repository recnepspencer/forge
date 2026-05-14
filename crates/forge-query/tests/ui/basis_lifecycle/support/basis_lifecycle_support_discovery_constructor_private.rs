use forge_query::facade::{
    BasisEligibilityCounters, BasisFamily, BasisLifecycleSupportDiscovery, BasisSupportPosture,
};

fn main() {
    let _ = BasisLifecycleSupportDiscovery {
        requested_family: BasisFamily::CurrentHead,
        requested_operation_lane: "observation",
        posture: BasisSupportPosture::Admitted,
        matched_row_digest: Some(String::new()),
        support_matrix_digest: String::new(),
        discovery_digest: String::new(),
        counters: BasisEligibilityCounters::default(),
    };
}
