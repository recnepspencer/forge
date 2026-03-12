use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::facade::{RecordPayload, SnapshotHandle};

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
    let packet = world.packet_for_case_probe(case);
    let result = world
        .runtime
        .visibility_reads().execute_read_packet(snapshot, &packet)
        .expect("snapshot probe should be readable");
    CaseTruthProbe {
        case_role: case,
        stage,
        snapshot_id: snapshot.snapshot_id.0,
        entity_count: result.entities.len(),
        relation_count: result.relations.len(),
        corrected_trade_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_has(&entity.payload, "corrected", |value| {
                    value.as_bool() == Some(true)
                })
            })
            .count(),
        repaired_settlement_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_type_is(&entity.payload, "settlement")
                    && payload_has(&entity.payload, "status", |value| {
                        value.as_str() == Some("repaired")
                    })
            })
            .count(),
        open_breach_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_type_is(&entity.payload, "limit_breach")
                    && payload_has(&entity.payload, "status", |value| {
                        value.as_str() == Some("open")
                    })
            })
            .count(),
        audit_record_count: result
            .entities
            .iter()
            .filter(|entity| payload_type_is(&entity.payload, "audit_record"))
            .count(),
        payload_fingerprints: result
            .entities
            .iter()
            .enumerate()
            .map(|(idx, entity)| {
                (
                    format!("entity-{idx}"),
                    serde_json::to_value(&entity.payload).unwrap_or_else(|_| json!(null)),
                )
            })
            .collect(),
    }
}

pub(crate) fn read_version_probe(
    world: &FintechWorld,
    case: FintechCaseRole,
    version_id: crate::facade::VersionId,
    stage: ProbeStage,
) -> CaseTruthProbe {
    let packet = world.packet_for_case_probe(case);
    let read = world.runtime.visibility_reads().read_version(version_id);
    let result = read.execute_packet(&packet);
    CaseTruthProbe {
        case_role: case,
        stage,
        snapshot_id: 0,
        entity_count: result.entities.len(),
        relation_count: result.relations.len(),
        corrected_trade_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_has(&entity.payload, "corrected", |value| {
                    value.as_bool() == Some(true)
                })
            })
            .count(),
        repaired_settlement_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_type_is(&entity.payload, "settlement")
                    && payload_has(&entity.payload, "status", |value| {
                        value.as_str() == Some("repaired")
                    })
            })
            .count(),
        open_breach_count: result
            .entities
            .iter()
            .filter(|entity| {
                payload_type_is(&entity.payload, "limit_breach")
                    && payload_has(&entity.payload, "status", |value| {
                        value.as_str() == Some("open")
                    })
            })
            .count(),
        audit_record_count: result
            .entities
            .iter()
            .filter(|entity| payload_type_is(&entity.payload, "audit_record"))
            .count(),
        payload_fingerprints: result
            .entities
            .iter()
            .enumerate()
            .map(|(idx, entity)| {
                (
                    format!("entity-{idx}"),
                    serde_json::to_value(&entity.payload).unwrap_or_else(|_| json!(null)),
                )
            })
            .collect(),
    }
}

fn payload_has(payload: &RecordPayload, key: &str, predicate: impl Fn(&Value) -> bool) -> bool {
    match payload {
        RecordPayload::StructuredJson(value) => value.get(key).is_some_and(predicate),
        _ => false,
    }
}

fn payload_type_is(payload: &RecordPayload, expected: &str) -> bool {
    payload_has(payload, "entity_type", |value| {
        value.as_str() == Some(expected)
    })
}
