use crate::history::data::BranchId;
use crate::indexes::data::{
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId, DerivedIndexKind,
    DerivedIndexPayload,
};
use crate::indexes::logic::unique_field_index::{payload_field_key, payload_field_key_optional};
use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{
    certification_digest, reduce_query_fragments, FallbackParityMode,
    FallbackParityVerifiedQueryOutcome, IndexQueryRejectionClass, PlannedQueryPacket,
    QueryAccessPath, QueryFallbackContract, QueryFragmentCounters, QueryScope, QueryWorkerFragment,
    SnapshotPinnedQueryPlan,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

const SAMPLED_PARITY_MODULUS: u128 = 8;
const SAMPLED_PARITY_REMAINDER: u128 = 0;

pub struct IndexAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

#[derive(Default)]
struct IndexQueryScratch {
    entity_capacity_hint: usize,
    relation_capacity_hint: usize,
}

impl IndexQueryScratch {
    fn entity_buffer(&self, candidate_count: usize) -> Vec<crate::storage::data::EntityReadRecord> {
        Vec::with_capacity(self.entity_capacity_hint.max(candidate_count))
    }

    fn relation_buffer(
        &self,
        candidate_count: usize,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        Vec::with_capacity(self.relation_capacity_hint.max(candidate_count))
    }

    fn remember_entity_capacity(&mut self, len: usize) {
        self.entity_capacity_hint = self.entity_capacity_hint.max(len);
    }

    fn remember_relation_capacity(&mut self, len: usize) {
        self.relation_capacity_hint = self.relation_capacity_hint.max(len);
    }
}

fn index_query_scratch_hints() -> &'static Mutex<BTreeMap<u64, IndexQueryScratch>> {
    static HINTS: OnceLock<Mutex<BTreeMap<u64, IndexQueryScratch>>> = OnceLock::new();
    HINTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub(crate) fn purge_index_query_scratch_hints(runtime_id: u64) {
    let mut hints = index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned");
    hints.remove(&runtime_id);
}

#[cfg(test)]
pub(crate) fn index_query_scratch_hint_count() -> usize {
    index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned")
        .len()
}

#[cfg(test)]
pub(crate) fn index_query_scratch_hint_exists(runtime_id: u64) -> bool {
    index_query_scratch_hints()
        .lock()
        .expect("index query scratch hints lock poisoned")
        .contains_key(&runtime_id)
}

impl RelationalRuntime {
    pub fn index_access(&self) -> IndexAccess<'_> {
        IndexAccess::new(self)
    }
}

impl<'runtime> IndexAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn latest_generation(
        &self,
        index_id: DerivedIndexId,
        branch_id: &BranchId,
    ) -> Option<&DerivedIndexGeneration> {
        let definition = self.runtime.indexes.definitions.get(&index_id)?;
        self.runtime
            .indexes
            .generations
            .get(&index_id)
            .and_then(|generations| {
                generations.iter().rev().find(|generation| {
                    !definition.branch_scoped || generation.compatibility.branch_id == *branch_id
                })
            })
    }

    pub fn generations_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<DerivedIndexGeneration> {
        let mut generations = self
            .runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .filter(|generation| generation.compatibility.version_id <= version_id)
            .cloned()
            .collect::<Vec<_>>();
        generations.sort_by(|left, right| {
            left.compatibility
                .branch_id
                .cmp(&right.compatibility.branch_id)
                .then_with(|| left.source_commit_id.cmp(&right.source_commit_id))
                .then_with(|| left.generation_id.cmp(&right.generation_id))
        });
        generations
    }

    pub fn execute_query_plan_with_fallback_parity(
        &self,
        plan: SnapshotPinnedQueryPlan,
        parity_mode: FallbackParityMode,
    ) -> Option<FallbackParityVerifiedQueryOutcome> {
        if plan.packet.fallback == QueryFallbackContract::IndexAdmissibleStorageEquivalent {
            self.runtime
                .performance_access()
                .count_query_index_attempt();
        }
        let storage_execution = || self.runtime.read_truth().execute_query_plan(plan.clone());
        let (execution, access_path) = match self.admissible_access_path(&plan) {
            QueryAccessPath::DerivedIndexGeneration { generation_id } => {
                let index_execution =
                    self.execute_index_backed_query_from_generation(&plan, generation_id)?;
                match parity_mode {
                    FallbackParityMode::ProductionAdmissibility => {
                        self.runtime.performance_access().count_query_index_path();
                        self.record_index_execution_counters(&index_execution);
                        (
                            index_execution,
                            QueryAccessPath::DerivedIndexGeneration { generation_id },
                        )
                    }
                    FallbackParityMode::SampledParity
                        if !self.should_verify_sampled_parity(&plan, generation_id) =>
                    {
                        self.runtime.performance_access().count_query_index_path();
                        self.record_index_execution_counters(&index_execution);
                        (
                            index_execution,
                            QueryAccessPath::DerivedIndexGeneration { generation_id },
                        )
                    }
                    FallbackParityMode::SampledParity | FallbackParityMode::CertificationParity => {
                        self.runtime
                            .performance_access()
                            .count_query_index_parity_verification();
                        let storage_execution = storage_execution()?;
                        if storage_execution.result == index_execution.result {
                            self.runtime.performance_access().count_query_index_path();
                            self.record_index_execution_counters(&index_execution);
                            (
                                index_execution,
                                QueryAccessPath::DerivedIndexGeneration { generation_id },
                            )
                        } else {
                            self.runtime
                                .performance_access()
                                .count_query_index_rejection();
                            self.record_index_execution_shape(&index_execution);
                            (
                                storage_execution,
                                QueryAccessPath::DerivedIndexRejectedStorageFallback {
                                    rejection: IndexQueryRejectionClass::CorruptPayload,
                                },
                            )
                        }
                    }
                }
            }
            access_path => {
                if matches!(
                    access_path,
                    QueryAccessPath::DerivedIndexRejectedStorageFallback { .. }
                ) {
                    self.runtime
                        .performance_access()
                        .count_query_index_rejection();
                }
                (storage_execution()?, access_path)
            }
        };
        let parity_basis_digest = certification_digest(&(
            &access_path,
            parity_mode,
            &execution.result,
            execution.plan.packet.plan_key,
        ));
        Some(FallbackParityVerifiedQueryOutcome {
            execution,
            access_path,
            parity_mode,
            parity_basis_digest,
        })
    }

    pub(crate) fn definitions_snapshot(&self) -> Vec<DerivedIndexDefinition> {
        self.runtime.indexes.definitions.values().cloned().collect()
    }

    pub(crate) fn generations_snapshot(&self) -> Vec<DerivedIndexGeneration> {
        self.runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter().cloned())
            .collect()
    }

    pub(crate) fn entity_unique_field_ids(
        &self,
        field: &str,
        value: &str,
    ) -> Option<&std::collections::BTreeSet<crate::identity::data::EntityId>> {
        self.runtime
            .indexes
            .entity_unique_field_index
            .get(field)
            .and_then(|values| values.get(value))
    }

    pub(crate) fn entity_unique_field_entries(
        &self,
        field: &str,
    ) -> Option<
        &std::collections::BTreeMap<
            String,
            std::collections::BTreeSet<crate::identity::data::EntityId>,
        >,
    > {
        self.runtime.indexes.entity_unique_field_index.get(field)
    }

    fn branch_id_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<BranchId> {
        self.runtime
            .history
            .commit_graph
            .values()
            .find(|node| node.commit.version_id == version_id)
            .map(|node| node.commit.branch_id.clone())
    }

    fn admissible_access_path(&self, plan: &SnapshotPinnedQueryPlan) -> QueryAccessPath {
        if plan.packet.fallback == QueryFallbackContract::StorageOnly {
            return QueryAccessPath::AuthoritativeStorage;
        }

        let branch_id = self
            .branch_id_for_version(plan.snapshot.version_id)
            .unwrap_or_else(|| self.runtime.config.history.main_branch.clone());
        let Some(generation) = self.candidate_generation_for_packet(&plan.packet, &branch_id)
        else {
            return QueryAccessPath::DerivedIndexRejectedStorageFallback {
                rejection: if self.matching_index_definition_exists(&plan.packet) {
                    IndexQueryRejectionClass::MissingGeneration
                } else if self
                    .runtime
                    .indexes
                    .generations
                    .values()
                    .flat_map(|generations| generations.iter())
                    .any(|generation| {
                        generation.compatibility.version_id <= plan.snapshot.version_id
                    })
                {
                    IndexQueryRejectionClass::UnsupportedScope
                } else {
                    IndexQueryRejectionClass::MissingGeneration
                },
            };
        };

        match self.index_rejection_for_packet(&plan.packet, generation, &branch_id) {
            Some(rejection) => QueryAccessPath::DerivedIndexRejectedStorageFallback { rejection },
            None => QueryAccessPath::DerivedIndexGeneration {
                generation_id: generation.generation_id,
            },
        }
    }

    fn should_verify_sampled_parity(
        &self,
        plan: &SnapshotPinnedQueryPlan,
        generation_id: crate::indexes::data::DerivedIndexGenerationId,
    ) -> bool {
        let sample_key = (plan.packet.plan_key.0)
            ^ ((generation_id.0 as u128) << 64)
            ^ (plan.snapshot.version_id.0 as u128);
        sample_key % SAMPLED_PARITY_MODULUS == SAMPLED_PARITY_REMAINDER
    }

    fn index_rejection_for_packet(
        &self,
        packet: &PlannedQueryPacket,
        generation: &DerivedIndexGeneration,
        branch_id: &BranchId,
    ) -> Option<IndexQueryRejectionClass> {
        if generation.status != crate::indexes::data::DerivedIndexPublicationStatus::Published {
            return Some(IndexQueryRejectionClass::CorruptPayload);
        }
        if generation.compatibility.branch_id != *branch_id
            && self
                .runtime
                .indexes
                .definitions
                .get(&generation.index_id)
                .is_some_and(|definition| definition.branch_scoped)
        {
            return Some(IndexQueryRejectionClass::IncompatibleBranch);
        }
        if generation.compatibility.version_id > packet.context_id.version_id {
            return Some(IndexQueryRejectionClass::IncompatibleVersion);
        }
        if generation.compatibility.schema_version != packet.context_id.schema_version {
            return Some(IndexQueryRejectionClass::IncompatibleVersion);
        }
        match &packet.scope {
            QueryScope::EntityPayloadFieldEquals { .. }
            | QueryScope::EntityPayloadFieldAnyOf { .. } => {
                if !matches!(
                    packet.ordering,
                    crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                        | crate::query::data::QueryOrderingContract::CanonicalEntityIdOrder
                ) {
                    return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
                }
            }
            QueryScope::RelationPayloadFieldEquals { .. }
            | QueryScope::RelationPayloadFieldAnyOf { .. } => {
                if !matches!(
                    packet.ordering,
                    crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                        | crate::query::data::QueryOrderingContract::CanonicalRelationIdOrder
                ) {
                    return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
                }
            }
            _ => {
                if !matches!(
                    packet.ordering,
                    crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                        | crate::query::data::QueryOrderingContract::CanonicalEntityIdOrder
                        | crate::query::data::QueryOrderingContract::CanonicalRelationIdOrder
                ) {
                    return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
                }
            }
        }
        match (
            &packet.scope,
            &generation.payload,
            self.runtime.indexes.definitions.get(&generation.index_id),
        ) {
            (
                QueryScope::EntityPayloadFieldEquals { field, .. },
                DerivedIndexPayload::EntityField(_),
                Some(definition),
            )
            | (
                QueryScope::EntityPayloadFieldAnyOf { field, .. },
                DerivedIndexPayload::EntityField(_),
                Some(definition),
            ) => match &definition.kind {
                DerivedIndexKind::EntityPayloadField {
                    field: indexed_field,
                } if indexed_field == field => None,
                _ => Some(IndexQueryRejectionClass::UnsupportedScope),
            },
            (
                QueryScope::RelationPayloadFieldEquals { field, .. },
                DerivedIndexPayload::RelationField(_),
                Some(definition),
            )
            | (
                QueryScope::RelationPayloadFieldAnyOf { field, .. },
                DerivedIndexPayload::RelationField(_),
                Some(definition),
            ) => match &definition.kind {
                DerivedIndexKind::RelationPayloadField {
                    field: indexed_field,
                } if indexed_field == field => None,
                _ => Some(IndexQueryRejectionClass::UnsupportedScope),
            },
            _ => Some(IndexQueryRejectionClass::UnsupportedScope),
        }
    }

    fn candidate_generation_for_packet(
        &self,
        packet: &PlannedQueryPacket,
        branch_id: &BranchId,
    ) -> Option<&DerivedIndexGeneration> {
        match &packet.scope {
            QueryScope::EntityPayloadFieldEquals { field, .. }
            | QueryScope::EntityPayloadFieldAnyOf { field, .. } => self
                .runtime
                .indexes
                .definitions
                .values()
                .filter(|definition| {
                    matches!(
                        &definition.kind,
                        DerivedIndexKind::EntityPayloadField { field: indexed_field }
                            if indexed_field == field
                    )
                })
                .flat_map(|definition| {
                    self.runtime
                        .indexes
                        .generations
                        .get(&definition.index_id)
                        .into_iter()
                        .flatten()
                })
                .max_by(|left, right| {
                    self.generation_preference(left, packet, branch_id)
                        .cmp(&self.generation_preference(right, packet, branch_id))
                        .then_with(|| left.generation_id.cmp(&right.generation_id))
                }),
            QueryScope::RelationPayloadFieldEquals { field, .. }
            | QueryScope::RelationPayloadFieldAnyOf { field, .. } => self
                .runtime
                .indexes
                .definitions
                .values()
                .filter(|definition| {
                    matches!(
                        &definition.kind,
                        DerivedIndexKind::RelationPayloadField { field: indexed_field }
                            if indexed_field == field
                    )
                })
                .flat_map(|definition| {
                    self.runtime
                        .indexes
                        .generations
                        .get(&definition.index_id)
                        .into_iter()
                        .flatten()
                })
                .max_by(|left, right| {
                    self.generation_preference(left, packet, branch_id)
                        .cmp(&self.generation_preference(right, packet, branch_id))
                        .then_with(|| left.generation_id.cmp(&right.generation_id))
                }),
            _ => None,
        }
    }

    fn matching_index_definition_exists(&self, packet: &PlannedQueryPacket) -> bool {
        match &packet.scope {
            QueryScope::EntityPayloadFieldEquals { field, .. }
            | QueryScope::EntityPayloadFieldAnyOf { field, .. } => {
                self.runtime.indexes.definitions.values().any(|definition| {
                    matches!(
                        &definition.kind,
                        DerivedIndexKind::EntityPayloadField { field: indexed_field }
                            if indexed_field == field
                    )
                })
            }
            QueryScope::RelationPayloadFieldEquals { field, .. }
            | QueryScope::RelationPayloadFieldAnyOf { field, .. } => {
                self.runtime.indexes.definitions.values().any(|definition| {
                    matches!(
                        &definition.kind,
                        DerivedIndexKind::RelationPayloadField { field: indexed_field }
                            if indexed_field == field
                    )
                })
            }
            _ => false,
        }
    }

    fn execute_index_backed_query_from_generation(
        &self,
        plan: &SnapshotPinnedQueryPlan,
        generation_id: crate::indexes::data::DerivedIndexGenerationId,
    ) -> Option<crate::query::data::QueryExecutionOutcome> {
        let generation = self
            .runtime
            .indexes
            .generations
            .values()
            .flat_map(|generations| generations.iter())
            .find(|generation| generation.generation_id == generation_id)?;
        let state = self.runtime.storage_access().current_state();
        match (&plan.packet.scope, &generation.payload) {
            (
                QueryScope::EntityPayloadFieldEquals {
                    field,
                    value,
                    partition_scope,
                },
                DerivedIndexPayload::EntityField(payload),
            ) => {
                let partition_scope = partition_scope.as_ref().map(|partitions| {
                    partitions
                        .iter()
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                });
                let candidate_count = payload.get(value).map_or(0, |matches| matches.len());
                let runtime_id = self.runtime.runtime_instance_id();
                let mut scratch = {
                    let mut hints = index_query_scratch_hints()
                        .lock()
                        .expect("index query scratch hints lock poisoned");
                    let scratch = hints.entry(runtime_id).or_default();
                    if scratch.entity_capacity_hint > 0 {
                        self.runtime
                            .performance_access()
                            .count_query_index_scratch_reuse();
                    }
                    IndexQueryScratch {
                        entity_capacity_hint: scratch.entity_capacity_hint,
                        relation_capacity_hint: scratch.relation_capacity_hint,
                    }
                };
                let mut entities = scratch.entity_buffer(candidate_count);
                let mut touched_partition_ids = BTreeSet::new();
                for entity_id in payload.get(value).into_iter().flatten().copied() {
                    if partition_scope
                        .as_ref()
                        .is_some_and(|partitions| !partitions.contains(&entity_id.partition_id))
                    {
                        continue;
                    }
                    let Some(record) = self.runtime.read_truth().entity_record_for_id_at_version(
                        &state,
                        entity_id,
                        plan.snapshot.version_id,
                    ) else {
                        continue;
                    };
                    if payload_field_key(&record.payload, field).as_deref() == Some(value.as_str())
                    {
                        touched_partition_ids.insert(record.entity_id.partition_id);
                        entities.push(record);
                    }
                }
                let touched_partitions = touched_partition_ids.len();
                let entity_count = entities.len();
                scratch.remember_entity_capacity(entity_count);
                let mut hints = index_query_scratch_hints()
                    .lock()
                    .expect("index query scratch hints lock poisoned");
                let shared = hints.entry(runtime_id).or_default();
                shared.remember_entity_capacity(scratch.entity_capacity_hint);
                shared.remember_relation_capacity(scratch.relation_capacity_hint);
                drop(hints);
                let result = reduce_query_fragments(
                    plan.packet.execution_shape,
                    plan.packet.ordering,
                    vec![QueryWorkerFragment {
                        plan_key: plan.packet.plan_key,
                        fragment_key: crate::query::data::deterministic_query_fragment_key(
                            plan.packet.plan_key,
                            0,
                        ),
                        ordering: plan.packet.ordering,
                        entities,
                        relations: Vec::new(),
                        counters: QueryFragmentCounters {
                            target_count: entity_count,
                            entity_records_emitted: entity_count,
                            relation_records_emitted: 0,
                            touched_partitions,
                        },
                        traversal_basis: None,
                    }],
                );
                Some(crate::query::data::QueryExecutionOutcome {
                    plan: plan.clone(),
                    complexity: crate::query::data::QueryComplexitySummary {
                        packet_count: 1,
                        fragment_count: 1,
                        touched_partitions,
                        target_count: entity_count,
                        entity_records_emitted: entity_count,
                        relation_records_emitted: 0,
                    },
                    result,
                })
            }
            (
                QueryScope::EntityPayloadFieldAnyOf {
                    field,
                    values,
                    partition_scope,
                },
                DerivedIndexPayload::EntityField(payload),
            ) => {
                let partition_scope = partition_scope.as_ref().map(|partitions| {
                    partitions
                        .iter()
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                });
                let values = QueryScope::canonical_value_scope(values.as_ref());
                let candidate_count = values
                    .iter()
                    .map(|value| payload.get(value).map_or(0, |matches| matches.len()))
                    .sum();
                let runtime_id = self.runtime.runtime_instance_id();
                let mut scratch = {
                    let mut hints = index_query_scratch_hints()
                        .lock()
                        .expect("index query scratch hints lock poisoned");
                    let scratch = hints.entry(runtime_id).or_default();
                    if scratch.entity_capacity_hint > 0 {
                        self.runtime
                            .performance_access()
                            .count_query_index_scratch_reuse();
                    }
                    IndexQueryScratch {
                        entity_capacity_hint: scratch.entity_capacity_hint,
                        relation_capacity_hint: scratch.relation_capacity_hint,
                    }
                };
                let selected = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
                let mut entities = scratch.entity_buffer(candidate_count);
                let mut candidate_ids = BTreeSet::new();
                let mut touched_partition_ids = BTreeSet::new();
                for value in values.iter() {
                    for entity_id in payload.get(value).into_iter().flatten().copied() {
                        candidate_ids.insert(entity_id);
                    }
                }
                let state = self.runtime.storage_access().current_state();
                for entity_id in candidate_ids {
                    if partition_scope
                        .as_ref()
                        .is_some_and(|partitions| !partitions.contains(&entity_id.partition_id))
                    {
                        continue;
                    }
                    let Some(record) = self.runtime.read_truth().entity_record_for_id_at_version(
                        &state,
                        entity_id,
                        plan.snapshot.version_id,
                    ) else {
                        continue;
                    };
                    if payload_field_key(&record.payload, field)
                        .as_deref()
                        .is_some_and(|value| selected.contains(value))
                    {
                        touched_partition_ids.insert(record.entity_id.partition_id);
                        entities.push(record);
                    }
                }
                let touched_partitions = touched_partition_ids.len();
                let entity_count = entities.len();
                scratch.remember_entity_capacity(entity_count);
                let mut hints = index_query_scratch_hints()
                    .lock()
                    .expect("index query scratch hints lock poisoned");
                let shared = hints.entry(runtime_id).or_default();
                shared.remember_entity_capacity(scratch.entity_capacity_hint);
                shared.remember_relation_capacity(scratch.relation_capacity_hint);
                drop(hints);
                let result = reduce_query_fragments(
                    plan.packet.execution_shape,
                    plan.packet.ordering,
                    vec![QueryWorkerFragment {
                        plan_key: plan.packet.plan_key,
                        fragment_key: crate::query::data::deterministic_query_fragment_key(
                            plan.packet.plan_key,
                            0,
                        ),
                        ordering: plan.packet.ordering,
                        entities,
                        relations: Vec::new(),
                        counters: QueryFragmentCounters {
                            target_count: values.len(),
                            entity_records_emitted: entity_count,
                            relation_records_emitted: 0,
                            touched_partitions,
                        },
                        traversal_basis: None,
                    }],
                );
                Some(crate::query::data::QueryExecutionOutcome {
                    plan: plan.clone(),
                    complexity: crate::query::data::QueryComplexitySummary {
                        packet_count: 1,
                        fragment_count: 1,
                        touched_partitions,
                        target_count: values.len(),
                        entity_records_emitted: entity_count,
                        relation_records_emitted: 0,
                    },
                    result,
                })
            }
            (
                QueryScope::RelationPayloadFieldEquals {
                    field,
                    value,
                    partition_scope,
                },
                DerivedIndexPayload::RelationField(payload),
            ) => {
                let partition_scope = partition_scope.as_ref().map(|partitions| {
                    partitions
                        .iter()
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                });
                let candidate_count = payload.get(value).map_or(0, |matches| matches.len());
                let runtime_id = self.runtime.runtime_instance_id();
                let mut scratch = {
                    let mut hints = index_query_scratch_hints()
                        .lock()
                        .expect("index query scratch hints lock poisoned");
                    let scratch = hints.entry(runtime_id).or_default();
                    if scratch.relation_capacity_hint > 0 {
                        self.runtime
                            .performance_access()
                            .count_query_index_scratch_reuse();
                    }
                    IndexQueryScratch {
                        entity_capacity_hint: scratch.entity_capacity_hint,
                        relation_capacity_hint: scratch.relation_capacity_hint,
                    }
                };
                let mut relations = scratch.relation_buffer(candidate_count);
                let mut touched_partition_ids = BTreeSet::new();
                for relation_id in payload.get(value).into_iter().flatten().copied() {
                    if partition_scope
                        .as_ref()
                        .is_some_and(|partitions| !partitions.contains(&relation_id.partition_id))
                    {
                        continue;
                    }
                    let Some(record) = self.runtime.read_truth().relation_record_for_id_at_version(
                        &state,
                        relation_id,
                        plan.snapshot.version_id,
                    ) else {
                        continue;
                    };
                    if payload_field_key_optional(&record.payload, field).as_deref()
                        == Some(value.as_str())
                    {
                        touched_partition_ids.insert(record.relation_id.partition_id);
                        relations.push(record);
                    }
                }
                let touched_partitions = touched_partition_ids.len();
                let relation_count = relations.len();
                scratch.remember_relation_capacity(relation_count);
                let mut hints = index_query_scratch_hints()
                    .lock()
                    .expect("index query scratch hints lock poisoned");
                let shared = hints.entry(runtime_id).or_default();
                shared.remember_entity_capacity(scratch.entity_capacity_hint);
                shared.remember_relation_capacity(scratch.relation_capacity_hint);
                drop(hints);
                let result = reduce_query_fragments(
                    plan.packet.execution_shape,
                    plan.packet.ordering,
                    vec![QueryWorkerFragment {
                        plan_key: plan.packet.plan_key,
                        fragment_key: crate::query::data::deterministic_query_fragment_key(
                            plan.packet.plan_key,
                            0,
                        ),
                        ordering: plan.packet.ordering,
                        entities: Vec::new(),
                        relations,
                        counters: QueryFragmentCounters {
                            target_count: relation_count,
                            entity_records_emitted: 0,
                            relation_records_emitted: relation_count,
                            touched_partitions,
                        },
                        traversal_basis: None,
                    }],
                );
                Some(crate::query::data::QueryExecutionOutcome {
                    plan: plan.clone(),
                    complexity: crate::query::data::QueryComplexitySummary {
                        packet_count: 1,
                        fragment_count: 1,
                        touched_partitions,
                        target_count: relation_count,
                        entity_records_emitted: 0,
                        relation_records_emitted: relation_count,
                    },
                    result,
                })
            }
            (
                QueryScope::RelationPayloadFieldAnyOf {
                    field,
                    values,
                    partition_scope,
                },
                DerivedIndexPayload::RelationField(payload),
            ) => {
                let partition_scope = partition_scope.as_ref().map(|partitions| {
                    partitions
                        .iter()
                        .copied()
                        .collect::<std::collections::BTreeSet<_>>()
                });
                let values = QueryScope::canonical_value_scope(values.as_ref());
                let candidate_count = values
                    .iter()
                    .map(|value| payload.get(value).map_or(0, |matches| matches.len()))
                    .sum();
                let runtime_id = self.runtime.runtime_instance_id();
                let mut scratch = {
                    let mut hints = index_query_scratch_hints()
                        .lock()
                        .expect("index query scratch hints lock poisoned");
                    let scratch = hints.entry(runtime_id).or_default();
                    if scratch.relation_capacity_hint > 0 {
                        self.runtime
                            .performance_access()
                            .count_query_index_scratch_reuse();
                    }
                    IndexQueryScratch {
                        entity_capacity_hint: scratch.entity_capacity_hint,
                        relation_capacity_hint: scratch.relation_capacity_hint,
                    }
                };
                let selected = values.iter().map(String::as_str).collect::<BTreeSet<_>>();
                let mut relations = scratch.relation_buffer(candidate_count);
                let mut candidate_ids = BTreeSet::new();
                let mut touched_partition_ids = BTreeSet::new();
                for value in values.iter() {
                    for relation_id in payload.get(value).into_iter().flatten().copied() {
                        candidate_ids.insert(relation_id);
                    }
                }
                let state = self.runtime.storage_access().current_state();
                for relation_id in candidate_ids {
                    if partition_scope
                        .as_ref()
                        .is_some_and(|partitions| !partitions.contains(&relation_id.partition_id))
                    {
                        continue;
                    }
                    let Some(record) = self.runtime.read_truth().relation_record_for_id_at_version(
                        &state,
                        relation_id,
                        plan.snapshot.version_id,
                    ) else {
                        continue;
                    };
                    if payload_field_key_optional(&record.payload, field)
                        .as_deref()
                        .is_some_and(|value| selected.contains(value))
                    {
                        touched_partition_ids.insert(record.relation_id.partition_id);
                        relations.push(record);
                    }
                }
                let touched_partitions = touched_partition_ids.len();
                let relation_count = relations.len();
                scratch.remember_relation_capacity(relation_count);
                let mut hints = index_query_scratch_hints()
                    .lock()
                    .expect("index query scratch hints lock poisoned");
                let shared = hints.entry(runtime_id).or_default();
                shared.remember_entity_capacity(scratch.entity_capacity_hint);
                shared.remember_relation_capacity(scratch.relation_capacity_hint);
                drop(hints);
                let result = reduce_query_fragments(
                    plan.packet.execution_shape,
                    plan.packet.ordering,
                    vec![QueryWorkerFragment {
                        plan_key: plan.packet.plan_key,
                        fragment_key: crate::query::data::deterministic_query_fragment_key(
                            plan.packet.plan_key,
                            0,
                        ),
                        ordering: plan.packet.ordering,
                        entities: Vec::new(),
                        relations,
                        counters: QueryFragmentCounters {
                            target_count: values.len(),
                            entity_records_emitted: 0,
                            relation_records_emitted: relation_count,
                            touched_partitions,
                        },
                        traversal_basis: None,
                    }],
                );
                Some(crate::query::data::QueryExecutionOutcome {
                    plan: plan.clone(),
                    complexity: crate::query::data::QueryComplexitySummary {
                        packet_count: 1,
                        fragment_count: 1,
                        touched_partitions,
                        target_count: values.len(),
                        entity_records_emitted: 0,
                        relation_records_emitted: relation_count,
                    },
                    result,
                })
            }
            _ => None,
        }
    }

    fn generation_preference(
        &self,
        generation: &DerivedIndexGeneration,
        packet: &PlannedQueryPacket,
        branch_id: &BranchId,
    ) -> (bool, bool, bool, bool) {
        let branch_compatible = self
            .runtime
            .indexes
            .definitions
            .get(&generation.index_id)
            .is_none_or(|definition| {
                !definition.branch_scoped || generation.compatibility.branch_id == *branch_id
            });
        let version_compatible =
            generation.compatibility.version_id <= packet.context_id.version_id;
        let schema_compatible =
            generation.compatibility.schema_version == packet.context_id.schema_version;
        let published =
            generation.status == crate::indexes::data::DerivedIndexPublicationStatus::Published;
        (
            published,
            branch_compatible,
            version_compatible,
            schema_compatible,
        )
    }

    fn record_index_execution_shape(&self, execution: &crate::query::data::QueryExecutionOutcome) {
        let target_count = execution.complexity.target_count.max(1);
        let scope_units = execution.complexity.touched_partitions.max(1);
        self.runtime.performance_access().count_query_packet_shape(
            execution.complexity.packet_count.max(1),
            target_count,
            target_count,
            scope_units,
        );
        self.runtime
            .performance_access()
            .count_query_serial_strategy();
    }

    fn record_index_execution_counters(
        &self,
        execution: &crate::query::data::QueryExecutionOutcome,
    ) {
        self.record_index_execution_shape(execution);
        self.runtime.performance_access().count_query_emissions(
            execution.result.entities.len(),
            execution.result.relations.len(),
        );
    }
}
