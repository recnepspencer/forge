use super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION;
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::maturity::EvidenceBundleReadiness;
use super::row::HarnessMaturityRow;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub(super) struct HarnessMaturityDigestBasis<'a> {
    pub(super) schema_version: &'static str,
    pub(super) artifact_kind: S0ArtifactKind,
    pub(super) source_revision: &'a str,
    pub(super) roadmap_parent_digest: &'a S0StableDigest,
    pub(super) generated_by: &'a str,
    pub(super) readiness: EvidenceBundleReadiness,
    pub(super) rows: &'a [HarnessMaturityRow],
}

pub(super) fn stable_digest<T: Serialize + ?Sized>(
    value: &T,
) -> Result<S0StableDigest, serde_json::Error> {
    let value = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(value);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| serde_json::Error::io(std::io::Error::other("invalid digest")))
}
