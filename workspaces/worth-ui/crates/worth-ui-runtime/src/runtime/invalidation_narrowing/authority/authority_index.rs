use std::collections::BTreeMap;

impl super::UiAllocationInvalidationAuthority {
    pub(super) fn rebuild_indexes(&mut self) {
        let mut query = BTreeMap::<Box<str>, Vec<usize>>::new();
        let mut host_targets = BTreeMap::<
            crate::evidence::UiHostMeasurementAuthorityWitness,
            Vec<crate::graph::UiGraphNodeIdentity>,
        >::new();
        let mut host_witnesses = BTreeMap::<
            (
                worth_ui_host_contract::UiMeasurementRequestIdentity,
                crate::evidence::UiMeasurementEvidenceCategory,
            ),
            Vec<crate::evidence::UiHostMeasurementAuthorityWitness>,
        >::new();
        let mut durable = BTreeMap::<u64, Vec<usize>>::new();
        for (ordinal, context) in self.active_contexts.iter().enumerate() {
            for (source, _) in context.basis.query_allocation_mappings() {
                query.entry(source.into()).or_default().push(ordinal);
            }
            for request in context.basis.host_allocation_requests() {
                if let Some(result) = context.basis.host_measurement_result(request) {
                    let witness = result.authority_witness();
                    if let Some(target) = context.basis.host_allocation_target(request) {
                        host_targets.entry(witness).or_default().push(target);
                    }
                    host_witnesses
                        .entry((request, result.evidence_category()))
                        .or_default()
                        .push(witness);
                }
            }
            for input in context.basis.durable_resize_inputs() {
                durable.entry(input).or_default().push(ordinal);
            }
        }
        self.query_contexts = freeze(query);
        self.host_targets_by_witness = freeze_sorted_unique(host_targets)
            .into_iter()
            .filter_map(|(witness, nodes)| {
                super::authority::UiHostInvalidationTargetMapping::seal(nodes, &self.graph_replan)
                    .map(|mapping| (witness, mapping))
            })
            .collect();
        self.host_witnesses_by_request = freeze_sorted_unique(host_witnesses);
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

fn freeze<K: Ord>(source: BTreeMap<K, Vec<usize>>) -> BTreeMap<K, Box<[usize]>> {
    source
        .into_iter()
        .map(|(key, values)| (key, values.into_boxed_slice()))
        .collect()
}
