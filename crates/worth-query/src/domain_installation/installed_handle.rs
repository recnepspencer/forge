use std::marker::PhantomData;
use std::sync::Arc;

use crate::application::WorthQueryDeclarationEntryContributionCategoryFamily;
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryRuntimeAuthorityIdentity;

use super::WorthQueryDomainPackageIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainInstallationGeneration(u64);

impl WorthQueryDomainInstallationGeneration {
    pub(crate) const fn initial() -> Self {
        Self(1)
    }

    pub const fn ordinal(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainHandle<D> {
    pub(crate) authority: Arc<WorthQueryInstalledDomainAuthority>,
    marker: PhantomData<fn() -> D>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainAuthority {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    generation: WorthQueryDomainInstallationGeneration,
    domain_owner: String,
    package_identity: WorthQueryDomainPackageIdentity,
    installation_identity: WorthQueryEvidenceIdentity,
    contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    world_identity: WorthQueryEvidenceIdentity,
    authority_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInstalledDomainAuthority {
    pub(crate) fn new(
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
        generation: WorthQueryDomainInstallationGeneration,
        domain_owner: String,
        package_identity: WorthQueryDomainPackageIdentity,
        installation_identity: WorthQueryEvidenceIdentity,
        contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    ) -> Self {
        let world_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainWorld)
                .field_value(
                    WorthQueryEvidenceTag::new("runtime_authority"),
                    runtime_authority.as_u64().to_string(),
                )
                .seal();
        let authority_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainHandle)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("installation"),
                    &installation_identity,
                )
                .field_value(
                    WorthQueryEvidenceTag::new("runtime_authority"),
                    runtime_authority.as_u64().to_string(),
                )
                .field_value(
                    WorthQueryEvidenceTag::new("generation"),
                    generation.ordinal().to_string(),
                )
                .field_value_sequence(
                    WorthQueryEvidenceTag::new("contribution_policy"),
                    contribution_policy.iter().map(|category| category.as_str()),
                )
                .seal();
        Self {
            runtime_authority,
            generation,
            domain_owner,
            package_identity,
            installation_identity,
            contribution_policy,
            world_identity,
            authority_identity,
        }
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }
    pub fn package_identity(&self) -> &WorthQueryDomainPackageIdentity {
        &self.package_identity
    }
    pub fn installation_identity(&self) -> &str {
        self.installation_identity.as_str()
    }
    pub const fn installation_generation(&self) -> WorthQueryDomainInstallationGeneration {
        self.generation
    }
    pub fn authority_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.authority_identity
    }
    pub fn contribution_policy(&self) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.contribution_policy
    }

    pub fn world_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.world_identity
    }

    pub(crate) fn permits_contribution(
        &self,
        category: WorthQueryDeclarationEntryContributionCategoryFamily,
    ) -> bool {
        self.contribution_policy.contains(&category)
    }

    pub(crate) fn permits_domain_capability_category(
        &self,
        category: crate::domain_capabilities::WorthQueryDomainCapabilityCategory,
    ) -> bool {
        let category = match category {
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::Admission => {
                WorthQueryDeclarationEntryContributionCategoryFamily::Admission
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::SupportTraceability => {
                WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::InvariantCapability => {
                WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::WorkflowPreview => {
                WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::ContinuityLineage => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::ConsequenceAftermath => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
            }
            crate::domain_capabilities::WorthQueryDomainCapabilityCategory::ExplanationInspection => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
            }
        };
        self.permits_contribution(category)
    }

    pub(crate) fn runtime_authority(&self) -> WorthQueryRuntimeAuthorityIdentity {
        self.runtime_authority
    }
}

impl<D> Clone for WorthQueryInstalledDomainHandle<D> {
    fn clone(&self) -> Self {
        Self {
            authority: Arc::clone(&self.authority),
            marker: PhantomData,
        }
    }
}

impl<D> WorthQueryInstalledDomainHandle<D> {
    pub(crate) fn mint(authority: Arc<WorthQueryInstalledDomainAuthority>) -> Self {
        Self {
            authority,
            marker: PhantomData,
        }
    }

    pub fn package_identity(&self) -> &WorthQueryDomainPackageIdentity {
        self.authority.package_identity()
    }

    pub fn installation_generation(&self) -> WorthQueryDomainInstallationGeneration {
        self.authority.installation_generation()
    }

    pub fn installation_identity(&self) -> &str {
        self.authority.installation_identity()
    }

    pub fn authority(&self) -> &WorthQueryInstalledDomainAuthority {
        &self.authority
    }

    pub(crate) fn authority_arc(&self) -> Arc<WorthQueryInstalledDomainAuthority> {
        Arc::clone(&self.authority)
    }

    pub fn contributions(
        &self,
        runtime: &crate::runtime::WorthQueryRuntime,
    ) -> Result<
        crate::domain_capabilities::WorthQueryInstalledDomainContributionSurface,
        WorthQueryDomainHandleDenial,
    >
    where
        D: 'static,
    {
        runtime.validate_installed_domain_handle(self)?;
        Ok(
            crate::domain_capabilities::WorthQueryInstalledDomainContributionSurface::new(
                self.authority_arc(),
            ),
        )
    }

    pub fn authority_witness(&self) -> super::WorthQueryInstalledDomainAuthorityWitness {
        super::WorthQueryInstalledDomainAuthorityWitness::from_authority(self.authority_arc())
    }

    pub fn rebind_request(&self) -> super::WorthQueryDomainRebindRequest<D> {
        super::WorthQueryDomainRebindRequest::new(self.authority_witness())
    }

    pub fn admit_read(
        &self,
        runtime: &crate::runtime::WorthQueryRuntime,
        family: &crate::runtime::WorthQueryReadFamily,
    ) -> Result<
        super::WorthQueryInstalledDomainReadAdmission<D>,
        super::WorthQueryInstalledDomainReadAdmissionError,
    >
    where
        D: 'static,
    {
        super::WorthQueryInstalledDomainReadAdmission::admit(self, runtime, family)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainHandleDenialKind {
    DomainNotInstalled,
    ForeignRuntime,
    StaleInstallationGeneration,
    PackageIdentityChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainHandleDenial {
    kind: WorthQueryDomainHandleDenialKind,
}

impl WorthQueryDomainHandleDenial {
    pub(crate) const fn new(kind: WorthQueryDomainHandleDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryDomainHandleDenialKind {
        self.kind
    }
}

impl std::fmt::Display for WorthQueryDomainHandleDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "installed domain handle denied: {:?}", self.kind)
    }
}

impl std::error::Error for WorthQueryDomainHandleDenial {}
