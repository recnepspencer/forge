mod bounded_entity_field_lookup;
mod bounded_related_entity_ordered_lookup;
mod bounded_relation_join_lookup;
mod execution;
mod generation_selection;
mod routing;
mod scratch;

use crate::history::data::BranchId;
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexId};
use crate::query::data::{
    query_index_parity_basis_digest, IndexParityMode, IndexParityVerifiedQueryOutcome,
    QueryAccessContract, QueryAccessPath, SnapshotPinnedQueryPlan,
};
use crate::runtime::RelationalRuntime;

use self::execution::execute_index_backed_query_from_generation;
use self::routing::{admissible_access_path, should_verify_sampled_parity};
#[cfg(test)]
pub(crate) use self::scratch::index_query_scratch_hint_exists;
pub(crate) use self::scratch::purge_index_query_scratch_hints;

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
                    !definition.branch_scoped || generation.applicability.branch_id == *branch_id
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
            .filter(|generation| generation.applicability.version_id <= version_id)
            .cloned()
            .collect::<Vec<_>>();
        generations.sort_by(|left, right| {
            left.applicability
                .branch_id
                .cmp(&right.applicability.branch_id)
                .then_with(|| left.source_commit_id.cmp(&right.source_commit_id))
                .then_with(|| left.generation_id.cmp(&right.generation_id))
        });
        generations
    }

    pub fn execute_query_plan_with_index_parity(
        &self,
        plan: SnapshotPinnedQueryPlan,
        parity_mode: IndexParityMode,
    ) -> Option<IndexParityVerifiedQueryOutcome> {
        if plan.packet.access_contract == QueryAccessContract::DerivedIndexWithStorageParity {
            self.runtime
                .performance_access()
                .count_query_index_attempt();
        }
        let storage_execution = || self.runtime.read_truth().execute_query_plan(plan.clone());
        let (execution, access_path) = match admissible_access_path(self.runtime, &plan) {
            QueryAccessPath::DerivedIndexGeneration { generation_id } => {
                let index_execution =
                    execute_index_backed_query_from_generation(self.runtime, &plan, generation_id)?;
                match parity_mode {
                    IndexParityMode::ProductionAdmissibility => {
                        self.runtime.performance_access().count_query_index_path();
                        record_index_execution_counters(self.runtime, &index_execution);
                        (
                            index_execution,
                            QueryAccessPath::DerivedIndexGeneration { generation_id },
                        )
                    }
                    IndexParityMode::SampledParity
                        if !should_verify_sampled_parity(&plan, generation_id) =>
                    {
                        self.runtime.performance_access().count_query_index_path();
                        record_index_execution_counters(self.runtime, &index_execution);
                        (
                            index_execution,
                            QueryAccessPath::DerivedIndexGeneration { generation_id },
                        )
                    }
                    IndexParityMode::SampledParity | IndexParityMode::CertificationParity => {
                        self.runtime
                            .performance_access()
                            .count_query_index_parity_verification();
                        let storage_execution = storage_execution()?;
                        if storage_execution.result == index_execution.result {
                            self.runtime.performance_access().count_query_index_path();
                            record_index_execution_counters(self.runtime, &index_execution);
                            (
                                index_execution,
                                QueryAccessPath::DerivedIndexGeneration { generation_id },
                            )
                        } else {
                            self.runtime
                                .performance_access()
                                .count_query_index_rejection();
                            record_index_execution_shape(self.runtime, &index_execution);
                            (
                                storage_execution,
                                QueryAccessPath::DerivedIndexRejectedStorageRead {
                                    rejection:
                                        crate::query::data::IndexQueryRejectionClass::CorruptIndexEntries,
                                },
                            )
                        }
                    }
                }
            }
            access_path => {
                if matches!(
                    access_path,
                    QueryAccessPath::DerivedIndexRejectedStorageRead { .. }
                ) {
                    self.runtime
                        .performance_access()
                        .count_query_index_rejection();
                }
                (storage_execution()?, access_path)
            }
        };
        let parity_basis_digest = query_index_parity_basis_digest(
            &access_path,
            parity_mode,
            &execution.result,
            execution.plan.packet.plan_key,
        );
        Some(IndexParityVerifiedQueryOutcome {
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

    #[cfg(test)]
    pub(crate) fn entity_unique_field_entries(
        &self,
        field_locator: &worth_foundational::facade::AspectFieldLocator,
    ) -> Option<
        &std::collections::BTreeMap<
            crate::storage::data::AuthoritativeFieldComparisonKey,
            std::collections::BTreeSet<crate::identity::data::EntityId>,
        >,
    > {
        self.runtime
            .indexes
            .entity_unique_aspect_field_index
            .get(field_locator)
    }
}

fn record_index_execution_shape(
    runtime: &RelationalRuntime,
    execution: &crate::query::data::QueryExecutionOutcome,
) {
    let target_count = execution.complexity.target_count.max(1);
    let scope_units = execution.complexity.touched_partitions.max(1);
    runtime.performance_access().count_query_packet_shape(
        execution.complexity.packet_count.max(1),
        target_count,
        target_count,
        scope_units,
    );
    runtime.performance_access().count_query_serial_strategy();
}

fn record_index_execution_counters(
    runtime: &RelationalRuntime,
    execution: &crate::query::data::QueryExecutionOutcome,
) {
    record_index_execution_shape(runtime, execution);
    runtime.performance_access().count_query_emissions(
        execution.result.entities.len(),
        execution.result.relations.len(),
    );
}
