use serde_json::{json, Value};

use super::super::{
    oracle::BoundedResidencyCourtroomEvidence,
    protocol::BoundedResidencyPerformanceClaim,
    timing::{BoundedResidencySiegePhase, BoundedResidencySiegeTimings},
    world::SERVING_APPEND_RECORDS,
};

pub(super) fn value(
    evidence: &BoundedResidencyCourtroomEvidence,
    timings: &BoundedResidencySiegeTimings,
) -> Result<Value, String> {
    let child = evidence.child();
    let environment = evidence.run().environment();
    let platform = environment.platform();
    let filesystem = environment.filesystem();
    let serving_ms = timings.elapsed_ms(BoundedResidencySiegePhase::SiegeServing)?;
    let profile = child
        .performance
        .first()
        .ok_or_else(|| "Courtroom C performance evidence omitted its profile".to_owned())?
        .profile();
    let queue_peak_members = counter(
        evidence,
        BoundedResidencyPerformanceClaim::GroupCommitAmplification,
        "store.durability.group_queue.peak_members",
    )?;
    let queue_member_limit = counter(
        evidence,
        BoundedResidencyPerformanceClaim::GroupCommitAmplification,
        "store.durability.group_queue.member_limit",
    )?;
    Ok(json!({
        "authority_posture": "one-way-counter-backed-evidence",
        "receipts": child.performance.iter().map(|receipt| json!({
            "claim": receipt.claim().label(),
            "backend_profile": receipt.profile().label(),
            "boundary": "authoritative-execution",
            "evidence_strength": "counter-backed-execution-receipt",
            "counters": receipt.counters().iter().map(|counter| json!({
                "name": counter.name(),
                "observed_count": counter.observed_count(),
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "qualification": {
            "name": "worth-store-c7-current-host-structural-v1",
            "hardware": {
                "identity_basis": "current-process-platform-observation",
                "operating_system": platform.operating_system(),
                "architecture": platform.architecture(),
                "family": platform.family(),
                "pointer_width": platform.pointer_width(),
                "endian": platform.endian(),
            },
            "filesystem": {
                "root_identity": crate::physical_work_evidence::hex(&filesystem.root_identity()),
                "volume_identity": crate::physical_work_evidence::hex(&filesystem.volume_identity()),
                "filesystem_type": filesystem.filesystem_type(),
                "allocation_granularity": filesystem.allocation_granularity().get(),
                "location": filesystem.location().label(),
                "removable": filesystem.is_removable(),
                "read_only": filesystem.is_read_only(),
            },
            "backend_profile": profile.label(),
            "scale": {
                "tier": "bounded-residency-default",
                "records": child.records(),
                "payload_bytes": child.payload_bytes(),
                "directory_bytes": child.directory_bytes(),
                "resident_budget_bytes": child.resident_budget(),
                "payload_to_resident_ratio_numerator": child.payload_bytes(),
                "payload_to_resident_ratio_denominator": child.resident_budget(),
            },
            "cold_warm_posture": {
                "name": "cold-open-with-cold-hot-and-refault-observations",
                "cold_effects": child.reads.cold_effects,
                "hot_effects": child.reads.hot_effects,
                "refault_effects": child.reads.refault_effects,
            },
            "arrival_model": {
                "name": "deterministic-closed-loop-stage-sequence",
                "external_arrival_rate": "not-applicable",
            },
            "burst_model": {
                "name": "bounded-scripted-serving-burst",
                "serving_append_records": SERVING_APPEND_RECORDS,
            },
            "queue_utilization": {
                "measure": "peak-group-queue-members-over-admitted-member-limit",
                "peak_members": queue_peak_members,
                "member_limit": queue_member_limit,
            },
            "repetitions": 1,
            "latency_percentiles": {
                "sample_count": 1,
                "metric": "serving-process-wall-time-ms",
                "p50_ms": serving_ms,
                "p95_ms": serving_ms,
                "p99_ms": serving_ms,
            },
        },
    }))
}

fn counter(
    evidence: &BoundedResidencyCourtroomEvidence,
    claim: BoundedResidencyPerformanceClaim,
    name: &str,
) -> Result<u64, String> {
    evidence
        .child()
        .performance
        .iter()
        .find(|receipt| receipt.claim() == claim)
        .and_then(|receipt| {
            receipt
                .counters()
                .iter()
                .find(|counter| counter.name() == name)
        })
        .map(|counter| counter.observed_count())
        .ok_or_else(|| format!("Courtroom C performance qualification omitted `{name}`"))
}
