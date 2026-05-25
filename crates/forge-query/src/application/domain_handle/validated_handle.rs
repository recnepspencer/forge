use super::admission::admit_configured_domain_handle;
use super::checked_outcome::ForgeQueryConfiguredDomainHandleAdmissionError;
use super::operating_context::ForgeQueryDomainOperatingContext;
use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryValidatedConfiguredDomainHandle<
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
    ForgeQueryValidatedConfiguredDomainHandle<D, C>
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

    pub(crate) fn marker(&self) -> D {
        self.marker
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

    pub fn admit(
        self,
    ) -> Result<
        ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
        ForgeQueryConfiguredDomainHandleAdmissionError<D, C>,
    > {
        admit_configured_domain_handle(self)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        D,
        C,
        ForgeQueryDomainEntrySupportSnapshot,
        Vec<ForgeQueryCapabilityFamily>,
        Vec<ForgeQueryConfigSectionFamily>,
        String,
        String,
    ) {
        (
            self.marker,
            self.operating_context,
            self.support_snapshot,
            self.required_capability_families,
            self.required_config_sections,
            self.operating_context_identity_digest,
            self.handle_identity_digest,
        )
    }
}
