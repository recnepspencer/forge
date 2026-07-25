use serde_json::{json, Value};
use worth_store::physical_runtime::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkHostileCurrentTruth,
};

use super::{
    oracle::C6PhysicalWorkCourtroomEvidence,
    timing::{C6SiegeTimings, TimedSiegePhase},
    world::{
        DIRTY_FRAMES, FRAME_ENTRIES, METADATA_BYTES, OPERATION_BYTES, PINNED_FRAMES, PIN_LEASES,
        RECORD_BYTES, RECORD_COUNT, RESIDENT_BYTES,
    },
};
use crate::physical_work_evidence::{
    hex, mutant_value, process_value, run_environment_value, source_value,
};

const SCHEMA: &str = "worth.store.c5_1.c6-inheritance-siege-courtroom.v6";

pub(super) fn encode(
    evidence: &C6PhysicalWorkCourtroomEvidence,
    timings: &C6SiegeTimings,
) -> Result<Vec<u8>, String> {
    let child = evidence.child();
    let run = evidence.run();
    let processes = run.execution().processes();
    let value = json!({
        "schema": SCHEMA,
        "source": source_value(evidence.source()),
        "runner_binary": source_value(evidence.runner()),
        "writer_binary": source_value(evidence.writer()),
        "observer_binary": source_value(evidence.observer()),
        "environment": run_environment_value(run.environment()),
        "seed": run.execution().seed(),
        "schedule": run.execution().schedule(),
        "configuration": {
            "record_bytes": RECORD_BYTES,
            "record_count": RECORD_COUNT,
            "resident_bytes": RESIDENT_BYTES,
            "metadata_bytes": METADATA_BYTES,
            "pinned_frames": PINNED_FRAMES,
            "pin_leases": PIN_LEASES,
            "dirty_frames": DIRTY_FRAMES,
            "operation_bytes": OPERATION_BYTES,
            "frame_entries": FRAME_ENTRIES,
        },
        "timings": timings.phases().iter().map(timing).collect::<Vec<_>>(),
        "processes": {
            "siege_writer": process_value(&processes[0]),
            "offline_observer": process_value(&processes[1]),
            "fresh_reopener": process_value(&processes[2]),
        },
        "world": {
            "store": hex(&child.store()),
            "runtime": child.runtime(),
            "generation": child.generation(),
            "records": child.records(),
            "payload_bytes": child.payload_bytes(),
            "directory_bytes": child.directory_bytes(),
            "resident_budget": child.resident_budget(),
        },
        "reads": {
            "cold_effects": child.reads.cold_effects,
            "hot_effects": child.reads.hot_effects,
            "refault_effects": child.reads.refault_effects,
            "physical_work": child.reads.physical_work,
            "peak_resident_bytes": child.reads.peak_resident_bytes,
            "peak_admitted_bytes": child.reads.peak_admitted_bytes,
            "faults": child.reads.faults,
            "hits": child.reads.hits,
            "evictions": child.reads.evictions,
        },
        "pins": {
            "cold_work": child.pins.cold_work,
            "hot_work": child.pins.hot_work,
            "refault_work": child.pins.refault_work,
            "peak_pinned_frames": child.pins.peak_pinned_frames,
            "peak_pin_leases": child.pins.peak_pin_leases,
            "over_pin_denial": "pin-lease-budget-exceeded",
        },
        "cancellation": {
            "open_physical_work": child.cancellation.physical_work,
            "first_open_operation": child.cancellation.first_operation,
            "last_open_operation": child.cancellation.last_operation,
            "open_work_handoff_bound": child.cancellation.handoff_bound,
            "unread_payload_bytes": child.cancellation.unread_payload_bytes,
            "open_media_effects": child.cancellation.open_media_effects,
            "cancellation_media_effects": child.cancellation.cancellation_media_effects,
        },
        "dirty_writeback": {
            "work_operation": child.dirty.work_operation,
            "source_work_count": child.dirty.source_work_count,
            "first_source_operation": child.dirty.first_source_operation,
            "last_source_operation": child.dirty.last_source_operation,
            "backend_operation": child.dirty.backend_operation,
            "effect_fate": "write-completed",
            "recovery": "continue-settlement",
            "signal": "committed",
            "dirty_at_pause": child.dirty.dirty_at_pause,
            "dirty_after_receipt": child.dirty.dirty_after_receipt,
            "positioned_writes": child.dirty.positioned_writes,
            "candidate_publications": child.dirty.candidate_publications,
            "writebacks": child.dirty.writebacks,
        },
        "close": {
            "inspection_required": child.close.inspection_required,
            "resident_bytes": child.close.resident_bytes,
            "pinned_frames": child.close.pinned_frames,
            "pin_leases": child.close.pin_leases,
            "dirty_frames": child.close.dirty_frames,
            "peak_resident_bytes": child.close.peak_resident_bytes,
            "peak_admitted_bytes": child.close.peak_admitted_bytes,
            "peak_dirty_frames": child.close.peak_dirty_frames,
        },
        "offline_current": current(evidence.offline().current()),
        "artifact_manifest": evidence.offline().artifacts().iter().map(artifact).collect::<Vec<_>>(),
        "reopen": reopen(evidence.reopen()),
        "oracle": {
            "identity": evidence.oracle().oracle(),
            "accepted": evidence.oracle().accepted(),
            "sha256": hex(&evidence.oracle().digest().bytes()),
        },
        "mutants": evidence.mutants().iter().map(mutant_value).collect::<Vec<_>>(),
        "verdict": {
            "accepted": true,
            "findings": Vec::<String>::new(),
        },
    });
    serde_json::to_vec_pretty(&value)
        .map_err(|error| format!("cannot encode Courtroom C evidence: {error}"))
}

fn timing(phase: &TimedSiegePhase) -> Value {
    json!({
        "name": phase.name(),
        "elapsed_ms": phase.elapsed_ms(),
    })
}

fn current(current: PhysicalWorkHostileCurrentTruth) -> Value {
    json!({
        "store": hex(&current.store()),
        "generation": current.generation(),
        "records": current.records(),
        "payload_bytes": current.payload_bytes(),
        "payload_sha256": hex(&current.payload_digest().bytes()),
    })
}

fn artifact(artifact: &super::offline_protocol::OfflineArtifactObservation) -> Value {
    json!({
        "path": artifact.path(),
        "byte_length": artifact.byte_length(),
        "sha256": hex(&artifact.digest()),
        "prefix": hex(artifact.prefix()),
        "recovery_obligation": artifact.is_recovery_obligation(),
    })
}

fn reopen(reopen: PhysicalWorkFreshReopenEvidence) -> Value {
    let identity = reopen.identity();
    let posture = reopen.posture();
    json!({
        "process": identity.process().get(),
        "store": hex(&identity.store()),
        "runtime": identity.runtime(),
        "generation": identity.generation(),
        "records": identity.records(),
        "residue": posture.residue(),
        "recovery_evidence_damaged": posture.recovery_evidence_damaged(),
        "recovery_obligations": posture.recovery_obligations(),
        "inspection_required": posture.inspection_required(),
    })
}

#[cfg(test)]
mod tests {
    use super::SCHEMA;

    #[test]
    fn schema_is_versioned_and_courtroom_specific() {
        assert_eq!(SCHEMA, "worth.store.c5_1.c6-inheritance-siege-courtroom.v6");
    }
}
