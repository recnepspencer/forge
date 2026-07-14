use worth_query::facade::foundation::{BasisDigest, CanonicalQueryDigest, IdentityEvolutionQueryContext, LineageTraversalDescriptor};

fn promote(query: CanonicalQueryDigest, basis: BasisDigest) {
    let _ = IdentityEvolutionQueryContext::lineage_traversal(
        query,
        basis,
        LineageTraversalDescriptor::direct_predecessor("entity"),
    );
}

fn main() {}
