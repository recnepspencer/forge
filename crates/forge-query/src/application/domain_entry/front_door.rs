use super::checked_outcome::{
    ForgeQueryDomainEntryChecked, ForgeQueryDomainEntryDeferred, ForgeQueryDomainEntryUnsupported,
};
use super::domain_marker::ForgeQueryDomainEntryMarker;
use super::entry_root::{ForgeQueryDomainEntryProofRoot, ForgeQueryDomainEntryRoot};
use super::support_snapshot::ForgeQueryDomainEntrySupportSnapshot;
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus,
};

pub(crate) fn forge_query_domain_entry_support_snapshot(
    facade: &ForgeQueryApplicationFacade,
) -> ForgeQueryDomainEntrySupportSnapshot {
    ForgeQueryDomainEntrySupportSnapshot::from_support_report(facade.support_report())
}

pub(crate) fn forge_query_domain_entry<D: ForgeQueryDomainEntryMarker>(
    facade: &ForgeQueryApplicationFacade,
    marker: D,
) -> ForgeQueryDomainEntryRoot<D> {
    ForgeQueryDomainEntryRoot::new(marker, forge_query_domain_entry_support_snapshot(facade))
}

pub(crate) fn forge_query_domain_proof_root<D: ForgeQueryDomainEntryMarker>(
    facade: &ForgeQueryApplicationFacade,
    marker: D,
) -> ForgeQueryDomainEntryProofRoot<D> {
    ForgeQueryDomainEntryProofRoot::new(marker, forge_query_domain_entry_support_snapshot(facade))
}

pub(crate) fn forge_query_checked_domain_entry<D: ForgeQueryDomainEntryMarker>(
    facade: &ForgeQueryApplicationFacade,
    marker: D,
) -> ForgeQueryDomainEntryChecked<D> {
    let support_snapshot = forge_query_domain_entry_support_snapshot(facade);
    let deferred = blocking_families(
        &support_snapshot,
        marker,
        ForgeQueryCapabilityStatus::DeferredDebt,
    );
    if !deferred.is_empty() {
        return ForgeQueryDomainEntryChecked::Deferred(ForgeQueryDomainEntryDeferred::new(
            marker,
            support_snapshot,
            deferred,
        ));
    }

    let unsupported = blocking_families(
        &support_snapshot,
        marker,
        ForgeQueryCapabilityStatus::Unsupported,
    );
    if !unsupported.is_empty() {
        return ForgeQueryDomainEntryChecked::Unsupported(ForgeQueryDomainEntryUnsupported::new(
            marker,
            support_snapshot,
            unsupported,
        ));
    }

    ForgeQueryDomainEntryChecked::Admitted(ForgeQueryDomainEntryRoot::new(marker, support_snapshot))
}

fn blocking_families<D: ForgeQueryDomainEntryMarker>(
    support_snapshot: &ForgeQueryDomainEntrySupportSnapshot,
    marker: D,
    target: ForgeQueryCapabilityStatus,
) -> Vec<ForgeQueryCapabilityFamily> {
    marker
        .required_capability_families()
        .iter()
        .copied()
        .filter(|family| support_snapshot.capability_status(*family) == Some(target))
        .collect()
}
