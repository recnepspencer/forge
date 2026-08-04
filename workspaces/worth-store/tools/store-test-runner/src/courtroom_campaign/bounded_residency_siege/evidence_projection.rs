use serde_json::{json, Value};

use super::{
    oracle::BoundedResidencyCourtroomEvidence,
    timing::{BoundedResidencySiegeTimings, TimedSiegePhase},
};
use crate::physical_work_evidence::{hex, mutant_value, run_environment_value, source_value};

mod allocation;
mod c7_crash_campaign;
mod campaign;
mod cancellation;
mod dirty_writeback;
mod generation_fencing;
mod json_object;
mod lifecycle;
mod performance;
mod process_allocation;
mod read_pressure;
mod schedule;
mod speculation;
mod work_reconciliation;

const SCHEMA: &str = "worth.store.physical-work-courtroom.bounded-residency-siege.v15";
const ARTIFACT_MANIFEST_STAGE: &str = "after-siege-writer-close-before-fresh-reopen";

pub(super) fn encode(
    evidence: &BoundedResidencyCourtroomEvidence,
    timings: &BoundedResidencySiegeTimings,
) -> Result<Vec<u8>, String> {
    let child = evidence.child();
    let producer = evidence.producer();
    let run = evidence.run();
    let processes = run.execution().processes();
    let mut encoded = json_object::JsonObjectEncoder::new();
    encoded.field("schema", SCHEMA)?;
    encoded.field("source", &source_value(evidence.source()))?;
    encoded.field("runner_binary", &source_value(evidence.runner()))?;
    encoded.field("writer_binary", &source_value(evidence.writer()))?;
    encoded.field("observer_binary", &source_value(evidence.observer()))?;
    encoded.field("environment", &run_environment_value(run.environment()))?;
    encoded.field("workload_seed", &evidence.workload_seed())?;
    encoded.field("schedule", &schedule::value(evidence))?;
    encoded.field(
        "c7_crash_campaign",
        &c7_crash_campaign::value(evidence.crash_campaign()),
    )?;
    encoded.field("configuration", &campaign::configuration())?;
    encoded.field(
        "timings",
        &timings.phases().iter().map(timing).collect::<Vec<_>>(),
    )?;
    encoded.field("processes", &campaign::processes(processes))?;
    encoded.field("producer", &campaign::producer(producer))?;
    encoded.field("world", &campaign::world(child))?;
    encoded.field(
        "process_allocation",
        &process_allocation::value(child.process_allocation),
    )?;
    encoded.field("reads", &read_pressure::reads(child.reads))?;
    encoded.field(
        "allocation",
        &allocation::AllocationProjection(&child.allocation),
    )?;
    encoded.field(
        "pins",
        &read_pressure::pins(child.pins, child.pinned_eviction),
    )?;
    encoded.field(
        "duplicate_fault",
        &read_pressure::duplicate_fault(child.duplicate),
    )?;
    encoded.field("cancellation", &cancellation::value(child.cancellation))?;
    encoded.field(
        "generation_fencing",
        &generation_fencing::value(child.generation_fencing),
    )?;
    encoded.field("dirty_writeback", &dirty_writeback::value(child.dirty))?;
    encoded.field("speculation", &speculation::value(child.speculation))?;
    encoded.field(
        "work_reconciliation",
        &work_reconciliation::WorkReconciliationProjection(&child.work_reconciliation),
    )?;
    encoded.field("close", &lifecycle::close(child.close))?;
    encoded.field("performance", &performance::value(evidence, timings)?)?;
    encoded.field(
        "offline_current",
        &lifecycle::current(evidence.offline().current()),
    )?;
    encoded.field("artifact_manifest_stage", ARTIFACT_MANIFEST_STAGE)?;
    encoded.field(
        "artifact_manifest",
        &evidence
            .offline()
            .artifacts()
            .iter()
            .map(artifact)
            .collect::<Vec<_>>(),
    )?;
    encoded.field("reopen", &lifecycle::reopen(evidence.reopen()))?;
    encoded.field(
        "oracle",
        &json!({
            "identity": evidence.oracle().oracle(),
            "accepted": evidence.oracle().accepted(),
            "sha256": hex(&evidence.oracle().digest().bytes()),
        }),
    )?;
    encoded.field(
        "mutants",
        &evidence
            .mutants()
            .iter()
            .map(mutant_value)
            .collect::<Vec<_>>(),
    )?;
    encoded.field(
        "verdict",
        &json!({
            "accepted": true,
            "findings": Vec::<String>::new(),
        }),
    )?;
    Ok(encoded.finish())
}

fn timing(phase: &TimedSiegePhase) -> Value {
    match (phase.case_count(), phase.source_workload()) {
        (None, Some(workload)) => json!({
            "name": phase.name(),
            "elapsed_ms": phase.elapsed_ms(),
            "source_files": workload.source_files(),
            "source_bytes": workload.source_bytes(),
        }),
        (Some(case_count), None) => json!({
            "name": phase.name(),
            "elapsed_ms": phase.elapsed_ms(),
            "case_count": case_count,
        }),
        (None, None) => json!({
            "name": phase.name(),
            "elapsed_ms": phase.elapsed_ms(),
        }),
        (Some(_), Some(_)) => unreachable!("timing phase cannot mix case and source workload"),
    }
}

pub(super) fn artifact(artifact: &super::offline_protocol::OfflineArtifactObservation) -> Value {
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
            "worth.store.physical-work-courtroom.bounded-residency-siege.v15"
        );
        assert_eq!(
            ARTIFACT_MANIFEST_STAGE,
            "after-siege-writer-close-before-fresh-reopen"
        );
    }

    #[test]
    fn c7_report_schema_requires_termination_process_projection() {
        let source = include_str!("evidence_projection/c7_crash_campaign.rs");
        let seam = "        \"processes\": process_values(campaign.processes()),";
        if source.matches(seam).count() != 1 {
            panic!("MUTANT_PREDICATE:c7-termination-process-report-omitted");
        }
    }
}
