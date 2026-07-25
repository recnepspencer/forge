use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceBindingDenial, PhysicalWorkEvidenceDigest, PhysicalWorkMutantBinding,
    PhysicalWorkMutantExecutionContext, PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome,
    PhysicalWorkMutantSubject, PhysicalWorkSourceBinding,
};

const REPORT_ENV: &str = "WORTH_STORE_C5_1_MUTANT_REPORT";
const REPORT_SCHEMA: &str = "worth.store.c5_1.mutation-evidence.v1";
const FIRST_MUTANT: u8 = 15;
const LAST_COURTROOM_A_MUTANT: u8 = 41;
const LAST_MUTANT: u8 = 43;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationReport {
    schema: String,
    observations: Vec<MutationObservation>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct MutationObservation {
    id: u8,
    source_binding: String,
    source_sha256: String,
    mutant_sha256: String,
    binary_binding: String,
    binary_sha256: String,
    profile_binding: String,
    scenario_binding: String,
    expected_failing_predicate: String,
    actual_failing_predicate: String,
    localization: String,
}

#[derive(Clone, Copy)]
struct MutantExpectation {
    predicate: &'static str,
    source: &'static str,
    scenario: &'static str,
}

struct ArtifactPolicy {
    parent: PathBuf,
    directory_prefix: String,
}

pub(super) fn load() -> Option<Vec<PhysicalWorkMutantLocalization>> {
    let path = PathBuf::from(std::env::var_os(REPORT_ENV)?);
    Some(
        decode_file(&path, &workspace_root())
            .unwrap_or_else(|error| panic!("invalid {REPORT_ENV} evidence: {error}")),
    )
}

fn decode_file(
    path: &Path,
    workspace: &Path,
) -> Result<Vec<PhysicalWorkMutantLocalization>, String> {
    let canonical = path
        .canonicalize()
        .map_err(|error| format!("cannot locate report {}: {error}", path.display()))?;
    let bytes = std::fs::read(&canonical)
        .map_err(|error| format!("cannot read report {}: {error}", canonical.display()))?;
    let report: MutationReport = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report: {error}"))?;
    if report.schema != REPORT_SCHEMA {
        return Err(format!(
            "unsupported mutation report schema `{}`",
            report.schema
        ));
    }
    validate_campaign_shape(&report.observations)?;
    let policy = ArtifactPolicy::for_report(&canonical)?;
    let mut courtroom_a = Vec::new();
    for observation in report.observations {
        let id = observation.id;
        let expected = expectation(id);
        let localization = validate_observation(observation, expected, workspace, &policy)?;
        if courtroom_a_owns(id) {
            courtroom_a.push(localization);
        }
    }
    Ok(courtroom_a)
}

fn validate_campaign_shape(observations: &[MutationObservation]) -> Result<(), String> {
    let expected_count = usize::from(LAST_MUTANT - FIRST_MUTANT + 1);
    if observations.len() != expected_count {
        return Err(format!(
            "mutation report requires {expected_count} observations, found {}",
            observations.len()
        ));
    }
    for (expected, observation) in (FIRST_MUTANT..=LAST_MUTANT).zip(observations) {
        if observation.id != expected {
            return Err(format!(
                "mutation report expected mutant {expected}, found {}",
                observation.id
            ));
        }
    }
    Ok(())
}

fn validate_observation(
    observation: MutationObservation,
    expected: MutantExpectation,
    workspace: &Path,
    artifacts: &ArtifactPolicy,
) -> Result<PhysicalWorkMutantLocalization, String> {
    if observation.source_binding != expected.source {
        return Err(format!("mutant {} source binding changed", observation.id));
    }
    if observation.expected_failing_predicate != expected.predicate
        || observation.actual_failing_predicate != expected.predicate
    {
        return Err(format!(
            "mutant {} predicate binding changed",
            observation.id
        ));
    }
    if observation.profile_binding != "test" || observation.scenario_binding != expected.scenario {
        return Err(format!(
            "mutant {} execution binding changed",
            observation.id
        ));
    }
    let source_digest = parse_digest(&observation.source_sha256)?;
    let current_source = hash_file(&workspace.join(expected.source))?;
    if source_digest != current_source {
        return Err(format!("mutant {} source is stale", observation.id));
    }
    let mutant_digest = parse_digest(&observation.mutant_sha256)?;
    if mutant_digest == source_digest {
        return Err(format!("mutant {} made no source change", observation.id));
    }
    let binary_path = artifacts.resolve(&observation.binary_binding)?;
    let binary_digest = parse_digest(&observation.binary_sha256)?;
    if hash_file(&binary_path)? != binary_digest {
        return Err(format!("mutant {} binary is stale", observation.id));
    }
    bind_localization(
        observation,
        expected,
        source_digest,
        mutant_digest,
        binary_path,
        binary_digest,
    )
}

fn bind_localization(
    observation: MutationObservation,
    expected: MutantExpectation,
    source_digest: PhysicalWorkEvidenceDigest,
    mutant_digest: PhysicalWorkEvidenceDigest,
    binary_path: PathBuf,
    binary_digest: PhysicalWorkEvidenceDigest,
) -> Result<PhysicalWorkMutantLocalization, String> {
    let subject = PhysicalWorkMutantSubject::new(
        u16::from(observation.id),
        expected.predicate,
        expected.source,
    )
    .map_err(binding_denial)?;
    let execution =
        PhysicalWorkMutantExecutionContext::new(observation.profile_binding, expected.scenario)
            .map_err(binding_denial)?;
    let binary = PhysicalWorkSourceBinding::new(binary_path.display().to_string(), binary_digest)
        .map_err(binding_denial)?;
    let binding =
        PhysicalWorkMutantBinding::new(subject, source_digest, mutant_digest, binary, execution);
    PhysicalWorkMutantLocalization::new(
        binding,
        PhysicalWorkMutantOutcome::new(true, observation.localization),
    )
    .map_err(binding_denial)
}

impl ArtifactPolicy {
    fn for_report(report: &Path) -> Result<Self, String> {
        let parent = report
            .parent()
            .ok_or_else(|| "mutation report has no parent".to_owned())?
            .canonicalize()
            .map_err(|error| format!("cannot canonicalize mutation report parent: {error}"))?;
        let name = report
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "mutation report filename must be Unicode".to_owned())?;
        Ok(Self {
            parent,
            directory_prefix: format!("{name}.artifacts."),
        })
    }

    fn resolve(&self, binding: &str) -> Result<PathBuf, String> {
        let claimed = PathBuf::from(binding);
        let resolved = if claimed.is_absolute() {
            claimed
        } else {
            self.parent.join(claimed)
        };
        let canonical = resolved.canonicalize().map_err(|error| {
            format!(
                "cannot locate mutant binary {}: {error}",
                resolved.display()
            )
        })?;
        let directory = canonical
            .parent()
            .ok_or_else(|| "mutant binary has no parent".to_owned())?;
        let directory_name = directory
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if !directory.starts_with(&self.parent)
            || !directory_name.starts_with(&self.directory_prefix)
            || !canonical.is_file()
        {
            return Err("mutant binary escaped its report artifact directory".into());
        }
        Ok(canonical)
    }
}

fn expectation(id: u8) -> MutantExpectation {
    MutantExpectation {
        predicate: predicate(id),
        source: source(id),
        scenario: scenario(id),
    }
}

const fn courtroom_a_owns(id: u8) -> bool {
    id <= LAST_COURTROOM_A_MUTANT
}

fn predicate(id: u8) -> &'static str {
    match id {
        15 => "settlement",
        16 => "scheduler-admission",
        17 => "backend-dispatch",
        18 => "derived-completion",
        19 => "post-dispatch-cancellation",
        20 => "stale-generation",
        21 => "health-revocation",
        22 => "physical-effect-no-retry",
        23 => "duplicate-work-registry",
        24..=26 => "store-local-async-registry",
        27 => "lifecycle-duplication",
        28 => "serialized-signal-reopen",
        29 => "internal-json-carrier",
        30 => "legacy-resource-node",
        31 => "raw-signal-slot-authority",
        32 => "foundational-mask-substitution",
        33 => "aspect-partition-broadening",
        34 => "branch-writer-registry",
        35 => "branch-label-disjointness",
        36 => "signal-evaluation-effect",
        37 => "generic-signal-settlement",
        38 => "scheduler-counter-settlement",
        39 => "skipped-backend-write",
        40 => "raw-backend-dispatch",
        41 => "signal-readiness",
        42 => "dirty-clean-without-exact-receipt",
        43 => "c6-local-scheduler",
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}

fn source(id: u8) -> &'static str {
    match id {
        15..=18 => "crates/worth-store/src/physical_runtime/work/observation/causal.rs",
        19 | 27 => "crates/worth-store/src/physical_runtime/instance/work_lifecycle.rs",
        20 => "crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_runtime/physical_work/batch_execution.rs",
        21 => "crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_health.rs",
        22 => "crates/worth-store/src/physical_runtime/instance/signal_owner/reconciliation.rs",
        23..=26 | 34 => "crates/worth-store/src/physical_runtime/instance/work_runtime.rs",
        28 | 29 => "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        30 => "crates/worth-store/src/physical_runtime/instance/signal_owner/mod.rs",
        31 | 32 => "crates/worth-store/src/physical_runtime/work/profile/aspect_bindings.rs",
        33 | 35 => "crates/worth-store/src/physical_runtime/work/concurrency_scope.rs",
        36 => "crates/worth-store/src/physical_runtime/instance/signal_owner/graph.rs",
        37 => "crates/worth-store/src/physical_runtime/instance/signal_owner/lifecycle_join.rs",
        38 => "crates/worth-store/src/physical_runtime/work/execution/settlement/classification.rs",
        39 => "crates/worth-store/src/physical_runtime/instance/executor/range_write.rs",
        40 => "crates/worth-store/src/physical_runtime/record_serving/residency/artifact_tree.rs",
        41 => "crates/worth-store/src/physical_runtime/instance/signal_owner/graph/publication_dependency.rs",
        42 => "crates/worth-store-buffer-pool/src/physical_residency/lease.rs",
        43 => {
            "crates/worth-store/src/physical_runtime/record_serving/c6_handoff/residency/mod.rs"
        }
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}

fn scenario(id: u8) -> &'static str {
    match id {
        15..=18 => "physical_work::publication_work_trace::successful_publication_exposes_each_causal_work_identity_once",
        19 => "physical_work::post_dispatch_cancellation::cancellation_after_backend_dispatch_retains_terminal_settlement_obligation",
        20 => "physical_work::execution_capability::stale_execution_capability_cannot_cross_the_real_effect_boundary",
        21 => "physical_work::failure::partial_write_retains_exact_prefix_and_revokes_serving_health",
        22 => "physical_work::authority_sealing::derived_reconciliation::derived_reconciliation_cannot_reacquire_physical_effect_authority",
        23 => "physical_work::authority_sealing::duplicate_runtime::a_second_pending_work_registry_is_forbidden",
        24..=26 => "physical_work::authority_sealing::duplicate_runtime::store_local_async_registries_are_forbidden",
        27 => "physical_work::authority_sealing::duplicate_runtime::a_second_physical_lifecycle_is_forbidden",
        28 => "physical_work::authority_sealing::reopen_boundary::reopen_cannot_consume_serialized_signal_state",
        29 => "physical_work::authority_sealing::reopen_boundary::ordinary_physical_work_cannot_add_an_internal_json_carrier",
        30 => "physical_work::authority_sealing::semantic_boundary::legacy_signal_resource_construction_is_forbidden",
        31 => "physical_work::authority_sealing::semantic_boundary::raw_signal_slots_cannot_become_semantic_authority",
        32 => "physical_work::authority_sealing::semantic_boundary::foundational_masks_cannot_substitute_for_native_bindings",
        33 => "physical_work::authority_sealing::semantic_boundary::callers_cannot_broaden_aspect_or_partition_scope",
        34 => "physical_work::authority_sealing::branch_boundary::store_cannot_own_a_branch_writer_registry",
        35 => "physical_work::authority_sealing::branch_boundary::branch_labels_cannot_define_physical_disjointness",
        36 => "physical_work::authority_mutants::signal_evaluation_is_filesystem_effect_free",
        37 => "physical_work::authority_mutants::generic_signal_completion_cannot_upgrade_proven_no_effect",
        38 => "physical_work::authority_mutants::scheduler_counters_cannot_settle_cross_bound_backend_receipts",
        39 | 40 => "physical_work::authority_mutants::one_canonical_write_requires_one_backend_effect",
        41 => "physical_work::publication_signal_progression::root_publication_waits_for_settled_child_signal_completion_without_repeating_media",
        42 => "physical_runtime::record_serving::residency::candidate_frame_residency::tests::exact_receipt::foreign_real_receipt_cannot_clean_dirty_writeback",
        43 => "physical_runtime::record_serving::c6_handoff::residency::tests::c6_handoff_carries_no_local_scheduler_or_pending_registry",
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

fn hash_file(path: &Path) -> Result<PhysicalWorkEvidenceDigest, String> {
    let bytes =
        std::fs::read(path).map_err(|error| format!("cannot hash {}: {error}", path.display()))?;
    PhysicalWorkEvidenceDigest::new(Sha256::digest(bytes).into())
        .ok_or_else(|| format!("{} has an all-zero digest", path.display()))
}

fn parse_digest(encoded: &str) -> Result<PhysicalWorkEvidenceDigest, String> {
    if encoded.len() != 64 || !encoded.is_ascii() {
        return Err("mutation digest must be 64 hexadecimal characters".into());
    }
    let mut bytes = [0_u8; 32];
    for (index, slot) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *slot = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| "mutation digest contains non-hexadecimal data".to_owned())?;
    }
    PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| "mutation digest cannot be all zero".to_owned())
}

fn binding_denial(denial: PhysicalWorkEvidenceBindingDenial) -> String {
    format!("mutation evidence binding denied: {denial:?}")
}

#[cfg(test)]
mod tests;
