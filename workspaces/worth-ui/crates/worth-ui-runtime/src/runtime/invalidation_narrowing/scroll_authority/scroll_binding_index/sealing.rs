use std::collections::BTreeMap;

use super::super::scroll_catalog_evidence::{bump, UiScrollCatalogIdentity};
use super::super::{
    UiAdmittedScrollInvalidationBinding, UiScrollBindingCatalogCounters,
    UiScrollInvalidationBindingDenial, UiScrollOwnerCatalogReceipt,
};
use super::{
    freeze, BindingKeyIndex, HostWitness, QueryBindingKey, QueryIndex, QueryKey,
    UiScrollInvalidationBindingIndex,
};

impl UiScrollInvalidationBindingIndex {
    pub(crate) fn seal(
        committed_bindings: &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        graph: &crate::graph::UiGraphReplanAuthority,
        predecessor_identity_digest: u64,
        successor_identity_digest: u64,
    ) -> Result<Self, super::super::UiScrollOwnerCatalogDenialReport> {
        let mut assembly = UiScrollBindingCatalogAssembly::default();
        let outcome = assembly
            .collect_committed_bindings(committed_bindings, graph)
            .and_then(|()| {
                assembly.finish(
                    committed_bindings.identity_digest(),
                    predecessor_identity_digest,
                    successor_identity_digest,
                )
            });
        outcome.map_err(|reason| {
            assembly.denial_report(
                reason,
                committed_bindings.identity_digest(),
                predecessor_identity_digest,
                successor_identity_digest,
            )
        })
    }
}

#[derive(Default)]
struct UiScrollBindingCatalogAssembly {
    host: BTreeMap<HostWitness, Vec<UiAdmittedScrollInvalidationBinding>>,
    query: BTreeMap<QueryKey, Vec<UiAdmittedScrollInvalidationBinding>>,
    query_by_binding: BTreeMap<QueryBindingKey, Vec<UiAdmittedScrollInvalidationBinding>>,
    query_binding_revisions:
        BTreeMap<QueryBindingKey, worth_ui_query_binding::WorthUiAdmittedQuerySettlementReference>,
    counters: UiScrollBindingCatalogCounters,
    activation_keys: Vec<crate::runtime::UiScrollReceiptActivationKey>,
}

impl UiScrollBindingCatalogAssembly {
    fn collect_committed_bindings(
        &mut self,
        committed_bindings: &crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Result<(), UiScrollInvalidationBindingDenial> {
        for committed_binding in committed_bindings.rows() {
            bump(&mut self.counters.extent_rows_visited)?;
            validate_committed_binding(committed_binding, &mut self.counters)?;
            for source in committed_binding.scroll_sources() {
                self.collect_source(committed_binding, source, graph)?;
            }
        }
        Ok(())
    }

    fn collect_source(
        &mut self,
        committed_binding: &crate::runtime::UiCommittedAllocationCatalogActivationRow,
        source: &crate::runtime::allocation_receipt::UiCommittedScrollActivationSource,
        graph: &crate::graph::UiGraphReplanAuthority,
    ) -> Result<(), UiScrollInvalidationBindingDenial> {
        bump(&mut self.counters.source_visits)?;
        let (contract, target_node, cause) = scroll_source_target(committed_binding, source)?;
        bump(&mut self.counters.target_probes)?;
        let target = graph
            .target_set_for_neighborhood(target_node, contract.neighborhood_identity())
            .ok_or(UiScrollInvalidationBindingDenial::MissingGraphTarget)?;
        bump(&mut self.counters.targets_emitted)?;
        bump(&mut self.counters.graph_target_validations)?;
        let binding = UiAdmittedScrollInvalidationBinding::seal(
            contract,
            target,
            cause,
            1,
            Some(committed_binding.receipt()),
        )?;
        self.activation_keys.push(
            binding
                .receipt_key()
                .ok_or(UiScrollInvalidationBindingDenial::MissingReceiptBinding)?
                .clone(),
        );
        bump(&mut self.counters.bindings_sealed)?;
        self.index_binding(source, binding)
    }

    fn index_binding(
        &mut self,
        source: &crate::runtime::allocation_receipt::UiCommittedScrollActivationSource,
        binding: UiAdmittedScrollInvalidationBinding,
    ) -> Result<(), UiScrollInvalidationBindingDenial> {
        match source {
            crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Host {
                witness,
                ..
            } => self.host.entry(*witness).or_default().push(binding),
            crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Query {
                ..
            } => {
                let crate::runtime::UiAdmittedScrollExtentSource::QueryContent(query_source) =
                    binding.contract().source()
                else {
                    return Err(UiScrollInvalidationBindingDenial::ContradictorySource);
                };
                let source_key = query_source.source_key();
                let binding_key = source_key.binding_key();
                if self
                    .query_binding_revisions
                    .get(&binding_key)
                    .is_some_and(|revision| revision != source_key.settlement_reference())
                {
                    return Err(UiScrollInvalidationBindingDenial::ContradictorySource);
                }
                self.query_binding_revisions
                    .entry(binding_key.clone())
                    .or_insert_with(|| source_key.settlement_reference().clone());
                self.query
                    .entry(source_key.clone())
                    .or_default()
                    .push(binding.clone());
                bump(&mut self.counters.index_writes)?;
                self.query_by_binding
                    .entry(binding_key)
                    .or_default()
                    .push(binding);
            }
        }
        bump(&mut self.counters.index_writes)
    }

    fn finish(
        &mut self,
        source_identity_digest: u64,
        predecessor_identity_digest: u64,
        successor_identity_digest: u64,
    ) -> Result<UiScrollInvalidationBindingIndex, UiScrollInvalidationBindingDenial> {
        let host = freeze(std::mem::take(&mut self.host), &mut self.counters)?;
        let query = freeze(std::mem::take(&mut self.query), &mut self.counters)?;
        let query_by_binding = freeze(
            std::mem::take(&mut self.query_by_binding),
            &mut self.counters,
        )?;
        let projection_contracts = collect_projection_contracts(&host, &query, &mut self.counters)?;
        let identity = UiScrollCatalogIdentity::new(
            source_identity_digest,
            predecessor_identity_digest,
            successor_identity_digest,
            self.activation_keys.clone().into_boxed_slice(),
        );
        let catalog_receipt =
            UiScrollOwnerCatalogReceipt::seal(self.counters, projection_contracts.len(), identity)?;
        Ok(UiScrollInvalidationBindingIndex {
            host,
            query,
            query_by_binding,
            projection_contracts,
            catalog_receipt: Some(catalog_receipt),
        })
    }

    fn denial_report(
        &mut self,
        reason: UiScrollInvalidationBindingDenial,
        source_identity_digest: u64,
        predecessor_identity_digest: u64,
        successor_identity_digest: u64,
    ) -> super::super::UiScrollOwnerCatalogDenialReport {
        let _ = bump(&mut self.counters.diagnostic_probes);
        super::super::UiScrollOwnerCatalogDenialReport::new(
            reason,
            self.counters,
            UiScrollCatalogIdentity::new(
                source_identity_digest,
                predecessor_identity_digest,
                successor_identity_digest,
                std::mem::take(&mut self.activation_keys).into_boxed_slice(),
            ),
        )
    }
}

fn validate_committed_binding(
    committed_binding: &crate::runtime::UiCommittedAllocationCatalogActivationRow,
    counters: &mut UiScrollBindingCatalogCounters,
) -> Result<(), UiScrollInvalidationBindingDenial> {
    bump(&mut counters.context_reads)?;
    bump(&mut counters.receipt_validations)?;
    bump(&mut counters.structural_comparisons)?;
    let receipt = committed_binding.receipt();
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
    Ok(())
}

fn scroll_source_target(
    committed_binding: &crate::runtime::UiCommittedAllocationCatalogActivationRow,
    source: &crate::runtime::allocation_receipt::UiCommittedScrollActivationSource,
) -> Result<
    (
        crate::runtime::UiAdmittedScrollOwnedContract,
        crate::graph::UiGraphNodeIdentity,
        crate::evidence::UiScrollOwnedExtentCause,
    ),
    UiScrollInvalidationBindingDenial,
> {
    match source {
        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Host {
            contract,
            ..
        } => Ok((
            contract.clone(),
            committed_binding.receipt_identity().graph_node_identity(),
            crate::evidence::UiScrollOwnedExtentCause::HostContainerViewport,
        )),
        crate::runtime::allocation_receipt::UiCommittedScrollActivationSource::Query {
            contract,
        } => {
            let crate::runtime::UiAdmittedScrollExtentSource::QueryContent(query_source) =
                contract.source()
            else {
                return Err(UiScrollInvalidationBindingDenial::ContradictorySource);
            };
            Ok((
                contract.clone(),
                query_source.target(),
                crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent,
            ))
        }
    }
}

fn collect_projection_contracts(
    host: &crate::runtime::persistent_index::UiPersistentOrdMap<HostWitness, BindingKeyIndex>,
    query: &QueryIndex,
    counters: &mut UiScrollBindingCatalogCounters,
) -> Result<
    crate::runtime::persistent_index::UiPersistentOrdMap<
        crate::runtime::UiScrollProjectionOwnerIdentity,
        (
            crate::runtime::UiAdmittedScrollOwnedContract,
            crate::graph::UiGraphNodeIdentity,
            crate::runtime::UiScrollReceiptActivationKey,
        ),
    >,
    UiScrollInvalidationBindingDenial,
> {
    let mut projection_contracts = crate::runtime::persistent_index::UiPersistentOrdMap::default();
    for binding in host
        .iter()
        .flat_map(|(_, index)| index.values())
        .chain(query.iter().flat_map(|(_, index)| index.values()))
    {
        let Some(receipt_key) = binding.receipt_key() else {
            continue;
        };
        let owner = binding.contract().projection_owner_identity();
        let row = (
            binding.contract().clone(),
            binding.target().primary().graph_node_identity(),
            receipt_key.clone(),
        );
        bump(&mut counters.owner_validations)?;
        if projection_contracts
            .get(&owner)
            .is_some_and(|prior| prior != &row)
        {
            return Err(UiScrollInvalidationBindingDenial::ConflictingOwnerCapability);
        }
        projection_contracts.insert(owner, row);
        bump(&mut counters.projection_writes)?;
    }
    Ok(projection_contracts)
}
