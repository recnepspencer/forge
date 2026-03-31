use crate::logic::runtime::RelationalRuntime;
use crate::performance::data::{
    ComplexityContract, RuntimeComplexityCounters, COMPLEXITY_CONTRACTS,
};
use crate::replay::data::ReplayAuthorityBasisKind;
use crate::replay::data::ReplayVerificationLayer;
use crate::schema::data::{
    HistoricalInterpretationSensitivity, SchemaContinuationClassification,
    SchemaReconciliationPolicy,
};
use crate::transactions::data::BulkMutationLocalityFootprint;

pub struct PerformanceAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReplayLineageAuthorityIndexedSource {
    DurableLog,
    Checkpoint,
}

impl RelationalRuntime {
    pub fn performance_access(&self) -> PerformanceAccess<'_> {
        PerformanceAccess::new(self)
    }
}

impl<'runtime> PerformanceAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn contracts(&self) -> &'static [ComplexityContract] {
        COMPLEXITY_CONTRACTS
    }

    pub fn counters(&self) -> RuntimeComplexityCounters {
        self.runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone()
    }

    pub fn reset_counters(&self) {
        *self
            .runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned") = RuntimeComplexityCounters::default();
    }

    pub(crate) fn count_invariant_entity_slot_scans(&self, slots: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.invariant_entity_slot_scans += slots);
    }

    pub(crate) fn count_invariant_relation_slot_scans(&self, slots: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.invariant_relation_slot_scans += slots);
    }

    pub(crate) fn count_custom_invariant_preparation(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.custom_invariant_preparation_count += 1);
    }

    pub(crate) fn count_custom_invariant_execution(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.custom_invariant_execution_count += 1);
    }

    pub(crate) fn count_custom_invariant_panic(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.custom_invariant_panic_count += 1);
    }

    pub(crate) fn count_custom_invariant_traversal(&self, frontier: usize, steps: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.custom_invariant_traversal_frontier_count += frontier;
            counters.custom_invariant_traversal_step_count += steps;
        });
    }

    pub(crate) fn count_relation_integrity_contracts_evaluated(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_integrity_contracts_evaluated += count);
    }

    pub(crate) fn count_relation_endpoint_kind_checks(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_endpoint_kind_checks += count);
    }

    pub(crate) fn count_relation_cardinality_checks(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_cardinality_checks += count);
    }

    pub(crate) fn count_relation_cardinality_minimum_certification(
        &self,
        contracts: usize,
        entity_slot_scans: usize,
        relation_slot_scans: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.relation_cardinality_minimum_certification_contracts_evaluated += contracts;
            counters.relation_cardinality_minimum_certification_entity_slot_scans +=
                entity_slot_scans;
            counters.relation_cardinality_minimum_certification_relation_slot_scans +=
                relation_slot_scans;
        });
    }

    pub(crate) fn count_relation_uniqueness_checks(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_uniqueness_checks += count);
    }

    pub(crate) fn count_relation_uniqueness_candidates(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_uniqueness_candidates_scanned += count);
    }

    pub(crate) fn count_relation_symmetry_checks(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_symmetry_checks += count);
    }

    pub(crate) fn count_relation_endpoint_deletion_checks(&self, count: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.relation_endpoint_deletion_checks += count);
    }

    pub(crate) fn count_preparation_packet_shape(
        &self,
        packets: usize,
        items: usize,
        max_width: usize,
        scope_units: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.preparation_packet_count += packets;
            counters.preparation_packet_item_count += items;
            counters.preparation_packet_peak_width_total += max_width;
            counters.preparation_scope_unit_count += scope_units;
        });
    }

    pub(crate) fn count_preparation_parallel_legal(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_parallel_legal_count += 1);
    }

    pub(crate) fn count_preparation_parallel_profitable(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_parallel_profitable_count += 1);
    }

    pub(crate) fn count_preparation_serial_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_serial_strategy_count += 1);
    }

    pub(crate) fn count_preparation_staged_parallel_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_staged_parallel_strategy_count += 1);
    }

    pub(crate) fn count_preparation_reducer_conflicts(&self, conflicts: usize) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.preparation_reducer_conflict_count += conflicts);
    }

    pub(crate) fn count_query_packet_shape(
        &self,
        packets: usize,
        items: usize,
        max_width: usize,
        scope_units: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.query_packet_count += packets;
            counters.query_packet_item_count += items;
            counters.query_packet_peak_width_total += max_width;
            counters.query_scope_unit_count += scope_units;
        });
    }

    pub(crate) fn count_query_parallel_legal(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_parallel_legal_count += 1);
    }

    pub(crate) fn count_query_parallel_profitable(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_parallel_profitable_count += 1);
    }

    pub(crate) fn count_query_serial_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_serial_strategy_count += 1);
    }

    pub(crate) fn count_query_staged_parallel_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_staged_parallel_strategy_count += 1);
    }

    pub(crate) fn count_query_fragment_scratch_reuse_by(&self, count: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.query_fragment_scratch_reuse_count += count;
        });
    }

    pub(crate) fn count_query_index_attempt(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_index_attempt_count += 1);
    }

    pub(crate) fn count_query_index_path(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_index_path_count += 1);
    }

    pub(crate) fn count_query_index_rejection(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_index_rejection_count += 1);
    }

    pub(crate) fn count_query_index_parity_verification(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_index_parity_verification_count += 1);
    }

    pub(crate) fn count_query_index_scratch_reuse(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.query_index_scratch_reuse_count += 1);
    }

    pub(crate) fn count_query_emissions(&self, entities: usize, relations: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.query_entity_records_emitted += entities;
            counters.query_relation_records_emitted += relations;
        });
    }

    pub(crate) fn count_bulk_mutation_plan(
        &self,
        locality: &BulkMutationLocalityFootprint,
        normalized_client_key_count: usize,
        lineage_transition_count: usize,
        provenance_record_count: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.bulk_mutation_batch_count += 1;
            counters.bulk_mutation_entity_target_count += locality.entity_target_count;
            counters.bulk_mutation_relation_target_count += locality.relation_target_count;
            counters.bulk_mutation_cross_partition_relation_count +=
                locality.cross_partition_relation_count;
            counters.bulk_mutation_naming_normalization_count += normalized_client_key_count;
            counters.bulk_mutation_lineage_transition_count += lineage_transition_count;
            counters.bulk_mutation_provenance_record_count += provenance_record_count;
        });
    }

    pub(crate) fn count_post_commit_consumer_shape(
        &self,
        packets: usize,
        items: usize,
        max_width: usize,
        scope_units: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.post_commit_consumer_packet_count += packets;
            counters.post_commit_consumer_item_count += items;
            counters.post_commit_consumer_peak_width_total += max_width;
            counters.post_commit_scope_unit_count += scope_units;
        });
    }

    pub(crate) fn count_post_commit_serial_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.post_commit_serial_strategy_count += 1);
    }

    pub(crate) fn count_post_commit_parallel_strategy(&self) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.post_commit_parallel_strategy_count += 1);
    }

    pub(crate) fn count_lineage_graph_snapshot_request(
        &self,
        node_count: usize,
        event_count: usize,
        candidate_count: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_graph_snapshot_requests += 1;
            counters.lineage_graph_snapshot_nodes_materialized += node_count;
            counters.lineage_graph_snapshot_events_materialized += event_count;
            counters.lineage_graph_snapshot_candidates_materialized += candidate_count;
        });
    }

    pub(crate) fn count_lineage_candidate_validation(
        &self,
        recorded_width: usize,
        validated_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_recorded_candidate_width += recorded_width;
            counters.lineage_validated_candidate_width += validated_width;
        });
    }

    pub(crate) fn count_lineage_promotion_rejection(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_rejection_count += 1;
        });
    }

    pub(crate) fn count_lineage_promotion_plan_lowering(&self, promotion_eligible_width: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_eligible_candidate_width += promotion_eligible_width;
        });
    }

    pub(crate) fn count_lineage_finalization(
        &self,
        event_batch_width: usize,
        decision_log_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_finalization_event_batch_width += event_batch_width;
            counters.lineage_finalization_decision_log_width += decision_log_width;
        });
    }

    pub(crate) fn count_lineage_publication_artifact(
        &self,
        event_width: usize,
        decision_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_publication_event_width += event_width;
            counters.lineage_publication_decision_width += decision_width;
        });
    }

    pub(crate) fn count_lineage_promotion_accepted(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_accepted_count += 1;
        });
    }

    pub(crate) fn count_lineage_graph_snapshot_visibility_cache(&self, hit: bool) {
        self.runtime.services.instrumentation.count(|counters| {
            if hit {
                counters.lineage_graph_snapshot_visibility_cache_hits += 1;
            } else {
                counters.lineage_graph_snapshot_visibility_cache_miss_reconstructions += 1;
            }
        });
    }

    pub(crate) fn count_lineage_historical_resolution(
        &self,
        branch_event_scans: usize,
        traversed_events: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_historical_resolution_requests += 1;
            counters.lineage_historical_resolution_branch_event_scans += branch_event_scans;
            counters.lineage_historical_resolution_traversed_events += traversed_events;
        });
    }

    pub(crate) fn count_lineage_branch_divergence(
        &self,
        left_event_count: usize,
        right_event_count: usize,
        left_node_count: usize,
        right_node_count: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_branch_divergence_requests += 1;
            counters.lineage_branch_divergence_event_scans += left_event_count + right_event_count;
            counters.lineage_branch_divergence_node_scans += left_node_count + right_node_count;
        });
    }

    pub(crate) fn count_schema_transition_classification(
        &self,
        atoms_inspected: usize,
        changed_subtrees_inspected: usize,
        unchanged_subtrees_reused_by_fingerprint: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.schema_transition_atoms_inspected += atoms_inspected;
            counters.schema_changed_subtrees_inspected += changed_subtrees_inspected;
            counters.schema_unchanged_subtrees_reused_by_fingerprint +=
                unchanged_subtrees_reused_by_fingerprint;
        });
    }

    pub(crate) fn count_schema_bridge_descriptor(
        &self,
        continuation: SchemaContinuationClassification,
        historical_interpretation: HistoricalInterpretationSensitivity,
        policy: SchemaReconciliationPolicy,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.schema_bridge_descriptors_built += 1;
            match continuation {
                SchemaContinuationClassification::ContinueUnchanged => {
                    counters.schema_transition_continue_unchanged_count += 1;
                }
                SchemaContinuationClassification::ContinueWithTransparentBridge => {
                    counters.schema_transition_continue_transparent_bridge_count += 1;
                }
                SchemaContinuationClassification::ContinueWithVisibleBridge => {
                    counters.schema_transition_continue_visible_bridge_count += 1;
                }
                SchemaContinuationClassification::ContinueWithContractUpgrade => {
                    counters.schema_transition_continue_contract_upgrade_count += 1;
                }
                SchemaContinuationClassification::RequireRenegotiation => {
                    counters.schema_transition_require_renegotiation_count += 1;
                }
                SchemaContinuationClassification::Rejected => {
                    counters.schema_transition_rejected_count += 1;
                }
            }
            if historical_interpretation != HistoricalInterpretationSensitivity::NotSensitive {
                counters.schema_historical_interpretation_sensitive_boundaries += 1;
            }
            match policy {
                SchemaReconciliationPolicy::RejectLossyNarrowing => {
                    counters.schema_reconciliation_reject_lossy_narrowing_count += 1;
                }
                SchemaReconciliationPolicy::PreserveInformation => {
                    counters.schema_reconciliation_preserve_information_count += 1;
                }
                SchemaReconciliationPolicy::PreserveTargetContract => {
                    counters.schema_reconciliation_preserve_target_contract_count += 1;
                }
                SchemaReconciliationPolicy::PreserveSourceContract => {
                    counters.schema_reconciliation_preserve_source_contract_count += 1;
                }
                SchemaReconciliationPolicy::PermitLossyNarrowingWithAnnotation => {
                    counters.schema_reconciliation_permit_lossy_narrowing_with_annotation_count +=
                        1;
                }
                SchemaReconciliationPolicy::RequireExplicitProjection => {
                    counters.schema_reconciliation_require_explicit_projection_count += 1;
                }
            }
        });
    }

    pub(crate) fn count_schema_normalized_descriptor_composition(&self, count: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.schema_normalized_descriptor_compositions += count;
        });
    }

    pub(crate) fn count_subscriber_resume_evaluation(
        &self,
        outcome: SchemaContinuationClassification,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.subscriber_resume_evaluations += 1;
            match outcome {
                SchemaContinuationClassification::ContinueUnchanged => {
                    counters.subscriber_continue_unchanged_count += 1;
                }
                SchemaContinuationClassification::ContinueWithTransparentBridge => {
                    counters.subscriber_continue_transparent_bridge_count += 1;
                }
                SchemaContinuationClassification::ContinueWithVisibleBridge => {
                    counters.subscriber_continue_visible_bridge_count += 1;
                }
                SchemaContinuationClassification::ContinueWithContractUpgrade => {
                    counters.subscriber_continue_contract_upgrade_count += 1;
                }
                SchemaContinuationClassification::RequireRenegotiation => {
                    counters.subscriber_require_renegotiation_count += 1;
                }
                SchemaContinuationClassification::Rejected => {
                    counters.subscriber_rejected_count += 1;
                }
            }
        });
    }

    pub(crate) fn count_replay_verification_layer(&self, layer: ReplayVerificationLayer) {
        self.runtime
            .services
            .instrumentation
            .count(|counters| match layer {
                ReplayVerificationLayer::DigestParity => counters.replay_digest_parity_checks += 1,
                ReplayVerificationLayer::SummaryParity => {
                    counters.replay_summary_parity_checks += 1
                }
                ReplayVerificationLayer::DeepArtifactParity => {
                    counters.replay_deep_artifact_parity_checks += 1
                }
            });
    }

    pub(crate) fn count_replay_lineage_authority_basis(
        &self,
        indexed_source: Option<ReplayLineageAuthorityIndexedSource>,
        kind: ReplayAuthorityBasisKind,
        event_width: usize,
        decision_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.replay_lineage_authority_lookup_requests += 1;
            match indexed_source {
                Some(ReplayLineageAuthorityIndexedSource::DurableLog) => {
                    counters.replay_lineage_log_index_hits += 1;
                }
                Some(ReplayLineageAuthorityIndexedSource::Checkpoint) => {
                    counters.replay_lineage_checkpoint_index_hits += 1;
                }
                None => {}
            }
            match kind {
                ReplayAuthorityBasisKind::DurableLogCanonical => {
                    counters.replay_lineage_durable_basis_selections += 1;
                }
                ReplayAuthorityBasisKind::HistoryEnvelopeFallback => {
                    counters.replay_lineage_history_fallback_basis_selections += 1;
                }
            }
            counters.replay_lineage_digest_event_width += event_width;
            counters.replay_lineage_digest_decision_width += decision_width;
        });
    }

    pub(crate) fn count_replay_lineage_authoritative_basis_rejection(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.replay_lineage_authoritative_basis_rejections += 1;
        });
    }

    pub(crate) fn count_merge_history_ancestry_traversal(&self, nodes_visited: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_history_ancestry_traversals += 1;
            counters.merge_history_ancestry_nodes_visited += nodes_visited;
        });
    }

    pub(crate) fn count_merge_history_parent_comparisons(&self, comparisons: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_history_parent_comparisons += comparisons;
        });
    }

    pub(crate) fn count_merge_history_replay_planning(
        &self,
        nodes_visited: usize,
        parent_checks: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_history_replay_planning_nodes_visited += nodes_visited;
            counters.merge_history_replay_parent_checks += parent_checks;
        });
    }

    pub(crate) fn count_merge_history_durability_validation(
        &self,
        nodes_visited: usize,
        parent_checks: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_history_durability_validation_nodes_visited += nodes_visited;
            counters.merge_history_durability_parent_checks += parent_checks;
        });
    }

    pub(crate) fn count_merge_planning_request(
        &self,
        schema_kinds: usize,
        target_commits: usize,
        source_commits: usize,
        target_records: usize,
        source_records: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_planning_requests += 1;
            counters.merge_planning_schema_kinds_snapshotted += schema_kinds;
            counters.merge_planning_target_commits_scoped += target_commits;
            counters.merge_planning_source_commits_scoped += source_commits;
            counters.merge_planning_target_records_scoped += target_records;
            counters.merge_planning_source_records_scoped += source_records;
        });
    }

    pub(crate) fn count_merge_identity_discovery(&self, candidates: usize, declarations: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_identity_candidates_discovered += candidates;
            counters.merge_identity_effective_declarations += declarations;
        });
    }

    pub(crate) fn count_merge_identity_target_indexing(&self, scanned: usize, indexed: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_identity_target_records_scanned += scanned;
            counters.merge_identity_target_records_indexed += indexed;
        });
    }

    pub(crate) fn count_merge_conflict_classification(&self, records: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_conflict_records_classified += records;
        });
    }

    pub(crate) fn count_merge_causal_annotation(&self, records: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_causal_records_annotated += records;
        });
    }

    pub(crate) fn count_merge_policy_resolution(&self, records: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_policy_records_resolved += records;
        });
    }

    pub(crate) fn count_merge_lowering(&self, lowered_records: usize, decision_log_width: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_lowered_records_emitted += lowered_records;
            counters.merge_decision_log_width += decision_log_width;
        });
    }

    pub(crate) fn count_merge_planning_elapsed(&self, elapsed_nanos: u128) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_planning_elapsed_nanos += elapsed_nanos;
        });
    }

    pub(crate) fn count_merge_topology_region_detection(
        &self,
        relation_candidates: usize,
        endpoint_incidences: usize,
        region_conflicts: usize,
        region_records_escalated: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_topology_relation_candidates_scoped += relation_candidates;
            counters.merge_topology_endpoint_incidences_scoped += endpoint_incidences;
            counters.merge_topology_region_conflicts_detected += region_conflicts;
            counters.merge_topology_region_records_escalated += region_records_escalated;
        });
    }

    pub(crate) fn count_merge_execution_verification_request(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_verification_requests += 1;
        });
    }

    pub(crate) fn count_merge_execution_branch_head_checks(&self, checks: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_branch_head_checks += checks;
        });
    }

    pub(crate) fn count_merge_execution_merge_base_checks(&self, checks: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_merge_base_checks += checks;
        });
    }

    pub(crate) fn count_merge_execution_schema_snapshot_kinds(&self, kinds: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_schema_kinds_snapshotted += kinds;
        });
    }

    pub(crate) fn count_merge_execution_compiled_plan_digest_checks(&self, checks: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_compiled_plan_digest_checks += checks;
        });
    }

    pub(crate) fn count_merge_execution_attempt(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_attempts += 1;
        });
    }

    pub(crate) fn count_merge_execution_request(
        &self,
        admitted_records: usize,
        emitted_mutation_intents: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.merge_execution_requests += 1;
            counters.merge_execution_records_admitted += admitted_records;
            counters.merge_execution_mutation_intents_emitted += emitted_mutation_intents;
        });
    }

    pub(crate) fn count_descriptor_version_mismatch(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.descriptor_version_mismatches_encountered += 1;
        });
    }

    pub(crate) fn count_inspection_connectivity_work(
        &self,
        entity_scans: u64,
        relation_scans: u64,
        frontier_expansions: u64,
        components_evaluated: u64,
    ) {
        let entity_scans = entity_scans.min(usize::MAX as u64) as usize;
        let relation_scans = relation_scans.min(usize::MAX as u64) as usize;
        let frontier_expansions = frontier_expansions.min(usize::MAX as u64) as usize;
        let components_evaluated = components_evaluated.min(usize::MAX as u64) as usize;
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_connectivity_entity_scans += entity_scans;
            counters.inspection_connectivity_relation_scans += relation_scans;
            counters.inspection_connectivity_frontier_expansions += frontier_expansions;
            counters.inspection_connectivity_components_evaluated += components_evaluated;
        });
    }

    pub(crate) fn count_inspection_retention_work(
        &self,
        entity_slot_scans: u64,
        relation_slot_scans: u64,
    ) {
        let entity_slot_scans = entity_slot_scans.min(usize::MAX as u64) as usize;
        let relation_slot_scans = relation_slot_scans.min(usize::MAX as u64) as usize;
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_retention_entity_slot_scans += entity_slot_scans;
            counters.inspection_retention_relation_slot_scans += relation_slot_scans;
        });
    }

    pub(crate) fn count_inspection_budget_refusal(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_budget_refusals += 1;
        });
    }
}
