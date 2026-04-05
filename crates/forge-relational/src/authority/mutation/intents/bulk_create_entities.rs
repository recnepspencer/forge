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
    let mut outcome = MutationOutcome::with_capacity(intent.payloads.len(), 1);
    outcome.record_event(
        crate::authority::mutation::outcomes::MutationEvent::BulkEntitiesCreated {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            count: 0,
        },
    );
    let staged_rows = stage_bulk_entity_rows(intent, workspace)?;
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
            entity_id
        });
        outcome.record_change(RecordMutation::EntityCreated {
            entity_id,
            kind_id: intent.kind_id,
            payload: payload.clone(),
        });
    }
    outcome.set_last_event_count(intent.payloads.len());
    Ok(outcome)
}

fn stage_bulk_entity_rows(
    intent: &BulkEntityCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<Vec<crate::payloads::data::RecordPayload>, CommitConflict> {
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
                    partition_scope: PreparationPartitionScope::TouchedPartitions(
                        vec![intent.partition_id].into(),
                    ),
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
            ImportStagedRow::Entity { payload } => Ok(payload),
            ImportStagedRow::Relation { .. } => Err(CommitConflict::new(
                crate::transactions::data::ConflictClass::MutationStateInconsistency {
                    detail: "bulk entity staging produced a relation row in the entity import lane"
                        .to_string(),
                    fields: serde_json::json!({
                        "expected_row_domain": "entity",
                        "actual_row_domain": "relation",
                        "phase": "bulk_entity_stage_import",
                    }),
                },
            )),
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
    if packets.len() == 1 {
        return packets.into_iter().next().unwrap().rows;
    }
    match strategy.selected_mode {
        // Packets are constructed in canonical packet-index order and each packet row is already
        // locally ordered, so serial execution can flatten directly without a redundant k-way
        // merge.
        PreparationStrategySelection::Serial => {
            packets.into_iter().flat_map(|packet| packet.rows).collect()
        }
        PreparationStrategySelection::StagedParallel => canonical_merge_streams(
            packets
                .par_iter()
                .map(import_packet_stream)
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .map(|(_, row)| row)
        .collect(),
    }
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
