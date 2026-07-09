use super::domain_marker::WorthQueryDomainEntryMarker;
use super::entry_root::WorthQueryDomainEntryRoot;
use super::support_snapshot::WorthQueryDomainEntrySupportSnapshot;
use crate::application::domain_handle::{
    worth_query_checked_configured_domain_handle, WorthQueryConfiguredDomainHandleChecked,
    WorthQueryDomainOperatingContext,
};
use crate::application::WorthQueryCapabilityFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEntryChecked<D: WorthQueryDomainEntryMarker> {
    Admitted(WorthQueryDomainEntryRoot<D>),
    Deferred(WorthQueryDomainEntryDeferred<D>),
    Unsupported(WorthQueryDomainEntryUnsupported<D>),
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainEntryChecked<D> {
    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        match self {
            Self::Admitted(root) => root.support_snapshot(),
            Self::Deferred(deferred) => deferred.support_snapshot(),
            Self::Unsupported(unsupported) => unsupported.support_snapshot(),
        }
    }

    pub fn with_operating_context<C: WorthQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> WorthQueryConfiguredDomainHandleChecked<D, C> {
        worth_query_checked_configured_domain_handle(self, operating_context)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEntryDeferred<D: WorthQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainEntryDeferred<D> {
    pub(crate) fn new(
        marker: D,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
        blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
    ) -> Self {
        Self {
            marker,
            support_snapshot,
            blocking_capability_families,
        }
    }

    pub fn marker(&self) -> D {
        self.marker
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEntryUnsupported<D: WorthQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: WorthQueryDomainEntrySupportSnapshot,
    blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
}

impl<D: WorthQueryDomainEntryMarker> WorthQueryDomainEntryUnsupported<D> {
    pub(crate) fn new(
        marker: D,
        support_snapshot: WorthQueryDomainEntrySupportSnapshot,
        blocking_capability_families: Vec<WorthQueryCapabilityFamily>,
    ) -> Self {
        Self {
            marker,
            support_snapshot,
            blocking_capability_families,
        }
    }

    pub fn marker(&self) -> D {
        self.marker
    }

    pub fn support_snapshot(&self) -> &WorthQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_capability_families(&self) -> &[WorthQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}
