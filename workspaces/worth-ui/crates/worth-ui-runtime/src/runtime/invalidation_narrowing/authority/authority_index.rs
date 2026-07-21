use std::collections::{BTreeMap, BTreeSet};

impl super::UiAllocationInvalidationAuthority {
    pub(super) fn rebuild_indexes(&mut self) {
        let mut query = BTreeMap::<
            crate::evidence::measurement::basis::UiQueryAllocationSourceKey,
            BTreeSet<crate::evidence::UiAllocationNeighborhoodScope>,
        >::new();
        let mut host_targets = BTreeMap::<
            crate::evidence::UiHostMeasurementAuthorityWitness,
            BTreeMap<crate::graph::UiGraphNodeIdentity, usize>,
        >::new();
        let mut host_scopes = BTreeMap::<
            crate::evidence::UiHostMeasurementAuthorityWitness,
            BTreeSet<crate::evidence::UiAllocationNeighborhoodScope>,
        >::new();
        let mut host_witnesses = BTreeMap::<
            (
                worth_ui_host_contract::UiMeasurementRequestIdentity,
                crate::evidence::UiMeasurementEvidenceCategory,
            ),
            BTreeMap<crate::evidence::UiHostMeasurementAuthorityWitness, usize>,
        >::new();
        let mut durable =
            BTreeMap::<u64, BTreeSet<crate::evidence::UiAllocationNeighborhoodScope>>::new();
        {
            let mut record =
                |scope: &crate::evidence::UiAllocationNeighborhoodScope,
                 context: &super::UiCommittedAllocationInvalidationContext| {
                    for (source, _) in context.basis.query_allocation_mappings() {
                        query
                            .entry(source.clone())
                            .or_default()
                            .insert(scope.clone());
                    }
                    for request in context.basis.host_allocation_requests() {
                        if let Some(result) = context.basis.host_measurement_result(request) {
                            let witness = result.authority_witness();
                            if let Some(target) = context.basis.host_allocation_target(request) {
                                increment_owner(host_targets.entry(witness).or_default(), target);
                            }
                            host_scopes
                                .entry(witness)
                                .or_default()
                                .insert(scope.clone());
                            increment_owner(
                                host_witnesses
                                    .entry((request, result.evidence_category()))
                                    .or_default(),
                                witness,
                            );
                        }
                    }
                    for input in context.basis.durable_resize_inputs() {
                        durable.entry(input).or_default().insert(scope.clone());
                    }
                };
            for (scope, row) in self.catalog.iter() {
                record(scope, row.committed_invalidation_context());
            }
            #[cfg(test)]
            for (scope, context) in self.fixture_contexts.iter() {
                record(scope, context);
            }
        }
        self.query_contexts = freeze(query);
        self.host_targets_by_witness = Default::default();
        for (witness, owners) in host_targets {
            self.host_targets_by_witness.insert(
                witness,
                super::authority::UiHostInvalidationTargetMapping::from_owners(freeze_map(owners)),
            );
        }
        self.host_scopes_by_witness = freeze(host_scopes);
        self.host_witnesses_by_request = Default::default();
        for (request, witnesses) in host_witnesses {
            self.host_witnesses_by_request
                .insert(request, freeze_map(witnesses));
        }
        self.durable_contexts = freeze(durable);
    }
}

fn increment_owner<K: Ord>(owners: &mut BTreeMap<K, usize>, key: K) {
    let count = owners.entry(key).or_default();
    *count = count
        .checked_add(1)
        .expect("catalog cardinality fits usize");
}

fn freeze<K: Ord + Clone, V: Ord + Clone>(
    source: BTreeMap<K, BTreeSet<V>>,
) -> crate::runtime::persistent_index::UiPersistentOrdMap<
    K,
    crate::runtime::persistent_index::UiPersistentOrdSet<V>,
> {
    let mut frozen = crate::runtime::persistent_index::UiPersistentOrdMap::default();
    for (key, values) in source {
        let mut set = crate::runtime::persistent_index::UiPersistentOrdSet::default();
        for value in values {
            set.insert(value);
        }
        frozen.insert(key, set);
    }
    frozen
}

fn freeze_map<K: Ord + Clone, V>(
    source: BTreeMap<K, V>,
) -> crate::runtime::persistent_index::UiPersistentOrdMap<K, V> {
    let mut frozen = crate::runtime::persistent_index::UiPersistentOrdMap::default();
    for (key, value) in source {
        frozen.insert(key, value);
    }
    frozen
}
