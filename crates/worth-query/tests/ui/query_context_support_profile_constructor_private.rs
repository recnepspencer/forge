use worth_query::facade::foundation::WorthQueryQueryContextSupportProfile;

fn main() {
    let _ = WorthQueryQueryContextSupportProfile {
        admitted_basis_families: Vec::new(),
        admitted_comparison_families: Vec::new(),
        deferred_scope_markers: Vec::new(),
        profile_digest: String::new(),
    };
}
