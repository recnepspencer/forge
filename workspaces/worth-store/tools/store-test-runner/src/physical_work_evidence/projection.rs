use serde_json::{json, Value};
use worth_store::physical_runtime::{
    PhysicalWorkArtifactBinding, PhysicalWorkCausalEvidence, PhysicalWorkCourtroomEvidence,
    PhysicalWorkMutantLocalization, PhysicalWorkProcessEvidence,
    PhysicalWorkRunEnvironmentEvidence,
};

use super::labels;

pub const PHYSICAL_WORK_COURTROOM_EVIDENCE_SCHEMA: &str =
    "worth.store.c5_1.physical-work-courtroom.v2";

pub struct PhysicalWorkCourtroomTerminalProjection(Box<str>);

impl PhysicalWorkCourtroomTerminalProjection {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn project_physical_work_courtroom_evidence(
    evidence: &PhysicalWorkCourtroomEvidence,
) -> Result<PhysicalWorkCourtroomTerminalProjection, String> {
    let run = evidence.run();
    let value = json!({
        "schema": PHYSICAL_WORK_COURTROOM_EVIDENCE_SCHEMA,
        "source": source_value(run.source()),
        "binary": source_value(run.binary()),
        "store": hex(&evidence.store()),
        "runtime": evidence.runtime(),
        "generation": evidence.generation(),
        "backend_profile": evidence.backend_profile().map(labels::backend_profile),
        "seed": run.execution().seed(),
        "schedule": run.execution().schedule(),
        "processes": run.execution().processes().iter().map(process_value).collect::<Vec<_>>(),
        "environment": run_environment_value(run.environment()),
        "artifact_manifest": evidence.artifacts().iter().map(artifact).collect::<Vec<_>>(),
        "causal_records": evidence.causal().iter().map(causal).collect::<Vec<_>>(),
        "causal_overflow": evidence.causal_overflow(),
        "shutdown": shutdown(evidence),
        "oracle": {
            "identity": evidence.oracle().oracle(),
            "accepted": evidence.oracle().accepted(),
            "digest": hex(&evidence.oracle().digest().bytes()),
        },
        "mutants": evidence.mutants().iter().map(mutant_value).collect::<Vec<_>>(),
        "verdict": {
            "accepted": evidence.verdict().accepted(),
            "findings": evidence.verdict().findings().iter().copied().map(labels::finding).collect::<Vec<_>>(),
        },
    });
    serde_json::to_string(&value)
        .map(|encoded| PhysicalWorkCourtroomTerminalProjection(encoded.into_boxed_str()))
        .map_err(|error| format!("cannot project physical-work courtroom evidence: {error}"))
}

pub(crate) fn process_value(process: &PhysicalWorkProcessEvidence) -> Value {
    json!({
        "role": process.role(),
        "id": process.process().get(),
        "fate": process.fate().label(),
        "yieldpoint": process.fate().yieldpoint(),
    })
}

pub(crate) fn run_environment_value(environment: &PhysicalWorkRunEnvironmentEvidence) -> Value {
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

pub(crate) fn source_value(
    binding: &worth_store::physical_runtime::PhysicalWorkSourceBinding,
) -> Value {
    json!({
        "path": binding.path(),
        "sha256": hex(&binding.digest().bytes()),
    })
}

fn artifact(binding: &PhysicalWorkArtifactBinding) -> Value {
    json!({
        "path": binding.path(),
        "byte_length": binding.byte_length(),
        "sha256": hex(&binding.digest().bytes()),
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
        "signal_settlement": evidence.signal_settlement().map(labels::signal_settlement),
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
    let stage = shutdown.stage_counts();
    let drain = shutdown.drain_counts();
    json!({
        "declared": shutdown.declared(),
        "stage_counts": {
            "blocked": stage[0],
            "ready": stage[1],
            "queued": stage[2],
            "dispatched": stage[3],
            "settling": stage[4],
            "terminal_observations": stage[5],
        },
        "residual": shutdown.residual(),
        "unaccounted_terminal": shutdown.unaccounted_terminal(),
        "drain_counts": {
            "settled": drain[0],
            "cancelled_before_dispatch": drain[1],
            "continued_after_cancellation": drain[2],
            "inspection_required": drain[3],
            "released_before_dispatch": drain[4],
            "reconciliation_deferred": drain[5],
        },
        "drain_residual": shutdown.drain_residual(),
        "drain_evidence_overflow": shutdown.drain_evidence_overflow(),
    })
}

pub(crate) fn mutant_value(evidence: &PhysicalWorkMutantLocalization) -> Value {
    let binding = evidence.binding();
    json!({
        "identity": evidence.identity(),
        "predicate": evidence.predicate(),
        "source": evidence.source(),
        "source_sha256": hex(&binding.source_digest().bytes()),
        "mutant_sha256": hex(&binding.mutant_digest().bytes()),
        "binary": {
            "path": binding.binary().path(),
            "sha256": hex(&binding.binary().digest().bytes()),
        },
        "profile": binding.execution().profile(),
        "scenario": binding.execution().scenario(),
        "killed": evidence.killed(),
        "localization": evidence.localization(),
    })
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::PHYSICAL_WORK_COURTROOM_EVIDENCE_SCHEMA;

    #[test]
    fn schema_identity_is_versioned_and_c5_1_specific() {
        assert_eq!(
            PHYSICAL_WORK_COURTROOM_EVIDENCE_SCHEMA,
            "worth.store.c5_1.physical-work-courtroom.v2"
        );
    }
}
