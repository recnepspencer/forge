use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::facade::identity::VersionId;
use crate::facade::runtime::EntityReadRecord;
use crate::facade::snapshots::SnapshotHandle;
use crate::tests::support::read_entity_field;

use super::super::fixture::{FintechCaseRole, FintechWorld};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProbeStage {
    Baseline,
    PostMutation,
    PostReplay,
    PostRecovery,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CaseTruthProbe {
    pub(crate) case_role: FintechCaseRole,
    pub(crate) stage: ProbeStage,
    pub(crate) snapshot_id: u64,
    pub(crate) entity_count: usize,
    pub(crate) relation_count: usize,
    pub(crate) corrected_trade_count: usize,
    pub(crate) repaired_settlement_count: usize,
    pub(crate) open_breach_count: usize,
    pub(crate) audit_record_count: usize,
    pub(crate) payload_fingerprints: BTreeMap<String, Value>,
}

pub(crate) fn capture_case_truth_probe(
    world: &FintechWorld,
    case: FintechCaseRole,
    stage: ProbeStage,
) -> CaseTruthProbe {
    let snapshot = world.latest_snapshot();
    read_snapshot_probe(world, case, &snapshot, stage)
}

pub(crate) fn read_snapshot_probe(
    world: &FintechWorld,
    case: FintechCaseRole,
    snapshot: &SnapshotHandle,
    stage: ProbeStage,
) -> CaseTruthProbe {
    let packet = world.packet_for_case_probe(case, snapshot);
    let result = world.read_query(snapshot, packet);
    CaseTruthProbe {
        case_role: case,
        stage,
        snapshot_id: snapshot.snapshot_id.0,
        entity_count: result.entities.len(),
        relation_count: result.relations.len(),
        corrected_trade_count: result
            .entities
            .iter()
            .filter(|entity| field_is(entity, "corrected", "true"))
            .count(),
        repaired_settlement_count: result
            .entities
            .iter()
            .filter(|entity| {
                field_is(entity, "entity_type", "settlement")
                    && field_is(entity, "status", "repaired")
            })
            .count(),
        open_breach_count: result
            .entities
            .iter()
            .filter(|entity| {
                field_is(entity, "entity_type", "limit_breach")
                    && field_is(entity, "status", "open")
            })
            .count(),
        audit_record_count: result
            .entities
            .iter()
            .filter(|entity| field_is(entity, "entity_type", "audit_record"))
            .count(),
        payload_fingerprints: result
            .entities
            .iter()
            .enumerate()
            .map(|(idx, entity)| {
                (
                    format!("entity-{idx}"),
                    serde_json::to_value(&entity.authoritative_aspect_state)
                        .unwrap_or_else(|_| json!(null)),
                )
            })
            .collect(),
    }
}

pub(crate) fn read_version_probe(
    world: &FintechWorld,
    case: FintechCaseRole,
    version_id: VersionId,
    stage: ProbeStage,
) -> CaseTruthProbe {
    let packet = world.packet_for_case_probe(case, &world.latest_snapshot());
    let read = world.runtime.read_truth().read_version(version_id);
    let fragment = read
        .execute_planned_packet_fragment(
            crate::facade::query::DeterministicQueryPlanKey(0),
            crate::facade::query::QueryOrderingContract::CanonicalRecordRefOrder,
            packet
                .explicit_target_refs()
                .expect("case version probe uses explicit target packet"),
            0,
        )
        .expect("version probe packet fragment");
    let reduced = crate::query::data::reduce_query_fragments(
        crate::facade::query::QueryExecutionShape::BulkPacketized,
        crate::facade::query::QueryOrderingContract::CanonicalRecordRefOrder,
        vec![fragment],
    );
    let entities = reduced.entities;
    let relations = reduced.relations;
    CaseTruthProbe {
        case_role: case,
        stage,
        snapshot_id: 0,
        entity_count: entities.len(),
        relation_count: relations.len(),
        corrected_trade_count: entities
            .iter()
            .filter(|entity| field_is(entity, "corrected", "true"))
            .count(),
        repaired_settlement_count: entities
            .iter()
            .filter(|entity| {
                field_is(entity, "entity_type", "settlement")
                    && field_is(entity, "status", "repaired")
            })
            .count(),
        open_breach_count: entities
            .iter()
            .filter(|entity| {
                field_is(entity, "entity_type", "limit_breach")
                    && field_is(entity, "status", "open")
            })
            .count(),
        audit_record_count: entities
            .iter()
            .filter(|entity| field_is(entity, "entity_type", "audit_record"))
            .count(),
        payload_fingerprints: entities
            .iter()
            .enumerate()
            .map(|(idx, entity)| {
                (
                    format!("entity-{idx}"),
                    serde_json::to_value(&entity.authoritative_aspect_state)
                        .unwrap_or_else(|_| json!(null)),
                )
            })
            .collect(),
    }
}

fn field_is(entity: &EntityReadRecord, field: &str, expected: &str) -> bool {
    read_entity_field(entity, field) == Some(expected.to_string())
}
