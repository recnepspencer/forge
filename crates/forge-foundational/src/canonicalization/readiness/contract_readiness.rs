use forge_proof::Artifact;

use crate::aspects::AspectContract;
use crate::canonicalization::{CanonicalDigestPreparationEntry, DigestPreparationReady};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestPreparationReadyAspectContract {
    contract: AspectContract,
    basis: Vec<CanonicalDigestPreparationEntry>,
}

impl DigestPreparationReadyAspectContract {
    pub(crate) fn new(
        contract: AspectContract,
        basis: Vec<CanonicalDigestPreparationEntry>,
    ) -> Self {
        Self { contract, basis }
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn basis(&self) -> &[CanonicalDigestPreparationEntry] {
        &self.basis
    }
}

pub type DigestPreparationReadyAspectContractArtifact =
    Artifact<DigestPreparationReady, DigestPreparationReadyAspectContract>;
