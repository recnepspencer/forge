use forge_query::facade::QuerySubscriptionCertificationBundle;

fn main() {
    let _ = QuerySubscriptionCertificationBundle {
        certification_bundle_digest: String::new(),
        admission_digest: String::new(),
        activation_digest: String::new(),
        query_declaration_digest: String::new(),
        bridge_declaration_digest: String::new(),
        basis_binding_digest: String::new(),
        signal_strategy_digest: String::new(),
        diagnostics_digest: String::new(),
        support_profile_digest: String::new(),
        admission_counter_digest: String::new(),
        activation_counter_digest: String::new(),
        scale_slope_digest: String::new(),
        scale_activation_digest: String::new(),
        scale_admission_digest: String::new(),
    };
}
