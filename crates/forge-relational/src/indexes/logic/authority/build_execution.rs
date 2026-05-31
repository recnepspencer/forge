use rayon::prelude::*;

use crate::authority::commit::preparation::packets::index::IndexPreparationPacket;
use crate::authority::commit::preparation::planning::strategy::{
    ParallelLegality, ParallelProfitability, PreparationStrategy, PreparationStrategySelection,
};
use crate::indexes::data::{DerivedIndexEntries, DerivedIndexId, DerivedIndexKind};
use crate::logic::runtime::{RelationalRuntime, VisibilityProjectionView};

use super::super::observed_field_indexes::{
    build_entity_aspect_field_index, build_relation_aspect_field_index,
};

#[derive(Debug, Clone)]
pub(super) struct IndexPreparationResult {
    pub(super) index_id: DerivedIndexId,
    pub(super) entries: Option<DerivedIndexEntries>,
}

pub(super) fn record_index_preparation_strategy_counters(
    runtime: &RelationalRuntime,
    definition_count: usize,
    strategy: &PreparationStrategy,
) {
    runtime.performance_access().count_preparation_packet_shape(
        definition_count,
        definition_count,
        usize::from(definition_count != 0),
        usize::from(definition_count != 0),
    );

    match strategy.parallel_legality {
        ParallelLegality::ProvenParallel => runtime
            .performance_access()
            .count_preparation_parallel_legal(),
        ParallelLegality::RequiresSerial => {}
    }
    match strategy.parallel_profitability {
        ParallelProfitability::Profitable => runtime
            .performance_access()
            .count_preparation_parallel_profitable(),
        ParallelProfitability::NotProfitable => {}
    }
    match strategy.selected_mode {
        PreparationStrategySelection::Serial => runtime
            .performance_access()
            .count_preparation_serial_strategy(),
        PreparationStrategySelection::StagedParallel => runtime
            .performance_access()
            .count_preparation_staged_parallel_strategy(),
    }
}

pub(super) fn execute_index_packets(
    projection: &VisibilityProjectionView<'_>,
    packets: &[IndexPreparationPacket],
    selected_mode: PreparationStrategySelection,
) -> Vec<IndexPreparationResult> {
    let entity_packets = packets
        .iter()
        .filter_map(entity_field_packet)
        .collect::<Vec<_>>();
    let relation_packets = packets
        .iter()
        .filter_map(relation_field_packet)
        .collect::<Vec<_>>();

    let entity_streams = match selected_mode {
        PreparationStrategySelection::StagedParallel => {
            let records = projection.all_unmasked_entity_records();
            entity_packets
                .par_iter()
                .map(|(reduction_key, index_id, field_locator)| {
                    singleton_result_stream(
                        reduction_key.clone(),
                        *index_id,
                        DerivedIndexEntries::EntityField(build_entity_aspect_field_index(
                            &records,
                            field_locator,
                        )),
                    )
                })
                .collect::<Vec<_>>()
        }
        PreparationStrategySelection::Serial => {
            let records = projection.all_unmasked_entity_records();
            entity_packets
                .iter()
                .map(|(reduction_key, index_id, field_locator)| {
                    singleton_result_stream(
                        reduction_key.clone(),
                        *index_id,
                        DerivedIndexEntries::EntityField(build_entity_aspect_field_index(
                            &records,
                            field_locator,
                        )),
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    let relation_streams = match selected_mode {
        PreparationStrategySelection::StagedParallel => {
            let records = projection.all_unmasked_relation_records();
            relation_packets
                .par_iter()
                .map(|(reduction_key, index_id, field_locator)| {
                    singleton_result_stream(
                        reduction_key.clone(),
                        *index_id,
                        DerivedIndexEntries::RelationField(build_relation_aspect_field_index(
                            &records,
                            field_locator,
                        )),
                    )
                })
                .collect::<Vec<_>>()
        }
        PreparationStrategySelection::Serial => {
            let records = projection.all_unmasked_relation_records();
            relation_packets
                .iter()
                .map(|(reduction_key, index_id, field_locator)| {
                    singleton_result_stream(
                        reduction_key.clone(),
                        *index_id,
                        DerivedIndexEntries::RelationField(build_relation_aspect_field_index(
                            &records,
                            field_locator,
                        )),
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    crate::authority::commit::preparation::reduction::merge::canonical_merge_streams(
        entity_streams
            .into_iter()
            .chain(relation_streams)
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .map(|(_, result)| result)
    .collect()
}

fn entity_field_packet(
    packet: &IndexPreparationPacket,
) -> Option<(
    crate::authority::commit::preparation::reduction::keys::IndexReductionKey,
    DerivedIndexId,
    forge_foundational::facade::AspectFieldLocator,
)> {
    match &packet.definition.kind {
        DerivedIndexKind::EntityField { field_locator } => Some((
            packet.header.reduction_key,
            packet.header.identity.index_id,
            field_locator.clone(),
        )),
        DerivedIndexKind::RelationField { .. } => None,
    }
}

fn relation_field_packet(
    packet: &IndexPreparationPacket,
) -> Option<(
    crate::authority::commit::preparation::reduction::keys::IndexReductionKey,
    DerivedIndexId,
    forge_foundational::facade::AspectFieldLocator,
)> {
    match &packet.definition.kind {
        DerivedIndexKind::EntityField { .. } => None,
        DerivedIndexKind::RelationField { field_locator } => Some((
            packet.header.reduction_key,
            packet.header.identity.index_id,
            field_locator.clone(),
        )),
    }
}

fn singleton_result_stream(
    reduction_key: crate::authority::commit::preparation::reduction::keys::IndexReductionKey,
    index_id: DerivedIndexId,
    entries: DerivedIndexEntries,
) -> crate::authority::commit::preparation::reduction::merge::OrderedReductionStream<
    crate::authority::commit::preparation::reduction::keys::IndexReductionKey,
    IndexPreparationResult,
> {
    crate::authority::commit::preparation::reduction::merge::OrderedReductionStream::singleton(
        reduction_key,
        IndexPreparationResult {
            index_id,
            entries: Some(entries),
        },
    )
}
