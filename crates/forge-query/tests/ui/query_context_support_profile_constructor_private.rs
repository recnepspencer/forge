use forge_query::facade::ForgeQueryQueryContextSupportProfile;

fn main() {
    let _ = ForgeQueryQueryContextSupportProfile {
        admitted_basis_families: Vec::new(),
        admitted_comparison_families: Vec::new(),
        deferred_scope_markers: Vec::new(),
        profile_digest: String::new(),
    };
}
