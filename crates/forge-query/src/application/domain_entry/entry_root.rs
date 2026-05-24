use super::domain_marker::ForgeQueryDomainEntryMarker;
use super::support_snapshot::ForgeQueryDomainEntrySupportSnapshot;
use crate::application::domain_handle::{
    ForgeQueryConfiguredDomainHandleDraft, ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntryRoot<D: ForgeQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
}

impl<D: ForgeQueryDomainEntryMarker> ForgeQueryDomainEntryRoot<D> {
    pub(crate) fn new(marker: D, support_snapshot: ForgeQueryDomainEntrySupportSnapshot) -> Self {
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn with_operating_context<C: ForgeQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> ForgeQueryConfiguredDomainHandleDraft<D, C> {
        ForgeQueryConfiguredDomainHandleDraft::new(
            self.marker,
            operating_context,
            self.support_snapshot,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntryProofRoot<D: ForgeQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
}

impl<D: ForgeQueryDomainEntryMarker> ForgeQueryDomainEntryProofRoot<D> {
    pub(crate) fn new(marker: D, support_snapshot: ForgeQueryDomainEntrySupportSnapshot) -> Self {
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn with_operating_context<C: ForgeQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> ForgeQueryConfiguredDomainHandleDraft<D, C> {
        ForgeQueryConfiguredDomainHandleDraft::new(
            self.marker,
            operating_context,
            self.support_snapshot,
        )
    }
}
