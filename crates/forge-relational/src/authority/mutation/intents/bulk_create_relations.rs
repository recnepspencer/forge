use rayon::prelude::*;

use crate::authority::commit::preparation::packets::import::{
    ImportFragmentIdentity, ImportFragmentKind, ImportStagedRow, ImportStagingHeader,
    ImportStagingPacket,
};
use crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection;
use crate::authority::commit::preparation::planning::strategy::{
    coarse_preparation_packet_count, strategy_for_parallel_packets,
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
use crate::authority::mutation::record_changes::{
    allocate_relation, reserve_bulk_relation_capacity,
};
use crate::authority::mutation::MutationWorkspace;
use crate::transactions::data::{
    BulkRelationCreateIntent, CommitConflict, ConflictClass, EntityReference,
};
use crate::validation::data::InvariantGroupSet;

pub(super) fn apply(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::with_capacity(intent.endpoints.len(), 1);
    outcome.record_event(
        crate::authority::mutation::outcomes::MutationEvent::BulkRelationsCreated {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            count: 0,
        },
    );
    workspace.with_context(|context| {
        reserve_bulk_relation_capacity(context.state, intent.partition_id, intent.endpoints.len());
    });
    for_each_staged_bulk_relation_row(intent, workspace, &mut outcome, version_id)?;
    outcome.set_last_event_count(intent.endpoints.len());
    Ok(outcome)
}

fn for_each_staged_bulk_relation_row(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
    outcome: &mut MutationOutcome,
    version_id: crate::identity::data::VersionId,
) -> Result<(), CommitConflict> {
    if intent.endpoints.len() <= TARGET_PREPARATION_ITEMS_PER_PACKET {
        workspace.record_preparation_strategy(
            1,
            intent.endpoints.len(),
            intent.endpoints.len(),
            1,
            strategy_for_parallel_packets(workspace.execution_model(), 1),
        );
        for (offset, (source, target)) in intent.endpoints.iter().cloned().enumerate() {
            let payload = intent.payloads.get(offset).cloned().unwrap_or(None);
            apply_staged_relation_row(
                intent, workspace, outcome, version_id, source, target, payload,
            )?;
        }
        return Ok(());
    }

    let packet_count = coarse_preparation_packet_count(
        intent.endpoints.len(),
        TARGET_PREPARATION_ITEMS_PER_PACKET,
    );
    let mut packets = Vec::with_capacity(packet_count);
    for (packet_index, endpoints) in intent
        .endpoints
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
                    fragment_kind: ImportFragmentKind::RelationCreate,
                    packet_index: packet_index_floor,
                },
                proof_kind: PreparationProofKind::FragmentIdentityDisjoint,
                locality: PreparationLocalityProof {
                    observation_scope:
                        crate::validation::engine::InvariantObservationKind::Speculative,
                    record_domain: PreparationRecordDomain::Relation,
                    partition_scope: PreparationPartitionScope::TouchedPartitions(
                        vec![intent.partition_id].into(),
                    ),
                    invariant_group_scope: InvariantGroupSet::empty(),
                    read_set_approximation: PreparationReadSetApproximation::TouchedOnly,
                    write_exclusion: PreparationWriteExclusionClass::PublicationExcluded,
                },
            },
            rows: endpoints
                .iter()
                .cloned()
                .enumerate()
                .map(|(offset, (source, target))| ImportStagedRow::Relation {
                    source,
                    target,
                    payload: intent
                        .payloads
                        .get(packet_index_floor + offset)
                        .cloned()
                        .unwrap_or(None),
                })
                .collect(),
        });
    }

    for row in stage_import_packets(workspace, packets) {
        match row {
            ImportStagedRow::Relation {
                source,
                target,
                payload,
            } => apply_staged_relation_row(
                intent, workspace, outcome, version_id, source, target, payload,
            )?,
            ImportStagedRow::Entity { .. } => return Err(CommitConflict::new(
                crate::transactions::data::ConflictClass::MutationStateInconsistency {
                    detail:
                        "bulk relation staging produced an entity row in the relation import lane"
                            .to_string(),
                    fields: serde_json::json!({
                        "expected_row_domain": "relation",
                        "actual_row_domain": "entity",
                        "phase": "bulk_relation_stage_import",
                    }),
                },
            )),
        }
    }
    Ok(())
}

fn apply_staged_relation_row(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
    outcome: &mut MutationOutcome,
    version_id: crate::identity::data::VersionId,
    source: EntityReference,
    target: EntityReference,
    payload: Option<crate::payloads::data::RecordPayload>,
) -> Result<(), CommitConflict> {
    let source_id = resolve_entity_reference(workspace, &source)?;
    let target_id = resolve_entity_reference(workspace, &target)?;
    let relation_id = workspace.with_context(|context| {
        let relation_id = allocate_relation(
            context.state,
            version_id,
            intent.partition_id,
            intent.kind_id,
            source_id,
            target_id,
            payload.clone(),
        );
        context.state.mark_relation_slot_touched(
            relation_id.partition_id,
            relation_id.local_slot.0 as usize,
        );
        relation_id
    });
    outcome.record_change(RecordMutation::RelationCreated {
        relation_id,
        kind_id: intent.kind_id,
        source: source_id,
        target: target_id,
        payload: payload.clone(),
    });
    Ok(())
}

fn resolve_entity_reference(
    workspace: &MutationWorkspace<'_>,
    entity_reference: &EntityReference,
) -> Result<crate::identity::data::EntityId, CommitConflict> {
    workspace
        .resolve_entity_reference(entity_reference)
        .ok_or_else(|| CommitConflict::new(ConflictClass::InvalidRelationEndpoint {
            detail: "relation endpoints must resolve within the same authoritative commit scope"
                .to_string(),
        }))
}

fn stage_import_packets(
    workspace: &mut MutationWorkspace<'_>,
    packets: Vec<ImportStagingPacket>,
) -> Vec<ImportStagedRow> {
    if packets.is_empty() {
        return Vec::new();
    }

    let strategy = strategy_for_parallel_packets(workspace.execution_model(), packets.len());
    let packet_item_count = packets.iter().map(|packet| packet.rows.len()).sum();
    let packet_max_width = packets
        .iter()
        .map(|packet| packet.rows.len())
        .max()
        .unwrap_or(0);
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
        // Packets are emitted in canonical packet-index order and relation rows are already
        // ordered within each packet, so the serial path can flatten directly.
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

fn import_packet_stream(
    packet: &ImportStagingPacket,
) -> OrderedReductionStream<ImportReductionKey, ImportStagedRow> {
    let mut stream = Vec::with_capacity(packet.rows.len());
    for (offset, row) in packet.rows.iter().cloned().enumerate() {
        stream.push((
            ImportReductionKey::new(
                packet.header.identity.partition_id,
                1,
                packet.header.packet_index_floor + offset,
            ),
            row,
        ));
    }
    OrderedReductionStream::new(stream)
}
