use crate::authority::commit::preparation::packets::index::{
    IndexFragmentIdentity, IndexPreparationHeader, IndexPreparationPacket,
};
use crate::authority::commit::preparation::planning::strategy::{
    strategy_for_parallel_packets, PreparationStrategy, SerialPreparationReason,
};
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationLocalityProof, PreparationPartitionScope, PreparationReadSetApproximation,
    PreparationRecordDomain, PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::reduction::keys::IndexReductionKey;
use crate::config::data::RelationalExecutionModel;
use crate::indexes::data::{DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind};
use crate::runtime::RelationalRuntime;
use crate::validation::data::InvariantGroupSet;

pub(super) fn planned_index_definitions(
    runtime: &RelationalRuntime,
    index_ids: &[DerivedIndexId],
) -> (Vec<DerivedIndexDefinition>, Vec<DerivedIndexId>) {
    let mut definitions = Vec::new();
    let mut missing_indexes = Vec::new();

    for index_id in index_ids {
        if let Some(definition) = runtime.indexes.definition(*index_id) {
            definitions.push(definition.as_ref().clone());
        } else {
            missing_indexes.push(*index_id);
        }
    }

    (definitions, missing_indexes)
}

pub(super) fn choose_index_preparation_strategy(
    runtime: &RelationalRuntime,
    packet_count: usize,
) -> PreparationStrategy {
    if !matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::ParallelPreparation
    ) {
        return PreparationStrategy::serial(SerialPreparationReason::ExecutionModelSerial);
    }

    strategy_for_parallel_packets(runtime.config.execution.execution_model, packet_count)
}

pub(super) fn plan_index_packets(
    definitions: &[DerivedIndexDefinition],
) -> Vec<IndexPreparationPacket> {
    definitions
        .iter()
        .cloned()
        .enumerate()
        .map(|(packet_index, definition)| {
            let record_domain = match definition.kind {
                DerivedIndexKind::EntityField { .. } => PreparationRecordDomain::Entity,
                DerivedIndexKind::RelationField { .. } => PreparationRecordDomain::Relation,
                DerivedIndexKind::RelatedEntityOrdering { .. } => PreparationRecordDomain::Mixed,
                DerivedIndexKind::RelationJoin(_) => PreparationRecordDomain::Mixed,
            };
            IndexPreparationPacket {
                header: IndexPreparationHeader {
                    packet_index,
                    identity: IndexFragmentIdentity {
                        index_id: definition.index_id,
                        packet_index,
                    },
                    reduction_key: IndexReductionKey::new(definition.index_id, packet_index),
                    proof_kind: PreparationProofKind::ReadOnlyShared,
                    locality: PreparationLocalityProof {
                        observation_scope:
                            crate::validation::engine::InvariantObservationKind::Committed,
                        record_domain,
                        partition_scope: PreparationPartitionScope::AllObserved,
                        invariant_group_scope: InvariantGroupSet::empty(),
                        read_set_approximation: PreparationReadSetApproximation::FullObservedScan,
                        write_exclusion: PreparationWriteExclusionClass::PublicationExcluded,
                    },
                },
                definition,
            }
        })
        .collect()
}
