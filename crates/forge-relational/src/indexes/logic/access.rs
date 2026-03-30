use crate::history::data::BranchId;
use crate::indexes::data::{
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId, DerivedIndexKind,
    DerivedIndexPayload,
};
use crate::indexes::logic::unique_field_index::payload_field_key;
use crate::logic::runtime::{IndexedReadOutcome, RelationalRuntime};
use crate::query::data::{
    certification_digest, reduce_query_fragments, FallbackParityMode,
    FallbackParityVerifiedQueryOutcome, IndexQueryRejectionClass, PlannedQueryPacket,
    QueryAccessPath, QueryFallbackContract, QueryFragmentCounters, QueryScope, QueryWorkPacket,
    QueryWorkerFragment, SnapshotPinnedQueryPlan,
};
use crate::snapshots::data::SnapshotHandle;

pub struct IndexAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
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

    pub fn read_with_storage_fallback(
        &self,
        handle: &SnapshotHandle,
        packet: &QueryWorkPacket,
    ) -> Option<IndexedReadOutcome> {
        let context_id = self.runtime.visibility_reads().query_plan_context(handle)?;
        let plan = self
            .runtime
            .visibility_reads()
            .plan_query_packet(handle, packet.clone().planned_with_context(context_id))?;
        let outcome = self.execute_query_plan_with_fallback_parity(
            plan,
            FallbackParityMode::ProductionAdmissibility,
        )?;
        let used_index_generation = match outcome.access_path {
            QueryAccessPath::DerivedIndexGeneration { generation_id } => Some(generation_id),
            QueryAccessPath::AuthoritativeStorage
            | QueryAccessPath::DerivedIndexRejectedStorageFallback { .. } => None,
        };
        Some(IndexedReadOutcome {
            result: crate::storage::data::PacketResult {
                execution_shape: outcome.execution.result.execution_shape,
                entities: outcome.execution.result.entities,
                relations: outcome.execution.result.relations,
            },
            used_index_generation,
        })
    }

    pub fn execute_query_plan_with_fallback_parity(
        &self,
        plan: SnapshotPinnedQueryPlan,
        parity_mode: FallbackParityMode,
    ) -> Option<FallbackParityVerifiedQueryOutcome> {
        if plan.packet.fallback == QueryFallbackContract::IndexAdmissibleStorageEquivalent {
            self.runtime.performance_access().count_query_index_attempt();
        }
        let storage_execution = || self.runtime.visibility_reads().execute_query_plan(plan.clone());
        let (execution, access_path) = match self.admissible_access_path(&plan) {
            QueryAccessPath::DerivedIndexGeneration { generation_id } => {
                let index_execution = self.execute_entity_field_equals_from_generation(&plan, generation_id)?;
                match parity_mode {
                    FallbackParityMode::ProductionAdmissibility => {
                        self.runtime.performance_access().count_query_index_path();
                        self.runtime
                            .performance_access()
                            .count_query_packet_shape(1, plan.packet.target_count_hint.max(1));
                        self.runtime
                            .performance_access()
                            .count_query_serial_strategy();
                        self.runtime.performance_access().count_query_emissions(
                            index_execution.result.entities.len(),
                            index_execution.result.relations.len(),
                        );
                        (index_execution, QueryAccessPath::DerivedIndexGeneration { generation_id })
                    }
                    FallbackParityMode::SampledParity | FallbackParityMode::CertificationParity => {
                        self.runtime
                            .performance_access()
                            .count_query_index_parity_verification();
                        let storage_execution = storage_execution()?;
                        if storage_execution.result == index_execution.result {
                            self.runtime.performance_access().count_query_index_path();
                            self.runtime
                                .performance_access()
                                .count_query_packet_shape(1, plan.packet.target_count_hint.max(1));
                            self.runtime
                                .performance_access()
                                .count_query_serial_strategy();
                            self.runtime.performance_access().count_query_emissions(
                                index_execution.result.entities.len(),
                                index_execution.result.relations.len(),
                            );
                            (index_execution, QueryAccessPath::DerivedIndexGeneration { generation_id })
                        } else {
                            self.runtime.performance_access().count_query_index_rejection();
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
                if matches!(access_path, QueryAccessPath::DerivedIndexRejectedStorageFallback { .. }) {
                    self.runtime.performance_access().count_query_index_rejection();
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
        let Some(generation) = self.candidate_generation_for_packet(&plan.packet, &branch_id) else {
            return QueryAccessPath::DerivedIndexRejectedStorageFallback {
                rejection: if self.matching_index_definition_exists(&plan.packet) {
                    IndexQueryRejectionClass::MissingGeneration
                } else if self
                    .runtime
                    .indexes
                    .generations
                    .values()
                    .flat_map(|generations| generations.iter())
                    .any(|generation| generation.compatibility.version_id <= plan.snapshot.version_id)
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
        if !matches!(packet.ordering, crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder)
            && !matches!(
                packet.ordering,
                crate::query::data::QueryOrderingContract::CanonicalEntityIdOrder
            )
        {
            return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
        }
        match (&packet.scope, &generation.payload, self.runtime.indexes.definitions.get(&generation.index_id)) {
            (
                QueryScope::EntityPayloadFieldEquals { field, .. },
                DerivedIndexPayload::EntityField(_),
                Some(definition),
            ) => match &definition.kind {
                DerivedIndexKind::EntityPayloadField { field: indexed_field } if indexed_field == field => None,
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
            QueryScope::EntityPayloadFieldEquals { field, .. } => self
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
            _ => None,
        }
    }

    fn matching_index_definition_exists(&self, packet: &PlannedQueryPacket) -> bool {
        match &packet.scope {
            QueryScope::EntityPayloadFieldEquals { field, .. } => self
                .runtime
                .indexes
                .definitions
                .values()
                .any(|definition| {
                    matches!(
                        &definition.kind,
                        DerivedIndexKind::EntityPayloadField { field: indexed_field }
                            if indexed_field == field
                    )
                }),
            _ => false,
        }
    }

    fn execute_entity_field_equals_from_generation(
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
        let QueryScope::EntityPayloadFieldEquals {
            field,
            value,
            partition_scope,
        } = &plan.packet.scope
        else {
            return None;
        };
        let DerivedIndexPayload::EntityField(payload) = &generation.payload else {
            return None;
        };
        let state = self.runtime.storage_access().current_state();
        let partition_scope = partition_scope
            .as_ref()
            .map(|partitions| partitions.iter().copied().collect::<std::collections::BTreeSet<_>>());
        let target_ids = payload
            .get(value)
            .into_iter()
            .flatten()
            .copied()
            .filter(|entity_id| {
                partition_scope
                    .as_ref()
                    .is_none_or(|partitions| partitions.contains(&entity_id.partition_id))
            })
            .filter(|entity_id| {
                self.runtime
                    .visibility_reads()
                    .entity_record_for_id_at_version(&state, *entity_id, plan.snapshot.version_id)
                    .is_some_and(|record| {
                        payload_field_key(&record.payload, field).as_deref() == Some(value.as_str())
                    })
            })
            .collect::<Vec<_>>();

        let entities = target_ids
            .iter()
            .filter_map(|entity_id| {
                self.runtime
                    .visibility_reads()
                    .entity_record_for_id_at_version(&state, *entity_id, plan.snapshot.version_id)
            })
            .collect::<Vec<_>>();
        let touched_partitions = entities
            .iter()
            .map(|record| record.entity_id.partition_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
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
                    target_count: target_ids.len(),
                    entity_records_emitted: target_ids.len(),
                    relation_records_emitted: 0,
                    touched_partitions,
                },
            }],
        );
        Some(crate::query::data::QueryExecutionOutcome {
            plan: plan.clone(),
            complexity: crate::query::data::QueryComplexitySummary {
                packet_count: 1,
                fragment_count: 1,
                touched_partitions,
                target_count: target_ids.len(),
                entity_records_emitted: target_ids.len(),
                relation_records_emitted: 0,
            },
            result,
        })
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
        let version_compatible = generation.compatibility.version_id <= packet.context_id.version_id;
        let schema_compatible =
            generation.compatibility.schema_version == packet.context_id.schema_version;
        let published =
            generation.status == crate::indexes::data::DerivedIndexPublicationStatus::Published;
        (published, branch_compatible, version_compatible, schema_compatible)
    }
}
