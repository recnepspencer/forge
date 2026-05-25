use super::admitted_handle::ForgeQueryAdmittedConfiguredDomainHandle;
use super::draft::ForgeQueryConfiguredDomainHandleDraft;
use super::operating_context::ForgeQueryDomainOperatingContext;
use super::validated_handle::ForgeQueryValidatedConfiguredDomainHandle;
use super::validation::validate_configured_domain_handle_draft;
use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainEntrySupportSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryConfiguredDomainHandleChecked<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    Admitted(ForgeQueryAdmittedConfiguredDomainHandle<D, C>),
    Deferred(ForgeQueryConfiguredDomainHandleDeferred<D, C>),
    Unsupported(ForgeQueryConfiguredDomainHandleUnsupported<D, C>),
    InvalidContext(ForgeQueryConfiguredDomainHandleInvalidContext<D, C>),
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryConfiguredDomainHandleChecked<D, C>
{
    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        match self {
            Self::Admitted(handle) => handle.support_snapshot(),
            Self::Deferred(denial) => denial.support_snapshot(),
            Self::Unsupported(denial) => denial.support_snapshot(),
            Self::InvalidContext(denial) => denial.support_snapshot(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryConfiguredDomainHandleAdmissionError<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    Deferred(ForgeQueryConfiguredDomainHandleDeferred<D, C>),
    Unsupported(ForgeQueryConfiguredDomainHandleUnsupported<D, C>),
    InvalidContext(ForgeQueryConfiguredDomainHandleInvalidContext<D, C>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConfiguredDomainHandleDeferred<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
    blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryConfiguredDomainHandleDeferred<D, C>
{
    pub(crate) fn new(
        validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
        blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
    ) -> Self {
        Self {
            validated_handle,
            blocking_capability_families,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.validated_handle.handle_identity_digest()
    }

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        self.validated_handle.support_snapshot()
    }

    pub fn blocking_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConfiguredDomainHandleUnsupported<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
    blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryConfiguredDomainHandleUnsupported<D, C>
{
    pub(crate) fn new(
        validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
        blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
    ) -> Self {
        Self {
            validated_handle,
            blocking_capability_families,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.validated_handle.handle_identity_digest()
    }

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        self.validated_handle.support_snapshot()
    }

    pub fn blocking_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConfiguredDomainHandleInvalidContext<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    blocking_config_sections: Vec<ForgeQueryConfigSectionFamily>,
    reason: &'static str,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryConfiguredDomainHandleInvalidContext<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        blocking_config_sections: Vec<ForgeQueryConfigSectionFamily>,
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_config_sections(&self) -> &[ForgeQueryConfigSectionFamily] {
        &self.blocking_config_sections
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub(crate) fn forge_query_checked_configured_domain_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    checked_entry: ForgeQueryDomainEntryChecked<D>,
    operating_context: C,
) -> ForgeQueryConfiguredDomainHandleChecked<D, C> {
    let (marker, support_snapshot) = match checked_entry {
        ForgeQueryDomainEntryChecked::Admitted(root) => {
            (root.marker(), root.support_snapshot().clone())
        }
        ForgeQueryDomainEntryChecked::Deferred(deferred) => {
            (deferred.marker(), deferred.support_snapshot().clone())
        }
        ForgeQueryDomainEntryChecked::Unsupported(unsupported) => {
            (unsupported.marker(), unsupported.support_snapshot().clone())
        }
    };

    let draft =
        ForgeQueryConfiguredDomainHandleDraft::new(marker, operating_context, support_snapshot);

    match validate_configured_domain_handle_draft(draft) {
        Ok(validated) => super::admission::checked_from_validated_handle(validated),
        Err(denial) => ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial),
    }
}
