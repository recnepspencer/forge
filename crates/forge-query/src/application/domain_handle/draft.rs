use super::operating_context::ForgeQueryDomainOperatingContext;
use super::validated_handle::ForgeQueryValidatedConfiguredDomainHandle;
use super::validation::validate_configured_domain_handle_draft;
use crate::application::{ForgeQueryDomainEntryMarker, ForgeQueryDomainEntrySupportSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConfiguredDomainHandleDraft<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryConfiguredDomainHandleDraft<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    ) -> Self {
        Self {
            marker,
            operating_context,
            support_snapshot,
        }
    }

    pub(crate) fn marker(&self) -> D {
        self.marker
    }

    pub(crate) fn operating_context(&self) -> &C {
        &self.operating_context
    }

    pub(crate) fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn validate(
        self,
    ) -> Result<
        ForgeQueryValidatedConfiguredDomainHandle<D, C>,
        crate::application::ForgeQueryConfiguredDomainHandleInvalidContext<D, C>,
    > {
        validate_configured_domain_handle_draft(self)
    }
}
