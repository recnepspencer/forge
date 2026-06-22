use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_recovery::domain::{
    PlanarRecoveryPostureDeclarationFamily, PlanarRecoveryPostureQueryDomain,
};
use crate::planar_contracts::planar_recovery::{
    planar_recovery_posture_authority_entries, PlanarRecoveryPostureBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPostureCase {
    basis: PlanarRecoveryPostureBasis,
}

impl PlanarRecoveryPostureCase {
    pub fn from_basis(basis: PlanarRecoveryPostureBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarRecoveryPostureBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPostureEntry {
    case: PlanarRecoveryPostureCase,
}

impl PlanarRecoveryPostureEntry {
    pub fn case(&self) -> &PlanarRecoveryPostureCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarRecoveryPostureQueryDomain> for PlanarRecoveryPostureEntry {
    type Family = PlanarRecoveryPostureDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_recovery_posture_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_recovery_posture_entry(
    case: PlanarRecoveryPostureCase,
) -> PlanarRecoveryPostureEntry {
    PlanarRecoveryPostureEntry { case }
}
