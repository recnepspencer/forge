use super::ForgeServerProductOperationBaseDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductSnapshotPrecondition {
    BaseDigest {
        base_digest: ForgeServerProductOperationBaseDigest,
        canonical_digest: String,
    },
}

impl ForgeServerProductSnapshotPrecondition {
    pub fn at_base_digest(base_digest: ForgeServerProductOperationBaseDigest) -> Self {
        let canonical_digest = format!(
            "forge-server-product-snapshot-precondition-v1|kind=base-digest|base:{}",
            base_digest.canonical_digest()
        );
        Self::BaseDigest {
            base_digest,
            canonical_digest,
        }
    }

    pub fn base_digest(&self) -> &ForgeServerProductOperationBaseDigest {
        match self {
            Self::BaseDigest { base_digest, .. } => base_digest,
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match self {
            Self::BaseDigest {
                canonical_digest, ..
            } => canonical_digest,
        }
    }
}
