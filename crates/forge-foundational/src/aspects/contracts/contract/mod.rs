mod construction;
mod evolution_classification;
mod mask_admission;

pub(crate) use mask_admission::MaskModeAdmission;

use super::{AbsenceLaw, AspectEquivalenceBasis, AspectShape};
use crate::aspects::identity::{AspectContractRevision, AspectIdentity};
use crate::aspects::keys::AspectKey;
use crate::aspects::masks::AspectMaskContract;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectContract {
    key: AspectKey,
    identity: AspectIdentity,
    revision: AspectContractRevision,
    shape: AspectShape,
    masks: AspectMaskContract,
    absence: AbsenceLaw,
    equivalence: AspectEquivalenceBasis,
    evolution: crate::aspects::evolution::AspectEvolutionPolicy,
}

impl AspectContract {
    pub fn key(&self) -> &AspectKey {
        &self.key
    }

    pub fn identity(&self) -> AspectIdentity {
        self.identity
    }

    pub fn revision(&self) -> AspectContractRevision {
        self.revision
    }

    pub fn shape(&self) -> &AspectShape {
        &self.shape
    }

    pub fn masks(&self) -> &AspectMaskContract {
        &self.masks
    }

    pub fn absence(&self) -> AbsenceLaw {
        self.absence
    }

    pub fn equivalence(&self) -> AspectEquivalenceBasis {
        self.equivalence
    }

    pub fn evolution(&self) -> crate::aspects::evolution::AspectEvolutionPolicy {
        self.evolution
    }
}
