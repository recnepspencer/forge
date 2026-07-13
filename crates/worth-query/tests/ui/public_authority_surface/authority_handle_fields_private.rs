use worth_query::facade::foundation::{CanonicalQueryDigest, QueryCanonicalAuthority};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn forge(identity: WorthQueryEvidenceIdentity, digest: CanonicalQueryDigest) {
    let _ = QueryCanonicalAuthority { identity, digest };
}

fn main() {}
