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

pub use declaration_entry::ForgeQueryDeclarationEntryProgressionError;
pub(crate) use route_plan::checked_route_plan_from_progressed_with_profile;

use super::operating_context::ForgeQueryDomainOperatingContext;
use crate::application::{
    ForgeQueryAdmittedWorldBasis, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedConfiguredDomainHandle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    required_capability_families: Vec<ForgeQueryCapabilityFamily>,
    required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
    operating_context_identity_digest: String,
    handle_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        required_capability_families: Vec<ForgeQueryCapabilityFamily>,
        required_config_sections: Vec<ForgeQueryConfigSectionFamily>,
        operating_context_identity_digest: String,
        handle_identity_digest: String,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn required_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.required_capability_families
    }

    pub fn required_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        &self.required_config_sections
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn retained_world_basis(&self) -> ForgeQueryAdmittedWorldBasis {
        ForgeQueryAdmittedWorldBasis::new(
            self.domain_key(),
            self.display_name(),
            self.operating_context_identity_digest.clone(),
            self.handle_identity_digest.clone(),
            self.support_snapshot.snapshot_digest().to_string(),
        )
    }
}
