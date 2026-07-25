use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::{json, Value};
use worth_store::physical_runtime::{
    PhysicalWorkCausalEvidence, PhysicalWorkCourtroomEvidence, PhysicalWorkProcessEvidence,
};

use super::terminal_labels as labels;

const REPORT_ENV: &str = "WORTH_STORE_C5_1_COURTROOM_A_REPORT";
const SCHEMA: &str = "worth.store.c5_1.lifecycle-maelstrom-courtroom.v1";
static PENDING_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) fn publish_if_requested(evidence: &PhysicalWorkCourtroomEvidence) {
    let Some(report) = std::env::var_os(REPORT_ENV).map(PathBuf::from) else {
        return;
    };
    assert!(
        evidence.verdict().accepted(),
        "Courtroom A cannot publish rejected evidence: {:?}",
        evidence.verdict().findings()
    );
    let encoded = encode(evidence).expect("Courtroom A terminal projection must encode");
    publish(&report, &encoded).expect("Courtroom A terminal projection must publish");
}

fn encode(evidence: &PhysicalWorkCourtroomEvidence) -> Result<Vec<u8>, serde_json::Error> {
    let run = evidence.run();
    let value = json!({
        "schema": SCHEMA,
        "source": source(run.source()),
        "binary": source(run.binary()),
        "store": hex(&evidence.store()),
        "runtime": evidence.runtime(),
        "generation": evidence.generation(),
        "backend_profile": evidence.backend_profile().map(labels::backend_profile),
        "seed": run.execution().seed(),
        "schedule": run.execution().schedule(),
        "processes": run.execution().processes().iter().map(process).collect::<Vec<_>>(),
        "environment": environment(evidence),
        "artifact_manifest": evidence.artifacts().iter().map(|artifact| json!({
            "path": artifact.path(),
            "byte_length": artifact.byte_length(),
            "sha256": hex(&artifact.digest().bytes()),
        })).collect::<Vec<_>>(),
        "causal_records": evidence.causal().iter().map(causal).collect::<Vec<_>>(),
        "causal_overflow": evidence.causal_overflow(),
        "shutdown": shutdown(evidence),
        "oracle": {
            "identity": evidence.oracle().oracle(),
            "accepted": evidence.oracle().accepted(),
            "sha256": hex(&evidence.oracle().digest().bytes()),
        },
        "mutants": evidence.mutants().iter().map(|mutant| {
            let binding = mutant.binding();
            json!({
                "identity": mutant.identity(),
                "predicate": mutant.predicate(),
                "source": mutant.source(),
                "source_sha256": hex(&binding.source_digest().bytes()),
                "mutant_sha256": hex(&binding.mutant_digest().bytes()),
                "binary": source(binding.binary()),
                "profile": binding.execution().profile(),
                "scenario": binding.execution().scenario(),
                "killed": mutant.killed(),
                "localization": mutant.localization(),
            })
        }).collect::<Vec<_>>(),
        "verdict": {
            "accepted": evidence.verdict().accepted(),
            "findings": evidence.verdict().findings().iter().copied()
                .map(labels::finding).collect::<Vec<_>>(),
        },
    });
    serde_json::to_vec_pretty(&value)
}

fn source(binding: &worth_store::physical_runtime::PhysicalWorkSourceBinding) -> Value {
    json!({
        "path": binding.path(),
        "sha256": hex(&binding.digest().bytes()),
    })
}

fn process(process: &PhysicalWorkProcessEvidence) -> Value {
    json!({
        "role": process.role(),
        "id": process.process().get(),
        "fate": process.fate().label(),
        "yieldpoint": process.fate().yieldpoint(),
    })
}

fn environment(evidence: &PhysicalWorkCourtroomEvidence) -> Value {
    let environment = evidence.run().environment();
    let graph = environment.feature_graph();
    let platform = environment.platform();
    let filesystem = environment.filesystem();
    let rerun = environment.rerun();
    json!({
        "feature_graph": {
            "roots": graph.roots(),
            "nodes": graph.nodes().iter().map(|node| json!({
                "package": node.package(),
                "features": node.features(),
                "runtime_dependencies": node.dependencies(),
            })).collect::<Vec<_>>(),
        },
        "platform": {
            "operating_system": platform.operating_system(),
            "architecture": platform.architecture(),
            "family": platform.family(),
            "pointer_width": platform.pointer_width(),
            "endian": platform.endian(),
        },
        "filesystem": {
            "root_identity": hex(&filesystem.root_identity()),
            "volume_identity": hex(&filesystem.volume_identity()),
            "filesystem_type": filesystem.filesystem_type(),
            "allocation_granularity": filesystem.allocation_granularity().get(),
            "location": filesystem.location().label(),
            "removable": filesystem.is_removable(),
            "read_only": filesystem.is_read_only(),
            "capabilities": filesystem.capabilities().iter().map(|observation| json!({
                "capability": observation.capability().label(),
                "support": observation.support().label(),
            })).collect::<Vec<_>>(),
        },
        "rerun": {
            "program": rerun.program(),
            "arguments": rerun.arguments(),
        },
    })
}

fn causal(evidence: &PhysicalWorkCausalEvidence) -> Value {
    let scheduler = evidence.scheduler();
    json!({
        "operation": evidence.operation(),
        "signal_request": evidence.signal_request(),
        "signal_generation": evidence.signal_generation(),
        "signal_predecessor_request": evidence.signal_predecessor_request(),
        "signal_predecessor_generation": evidence.signal_predecessor_generation(),
        "signal_attempt": evidence.signal_attempt(),
        "scheduler": {
            "backend_profile": labels::backend_profile(scheduler.backend_profile()),
            "evidence_class": labels::evidence_class(scheduler.evidence_class()),
            "grouped_writes": scheduler.grouped_writes(),
            "primary_backend_requirement": scheduler.primary_backend_requirement(),
            "secondary_present": scheduler.secondary_present(),
        },
        "backend_operation": evidence.backend_operation(),
        "effect_fate": labels::effect_fate(evidence.effect_fate()),
        "recovery": labels::recovery(evidence.recovery()),
        "signal_settlement": evidence.signal_settlement().map(labels::signal),
        "counters": evidence.counters().iter().map(|counter| json!({
            "family": labels::family(counter.family()),
            "pressure": labels::pressure(counter.pressure()),
            "stage": labels::counter_stage(counter.stage()),
            "count": counter.count(),
        })).collect::<Vec<_>>(),
    })
}

fn shutdown(evidence: &PhysicalWorkCourtroomEvidence) -> Value {
    let shutdown = evidence.shutdown();
    let stages = shutdown.stage_counts();
    let drain = shutdown.drain_counts();
    json!({
        "declared": shutdown.declared(),
        "stage_counts": stages,
        "residual": shutdown.residual(),
        "unaccounted_terminal": shutdown.unaccounted_terminal(),
        "drain_counts": drain,
        "drain_residual": shutdown.drain_residual(),
        "drain_evidence_overflow": shutdown.drain_evidence_overflow(),
    })
}

fn publish(report: &Path, encoded: &[u8]) -> Result<(), String> {
    let parent = report
        .parent()
        .ok_or_else(|| "Courtroom A report has no parent".to_owned())?;
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create Courtroom A report directory: {error}"))?;
    let pending = pending_path(report)?;
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&pending)
        .map_err(|error| format!("cannot create Courtroom A pending report: {error}"))?;
    let result = (|| {
        file.write_all(encoded)
            .map_err(|error| format!("cannot write Courtroom A report: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("cannot synchronize Courtroom A report: {error}"))?;
        if report.exists() {
            std::fs::remove_file(report)
                .map_err(|error| format!("cannot replace prior Courtroom A report: {error}"))?;
        }
        std::fs::rename(&pending, report)
            .map_err(|error| format!("cannot publish Courtroom A report: {error}"))
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&pending);
    }
    result
}

fn pending_path(report: &Path) -> Result<PathBuf, String> {
    let parent = report
        .parent()
        .ok_or_else(|| "Courtroom A report has no parent".to_owned())?;
    let name = report
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Courtroom A report filename must be Unicode".to_owned())?;
    for _ in 0..32 {
        let sequence = PENDING_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{name}.{}.{}.pending",
            std::process::id(),
            sequence
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("cannot allocate Courtroom A pending report".into())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::SCHEMA;

    #[test]
    fn schema_is_versioned_and_courtroom_specific() {
        assert_eq!(SCHEMA, "worth.store.c5_1.lifecycle-maelstrom-courtroom.v1");
    }
}
