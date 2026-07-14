mod bridge_routing;
mod continuation;
mod contribution_composed_orchestration;
mod declaration;
mod declaration_entry;
mod envelope;
mod evidence;
mod family_helpers;
mod grouped_authoring;
mod receipt;
mod recovery;
mod relational_routing;
mod route_plan;
mod signal_compatibility;
mod signal_compatibility_orchestration;

pub use declaration_entry::WorthQueryDeclarationEntryProgressionError;
pub(crate) use route_plan::checked_route_plan_from_progressed_with_profile;

use super::operating_context::WorthQueryDomainOperatingContext;
use super::{
    compose_admitted_configured_domain_handle_identity, compose_basis_lifecycle_support_identity,
    WorthQueryAdmittedWorldBasis,
};
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot, WorthQueryDomainOperatingRequirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedConfiguredDomainHandle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    required_capability_families: Vec<WorthQueryCapabilityFamily>,
    required_config_sections: Vec<WorthQueryConfigSectionFamily>,
    required_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    operating_context_identity_digest: String,
    handle_identity_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
        required_capability_families: Vec<WorthQueryCapabilityFamily>,
        required_config_sections: Vec<WorthQueryConfigSectionFamily>,
        required_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
        operating_context_identity_digest: String,
        handle_identity_digest: String,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
            required_operating_requirements,
            operating_context_identity_digest,
            handle_identity_digest,
        }
    }

    pub fn domain_key(&self) -> &'static str {
        self.marker.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.marker.display_name()
    }

    pub fn operating_context(&self) -> &C {
        &self.operating_context
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn required_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &[WorthQueryConfigSectionFamily] {
        &self.required_config_sections
    }

    pub fn required_operating_requirements(&self) -> &[WorthQueryDomainOperatingRequirement] {
        &self.required_operating_requirements
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn retained_world_basis(&self) -> WorthQueryAdmittedWorldBasis {
        let basis_lifecycle_support = crate::basis_lifecycle::basis_lifecycle_support_matrix();
        WorthQueryAdmittedWorldBasis::new(
            self.domain_key(),
            self.display_name(),
            self.operating_context_identity_digest.clone(),
            compose_admitted_configured_domain_handle_identity(self),
            self.support_snapshot.snapshot_digest().to_string(),
            compose_basis_lifecycle_support_identity(basis_lifecycle_support.matrix_digest()),
        )
    }
}
