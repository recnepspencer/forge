use super::checked_outcome::{
    WorthQueryDomainEntryChecked, WorthQueryDomainEntryDeferred, WorthQueryDomainEntryUnsupported,
};
use super::domain_marker::WorthQueryDomainEntryMarker;
use super::entry_root::{WorthQueryDomainEntryProofRoot, WorthQueryDomainEntryRoot};
use super::support_snapshot::WorthQueryDomainEntrySupportSnapshot;
use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryCapabilityStatus,
};

pub(crate) fn worth_query_domain_entry_support_snapshot(
    facade: &WorthQueryApplicationFacade,
) -> WorthQueryDomainEntrySupportSnapshot {
    WorthQueryDomainEntrySupportSnapshot::from_support_report(facade.support_report())
}

pub(crate) fn worth_query_domain_entry<D: WorthQueryDomainEntryMarker>(
    facade: &WorthQueryApplicationFacade,
    marker: D,
) -> WorthQueryDomainEntryRoot<D> {
    WorthQueryDomainEntryRoot::new(marker, worth_query_domain_entry_support_snapshot(facade))
}

pub(crate) fn worth_query_domain_proof_root<D: WorthQueryDomainEntryMarker>(
    facade: &WorthQueryApplicationFacade,
    marker: D,
) -> WorthQueryDomainEntryProofRoot<D> {
    WorthQueryDomainEntryProofRoot::new(marker, worth_query_domain_entry_support_snapshot(facade))
}

pub(crate) fn worth_query_checked_domain_entry<D: WorthQueryDomainEntryMarker>(
    facade: &WorthQueryApplicationFacade,
    marker: D,
) -> WorthQueryDomainEntryChecked<D> {
    let support_snapshot = worth_query_domain_entry_support_snapshot(facade);
    let deferred = blocking_families(
        &support_snapshot,
        marker,
        WorthQueryCapabilityStatus::DeferredDebt,
    );
    if !deferred.is_empty() {
        return WorthQueryDomainEntryChecked::Deferred(WorthQueryDomainEntryDeferred::new(
            marker,
            support_snapshot,
            deferred,
        ));
    }

    let unsupported = blocking_families(
        &support_snapshot,
        marker,
        WorthQueryCapabilityStatus::Unsupported,
    );
    if !unsupported.is_empty() {
        return WorthQueryDomainEntryChecked::Unsupported(WorthQueryDomainEntryUnsupported::new(
            marker,
            support_snapshot,
            unsupported,
        ));
    }

    WorthQueryDomainEntryChecked::Admitted(WorthQueryDomainEntryRoot::new(marker, support_snapshot))
}

fn blocking_families<D: WorthQueryDomainEntryMarker>(
    support_snapshot: &WorthQueryDomainEntrySupportSnapshot,
    marker: D,
    target: WorthQueryCapabilityStatus,
) -> Vec<WorthQueryCapabilityFamily> {
    marker
        .required_capability_families()
        .iter()
        .copied()
        .filter(|family| support_snapshot.capability_status(*family) == Some(target))
        .collect()
}
