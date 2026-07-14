use super::domain_marker::WorthQueryDomainEntryMarker;
use super::support_snapshot::WorthQueryDomainEntrySupportSnapshot;
use crate::application::domain_handle::{
    WorthQueryConfiguredDomainHandleDraft, WorthQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEntryRoot<D: WorthQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainEntryRoot<D> {
    pub(crate) fn new(marker: D, support_snapshot: WorthQueryDomainEntrySupportSnapshot) -> Self {
        Self {
            marker,
            support_snapshot,
        }
    }

    pub fn marker(&self) -> D {
        self.marker
    }

    pub fn domain_key(&self) -> &'static str {
        self.marker.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.marker.display_name()
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn with_operating_context<C: WorthQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> WorthQueryConfiguredDomainHandleDraft<D, C> {
        WorthQueryConfiguredDomainHandleDraft::new(
            self.marker,
            operating_context,
            self.support_snapshot,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEntryProofRoot<D: WorthQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainEntryProofRoot<D> {
    pub(crate) fn new(marker: D, support_snapshot: WorthQueryDomainEntrySupportSnapshot) -> Self {
        Self {
            marker,
            support_snapshot,
        }
    }

    pub fn marker(&self) -> D {
        self.marker
    }

    pub fn domain_key(&self) -> &'static str {
        self.marker.domain_key()
    }

    pub fn display_name(&self) -> &'static str {
        self.marker.display_name()
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn with_operating_context<C: WorthQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> WorthQueryConfiguredDomainHandleDraft<D, C> {
        WorthQueryConfiguredDomainHandleDraft::new(
            self.marker,
            operating_context,
            self.support_snapshot,
        )
    }
}
