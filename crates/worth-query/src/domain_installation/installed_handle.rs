use std::any::TypeId;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntrySupportSnapshot,
    WorthQueryDomainOperatingRequirement,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryRuntimeAuthorityIdentity;

use super::{WorthQueryDomainDeclarationFamilyDefinition, WorthQueryDomainPackageIdentity};

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
    marker_type: TypeId,
    domain_key: &'static str,
    display_name: &'static str,
    domain_owner: String,
    package_identity: WorthQueryDomainPackageIdentity,
    installation_identity: WorthQueryEvidenceIdentity,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    required_capabilities: Vec<WorthQueryCapabilityFamily>,
    required_configuration: Vec<WorthQueryConfigSectionFamily>,
    operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    declaration_families: BTreeMap<String, u32>,
    contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    world_identity: WorthQueryEvidenceIdentity,
    authority_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInstalledDomainAuthority {
    pub(crate) fn new(
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
        generation: WorthQueryDomainInstallationGeneration,
        marker_type: TypeId,
        domain_key: &'static str,
        display_name: &'static str,
        domain_owner: String,
        package_identity: WorthQueryDomainPackageIdentity,
        installation_identity: WorthQueryEvidenceIdentity,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
        required_capabilities: Vec<WorthQueryCapabilityFamily>,
        required_configuration: Vec<WorthQueryConfigSectionFamily>,
        operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
        declaration_families: Vec<WorthQueryDomainDeclarationFamilyDefinition>,
        contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    ) -> Self {
        let declaration_families = declaration_families
            .into_iter()
            .map(|family| (family.family_key().to_string(), family.version()))
            .collect();
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
            marker_type,
            domain_key,
            display_name,
            domain_owner,
            package_identity,
            installation_identity,
            support_snapshot,
            required_capabilities,
            required_configuration,
            operating_requirements,
            declaration_families,
            contribution_policy,
            world_identity,
            authority_identity,
        }
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }
    pub fn domain_key(&self) -> &'static str {
        self.domain_key
    }
    pub fn display_name(&self) -> &'static str {
        self.display_name
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

    pub fn required_capabilities(&self) -> &[WorthQueryCapabilityFamily] {
        &self.required_capabilities
    }

    pub fn required_configuration(&self) -> &[WorthQueryConfigSectionFamily] {
        &self.required_configuration
    }

    pub fn operating_requirements(&self) -> &[WorthQueryDomainOperatingRequirement] {
        &self.operating_requirements
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn declaration_family_version(&self, family_key: &str) -> Option<u32> {
        self.declaration_families.get(family_key).copied()
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

    pub(crate) fn marker_type(&self) -> TypeId {
        self.marker_type
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

    pub fn domain_key(&self) -> &'static str {
        self.authority.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.authority.display_name()
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

    pub fn contributions_in(
        &self,
        workspace: &crate::runtime::WorthQueryWorkspace,
    ) -> Result<
        crate::domain_capabilities::WorthQueryInstalledDomainContributionSurface,
        WorthQueryDomainHandleDenial,
    >
    where
        D: 'static,
    {
        workspace.validate_installed_domain_witness::<D>(&self.authority_witness())?;
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

    pub fn declarations<C>(
        &self,
        runtime: &crate::runtime::WorthQueryRuntime,
        operating_context: C,
    ) -> Result<
        super::WorthQueryInstalledDomainDeclarationContext<D, C>,
        super::WorthQueryInstalledDomainDeclarationContextDenial,
    >
    where
        D: crate::application::WorthQueryDomainEntryMarker + 'static,
        C: crate::application::WorthQueryDomainOperatingContext<D>,
    {
        runtime
            .validate_installed_domain_handle(self)
            .map_err(super::WorthQueryInstalledDomainDeclarationContextDenial::handle)?;
        super::WorthQueryInstalledDomainDeclarationContext::admit(
            self.authority_witness(),
            operating_context,
        )
    }

    pub fn declarations_in<C>(
        &self,
        workspace: &crate::runtime::WorthQueryWorkspace,
        operating_context: C,
    ) -> Result<
        super::WorthQueryInstalledDomainDeclarationContext<D, C>,
        super::WorthQueryInstalledDomainDeclarationContextDenial,
    >
    where
        D: crate::application::WorthQueryDomainEntryMarker + 'static,
        C: crate::application::WorthQueryDomainOperatingContext<D>,
    {
        let witness = self.authority_witness();
        workspace
            .validate_installed_domain_witness::<D>(&witness)
            .map_err(super::WorthQueryInstalledDomainDeclarationContextDenial::handle)?;
        super::WorthQueryInstalledDomainDeclarationContext::admit(witness, operating_context)
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
