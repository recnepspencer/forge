use worth_proof::Artifact;

use crate::aspects::AuthoritativeRecordAspectPatch;
use crate::canonicalization::{CanonicalDigestPreparationEntry, DigestPreparationReady};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestPreparationReadyAspectPatch {
    patch: AuthoritativeRecordAspectPatch,
    basis: Vec<CanonicalDigestPreparationEntry>,
}

impl DigestPreparationReadyAspectPatch {
    pub(crate) fn new(
        patch: AuthoritativeRecordAspectPatch,
        basis: Vec<CanonicalDigestPreparationEntry>,
    ) -> Self {
        Self { patch, basis }
    }

    pub fn patch(&self) -> &AuthoritativeRecordAspectPatch {
        &self.patch
    }

    pub fn basis(&self) -> &[CanonicalDigestPreparationEntry] {
        &self.basis
    }
}

pub type DigestPreparationReadyAspectPatchArtifact =
    Artifact<DigestPreparationReady, DigestPreparationReadyAspectPatch>;
