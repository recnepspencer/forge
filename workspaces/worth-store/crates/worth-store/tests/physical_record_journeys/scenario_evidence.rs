use std::path::Path;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use worth_store::physical_runtime::AdmittedRecordPlacementPolicy;
use worth_store_offline_verifier::OfflineDurableManifestWalk;

pub(super) use super::scenario_process_evidence::{emit_process, ScenarioProcessEvidence};

pub(super) struct ScenarioEvidence<'world> {
    pub(super) courtroom: &'world str,
    pub(super) world: &'world str,
    pub(super) root: &'world Path,
    pub(super) seed: u64,
    pub(super) action_trace: &'world [&'world str],
    pub(super) authority_transitions: &'world [&'world str],
    pub(super) walk: &'world OfflineDurableManifestWalk,
    pub(super) placement: AdmittedRecordPlacementPolicy,
    pub(super) publication_identity: Option<u64>,
    pub(super) processes: &'world [ScenarioProcessEvidence],
    pub(super) counters: Value,
    pub(super) runtime_result: Value,
    pub(super) oracle_result: Value,
    pub(super) mutant_posture: &'world str,
    pub(super) predicates: &'world [ScenarioPredicate<'world>],
}

pub(super) struct ScenarioPredicate<'evidence> {
    name: &'evidence str,
    expected: Value,
    observed: Value,
}

impl<'evidence> ScenarioPredicate<'evidence> {
    pub(super) fn equality(
        name: &'evidence str,
        expected: impl Into<Value>,
        observed: impl Into<Value>,
    ) -> Self {
        Self {
            name,
            expected: expected.into(),
            observed: observed.into(),
        }
    }

    fn as_evidence(&self) -> Value {
        json!({
            "expected": self.expected,
            "observed": self.observed,
            "passed": self.expected == self.observed,
        })
    }
}

pub(super) fn emit(input: ScenarioEvidence<'_>) {
    let record_map_digest = placement_map_digest(input.walk);
    let evidence = json!({
        "kind": "worth-store-c5-courtroom",
        "courtroom": input.courtroom,
        "world": input.world,
        "source_identity": super::scenario_artifact_evidence::source_identity(),
        "binary_identities": input.processes.iter().map(ScenarioProcessEvidence::binary_identity).collect::<Vec<_>>(),
        "format_identity": hex(&input.walk.format_identity()),
        "placement_policy_identity": placement_identity(input.placement),
        "backend_os_profile": std::env::consts::OS,
        "seed": input.seed,
        "processes": input.processes,
        "fault_schedule": input.world,
        "action_trace": input.action_trace,
        "authority_transitions": input.authority_transitions,
        "publication_identity": input.publication_identity,
        "current_root_generation": input.walk.root_generation(),
        "artifact_manifest": super::scenario_artifact_evidence::artifact_manifest(input.root),
        "record_id_to_placement": {
            "records": input.walk.placements().len(),
            "sha256": record_map_digest,
        },
        "counter_snapshots": input.counters,
        "runtime_result": input.runtime_result,
        "offline_result": {
            "root_generation": input.walk.root_generation(),
            "records": input.walk.placements().len(),
            "manifest_blocks": input.walk.manifest_blocks(),
            "payload_frames": input.walk.payload_frames(),
            "payload_bytes": input.walk.payload_bytes(),
            "payload_digest": hex(&input.walk.payload_digest()),
        },
        "oracle_result": input.oracle_result,
        "mutant_posture": input.mutant_posture,
        "predicates": input.predicates.iter().map(|predicate| {
            (predicate.name.to_owned(), predicate.as_evidence())
        }).collect::<serde_json::Map<_, _>>(),
    });
    validate_evidence(&evidence);
    println!("C5_SCENARIO_EVIDENCE {evidence}");
}

fn validate_evidence(evidence: &Value) {
    let artifacts = evidence["artifact_manifest"].as_object().unwrap();
    assert!(artifacts["artifacts"]
        .as_u64()
        .is_some_and(|count| count != 0));
    assert!(artifacts["bytes"].as_u64().is_some_and(|bytes| bytes != 0));
    assert!(artifacts["sha256"]
        .as_str()
        .is_some_and(|digest| digest.len() == 64));
    let processes = evidence["processes"].as_array().unwrap();
    assert!(!processes.is_empty());
    assert!(processes.iter().all(|process| {
        process["process_id"].as_u64().is_some_and(|id| id != 0)
            && process["binary_identity"]
                .as_str()
                .is_some_and(|digest| digest.len() == 64)
    }));
    let process_ids = processes
        .iter()
        .map(|process| process["process_id"].as_u64().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(process_ids.len(), processes.len());
    assert!(processes.iter().all(|process| {
        process["runtime_identity"].is_null()
            || (process["store_identity"].as_str().is_some()
                && process["media_owner_identity"].as_str().is_some()
                && process["mutation_attempt_identity"].as_str().is_some()
                && process["backend_profile_identity"].as_str().is_some())
    }));
    assert!(
        evidence["predicates"]
            .as_object()
            .is_some_and(|predicates| {
                !predicates.is_empty()
                    && predicates.values().all(|predicate| {
                        predicate["passed"]
                            == Value::Bool(predicate["expected"] == predicate["observed"])
                            && predicate["passed"] == Value::Bool(true)
                    })
            }),
        "scenario predicates diverged: {}",
        evidence["predicates"]
    );
    assert!(evidence["runtime_result"].is_object());
    assert!(evidence["oracle_result"].is_object());
}

fn placement_identity(placement: AdmittedRecordPlacementPolicy) -> String {
    let mut digest = Sha256::new();
    digest.update(placement.segment_pages().get().to_le_bytes());
    digest.update(placement.extent_threshold().get().to_le_bytes());
    digest.update(placement.page_fill().get().to_le_bytes());
    digest.update(placement.manifest_capacity().get().to_le_bytes());
    hex(&digest.finalize())
}

fn placement_map_digest(walk: &OfflineDurableManifestWalk) -> String {
    let mut digest = Sha256::new();
    for placement in walk.placements() {
        let encoded = format!("{placement:?}");
        digest.update((encoded.len() as u64).to_le_bytes());
        digest.update(encoded.as_bytes());
    }
    hex(&digest.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
