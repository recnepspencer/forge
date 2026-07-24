use super::scroll_catalog_evidence::bump;
use super::{
    UiAdmittedScrollInvalidationBinding, UiScrollBindingCatalogCounters,
    UiScrollInvalidationBindingDenial, UiScrollOwnerCatalogReceipt,
};
use std::collections::BTreeMap;
type HostWitness = crate::evidence::UiHostMeasurementAuthorityWitness;
type QueryKey = crate::evidence::measurement::basis::UiQueryAllocationSourceKey;
type QueryBindingKey = crate::evidence::measurement::basis::UiQueryAllocationBindingKey;
use super::scroll_binding_key_index::BindingKeyIndex;
use super::scroll_owner_acquisition::acquire_exact;
type QueryIndex = crate::runtime::persistent_index::UiPersistentOrdMap<QueryKey, BindingKeyIndex>;
type QueryBindingIndex =
    crate::runtime::persistent_index::UiPersistentOrdMap<QueryBindingKey, BindingKeyIndex>;

#[path = "scroll_binding_index/sealing.rs"]
mod sealing;

#[derive(Clone, Debug, Default)]
pub(crate) struct UiScrollInvalidationBindingIndex {
    pub(super) host:
        crate::runtime::persistent_index::UiPersistentOrdMap<HostWitness, BindingKeyIndex>,
    pub(super) query: QueryIndex,
    pub(super) query_by_binding: QueryBindingIndex,
    pub(super) projection_contracts: crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::runtime::UiScrollProjectionOwnerIdentity,
        (
            crate::runtime::UiAdmittedScrollOwnedContract,
            crate::graph::UiGraphNodeIdentity,
            crate::runtime::UiScrollReceiptActivationKey,
        ),
    >,
    pub(super) catalog_receipt: Option<UiScrollOwnerCatalogReceipt>,
}

impl UiScrollInvalidationBindingIndex {
    pub(super) fn host_extent(&self, witness: HostWitness) -> Option<&BindingKeyIndex> {
        self.host.get(&witness)
    }
    pub(super) fn settled_query_extent(
        &self,
        key: &crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
    ) -> (Option<&BindingKeyIndex>, u16) {
        (self.query.get(key), 1)
    }
    pub(super) fn settled_query_binding_extent(
        &self,
        key: &crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
    ) -> (Option<&BindingKeyIndex>, u16) {
        (self.query_by_binding.get(&key.binding_key()), 1)
    }
    pub(crate) fn projection_for_host(
        &self,
        witness: HostWitness,
        receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        let mut probes = 0;
        record_acquisition_lookup(&mut probes)?;
        let rows = self.host.get(&witness);
        let requested = crate::runtime::UiScrollReceiptActivationKey::from_receipt_and_source(
            receipt,
            crate::runtime::UiAdmittedScrollExtentSource::HostViewport { witness },
        );
        record_acquisition_lookup(&mut probes)?;
        let exact = rows.and_then(|rows| rows.get(&requested));
        let mismatch = rows.and_then(|rows| rows.classify(&requested));
        acquire_exact(exact, requested, mismatch, probes)
    }
    pub(crate) fn projection_for_settled_query(
        &self,
        receipt: &crate::evidence::UiSettledQueryFactReceipt,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        let key = crate::evidence::measurement::basis::UiQueryAllocationSourceKey::SettledSnapshot(
            receipt.key().clone(),
        );
        self.projection_for_query_key(&key, allocation_receipt)
    }

    fn projection_for_query_key(
        &self,
        key: &QueryKey,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        let mut probes = 0;
        record_acquisition_lookup(&mut probes)?;
        let (rows, authority_probes) = self.settled_query_extent(key);
        for _ in 0..authority_probes {
            record_acquisition_lookup(&mut probes)?;
        }
        let source_identity = rows
            .and_then(|rows| rows.values().next())
            .map(|binding| binding.contract().source().clone())
            .ok_or(crate::runtime::UiScrollOwnerAcquisitionDenial::ContradictorySource)?;
        let requested = crate::runtime::UiScrollReceiptActivationKey::from_receipt_and_source(
            allocation_receipt,
            source_identity,
        );
        record_acquisition_lookup(&mut probes)?;
        let exact = rows.and_then(|rows| rows.get(&requested));
        let mismatch = rows.and_then(|rows| rows.classify(&requested));
        acquire_exact(exact, requested, mismatch, probes)
    }
    pub(crate) fn projection(
        &self,
        owner: crate::runtime::UiScrollProjectionOwnerIdentity,
    ) -> Option<crate::runtime::UiActivatedScrollProjectionTarget> {
        self.projection_contracts
            .get(&owner)
            .map(|(contract, node, _)| {
                crate::runtime::UiActivatedScrollProjectionTarget::new(
                    *node,
                    contract.graph_generation(),
                    contract.identity_digest(),
                )
            })
    }
    pub(crate) fn validate_projection_receipt(
        &self,
        target: crate::runtime::UiActivatedScrollProjectionTarget,
        key: &crate::runtime::UiScrollReceiptActivationKey,
    ) -> Result<(), crate::runtime::UiScrollOwnerAcquisitionDenial> {
        let (_, _, active_key) = self
            .projection_contracts
            .get(&target.owner_identity())
            .ok_or(crate::runtime::UiScrollOwnerAcquisitionDenial::OwnerNotActive)?;
        if active_key == key {
            Ok(())
        } else {
            Err(active_key.mismatch_denial(key))
        }
    }
}

fn record_acquisition_lookup(
    probes: &mut u16,
) -> Result<(), crate::runtime::UiScrollOwnerAcquisitionDenial> {
    *probes = probes
        .checked_add(1)
        .ok_or(crate::runtime::UiScrollOwnerAcquisitionDenial::AuthorityCounterExhausted)?;
    Ok(())
}

fn freeze<K: Ord + Clone>(
    source: BTreeMap<K, Vec<UiAdmittedScrollInvalidationBinding>>,
    counters: &mut UiScrollBindingCatalogCounters,
) -> Result<
    crate::runtime::persistent_index::UiPersistentOrdMap<K, BindingKeyIndex>,
    UiScrollInvalidationBindingDenial,
> {
    bump(&mut counters.freeze_operations)?;
    let mut frozen = crate::runtime::persistent_index::UiPersistentOrdMap::default();
    for (key, mut rows) in source {
        rows.sort_by_key(|row| {
            row.receipt_key().map_or(
                row.contract().identity_digest(),
                crate::runtime::UiScrollReceiptActivationKey::identity_digest,
            )
        });
        let mut sealed = BindingKeyIndex::default();
        for row in rows {
            let identity = row.receipt_key().map_or(
                row.contract().identity_digest(),
                crate::runtime::UiScrollReceiptActivationKey::identity_digest,
            );
            let Some(receipt_key) = row.receipt_key().cloned() else {
                return Err(UiScrollInvalidationBindingDenial::MissingReceiptBinding);
            };
            if sealed.contains_key(&receipt_key) {
                bump(&mut counters.duplicate_probes)?;
            }
            if let Some(prior) = sealed.get(&receipt_key) {
                if prior != &row {
                    return Err(UiScrollInvalidationBindingDenial::ConflictingContractTarget);
                }
                continue;
            }
            sealed.insert(identity, row);
        }
        frozen.insert(key, sealed);
    }
    Ok(frozen)
}
