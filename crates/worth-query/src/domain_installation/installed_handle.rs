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
