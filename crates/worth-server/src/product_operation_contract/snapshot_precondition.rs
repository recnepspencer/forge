use super::WorthServerProductOperationBaseDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductSnapshotPrecondition {
    BaseDigest {
        base_digest: WorthServerProductOperationBaseDigest,
        canonical_digest: String,
    },
}

impl WorthServerProductSnapshotPrecondition {
    pub fn at_base_digest(base_digest: WorthServerProductOperationBaseDigest) -> Self {
        let canonical_digest = format!(
            "worth-server-product-snapshot-precondition-v1|kind=base-digest|base:{}",
            base_digest.canonical_digest()
        );
        Self::BaseDigest {
            base_digest,
            canonical_digest,
        }
    }

    pub fn base_digest(&self) -> &WorthServerProductOperationBaseDigest {
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
