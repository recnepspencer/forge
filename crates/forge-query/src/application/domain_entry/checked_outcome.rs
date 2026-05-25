use super::domain_marker::ForgeQueryDomainEntryMarker;
use super::entry_root::ForgeQueryDomainEntryRoot;
use super::support_snapshot::ForgeQueryDomainEntrySupportSnapshot;
use crate::application::domain_handle::{
    forge_query_checked_configured_domain_handle, ForgeQueryConfiguredDomainHandleChecked,
    ForgeQueryDomainOperatingContext,
};
use crate::application::ForgeQueryCapabilityFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainEntryChecked<D: ForgeQueryDomainEntryMarker> {
    Admitted(ForgeQueryDomainEntryRoot<D>),
    Deferred(ForgeQueryDomainEntryDeferred<D>),
    Unsupported(ForgeQueryDomainEntryUnsupported<D>),
}

impl<D: ForgeQueryDomainEntryMarker> ForgeQueryDomainEntryChecked<D> {
    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        match self {
            Self::Admitted(root) => root.support_snapshot(),
            Self::Deferred(deferred) => deferred.support_snapshot(),
            Self::Unsupported(unsupported) => unsupported.support_snapshot(),
        }
    }

    pub fn with_operating_context<C: ForgeQueryDomainOperatingContext<D>>(
        self,
        operating_context: C,
    ) -> ForgeQueryConfiguredDomainHandleChecked<D, C> {
        forge_query_checked_configured_domain_handle(self, operating_context)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntryDeferred<D: ForgeQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
}

impl<D: ForgeQueryDomainEntryMarker> ForgeQueryDomainEntryDeferred<D> {
    pub(crate) fn new(
        marker: D,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntryUnsupported<D: ForgeQueryDomainEntryMarker> {
    marker: D,
    support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
    blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
}

impl<D: ForgeQueryDomainEntryMarker> ForgeQueryDomainEntryUnsupported<D> {
    pub(crate) fn new(
        marker: D,
        support_snapshot: ForgeQueryDomainEntrySupportSnapshot,
        blocking_capability_families: Vec<ForgeQueryCapabilityFamily>,
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

    pub fn support_snapshot(&self) -> &ForgeQueryDomainEntrySupportSnapshot {
        &self.support_snapshot
    }

    pub fn blocking_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        &self.blocking_capability_families
    }
}
