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
use crate::authority::mutation::aspect_versions::write_relation_aspect_versions;
use crate::authority::mutation::outcomes::{MutationOutcome, RecordMutation};
use crate::authority::mutation::record_changes::{
    allocate_relation, reserve_bulk_relation_capacity,
};
use crate::authority::mutation::MutationWorkspace;
use crate::symbols::data::InternedString;
use crate::transactions::data::{BulkRelationCreateIntent, CommitConflict, RelationSpec};
use crate::validation::data::InvariantGroupSet;

pub(super) fn apply(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Result<MutationOutcome, CommitConflict> {
    let version_id = workspace.version_id();
    let mut outcome = MutationOutcome::bulk_relations_created(intent.partition_id, intent.kind_id);
    let staged_rows = stage_bulk_relation_rows(intent, workspace);
    workspace.with_context(|context| {
        reserve_bulk_relation_capacity(context.state, intent.partition_id, intent.endpoints.len());
    });
    for row in staged_rows {
        let ImportStagedRow::Relation {
            source,
            target,
            payload,
        } = row
        else {
            unreachable!("relation staging only emits relation rows");
        };
        let spec = RelationSpec {
            partition_id: intent.partition_id,
            kind_id: intent.kind_id,
            client_key: InternedString::from("bulk"),
            source,
            target,
            payload,
        };
        let relation_id = workspace.with_context(|context| {
            let relation_id = allocate_relation(context.state, version_id, &spec);
            context.state.mark_relation_slot_touched(
                relation_id.partition_id,
                relation_id.local_slot.0 as usize,
            );
            write_relation_aspect_versions(
                context.state,
                relation_id,
                version_id,
                spec.payload.as_ref(),
                context.symbols,
            );
            relation_id
        });
        outcome.record_change(RecordMutation::RelationCreated {
            relation_id,
            kind_id: intent.kind_id,
            source: spec.source,
            target: spec.target,
            payload: spec.payload.clone(),
        });
    }
    outcome.set_last_event_count(intent.endpoints.len());
    Ok(outcome)
}

fn stage_bulk_relation_rows(
    intent: &BulkRelationCreateIntent,
    workspace: &mut MutationWorkspace<'_>,
) -> Vec<ImportStagedRow> {
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
                    partition_scope: PreparationPartitionScope::TouchedPartitions(vec![
                        intent.partition_id,
                    ]),
                    invariant_group_scope: InvariantGroupSet::empty(),
                    read_set_approximation: PreparationReadSetApproximation::TouchedOnly,
                    write_exclusion: PreparationWriteExclusionClass::PublicationExcluded,
                },
            },
            rows: endpoints
                .iter()
                .copied()
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

    stage_import_packets(workspace, packets)
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
