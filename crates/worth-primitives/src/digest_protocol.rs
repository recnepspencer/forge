use sha2::{Digest, Sha256};

pub const DIGEST_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TruthDigestVersion(u32);

impl TruthDigestVersion {
    pub const fn current() -> Self {
        Self(DIGEST_VERSION)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TruthDigestScope {
    ArtifactIdentity,
    GeometryIdentity,
    WitnessIdentity,
    ContractIdentity,
}

impl TruthDigestScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentity => "artifact-identity",
            Self::GeometryIdentity => "geometry-identity",
            Self::WitnessIdentity => "witness-identity",
            Self::ContractIdentity => "contract-identity",
        }
    }
}

pub fn truth_digest_parts(scope: TruthDigestScope, parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("worth-primitives-digest:v{}", DIGEST_VERSION).as_bytes());
    hasher.update([0u8]);
    hasher.update(scope.as_str().as_bytes());
    hasher.update([0u8]);
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::{truth_digest_parts, TruthDigestScope, TruthDigestVersion, DIGEST_VERSION};

    #[test]
    fn digest_protocol_is_domain_separated_and_versioned() {
        let parts = vec!["alpha".to_string(), "beta".to_string()];
        let geometry = truth_digest_parts(TruthDigestScope::GeometryIdentity, &parts);
        let witness = truth_digest_parts(TruthDigestScope::WitnessIdentity, &parts);
        let contract = truth_digest_parts(TruthDigestScope::ContractIdentity, &parts);

        assert_eq!(TruthDigestVersion::current().value(), DIGEST_VERSION);
        assert_ne!(geometry, witness);
        assert_ne!(geometry, contract);
        assert_ne!(witness, contract);
    }

    #[test]
    fn digest_protocol_is_order_sensitive_and_length_delimited() {
        let canonical = vec!["ab".to_string(), "c".to_string()];
        let reordered = vec!["c".to_string(), "ab".to_string()];
        let fused = vec!["abc".to_string()];

        let canonical_digest = truth_digest_parts(TruthDigestScope::GeometryIdentity, &canonical);
        let reordered_digest = truth_digest_parts(TruthDigestScope::GeometryIdentity, &reordered);
        let fused_digest = truth_digest_parts(TruthDigestScope::GeometryIdentity, &fused);

        assert_ne!(canonical_digest, reordered_digest);
        assert_ne!(canonical_digest, fused_digest);
    }
}
