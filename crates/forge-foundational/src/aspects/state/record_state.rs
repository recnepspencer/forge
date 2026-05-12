use forge_proof::{Artifact, PhaseMarker};

use super::{AuthoritativeStateAdmissionDenial, CanonicalAspectStateMap};
use crate::aspects::keys::AspectKey;
use crate::aspects::validation::ContractValidatedAspectValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeRecordAspectState {
    aspects: CanonicalAspectStateMap,
}

impl AuthoritativeRecordAspectState {
    pub(crate) fn from_validated_entries(
        entries: impl IntoIterator<Item = ContractValidatedAspectValue>,
    ) -> Result<Self, AuthoritativeStateAdmissionDenial> {
        Ok(Self {
            aspects: CanonicalAspectStateMap::from_validated_entries(entries)?,
        })
    }

    pub(crate) fn from_canonical_map(aspects: CanonicalAspectStateMap) -> Self {
        Self { aspects }
    }

    pub fn aspects(&self) -> &CanonicalAspectStateMap {
        &self.aspects
    }

    pub fn get(&self, key: &AspectKey) -> Option<&ContractValidatedAspectValue> {
        self.aspects.get(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeRecordAspectStateAdmitted;

impl PhaseMarker for AuthoritativeRecordAspectStateAdmitted {}

pub type AuthoritativeRecordAspectStateArtifact =
    Artifact<AuthoritativeRecordAspectStateAdmitted, AuthoritativeRecordAspectState>;
