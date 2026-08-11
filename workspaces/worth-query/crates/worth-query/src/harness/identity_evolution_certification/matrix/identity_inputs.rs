use crate::identity::{BasisDigest, CanonicalQueryDigest};

pub(super) fn query_digest(seed: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("identity-evolution-query:{seed}")])
}

pub(super) fn basis_digest(seed: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("identity-evolution-basis:{seed}")])
}
