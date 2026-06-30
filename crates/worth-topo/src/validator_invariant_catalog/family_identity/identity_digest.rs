#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthTopologyLegalityFamilyIdentityDigest(String);

pub(in crate::validator_invariant_catalog) fn legality_family_identity_digest(
    parts: &[&str],
) -> WorthTopologyLegalityFamilyIdentityDigest {
    let mut digest = "worth-topo-legality-family-identity-v1".to_string();
    for part in parts {
        digest.push('|');
        digest.push_str(&(part.len()).to_string());
        digest.push(':');
        digest.push_str(part);
    }
    WorthTopologyLegalityFamilyIdentityDigest(digest)
}

impl WorthTopologyLegalityFamilyIdentityDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WorthTopologyLegalityFamilyIdentityDigest {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
