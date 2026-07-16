use crate::RecoveryEntryBasis;
use worth_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEntryIdentity {
    digest: StableDigest,
}

impl RecoveryEntryIdentity {
    pub(crate) fn from_basis(basis: &RecoveryEntryBasis) -> Self {
        Self {
            digest: StableDigest::new(format!("s4-entry:{basis:?}"))
                .expect("S.4 recovery entry identity basis is non-empty"),
        }
    }

    pub fn digest(&self) -> &StableDigest {
        &self.digest
    }
}
