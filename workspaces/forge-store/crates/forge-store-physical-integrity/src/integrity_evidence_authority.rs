use crate::{
    PhysicalIntegrityEvidenceBundle, PhysicalIntegrityEvidenceDenial,
    PhysicalIntegrityEvidenceProfile, StoreExecutedIntegrityEvidence,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityEvidenceAuthority;

impl PhysicalIntegrityEvidenceAuthority {
    pub const fn store_local() -> Self {
        Self
    }

    pub fn materialize(
        self,
        source: StoreExecutedIntegrityEvidence<'_>,
        profile: PhysicalIntegrityEvidenceProfile,
    ) -> Result<PhysicalIntegrityEvidenceBundle, PhysicalIntegrityEvidenceDenial> {
        PhysicalIntegrityEvidenceBundle::from_source(source, profile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityEvidenceEquivalence {
    left: PhysicalIntegrityEvidenceBundle,
    right: PhysicalIntegrityEvidenceBundle,
}

impl PhysicalIntegrityEvidenceEquivalence {
    pub fn from_independent_materializations(
        left: PhysicalIntegrityEvidenceBundle,
        right: PhysicalIntegrityEvidenceBundle,
    ) -> Result<Self, PhysicalIntegrityEvidenceDenial> {
        if left.materialization_path() == right.materialization_path() {
            return Err(PhysicalIntegrityEvidenceDenial::SameMaterializationPath);
        }
        if !left.has_same_evidence_basis_as(&right) {
            return Err(PhysicalIntegrityEvidenceDenial::EvidenceBasisMismatch);
        }
        Ok(Self { left, right })
    }

    pub const fn left(&self) -> &PhysicalIntegrityEvidenceBundle {
        &self.left
    }

    pub const fn right(&self) -> &PhysicalIntegrityEvidenceBundle {
        &self.right
    }
}
