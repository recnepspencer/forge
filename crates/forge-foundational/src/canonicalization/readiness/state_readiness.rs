use forge_proof::Artifact;

use crate::aspects::AuthoritativeRecordAspectState;
use crate::canonicalization::{CanonicalDigestPreparationEntry, DigestPreparationReady};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestPreparationReadyAspectState {
    state: AuthoritativeRecordAspectState,
    basis: Vec<CanonicalDigestPreparationEntry>,
}

impl DigestPreparationReadyAspectState {
    pub(crate) fn new(
        state: AuthoritativeRecordAspectState,
        basis: Vec<CanonicalDigestPreparationEntry>,
    ) -> Self {
        Self { state, basis }
    }

    pub fn state(&self) -> &AuthoritativeRecordAspectState {
        &self.state
    }

    pub fn basis(&self) -> &[CanonicalDigestPreparationEntry] {
        &self.basis
    }
}

pub type DigestPreparationReadyAspectStateArtifact =
    Artifact<DigestPreparationReady, DigestPreparationReadyAspectState>;
