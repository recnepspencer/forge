use worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupIndexRebuildDenial;

fn requires_denial(_: EvidenceLookupIndexRebuildDenial) {}
fn fake<T>() -> T {
    panic!("compile-fail placeholder")
}

fn main() {
    requires_denial(EvidenceLookupIndexRebuildDenial {
        denial_identity_digest: String::new(),
        mismatch_loci: Vec::new(),
        selected_equivalence_family_identity: fake(),
        selected_equivalence_basis_identity_digest: String::new(),
        selected_compatibility_basis_identity_digest: String::new(),
        selected_reuse_basis_identity_digest: String::new(),
        counters: fake(),
    });
}
