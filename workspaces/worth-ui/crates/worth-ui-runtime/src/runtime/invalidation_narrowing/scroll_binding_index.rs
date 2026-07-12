use super::scroll_catalog_evidence::bump;
use super::scroll_catalog_evidence::UiScrollCatalogIdentity;
use super::{
    UiAdmittedScrollInvalidationBinding, UiScrollBindingCatalogCounters,
    UiScrollInvalidationBindingDenial, UiScrollOwnerCatalogReceipt,
};
use std::collections::BTreeMap;
type HostWitness = crate::evidence::UiHostMeasurementAuthorityWitness;
type QueryKey = worth_ui_query_binding::WorthUiQueryMeasurementConsumptionIdentity;
use super::scroll_binding_key_index::BindingKeyIndex;
use super::scroll_owner_acquisition::acquire_exact;
type QueryIndex = BTreeMap<QueryKey, BindingKeyIndex>;
#[derive(Clone, Debug, Default)]
pub(crate) struct UiScrollInvalidationBindingIndex {
    host: BTreeMap<HostWitness, BindingKeyIndex>,
    query: QueryIndex,
    projection_contracts: BTreeMap<
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
    pub(crate) fn seal(
        committed_bindings: &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        graph: &crate::graph::UiGraphReplanAuthority,
        predecessor_identity_digest: u64,
        successor_identity_digest: u64,
    ) -> Result<Self, super::UiScrollOwnerCatalogDenialReport> {
        let mut host = BTreeMap::<HostWitness, Vec<UiAdmittedScrollInvalidationBinding>>::new();
        let mut query = BTreeMap::<QueryKey, Vec<UiAdmittedScrollInvalidationBinding>>::new();
        let mut counters = UiScrollBindingCatalogCounters::default();
        let mut activation_keys = Vec::new();
        let outcome = (|| -> Result<Self, UiScrollInvalidationBindingDenial> {
            for committed_binding in committed_bindings.rows() {
                bump(&mut counters.extent_rows_visited)?;
                let receipt = Some(committed_binding.receipt());
                bump(&mut counters.context_reads)?;
                {
                    let receipt = committed_binding.receipt();
                    bump(&mut counters.receipt_validations)?;
                    bump(&mut counters.structural_comparisons)?;
                    if receipt.identity().graph_node_identity()
                        != committed_binding.measurement_basis().graph_node_identity()
                        || Some(receipt.generation().planning_evidence_digest())
                            != committed_binding.planning_identity_digest()
                        || committed_binding.receipt_identity() != receipt.identity()
                        || committed_binding.receipt_generation() != receipt.generation()
                        || committed_binding.measurement_basis_identity_digest()
                            != committed_binding.measurement_basis().identity_digest()
                    {
                        return Err(UiScrollInvalidationBindingDenial::ReceiptContextMismatch);
                    }
                }
                for source in committed_binding.scroll_sources() {
                    bump(&mut counters.source_visits)?;
                    let (contract, target_node, cause) = match source {
                        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Host {
                            witness: _,
                            contract,
                        } => (
                            contract.clone(),
                            committed_binding.receipt_identity().graph_node_identity(),
                            crate::evidence::UiScrollOwnedExtentCause::HostContainerViewport,
                        ),
                        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Query {
                            contract,
                        } => {
                            let crate::runtime::UiAdmittedScrollExtentSource::QueryContent(query_source) = contract.source() else {
                                return Err(UiScrollInvalidationBindingDenial::ContradictorySource);
                            };
                            (
                                contract.clone(),
                                query_source.target(),
                                crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent,
                            )
                        }
                    };
                    bump(&mut counters.target_probes)?;
                    let target = graph
                        .target_set_for_neighborhood(target_node, contract.neighborhood_identity())
                        .ok_or(UiScrollInvalidationBindingDenial::MissingGraphTarget)?;
                    bump(&mut counters.targets_emitted)?;
                    bump(&mut counters.graph_target_validations)?;
                    let binding = UiAdmittedScrollInvalidationBinding::seal(
                        contract, target, cause, 1, receipt,
                    )?;
                    activation_keys.push(
                        binding
                            .receipt_key()
                            .ok_or(UiScrollInvalidationBindingDenial::MissingReceiptBinding)?
                            .clone(),
                    );
                    bump(&mut counters.bindings_sealed)?;
                    match source {
                        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Host { witness, .. } => {
                            host.entry(*witness).or_default().push(binding);
                            bump(&mut counters.index_writes)?;
                        }
                        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Query { .. } => {
                            let crate::runtime::UiAdmittedScrollExtentSource::QueryContent(query_source) = binding.contract().source() else {
                                return Err(UiScrollInvalidationBindingDenial::ContradictorySource);
                            };
                            let query_key = query_source.query_consumption_identity().clone();
                            query.entry(query_key).or_default().push(binding);
                            bump(&mut counters.index_writes)?;
                        }
                    }
                }
            }
            let host = freeze(host, &mut counters)?;
            let frozen_query = freeze(query, &mut counters)?;
            let mut projection_contracts = BTreeMap::new();
            for binding in host
                .values()
                .flat_map(BindingKeyIndex::values)
                .chain(frozen_query.values().flat_map(BindingKeyIndex::values))
            {
                let Some(receipt_key) = binding.receipt_key() else {
                    continue;
                };
                let node = binding.target().primary().graph_node_identity();
                let owner = binding.contract().projection_owner_identity();
                bump(&mut counters.owner_validations)?;
                let row = (binding.contract().clone(), node, receipt_key.clone());
                if projection_contracts
                    .insert(owner, row.clone())
                    .is_some_and(|prior| prior != row)
                {
                    return Err(UiScrollInvalidationBindingDenial::ConflictingOwnerCapability);
                }
                bump(&mut counters.projection_writes)?;
            }
            let query = frozen_query;
            let identity = UiScrollCatalogIdentity::new(
                committed_bindings.identity_digest(),
                predecessor_identity_digest,
                successor_identity_digest,
                activation_keys.clone().into_boxed_slice(),
            );
            let catalog_receipt =
                UiScrollOwnerCatalogReceipt::seal(counters, projection_contracts.len(), identity)?;
            Ok(Self {
                host,
                query,
                projection_contracts,
                catalog_receipt: Some(catalog_receipt),
            })
        })();
        outcome.map_err(|reason| {
            let _ = bump(&mut counters.diagnostic_probes);
            super::UiScrollOwnerCatalogDenialReport::new(
                reason,
                counters,
                UiScrollCatalogIdentity::new(
                    committed_bindings.identity_digest(),
                    predecessor_identity_digest,
                    successor_identity_digest,
                    activation_keys.into_boxed_slice(),
                ),
            )
        })
    }

    pub(super) fn host_extent(&self, witness: HostWitness) -> Option<&BindingKeyIndex> {
        self.host.get(&witness)
    }
    pub(super) fn query_extent(
        &self,
        identity: &QueryKey,
    ) -> Result<
        (Option<&BindingKeyIndex>, u16),
        (super::authority::UiInvalidationAuthorityLookupDenial, u16),
    > {
        Ok((self.query.get(identity), 1))
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
    pub(crate) fn projection_for_query(
        &self,
        identity: &QueryKey,
        allocation_receipt: &crate::runtime::UiAllocationReceipt,
    ) -> Result<
        crate::runtime::UiActivatedScrollOwner,
        crate::runtime::UiScrollOwnerAcquisitionDenial,
    > {
        let mut probes = 0;
        record_acquisition_lookup(&mut probes)?;
        let rows = self.query.get(identity);
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

fn freeze<K: Ord>(
    source: BTreeMap<K, Vec<UiAdmittedScrollInvalidationBinding>>,
    counters: &mut UiScrollBindingCatalogCounters,
) -> Result<BTreeMap<K, BindingKeyIndex>, UiScrollInvalidationBindingDenial> {
    bump(&mut counters.freeze_operations)?;
    source
        .into_iter()
        .map(|(key, mut rows)| {
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
            Ok((key, sealed))
        })
        .collect()
}
