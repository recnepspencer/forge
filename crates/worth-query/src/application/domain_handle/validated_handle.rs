use super::admission::admit_configured_domain_handle;
use super::checked_outcome::WorthQueryConfiguredDomainHandleAdmissionError;
use super::operating_context::{
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
};
use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryValidatedConfiguredDomainHandle<
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
    WorthQueryValidatedConfiguredDomainHandle<D, C>
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

    pub(crate) fn marker(&self) -> D {
        self.marker
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

    pub fn admit(
        self,
    ) -> Result<
        WorthQueryAdmittedConfiguredDomainHandle<D, C>,
        WorthQueryConfiguredDomainHandleAdmissionError<D, C>,
    > {
        admit_configured_domain_handle(self)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        D,
        C,
        WorthQueryDomainEntrySupportSnapshot,
        Vec<WorthQueryCapabilityFamily>,
        Vec<WorthQueryConfigSectionFamily>,
        Vec<WorthQueryDomainOperatingRequirement>,
        String,
        String,
    ) {
        (
            self.marker,
            self.operating_context,
            self.support_snapshot,
            self.required_capability_families,
            self.required_config_sections,
            self.required_operating_requirements,
            self.operating_context_identity_digest,
            self.handle_identity_digest,
        )
    }
}
