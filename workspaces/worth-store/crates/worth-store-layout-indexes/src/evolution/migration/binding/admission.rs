use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_compatibility::RollingWindowCompatibilityReceipt;

use crate::{AdmittedPhysicalArtifactFamily, ArtifactFamilyLifecycleAdmission};

use super::{LayoutEvolutionDeclaration, LayoutEvolutionDenial, LayoutVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutBindingSourceIdentity {
    Bootstrap {
        root: worth_store_physical_format::PhysicalRootReference,
        owner: worth_store_physical_format::PhysicalGenerationOwner,
        format: worth_store_physical_format::PhysicalFormatVersion,
        physical_source: worth_store_physical_format::PhysicalReference,
    },
    Publication(worth_store_physical_format::PhysicalReference),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::evolution::migration) struct LayoutBindingFingerprint {
    family: worth_store_contracts::DurableArtifactFamilyId,
    security: worth_store_security::StoreSecurityScopeIdentity,
    authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    source: LayoutBindingSourceIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBindingWitness {
    family: AdmittedPhysicalArtifactFamily,
    bound_version: LayoutVersion,
    observed_version: LayoutVersion,
    bound_authority: StoreCurrentAuthorityWitness,
    source: LayoutBindingSourceIdentity,
}

impl LayoutBindingWitness {
    fn issue(
        family: AdmittedPhysicalArtifactFamily,
        bound_version: LayoutVersion,
        observed_version: LayoutVersion,
        bound_authority: StoreCurrentAuthorityWitness,
        source: LayoutBindingSourceIdentity,
    ) -> Self {
        Self {
            family,
            bound_version,
            observed_version,
            bound_authority,
            source,
        }
    }

    pub(in crate::evolution::migration) fn issue_transition(
        source: &Self,
        version: LayoutVersion,
        publication: worth_store_physical_format::RootPublicationValidationWitness,
    ) -> Self {
        Self {
            family: source.family,
            bound_version: version,
            observed_version: version,
            bound_authority: source.bound_authority.clone(),
            source: LayoutBindingSourceIdentity::Publication(publication.reference()),
        }
    }

    pub const fn family(&self) -> ArtifactFamilyLifecycleAdmission {
        self.family.lifecycle()
    }
    pub const fn admitted_family(&self) -> AdmittedPhysicalArtifactFamily {
        self.family
    }
    pub const fn security_identity(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.family.security_identity()
    }
    pub const fn bound_version(&self) -> LayoutVersion {
        self.bound_version
    }
    pub const fn observed_version(&self) -> LayoutVersion {
        self.observed_version
    }
    pub const fn bound_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.bound_authority
    }
    pub const fn bootstrap_source(
        &self,
    ) -> Option<worth_store_physical_format::PhysicalRootReference> {
        match self.source {
            LayoutBindingSourceIdentity::Bootstrap { root, .. } => Some(root),
            LayoutBindingSourceIdentity::Publication(_) => None,
        }
    }
    pub const fn publication_source(
        &self,
    ) -> Option<worth_store_physical_format::PhysicalReference> {
        match self.source {
            LayoutBindingSourceIdentity::Publication(source) => Some(source),
            LayoutBindingSourceIdentity::Bootstrap { .. } => None,
        }
    }

    pub const fn source_identity(&self) -> LayoutBindingSourceIdentity {
        self.source
    }

    pub(in crate::evolution::migration) fn accepts_publication_source(
        &self,
        actual: worth_store_physical_format::RootPublicationValidationWitness,
    ) -> bool {
        match self.source {
            LayoutBindingSourceIdentity::Bootstrap {
                physical_source, ..
            } => actual.reference() == physical_source,
            LayoutBindingSourceIdentity::Publication(expected) => actual.reference() == expected,
        }
    }

    pub(in crate::evolution::migration) const fn fingerprint(&self) -> LayoutBindingFingerprint {
        LayoutBindingFingerprint {
            family: self.family.family_id(),
            security: self.family.security_identity(),
            authority: self.family.authority_identity(),
            bound_version: self.bound_version,
            observed_version: self.observed_version,
            source: self.source,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutBindingRequest {
    declaration: LayoutEvolutionDeclaration,
    family: AdmittedPhysicalArtifactFamily,
    authority: StoreCurrentAuthorityWitness,
    compatibility: RollingWindowCompatibilityReceipt,
    physical_source: worth_store_physical_isolation::PublicationRootCandidate,
    catalog: crate::BootstrapCatalogReadAdmission,
}

impl LayoutBindingRequest {
    pub fn from_bootstrap_catalog(
        declaration: LayoutEvolutionDeclaration,
        family: AdmittedPhysicalArtifactFamily,
        authority: StoreCurrentAuthorityWitness,
        compatibility: RollingWindowCompatibilityReceipt,
        physical_source: worth_store_physical_isolation::PublicationRootCandidate,
        catalog: &crate::BootstrapCatalogReadAdmission,
    ) -> Self {
        Self {
            declaration,
            family,
            authority,
            compatibility,
            physical_source,
            catalog: catalog.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LayoutBindingAdmissionCase {
    Admitted(LayoutBindingWitness),
    Denied(LayoutEvolutionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutBindingAdmissionCaseId {
    Admitted,
    Denied(super::super::LayoutEvolutionDenialKind),
}

impl LayoutBindingAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        use super::super::LayoutEvolutionDenialKind as Denial;
        match self {
            Self::Admitted => "layout.evolution.binding.admitted",
            Self::Denied(Denial::FamilyMismatch) => {
                "layout.evolution.binding.denied.family_mismatch"
            }
            Self::Denied(Denial::StoreAuthorityMismatch) => {
                "layout.evolution.binding.denied.store_authority"
            }
            Self::Denied(Denial::PhysicalSourceStoreAuthorityMismatch) => {
                "layout.evolution.binding.denied.physical_source_authority"
            }
            Self::Denied(Denial::CompatibilityAdmissionMismatch) => {
                "layout.evolution.binding.denied.compatibility"
            }
            Self::Denied(_) => "layout.evolution.binding.denied.unadvertised",
        }
    }
}

pub fn layout_binding_admission_cases() -> impl Iterator<Item = LayoutBindingAdmissionCaseId> {
    use super::super::LayoutEvolutionDenialKind as Denial;
    [
        LayoutBindingAdmissionCaseId::Admitted,
        LayoutBindingAdmissionCaseId::Denied(Denial::FamilyMismatch),
        LayoutBindingAdmissionCaseId::Denied(Denial::StoreAuthorityMismatch),
        LayoutBindingAdmissionCaseId::Denied(Denial::PhysicalSourceStoreAuthorityMismatch),
        LayoutBindingAdmissionCaseId::Denied(Denial::CompatibilityAdmissionMismatch),
    ]
    .into_iter()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBindingAdmissionOutcome {
    case: LayoutBindingAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutBindingAdmissionView<'a> {
    Admitted(&'a LayoutBindingWitness),
    Denied(&'a LayoutEvolutionDenial),
}

impl LayoutBindingAdmissionOutcome {
    fn admitted(witness: LayoutBindingWitness) -> Self {
        Self {
            case: LayoutBindingAdmissionCase::Admitted(witness),
        }
    }

    fn denied(denial: LayoutEvolutionDenial) -> Self {
        Self {
            case: LayoutBindingAdmissionCase::Denied(denial),
        }
    }

    pub const fn view(&self) -> LayoutBindingAdmissionView<'_> {
        match &self.case {
            LayoutBindingAdmissionCase::Admitted(value) => {
                LayoutBindingAdmissionView::Admitted(value)
            }
            LayoutBindingAdmissionCase::Denied(value) => LayoutBindingAdmissionView::Denied(value),
        }
    }

    pub const fn case_id(&self) -> LayoutBindingAdmissionCaseId {
        match &self.case {
            LayoutBindingAdmissionCase::Admitted(_) => LayoutBindingAdmissionCaseId::Admitted,
            LayoutBindingAdmissionCase::Denied(denial) => {
                LayoutBindingAdmissionCaseId::Denied(denial.kind())
            }
        }
    }

    pub fn into_admitted(self) -> Result<LayoutBindingWitness, LayoutEvolutionDenial> {
        match self.case {
            LayoutBindingAdmissionCase::Admitted(value) => Ok(value),
            LayoutBindingAdmissionCase::Denied(value) => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutEvolutionBinding;

pub const fn layout_evolution_binding() -> LayoutEvolutionBinding {
    LayoutEvolutionBinding
}

impl LayoutEvolutionBinding {
    pub fn admit(self, request: LayoutBindingRequest) -> LayoutBindingAdmissionOutcome {
        let declared = request.declaration.family().declaration();
        let admitted = request.family.lifecycle().declaration();
        if declared != admitted {
            return LayoutBindingAdmissionOutcome::denied(LayoutEvolutionDenial::FamilyMismatch {
                declared,
                binding: admitted,
            });
        }
        if request.family.authority_identity() != request.authority.authority_identity() {
            return LayoutBindingAdmissionOutcome::denied(
                LayoutEvolutionDenial::StoreAuthorityMismatch {
                    family: request.family.authority_identity(),
                    binding: request.authority.authority_identity(),
                },
            );
        }
        let physical_source_authority = request.physical_source.root().store_authority_identity();
        if physical_source_authority != request.authority.authority_identity() {
            return LayoutBindingAdmissionOutcome::denied(
                LayoutEvolutionDenial::PhysicalSourceStoreAuthorityMismatch {
                    binding: request.authority.authority_identity(),
                    physical_source: physical_source_authority,
                },
            );
        }
        if request.compatibility.plan().window()
            != request.declaration.compatibility_window().artifact_window()
        {
            return LayoutBindingAdmissionOutcome::denied(
                LayoutEvolutionDenial::CompatibilityAdmissionMismatch,
            );
        }
        LayoutBindingAdmissionOutcome::admitted(LayoutBindingWitness::issue(
            request.family,
            request.declaration.migration_source(),
            request.declaration.migration_source(),
            request.authority,
            LayoutBindingSourceIdentity::Bootstrap {
                root: request.catalog.identity().root_reference(),
                owner: request.catalog.identity().root_owner(),
                format: request.catalog.identity().physical_format_version(),
                physical_source: request.physical_source.validation().reference(),
            },
        ))
    }
}
