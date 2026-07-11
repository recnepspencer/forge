use crate::ScopedPhysicalValidatorInput;
use crate::{IndexPageIntegrityCounters, ManifestIntegrityDenial, ManifestReferenceBasis};
use crate::{IndexPageIntegrityDenial, IndexPageIntegrityDenialKind};
use forge_store_physical_format::PhysicalScopeFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexIntegrityInspectionRequest<'lease> {
    input: ScopedPhysicalValidatorInput<'lease>,
    authority_evidence: DerivedIndexAuthorityEvidence,
}

impl<'lease> DerivedIndexIntegrityInspectionRequest<'lease> {
    pub fn from_admitted_scope(
        input: ScopedPhysicalValidatorInput<'lease>,
        authority_basis: ManifestReferenceBasis,
    ) -> Result<Self, IndexPageIntegrityDenial> {
        reject_non_derived_family(&input)?;
        Ok(Self {
            input,
            authority_evidence: DerivedIndexAuthorityEvidence::Intact(authority_basis),
        })
    }

    pub fn with_damaged_authority(
        input: ScopedPhysicalValidatorInput<'lease>,
        manifest_denial: ManifestIntegrityDenial,
    ) -> Result<Self, IndexPageIntegrityDenial> {
        reject_non_derived_family(&input)?;
        Ok(Self {
            input,
            authority_evidence: DerivedIndexAuthorityEvidence::Damaged(manifest_denial),
        })
    }

    pub fn without_authority_basis(
        input: ScopedPhysicalValidatorInput<'lease>,
    ) -> Result<Self, IndexPageIntegrityDenial> {
        reject_non_derived_family(&input)?;
        Ok(Self {
            input,
            authority_evidence: DerivedIndexAuthorityEvidence::Missing,
        })
    }

    pub(crate) const fn input(&self) -> &ScopedPhysicalValidatorInput<'lease> {
        &self.input
    }

    pub(crate) const fn authority_evidence(&self) -> &DerivedIndexAuthorityEvidence {
        &self.authority_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DerivedIndexAuthorityEvidence {
    Intact(ManifestReferenceBasis),
    Damaged(ManifestIntegrityDenial),
    Missing,
}

fn reject_non_derived_family(
    input: &ScopedPhysicalValidatorInput<'_>,
) -> Result<(), IndexPageIntegrityDenial> {
    if input.family() == PhysicalScopeFamily::DerivedIndex {
        return Ok(());
    }
    Err(IndexPageIntegrityDenial::new(
        IndexPageIntegrityDenialKind::WrongPhysicalFamily,
        IndexPageIntegrityCounters::start(),
    )
    .with_derived_basis(input.admission().basis().clone()))
}
