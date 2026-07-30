use serde_json::{json, Value};
use worth_store::physical_runtime::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkHostileCurrentTruth,
    PhysicalWorkHostileTruthCampaignEvidence, PhysicalWorkHostileTruthCaseEvidence,
    PhysicalWorkHostileTruthFinding,
};

use super::timing::CampaignTimings;
use crate::physical_work_evidence::{
    hex, mutant_value, process_value, run_environment_value, source_value,
};

const SCHEMA: &str = "worth.store.c5_1.hostile-physical-truth-courtroom.v4";

pub(super) fn encode(
    evidence: &PhysicalWorkHostileTruthCampaignEvidence,
    timings: &CampaignTimings,
    runner: &worth_store::physical_runtime::PhysicalWorkSourceBinding,
) -> Result<Vec<u8>, String> {
    let first = evidence
        .cases()
        .first()
        .ok_or_else(|| "cannot project an empty Courtroom B campaign".to_owned())?;
    let run = first.binding().run();
    let value = json!({
        "schema": SCHEMA,
        "source": source_value(run.source()),
        "runner_binary": source_value(runner),
        "writer_binary": source_value(run.binary()),
        "observer_binary": source_value(first.binding().observer_binary()),
        "timings": timings.phases().iter().map(timing).collect::<Vec<_>>(),
        "cases": evidence.cases().iter().map(case).collect::<Vec<_>>(),
        "mutants": evidence.mutants().iter().map(mutant_value).collect::<Vec<_>>(),
        "verdict": verdict(evidence.verdict().accepted(), evidence.verdict().findings()),
    });
    serde_json::to_vec_pretty(&value)
        .map_err(|error| format!("cannot encode Courtroom B evidence: {error}"))
}

fn timing(phase: &super::timing::TimedCampaignPhase) -> Value {
    json!({
        "name": phase.name(),
        "elapsed_ms": phase.elapsed_ms(),
    })
}

fn case(evidence: &PhysicalWorkHostileTruthCaseEvidence) -> Value {
    let binding = evidence.binding();
    let execution = binding.run().execution();
    let processes = binding.processes().ordered();
    let comparison = evidence.comparison();
    let artifact_total_bytes = evidence
        .artifacts()
        .iter()
        .map(|artifact| artifact.binding().byte_length())
        .sum::<u64>();
    json!({
        "scenario": binding.scenario().label(),
        "seed": execution.workload_seed().value(),
        "schedule_seed": execution.schedule_seed().value(),
        "schedule": execution.schedule(),
        "environment": run_environment_value(binding.run().environment()),
        "processes": {
            "seed_writer": process_value(processes[0]),
            "baseline_observer": process_value(processes[1]),
            "faulting_writer": process_value(processes[2]),
            "post_kill_observer": process_value(processes[3]),
            "fresh_reopener": process_value(processes[4]),
        },
        "baseline": current(comparison.baseline()),
        "expected": current(comparison.expected()),
        "observed": current(comparison.observed()),
        "artifact_total_bytes": artifact_total_bytes,
        "artifact_manifest": evidence.artifacts().iter().map(artifact).collect::<Vec<_>>(),
        "reopen": reopen(evidence.reopener()),
        "oracle": {
            "identity": evidence.oracle().oracle(),
            "accepted": evidence.oracle().accepted(),
            "sha256": hex(&evidence.oracle().digest().bytes()),
        },
        "verdict": verdict(evidence.verdict().accepted(), evidence.verdict().findings()),
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

fn artifact(
    artifact: &worth_store::physical_runtime::PhysicalWorkHostileArtifactEvidence,
) -> Value {
    json!({
        "path": artifact.binding().path(),
        "byte_length": artifact.binding().byte_length(),
        "sha256": hex(&artifact.binding().digest().bytes()),
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

fn verdict(accepted: bool, findings: &[PhysicalWorkHostileTruthFinding]) -> Value {
    json!({
        "accepted": accepted,
        "findings": findings.iter().copied().map(finding).collect::<Vec<_>>(),
    })
}

const fn finding(finding: PhysicalWorkHostileTruthFinding) -> &'static str {
    match finding {
        PhysicalWorkHostileTruthFinding::ProcessBindingMismatch => "process-binding-mismatch",
        PhysicalWorkHostileTruthFinding::StoreIdentityMismatch => "store-identity-mismatch",
        PhysicalWorkHostileTruthFinding::UnexpectedCurrentTruth => "unexpected-current-truth",
        PhysicalWorkHostileTruthFinding::InvalidScenarioTransition => "invalid-scenario-transition",
        PhysicalWorkHostileTruthFinding::MissingArtifactManifest => "missing-artifact-manifest",
        PhysicalWorkHostileTruthFinding::DuplicateArtifactPath => "duplicate-artifact-path",
        PhysicalWorkHostileTruthFinding::MissingMutationCoordinationArtifact => {
            "missing-mutation-coordination-artifact"
        }
        PhysicalWorkHostileTruthFinding::MissingRecoveryObligation => "missing-recovery-obligation",
        PhysicalWorkHostileTruthFinding::UnexpectedRecoveryObligation => {
            "unexpected-recovery-obligation"
        }
        PhysicalWorkHostileTruthFinding::ReopenTruthMismatch => "reopen-truth-mismatch",
        PhysicalWorkHostileTruthFinding::ReopenRecoveryMismatch => "reopen-recovery-mismatch",
        PhysicalWorkHostileTruthFinding::OracleRejected => "oracle-rejected",
        PhysicalWorkHostileTruthFinding::MissingScenario => "missing-scenario",
        PhysicalWorkHostileTruthFinding::DuplicateScenario => "duplicate-scenario",
        PhysicalWorkHostileTruthFinding::DuplicateStoreIdentity => "duplicate-store-identity",
        PhysicalWorkHostileTruthFinding::MixedSourceBinding => "mixed-source-binding",
        PhysicalWorkHostileTruthFinding::MixedBinaryBinding => "mixed-binary-binding",
        PhysicalWorkHostileTruthFinding::MixedRunEnvironment => "mixed-run-environment",
        PhysicalWorkHostileTruthFinding::MixedFilesystemVolumeProfile => {
            "mixed-filesystem-volume-profile"
        }
        PhysicalWorkHostileTruthFinding::DuplicateFilesystemRootIdentity => {
            "duplicate-filesystem-root-identity"
        }
        PhysicalWorkHostileTruthFinding::RejectedScenario => "rejected-scenario",
        PhysicalWorkHostileTruthFinding::MissingMutantLocalization => "missing-mutant-localization",
        PhysicalWorkHostileTruthFinding::MutantSurvived => "mutant-survived",
    }
}

#[cfg(test)]
mod tests {
    use super::SCHEMA;

    #[test]
    fn schema_is_versioned_and_courtroom_specific() {
        assert_eq!(
            SCHEMA,
            "worth.store.c5_1.hostile-physical-truth-courtroom.v4"
        );
    }
}
