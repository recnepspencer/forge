use std::marker::PhantomData;

use forge_proof::Artifact;

use crate::aspects::{AspectKey, AspectMask};
use crate::canonicalization::{CanonicalDigestPreparationEntry, DigestPreparationReady};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestPreparationReadyAspectMask<Mode> {
    aspect_key: AspectKey,
    mask: AspectMask<Mode>,
    basis: Vec<CanonicalDigestPreparationEntry>,
    mode: PhantomData<Mode>,
}

impl<Mode> DigestPreparationReadyAspectMask<Mode> {
    pub(crate) fn new(
        aspect_key: AspectKey,
        mask: AspectMask<Mode>,
        basis: Vec<CanonicalDigestPreparationEntry>,
    ) -> Self {
        Self {
            aspect_key,
            mask,
            basis,
            mode: PhantomData,
        }
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn mask(&self) -> &AspectMask<Mode> {
        &self.mask
    }

    pub fn basis(&self) -> &[CanonicalDigestPreparationEntry] {
        &self.basis
    }
}

pub type DigestPreparationReadyAspectMaskArtifact<Mode> =
    Artifact<DigestPreparationReady, DigestPreparationReadyAspectMask<Mode>>;
