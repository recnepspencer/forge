use std::collections::BTreeSet;

pub(crate) struct WorthQuerySharedInvalidationSeed {
    touches: Vec<crate::domain_installation::WorthQueryNativeTouchCoordinate>,
    delivery_causes: Vec<crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind>,
    counters: crate::domain_installation::WorthQueryConsumerInvalidationEpochCounters,
}

impl WorthQuerySharedInvalidationSeed {
    pub(crate) fn compile(
        delivery: &crate::ordinary::live::WorthQueryManagedLiveDelivery,
        fanout_targets: usize,
    ) -> Self {
        let mut touches = BTreeSet::new();
        let mut delivery_causes = Vec::new();
        let mut counters =
            crate::domain_installation::WorthQueryConsumerInvalidationEpochCounters {
                fanout_targets,
                ..Default::default()
            };
        for batch in delivery.batches() {
            counters.capability_index_lookups += batch.routing_work().capability_index_lookups;
            counters.live_collection_index_probes +=
                batch.routing_work().live_collection_index_probes;
            counters.live_relevance_index_probes +=
                batch.routing_work().live_relevance_index_probes;
            counters.installed_collection_index_probes +=
                batch.routing_work().installed_collection_index_probes;
            counters.installed_relevance_index_probes +=
                batch.routing_work().installed_relevance_index_probes;
            counters.live_target_candidates_visited +=
                batch.routing_work().live_target_candidates_visited;
            counters.installed_target_candidates_selected +=
                batch.routing_work().installed_target_candidates_selected;
            counters.installed_candidates_skipped +=
                batch.routing_work().installed_candidates_skipped;
            counters.target_overlap_deduplications +=
                batch.routing_work().target_overlap_deduplications;
            counters.installed_route_index_probes +=
                batch.routing_work().installed_route_index_probes;
            counters.delivery_batches_visited += 1;
            delivery_causes.push(batch.cause_kind());
            let Some(mutation) = batch.mutation_delta() else {
                continue;
            };
            counters.mutation_deltas_visited += 1;
            for touch in mutation.admitted_touched_aspects() {
                counters.touched_aspects_visited += 1;
                touches.insert(
                    crate::domain_installation::WorthQueryNativeTouchCoordinate {
                        aspect_key: touch.native_aspect_key().clone(),
                        field_path: touch.native_field_path().cloned(),
                    },
                );
            }
        }
        delivery_causes.sort_by_key(|cause| cause.as_str());
        delivery_causes.dedup();
        Self {
            touches: touches.into_iter().collect(),
            delivery_causes,
            counters,
        }
    }

    pub(crate) fn touches(&self) -> &[crate::domain_installation::WorthQueryNativeTouchCoordinate] {
        &self.touches
    }

    pub(crate) fn delivery_causes(
        &self,
    ) -> &[crate::ordinary::live::WorthQueryManagedLiveDeliveryCauseKind] {
        &self.delivery_causes
    }

    pub(crate) const fn counters(
        &self,
    ) -> crate::domain_installation::WorthQueryConsumerInvalidationEpochCounters {
        self.counters
    }
}
