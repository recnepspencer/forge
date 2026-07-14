use worth_query::facade::foundation::{BasisDigest, CanonicalQueryDigest, SchemaBasisDigest};

fn main() {
    let parts = vec!["raw".to_string(), "authority".to_string()];
    let _ = CanonicalQueryDigest::from_domain_parts(&parts);
    let _ = SchemaBasisDigest::from_domain_parts(&parts);
    let _ = BasisDigest::from_domain_parts(&parts);
}
