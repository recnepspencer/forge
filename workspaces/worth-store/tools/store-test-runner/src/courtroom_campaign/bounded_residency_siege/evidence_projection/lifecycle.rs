use serde_json::{json, Value};
use worth_store::physical_runtime::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkHostileCurrentTruth,
};

use super::super::protocol::BoundedResidencyCloseObservation;
use crate::physical_work_evidence::hex;

pub(super) fn close(close: BoundedResidencyCloseObservation) -> Value {
    json!({
        "inspection_required": close.inspection_required,
        "resident_bytes": close.resident_bytes,
        "pinned_frames": close.pinned_frames,
        "pin_leases": close.pin_leases,
        "dirty_frames": close.dirty_frames,
        "peak_resident_bytes": close.peak_resident_bytes,
        "peak_admitted_bytes": close.peak_admitted_bytes,
        "peak_dirty_frames": close.peak_dirty_frames,
    })
}

pub(super) fn current(current: PhysicalWorkHostileCurrentTruth) -> Value {
    json!({
        "store": hex(&current.store()),
        "generation": current.generation(),
        "records": current.records(),
        "payload_bytes": current.payload_bytes(),
        "payload_sha256": hex(&current.payload_digest().bytes()),
    })
}

pub(super) fn reopen(reopen: PhysicalWorkFreshReopenEvidence) -> Value {
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
