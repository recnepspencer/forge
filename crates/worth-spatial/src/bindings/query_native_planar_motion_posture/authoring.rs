use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_motion_posture::domain::{
    PlanarMotionPostureDeclarationFamily, PlanarMotionPostureQueryDomain,
};
use crate::planar_contracts::motion_posture::{
    planar_motion_posture_authority_entries, PlanarMotionPostureBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarMotionPostureCase {
    basis: PlanarMotionPostureBasis,
}

impl PlanarMotionPostureCase {
    pub fn from_basis(basis: PlanarMotionPostureBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarMotionPostureBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarMotionPostureEntry {
    case: PlanarMotionPostureCase,
}

impl PlanarMotionPostureEntry {
    pub fn case(&self) -> &PlanarMotionPostureCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarMotionPostureQueryDomain> for PlanarMotionPostureEntry {
    type Family = PlanarMotionPostureDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_motion_posture_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_motion_posture_entry(case: PlanarMotionPostureCase) -> PlanarMotionPostureEntry {
    PlanarMotionPostureEntry { case }
}
