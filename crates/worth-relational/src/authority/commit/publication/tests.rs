use super::{diff_packet_stream, direct_diff_record_order};
use crate::authority::commit::preparation::packets::diff::{
    DiffPreparationHeader, DiffPreparationPacket,
};
use crate::authority::commit::preparation::reduction::merge::canonical_merge_streams;
use crate::identity::data::{EntityId, PartitionId};
use crate::publication::patch::data::{
    PatchDetail, PublishedAuthoritativeRecordPatch, RecordStructuralChange,
};
use crate::transactions::data::RecordRef;

#[test]
fn direct_serial_patch_preparation_matches_packet_merge_order() {
    let records = vec![
        patch_record(7, RecordStructuralChange::Updated),
        patch_record(3, RecordStructuralChange::Deleted),
        patch_record(3, RecordStructuralChange::Created),
        patch_record(8, RecordStructuralChange::RetainedForAudit),
        patch_record(7, RecordStructuralChange::Created),
        patch_record(3, RecordStructuralChange::Updated),
    ];

    let mut packets = Vec::new();
    for (packet_index, chunk) in records.chunks(2).enumerate() {
        packets.push(DiffPreparationPacket {
            header: DiffPreparationHeader {
                packet_index_floor: packet_index * 2,
            },
            authoritative_record_patches: chunk.to_vec(),
        });
    }

    let merged =
        canonical_merge_streams(packets.iter().map(diff_packet_stream).collect::<Vec<_>>())
            .into_iter()
            .map(|(_key, record)| record)
            .collect::<Vec<_>>();
    let direct = direct_diff_record_order(records);

    assert_eq!(direct, merged);
}

fn patch_record(
    raw_entity_id: u64,
    structural_change: RecordStructuralChange,
) -> PublishedAuthoritativeRecordPatch {
    let structural_change_label = format!("{structural_change:?}");
    PublishedAuthoritativeRecordPatch {
        target: RecordRef::Entity(EntityId::new(PartitionId(0), raw_entity_id, 0)),
        structural_change,
        authoritative_patch: crate::publication::patch::data::PublishedAuthoritativePatch::empty(),
        semantic_changes: Vec::new(),
        contains_opaque_aspect: false,
        detail: PatchDetail::DenseBitset(vec![raw_entity_id, structural_change_label.len() as u64]),
    }
}
