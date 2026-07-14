use worth_store_aspect_native::StoreAspectIdentity;

use crate::StoreCurrentAuthorityWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreDerivedAuthorityEvidenceRole {
    DigestProjection,
    TerminalProjection,
    FilenameProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDerivedAuthorityEvidence {
    identity: StoreAspectIdentity,
    role: StoreDerivedAuthorityEvidenceRole,
}

impl StoreDerivedAuthorityEvidence {
    pub(crate) fn from_current_authority(
        current_authority: &StoreCurrentAuthorityWitness,
        role: StoreDerivedAuthorityEvidenceRole,
    ) -> Self {
        Self {
            identity: current_authority.identity().clone(),
            role,
        }
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn role(&self) -> StoreDerivedAuthorityEvidenceRole {
        self.role
    }
}

pub fn report_derived_store_authority_evidence(
    current_authority: &StoreCurrentAuthorityWitness,
    role: StoreDerivedAuthorityEvidenceRole,
) -> StoreDerivedAuthorityEvidence {
    StoreDerivedAuthorityEvidence::from_current_authority(current_authority, role)
}
