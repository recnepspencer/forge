use std::collections::BTreeMap;

impl super::UiAllocationInvalidationAuthority {
    pub(super) fn rebuild_indexes(&mut self) {
        let mut query = BTreeMap::<
            worth_ui_query_binding::WorthUiQueryAuthorityIndexKey,
            Vec<crate::evidence::UiAllocationNeighborhoodScope>,
        >::new();
        let mut host_targets = BTreeMap::<
            crate::evidence::UiHostMeasurementAuthorityWitness,
            Vec<crate::graph::UiGraphNodeIdentity>,
        >::new();
        let mut host_scopes = BTreeMap::<
            crate::evidence::UiHostMeasurementAuthorityWitness,
            Vec<crate::evidence::UiAllocationNeighborhoodScope>,
        >::new();
        let mut host_witnesses = BTreeMap::<
            (
                worth_ui_host_contract::UiMeasurementRequestIdentity,
                crate::evidence::UiMeasurementEvidenceCategory,
            ),
            Vec<crate::evidence::UiHostMeasurementAuthorityWitness>,
        >::new();
        let mut durable =
            BTreeMap::<u64, Vec<crate::evidence::UiAllocationNeighborhoodScope>>::new();
        {
            let mut record =
                |scope: &crate::evidence::UiAllocationNeighborhoodScope,
                 context: &super::UiCommittedAllocationInvalidationContext| {
                    for (source, _) in context.basis.query_allocation_mappings() {
                        query.entry(source.clone()).or_default().push(scope.clone());
                    }
                    for request in context.basis.host_allocation_requests() {
                        if let Some(result) = context.basis.host_measurement_result(request) {
                            let witness = result.authority_witness();
                            if let Some(target) = context.basis.host_allocation_target(request) {
                                host_targets.entry(witness).or_default().push(target);
                            }
                            host_scopes.entry(witness).or_default().push(scope.clone());
                            host_witnesses
                                .entry((request, result.evidence_category()))
                                .or_default()
                                .push(witness);
                        }
                    }
                    for input in context.basis.durable_resize_inputs() {
                        durable.entry(input).or_default().push(scope.clone());
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
        for (witness, nodes) in freeze_sorted_unique(host_targets) {
            if let Some(mapping) =
                super::authority::UiHostInvalidationTargetMapping::seal(nodes, &self.graph_replan)
            {
                self.host_targets_by_witness.insert(witness, mapping);
            }
        }
        self.host_scopes_by_witness = freeze(host_scopes);
        self.host_witnesses_by_request = Default::default();
        for (request, witnesses) in freeze_sorted_unique(host_witnesses) {
            self.host_witnesses_by_request.insert(request, witnesses);
        }
        self.durable_contexts = freeze(durable);
    }
}

fn freeze_sorted_unique<K: Ord, V: Ord>(source: BTreeMap<K, Vec<V>>) -> BTreeMap<K, Box<[V]>> {
    source
        .into_iter()
        .map(|(key, mut values)| {
            values.sort_unstable();
            values.dedup();
            (key, values.into_boxed_slice())
        })
        .collect()
}

fn freeze<K: Ord + Clone, V>(
    source: BTreeMap<K, Vec<V>>,
) -> crate::runtime::persistent_index::UiPersistentOrdMap<K, Box<[V]>> {
    let mut frozen = crate::runtime::persistent_index::UiPersistentOrdMap::default();
    for (key, values) in source {
        frozen.insert(key, values.into_boxed_slice());
    }
    frozen
}
