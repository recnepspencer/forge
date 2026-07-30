use serde_json::{json, Value};
use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

use super::super::{
    execution::BoundedResidencyProducerObservation,
    protocol::BoundedResidencySiegeObservation,
    world::{
        DIRTY_FRAMES, EXTENT_RECORDS, EXTENT_RECORD_BYTES, FRAME_ENTRIES, INLINE_RECORDS,
        INLINE_RECORD_BYTES, METADATA_BYTES, OPERATION_BYTES, PINNED_FRAMES, PIN_LEASES,
        RESIDENT_BYTES, TOTAL_BYTES,
    },
};
use crate::physical_work_evidence::{hex, process_value};

pub(super) fn configuration() -> Value {
    json!({
        "inline_record_bytes": INLINE_RECORD_BYTES,
        "inline_records": INLINE_RECORDS,
        "extent_record_bytes": EXTENT_RECORD_BYTES,
        "extent_records": EXTENT_RECORDS,
        "total_bytes": TOTAL_BYTES,
        "resident_bytes": RESIDENT_BYTES,
        "metadata_bytes": METADATA_BYTES,
        "pinned_frames": PINNED_FRAMES,
        "pin_leases": PIN_LEASES,
        "dirty_frames": DIRTY_FRAMES,
        "operation_bytes": OPERATION_BYTES,
        "frame_entries": FRAME_ENTRIES,
    })
}

pub(super) fn processes(processes: &[PhysicalWorkProcessEvidence]) -> Value {
    json!({
        "producer": process_value(&processes[0]),
        "serving": process_value(&processes[1]),
        "offline_verifier": process_value(&processes[2]),
        "fresh_reopener": process_value(&processes[3]),
    })
}

pub(super) fn producer(producer: BoundedResidencyProducerObservation) -> Value {
    json!({
        "process": producer.process.get(),
        "store": hex(&producer.store),
        "runtime": producer.runtime,
        "generation": producer.generation,
        "records": producer.records,
        "payload_bytes": producer.payload_bytes,
        "expectation_sha256": hex(&producer.expectation_digest),
        "peak_resident_bytes": producer.peak_resident_bytes,
    })
}

pub(super) fn world(child: &BoundedResidencySiegeObservation) -> Value {
    json!({
        "store": hex(&child.store()),
        "runtime": child.runtime(),
        "generation": child.generation(),
        "records": child.records(),
        "payload_bytes": child.payload_bytes(),
        "directory_bytes": child.directory_bytes(),
        "resident_budget": child.resident_budget(),
    })
}
