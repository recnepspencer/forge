use rayon::prelude::*;

use crate::authority::commit::preparation::packets::import::{
    ImportFragmentIdentity, ImportFragmentKind, ImportStagedRow, ImportStagingHeader,
    ImportStagingPacket,
};
use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, strategy_for_parallel_packets, PreparationStrategySelection,
    TARGET_PREPARATION_ITEMS_PER_PACKET,
};
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationLocalityProof, PreparationPartitionScope, PreparationReadSetApproximation,
    PreparationRecordDomain, PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::reduction::keys::ImportReductionKey;
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::authority::mutation::aspect_versions::write_entity_aspect_versions;
use crate::authority::mutation::outcomes::{MutationOutcome, RecordMutation};
use crate::authority::mutation::record_changes::{allocate_entity, reserve_bulk_entity_capacity};
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{BulkEntityCreateIntent, CommitConflict};
use crate::validation::data::InvariantGroupSet;

pub(super) fn apply(
    intent: &BulkEntityCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::bulk_entities_created(intent.partition_id, intent.kind_id);
    let staged_rows = stage_bulk_entity_rows(intent, workspace);
    workspace.with_context(|context| {
        reserve_bulk_entity_capacity(context.state, intent.partition_id, intent.payloads.len());
    });
    for payload in staged_rows {
        let entity_id = workspace.with_context(|context| {
            let entity_id = allocate_entity(
                context.state,
                version_id,
                intent.partition_id,
                intent.kind_id,
                payload.clone(),
            );
            context
                .state
                .mark_entity_slot_touched(entity_id.partition_id, entity_id.local_slot.0 as usize);
            write_entity_aspect_versions(
                context.state,
                entity_id,
                version_id,
                &payload,
                context.symbols,
            );
            entity_id
        });
        outcome.record_change(RecordMutation::EntityCreated {
            entity_id,
            payload: payload.clone(),
        });
    }
    outcome.set_last_event_count(intent.payloads.len());
    Ok(outcome)
}

fn stage_bulk_entity_rows(
    intent: &BulkEntityCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Vec<crate::payloads::data::RecordPayload> {
    let packet_count =
        coarse_preparation_packet_count(intent.payloads.len(), TARGET_PREPARATION_ITEMS_PER_PACKET);
    let mut packets = Vec::with_capacity(packet_count);
    for (packet_index, payloads) in intent
        .payloads
        .chunks(TARGET_PREPARATION_ITEMS_PER_PACKET)
        .enumerate()
    {
        let packet_index_floor = packet_index * TARGET_PREPARATION_ITEMS_PER_PACKET;
        packets.push(ImportStagingPacket {
            header: ImportStagingHeader {
                packet_index_floor,
                identity: ImportFragmentIdentity {
                    partition_id: intent.partition_id,
                    kind_id: intent.kind_id,
                    fragment_kind: ImportFragmentKind::EntityCreate,
                    packet_index: packet_index_floor,
                },
                proof_kind: PreparationProofKind::FragmentIdentityDisjoint,
                locality: PreparationLocalityProof {
                    observation_scope:
                        crate::validation::engine::InvariantObservationKind::Speculative,
                    record_domain: PreparationRecordDomain::Entity,
                    partition_scope: PreparationPartitionScope::TouchedPartitions(vec![
                        intent.partition_id,
                    ]),
                    invariant_group_scope: InvariantGroupSet::empty(),
                    read_set_approximation: PreparationReadSetApproximation::TouchedOnly,
                    write_exclusion: PreparationWriteExclusionClass::PublicationExcluded,
                },
            },
            rows: payloads
                .iter()
                .cloned()
                .map(|payload| ImportStagedRow::Entity { payload })
                .collect(),
        });
    }

    stage_import_packets(workspace, packets)
        .into_iter()
        .map(|row| match row {
            ImportStagedRow::Entity { payload } => payload,
            ImportStagedRow::Relation { .. } => {
                unreachable!("entity staging only emits entity rows")
            }
        })
        .collect()
}

fn stage_import_packets(
    workspace: &mut MutationWorkspace<'_>,
    packets: Vec<ImportStagingPacket>,
) -> Vec<ImportStagedRow> {
    if packets.is_empty() {
        return Vec::new();
    }

    let packet_item_count = packets.iter().map(|packet| packet.rows.len()).sum();
    let packet_max_width = packets
        .iter()
        .map(|packet| packet.rows.len())
        .max()
        .unwrap_or(0);
    let strategy = record_import_strategy(workspace, packets.len());
    workspace.record_preparation_strategy(
        packets.len(),
        packet_item_count,
        packet_max_width,
        1,
        strategy,
    );
    let staged_streams = match strategy.selected_mode {
        PreparationStrategySelection::StagedParallel => packets
            .par_iter()
            .map(import_packet_stream)
            .collect::<Vec<_>>(),
        PreparationStrategySelection::Serial => packets
            .into_iter()
            .map(|packet| import_packet_stream(&packet))
            .collect::<Vec<_>>(),
    };
    canonical_merge_streams(staged_streams)
        .into_iter()
        .map(|(_, row)| row)
        .collect()
}

fn record_import_strategy(
    workspace: &MutationWorkspace<'_>,
    packet_count: usize,
) -> crate::authority::commit::preparation::planning::strategy::PreparationStrategy {
    strategy_for_parallel_packets(workspace.execution_model(), packet_count)
}

fn import_packet_stream(
    packet: &ImportStagingPacket,
) -> OrderedReductionStream<ImportReductionKey, ImportStagedRow> {
    let mut stream = Vec::with_capacity(packet.rows.len());
    for (offset, row) in packet.rows.iter().cloned().enumerate() {
        stream.push((
            ImportReductionKey::new(
                packet.header.identity.partition_id,
                0,
                packet.header.packet_index_floor + offset,
            ),
            row,
        ));
    }
    OrderedReductionStream::new(stream)
}
