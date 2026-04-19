use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeWritebackIdempotenceBasis, BridgeWritebackOutcomeClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackAuthorityOutcome {
    outcome_class: BridgeWritebackOutcomeClass,
    idempotence_digest: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackAuthorityOutcome {
    pub fn canonical_noop(idempotence: &BridgeWritebackIdempotenceBasis) -> Self {
        Self::new(
            BridgeWritebackOutcomeClass::CanonicalNoop,
            idempotence,
            "bridge-writeback-authority-outcome:noop",
        )
    }

    pub fn authoritative_commit(
        idempotence: &BridgeWritebackIdempotenceBasis,
        authoritative_artifact_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            idempotence,
            authoritative_artifact_digest,
        )
    }

    pub fn rejected(
        idempotence: &BridgeWritebackIdempotenceBasis,
        rejection_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(
            BridgeWritebackOutcomeClass::Rejected,
            idempotence,
            rejection_digest,
        )
    }

    fn new(
        outcome_class: BridgeWritebackOutcomeClass,
        idempotence: &BridgeWritebackIdempotenceBasis,
        authoritative_artifact_digest: impl Into<Arc<str>>,
    ) -> Self {
        let authoritative_artifact_digest = authoritative_artifact_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-authority-outcome|class:{outcome_class:?}|idempotence={}|authoritative={}",
            idempotence.digest(),
            authoritative_artifact_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            outcome_class,
            idempotence_digest: Arc::from(idempotence.digest().to_owned()),
            authoritative_artifact_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-authority-outcome:sha256:{digest:x}"
            )),
        }
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn outcome_class(&self) -> BridgeWritebackOutcomeClass {
        self.outcome_class
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }
}
