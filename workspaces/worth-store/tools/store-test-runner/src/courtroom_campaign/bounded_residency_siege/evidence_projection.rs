use serde_json::{json, Value};

use super::{
    oracle::BoundedResidencyCourtroomEvidence,
    timing::{BoundedResidencySiegeTimings, TimedSiegePhase},
};
use crate::physical_work_evidence::{hex, mutant_value, run_environment_value, source_value};

mod allocation;
mod campaign;
mod cancellation;
mod dirty_writeback;
mod generation_fencing;
mod lifecycle;
mod process_allocation;
mod read_pressure;
mod schedule;
mod speculation;
mod work_reconciliation;

const SCHEMA: &str = "worth.store.physical-work-courtroom.bounded-residency-siege.v10";
const ARTIFACT_MANIFEST_STAGE: &str = "after-siege-writer-close-before-fresh-reopen";

pub(super) fn encode(
    evidence: &BoundedResidencyCourtroomEvidence,
    timings: &BoundedResidencySiegeTimings,
) -> Result<Vec<u8>, String> {
    let child = evidence.child();
    let producer = evidence.producer();
    let run = evidence.run();
    let processes = run.execution().processes();
    let value = json!({
        "schema": SCHEMA,
        "source": source_value(evidence.source()),
        "runner_binary": source_value(evidence.runner()),
        "writer_binary": source_value(evidence.writer()),
        "observer_binary": source_value(evidence.observer()),
        "environment": run_environment_value(run.environment()),
        "workload_seed": evidence.workload_seed(),
        "schedule": schedule::value(evidence),
        "configuration": campaign::configuration(),
        "timings": timings.phases().iter().map(timing).collect::<Vec<_>>(),
        "processes": campaign::processes(processes),
        "producer": campaign::producer(producer),
        "world": campaign::world(child),
        "process_allocation": process_allocation::value(child.process_allocation),
        "reads": read_pressure::reads(child.reads),
        "allocation": allocation::value(&child.allocation),
        "pins": read_pressure::pins(child.pins, child.pinned_eviction),
        "duplicate_fault": read_pressure::duplicate_fault(child.duplicate),
        "cancellation": cancellation::value(child.cancellation),
        "generation_fencing": generation_fencing::value(child.generation_fencing),
        "dirty_writeback": dirty_writeback::value(child.dirty),
        "speculation": speculation::value(child.speculation),
        "work_reconciliation": work_reconciliation::value(&child.work_reconciliation),
        "close": lifecycle::close(child.close),
        "offline_current": lifecycle::current(evidence.offline().current()),
        "artifact_manifest_stage": ARTIFACT_MANIFEST_STAGE,
        "artifact_manifest": evidence.offline().artifacts().iter().map(artifact).collect::<Vec<_>>(),
        "reopen": lifecycle::reopen(evidence.reopen()),
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
    serde_json::to_vec(&value)
        .map_err(|error| format!("cannot encode Courtroom C evidence: {error}"))
}

fn timing(phase: &TimedSiegePhase) -> Value {
    json!({
        "name": phase.name(),
        "elapsed_ms": phase.elapsed_ms(),
    })
}

fn artifact(artifact: &super::offline_protocol::OfflineArtifactObservation) -> Value {
    json!({
        "path": artifact.path(),
        "byte_length": artifact.byte_length(),
        "sha256": hex(&artifact.digest()),
        "prefix": hex(artifact.prefix()),
        "recovery_obligation": artifact.is_recovery_obligation(),
        "content_stability": artifact.content_stability().evidence_label(),
    })
}

#[cfg(test)]
mod tests {
    use super::{ARTIFACT_MANIFEST_STAGE, SCHEMA};

    #[test]
    fn schema_is_versioned_and_courtroom_specific() {
        assert_eq!(
            SCHEMA,
            "worth.store.physical-work-courtroom.bounded-residency-siege.v10"
        );
        assert_eq!(
            ARTIFACT_MANIFEST_STAGE,
            "after-siege-writer-close-before-fresh-reopen"
        );
    }
}
