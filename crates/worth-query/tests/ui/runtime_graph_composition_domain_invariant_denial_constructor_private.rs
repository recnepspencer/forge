use worth_query::facade::WorthQueryGraphCompositionDomainInvariantDenial;

fn main() {
    let _ = WorthQueryGraphCompositionDomainInvariantDenial {
        hook_family: "domain_invariant_pack_hook".to_string(),
        invariant_family: "non_manifold_topology".to_string(),
        message: "should not compile".to_string(),
        admission_trace: unsafe { std::mem::zeroed() },
        denial_digest: "fake".to_string(),
    };
}
