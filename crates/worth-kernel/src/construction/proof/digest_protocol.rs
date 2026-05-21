use sha2::{Digest, Sha256};

const DIGEST_VERSION: &str = "worth-kernel.v1";

// Later 5.7 families will use the full scope inventory; keep the protocol
// complete now without leaving avoidable warnings during the migration.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ConstructionDigestScope {
    ArtifactIdentity,
    ReplayIdentity,
    ParityIdentity,
    CacheIdentity,
}

impl ConstructionDigestScope {
    fn label(self) -> &'static str {
        match self {
            Self::ArtifactIdentity => "artifact-identity",
            Self::ReplayIdentity => "replay-identity",
            Self::ParityIdentity => "parity-identity",
            Self::CacheIdentity => "cache-identity",
        }
    }
}

pub(crate) fn digest_owned_parts(parts: &[String]) -> String {
    digest_owned_parts_with_scope(ConstructionDigestScope::ArtifactIdentity, parts)
}

pub(crate) fn digest_owned_parts_with_scope(
    scope: ConstructionDigestScope,
    parts: &[String],
) -> String {
    let mut hasher = Sha256::new();
    let domain = format!("{DIGEST_VERSION}:{}", scope.label());
    update_part(&mut hasher, &domain);
    for part in parts {
        update_part(&mut hasher, part);
    }
    format!("{domain}:sha256:{:x}", hasher.finalize())
}

fn update_part(hasher: &mut Sha256, part: &str) {
    let bytes = part.as_bytes();
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

#[cfg(test)]
mod tests {
    use super::{
        digest_owned_parts, digest_owned_parts_with_scope, ConstructionDigestScope, DIGEST_VERSION,
    };

    #[test]
    fn digest_protocol_is_domain_separated_and_versioned() {
        let parts = vec!["alpha".to_string(), "beta".to_string()];

        let artifact =
            digest_owned_parts_with_scope(ConstructionDigestScope::ArtifactIdentity, &parts);
        let replay = digest_owned_parts_with_scope(ConstructionDigestScope::ReplayIdentity, &parts);
        let parity = digest_owned_parts_with_scope(ConstructionDigestScope::ParityIdentity, &parts);

        assert!(artifact.starts_with(&format!("{DIGEST_VERSION}:artifact-identity:sha256:")));
        assert!(replay.starts_with(&format!("{DIGEST_VERSION}:replay-identity:sha256:")));
        assert!(parity.starts_with(&format!("{DIGEST_VERSION}:parity-identity:sha256:")));
        assert_ne!(artifact, replay);
        assert_ne!(artifact, parity);
        assert_ne!(replay, parity);
    }

    #[test]
    fn digest_protocol_is_stable_and_order_sensitive() {
        let canonical = vec!["alpha".to_string(), "beta".to_string()];
        let reordered = vec!["beta".to_string(), "alpha".to_string()];

        let canonical_a = digest_owned_parts(&canonical);
        let canonical_b = digest_owned_parts(&canonical);
        let reordered_digest = digest_owned_parts(&reordered);

        assert_eq!(canonical_a, canonical_b);
        assert_ne!(canonical_a, reordered_digest);
    }

    #[test]
    fn digest_protocol_uses_length_delimited_parts() {
        let split = vec!["ab".to_string(), "c".to_string()];
        let fused = vec!["a".to_string(), "bc".to_string()];

        let split_digest = digest_owned_parts(&split);
        let fused_digest = digest_owned_parts(&fused);

        assert_ne!(split_digest, fused_digest);
    }
}
