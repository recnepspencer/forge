use serde_json::json;

use crate::config::data::PatchSurfacePolicy;
use crate::identity::data::{EntityId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::publication::data::diff::{PatchDetail, PatchRecordKind};

pub(super) fn patch_detail_for_entity(
    patch_surface_policy: PatchSurfacePolicy,
    kind: PatchRecordKind,
    entity_id: EntityId,
    payload: Option<&RecordPayload>,
) -> PatchDetail {
    match patch_surface_policy {
        PatchSurfacePolicy::StructuredPatchSurface => match payload {
            Some(payload) => PatchDetail::Payload(payload.clone()),
            None => PatchDetail::StructuredJson(json!({})),
        },
        PatchSurfacePolicy::DensePatchSurface => PatchDetail::DenseBitset(vec![
            entity_patch_kind_code(kind),
            entity_id.partition_id.0 as u64,
            entity_id.local_slot.0,
            entity_id.generation.0 as u64,
            payload.map(payload_class_code).unwrap_or(0),
        ]),
    }
}

pub(super) fn patch_detail_for_relation(
    patch_surface_policy: PatchSurfacePolicy,
    kind: PatchRecordKind,
    relation_id: RelationId,
    source: EntityId,
    target: EntityId,
    payload: Option<&RecordPayload>,
) -> PatchDetail {
    match patch_surface_policy {
        PatchSurfacePolicy::StructuredPatchSurface => match payload {
            Some(payload) => PatchDetail::Payload(payload.clone()),
            None => PatchDetail::StructuredJson(json!({"payload_class":"topology_only"})),
        },
        PatchSurfacePolicy::DensePatchSurface => PatchDetail::DenseBitset(vec![
            relation_patch_kind_code(kind),
            relation_id.partition_id.0 as u64,
            relation_id.local_slot.0,
            relation_id.generation.0 as u64,
            source.partition_id.0 as u64,
            source.local_slot.0,
            target.partition_id.0 as u64,
            target.local_slot.0,
            payload.map(payload_class_code).unwrap_or(0),
        ]),
    }
}

fn entity_patch_kind_code(kind: PatchRecordKind) -> u64 {
    match kind {
        PatchRecordKind::Created => 1,
        PatchRecordKind::Updated => 2,
        PatchRecordKind::Deleted => 3,
        PatchRecordKind::RetainedForAudit => {
            unreachable!("entities do not emit retained-for-audit patch records")
        }
    }
}

fn relation_patch_kind_code(kind: PatchRecordKind) -> u64 {
    match kind {
        PatchRecordKind::Created => 4,
        PatchRecordKind::Updated => unreachable!("relations do not emit update patch records"),
        PatchRecordKind::Deleted => 5,
        PatchRecordKind::RetainedForAudit => 6,
    }
}

fn payload_class_code(payload: &RecordPayload) -> u64 {
    match payload {
        RecordPayload::StructuredJson(_) => 1,
        RecordPayload::OpaqueBytes(_) => 2,
    }
}
