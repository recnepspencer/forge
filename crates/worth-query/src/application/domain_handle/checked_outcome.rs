use super::admitted_handle::WorthQueryAdmittedConfiguredDomainHandle;
use super::draft::WorthQueryConfiguredDomainHandleDraft;
use super::operating_context::{
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
};
use super::validated_handle::WorthQueryValidatedConfiguredDomainHandle;
use super::validation::validate_configured_domain_handle_draft;
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryChecked,
    WorthQueryDomainEntryMarker, WorthQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConfiguredDomainHandleChecked<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    Admitted(WorthQueryAdmittedConfiguredDomainHandle<D, C>),
    Deferred(WorthQueryConfiguredDomainHandleDeferred<D, C>),
    Unsupported(WorthQueryConfiguredDomainHandleUnsupported<D, C>),
    InvalidContext(WorthQueryConfiguredDomainHandleInvalidContext<D, C>),
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryConfiguredDomainHandleChecked<D, C>
{
    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        match self {
            Self::Admitted(handle) => handle.support_snapshot(),
            Self::Deferred(denial) => denial.support_snapshot(),
            Self::Unsupported(denial) => denial.support_snapshot(),
            Self::InvalidContext(denial) => denial.support_snapshot(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConfiguredDomainHandleAdmissionError<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    Deferred(WorthQueryConfiguredDomainHandleDeferred<D, C>),
    Unsupported(WorthQueryConfiguredDomainHandleUnsupported<D, C>),
    InvalidContext(WorthQueryConfiguredDomainHandleInvalidContext<D, C>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfiguredDomainHandleDeferred<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
    blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
    blocking_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryConfiguredDomainHandleDeferred<D, C>
{
    pub(crate) fn new(
        validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
        blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
        blocking_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    ) -> Self {
        Self {
            validated_handle,
            blocking_capability_families,
            blocking_operating_requirements,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.validated_handle.handle_identity_digest()
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        self.validated_handle.support_snapshot()
    }

    pub fn blocking_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.blocking_capability_families
    }

    pub fn blocking_operating_requirements(&self) -> &[WorthQueryDomainOperatingRequirement] {
        &self.blocking_operating_requirements
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfiguredDomainHandleUnsupported<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
    blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
    blocking_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryConfiguredDomainHandleUnsupported<D, C>
{
    pub(crate) fn new(
        validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
        blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
        blocking_operating_requirements: Vec<WorthQueryDomainOperatingRequirement>,
    ) -> Self {
        Self {
            validated_handle,
            blocking_capability_families,
            blocking_operating_requirements,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.validated_handle.handle_identity_digest()
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        self.validated_handle.support_snapshot()
    }

    pub fn blocking_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.blocking_capability_families
    }

    pub fn blocking_operating_requirements(&self) -> &[WorthQueryDomainOperatingRequirement] {
        &self.blocking_operating_requirements
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfiguredDomainHandleInvalidContext<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    blocking_config_sections: Vec<WorthQueryConfigSectionFamily>,
    reason: &'static str,
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryConfiguredDomainHandleInvalidContext<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
        blocking_config_sections: Vec<WorthQueryConfigSectionFamily>,
        reason: &'static str,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
            blocking_config_sections,
            reason,
        }
    }

    pub fn marker(&self) -> D {
        self.marker
    }

    pub fn operating_context(&self) -> &C {
        &self.operating_context
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_config_sections(&self) -> &[WorthQueryConfigSectionFamily] {
        &self.blocking_config_sections
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub(crate) fn worth_query_checked_configured_domain_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    checked_entry: WorthQueryDomainEntryChecked<D>,
    operating_context: C,
) -> WorthQueryConfiguredDomainHandleChecked<D, C> {
    let (marker, support_snapshot) = match checked_entry {
        WorthQueryDomainEntryChecked::Admitted(root) => {
            (root.marker(), root.support_snapshot().clone())
        }
        WorthQueryDomainEntryChecked::Deferred(deferred) => {
            (deferred.marker(), deferred.support_snapshot().clone())
        }
        WorthQueryDomainEntryChecked::Unsupported(unsupported) => {
            (unsupported.marker(), unsupported.support_snapshot().clone())
        }
    };

    let draft =
        WorthQueryConfiguredDomainHandleDraft::new(marker, operating_context, support_snapshot);

    match validate_configured_domain_handle_draft(draft) {
        Ok(validated) => super::admission::checked_from_validated_handle(validated),
        Err(denial) => WorthQueryConfiguredDomainHandleChecked::InvalidContext(denial),
    }
}
