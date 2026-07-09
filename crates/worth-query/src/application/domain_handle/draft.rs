use super::operating_context::WorthQueryDomainOperatingContext;
use super::validated_handle::WorthQueryValidatedConfiguredDomainHandle;
use super::validation::validate_configured_domain_handle_draft;
use crate::application::{WorthQueryDomainEntryMarker, WorthQueryDomainEntrySupportSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConfiguredDomainHandleDraft<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
> {
    marker: D,
    operating_context: C,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryConfiguredDomainHandleDraft<D, C>
{
    pub(crate) fn new(
        marker: D,
        operating_context: C,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
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

    pub(crate) fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn validate(
        self,
    ) -> Result<
        WorthQueryValidatedConfiguredDomainHandle<D, C>,
        crate::application::WorthQueryConfiguredDomainHandleInvalidContext<D, C>,
    > {
        validate_configured_domain_handle_draft(self)
    }
}
