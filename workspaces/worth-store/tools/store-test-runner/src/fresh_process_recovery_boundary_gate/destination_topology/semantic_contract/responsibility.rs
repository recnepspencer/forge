use super::phase_four::phase_four_responsibility;
use super::phase_seven::phase_seven_responsibility;

const STEM_RESPONSIBILITIES: &[(&str, &str)] = &[
    ("request", "persisted-input-request-declaration"),
    ("admission", "fresh-process-entry-admission"),
    ("authority", "concrete-platform-authority"),
    (
        "authority_binding",
        "exact-entry-and-admitted-world-binding-axes",
    ),
    ("session", "linear-recovery-session"),
    ("outcome", "top-level-outcome-vocabulary"),
    ("source_denial", "typed-source-denial-evidence"),
    ("admitted", "admitted-recovery-world"),
    ("discovered", "bounded-candidate-discovery"),
    ("selected", "deterministic-source-selection"),
    ("planned", "immutable-recovery-plan"),
    ("staged", "closed-staging-generation"),
    ("published", "namespace-durable-publication"),
    ("namespace_durable", "namespace-durable-publication"),
    ("reopened", "independent-reopen-proof"),
    ("performed_effect", "concrete-effect-substrate-wrappers"),
    ("discovery", "cross-owner-discovery-sequencing"),
    ("observation", "bounded-source-observation-and-counters"),
    ("manifest_facts", "manifest-addressed-page-fact-discovery"),
    ("planning", "cross-owner-planning-sequencing"),
    ("staging", "cross-owner-staging-sequencing"),
    ("publication", "cross-owner-publication-sequencing"),
    ("reopen", "cross-owner-reopen-sequencing"),
    ("operation_fates", "operation-fate-handoff"),
    ("blocked", "persisted-source-blocked-terminal"),
    ("cleanup_posture", "cleanup-posture-handoff"),
    ("plan", "post-publication-cleanup-plan"),
    ("eligibility", "owner-sampled-cleanup-eligibility"),
    ("execution", "one-artifact-cleanup-execution"),
    ("counters", "cheap-recovery-counter-facts"),
    ("protocol", "foundational-report-protocol"),
    ("report", "typed-recovery-report"),
    ("candidate", "source-candidate-meaning"),
    ("current_previous_root", "current-previous-root-precedence"),
    ("page_facts", "selected-page-fact-admission"),
    ("checkpoint_base", "checkpoint-base-admission"),
    ("wal_tail", "wal-tail-source-admission"),
    ("compaction_product", "compaction-product-visibility"),
    ("residue", "backend-residue-rejection"),
    ("selection", "source-selection-transition"),
    ("continuity", "wal-prefix-continuity"),
    ("valid_prefix", "maximal-valid-wal-prefix"),
    ("torn_tail", "torn-tail-classification"),
    ("record", "redo-record-meaning"),
    ("cursor", "redo-cursor-progression"),
    ("page_lsn", "page-lsn-replay-currency"),
    ("transition", "page-redo-transition"),
    ("identity", "operation-identity-reconciliation"),
    ("evidence_join", "operation-effect-evidence-join"),
    ("binding_freshness", "owner-sampled-binding-freshness"),
    ("fate", "operation-fate-classification"),
    ("limits", "finite-recovery-limits"),
    ("plan_cost", "recovery-plan-cost"),
    ("runtime_identity", "fresh-runtime-identity"),
    ("handoff", "quiescent-runtime-handoff"),
    ("c8_recovery_writer", "crash-courtroom-writer"),
    ("artifact_walk", "bounded-persisted-artifact-walk"),
    ("physical_format", "independent-format-interpretation"),
    ("conclusion", "observer-conclusion"),
    ("report_protocol", "observer-foundational-protocol"),
    ("scenario", "fresh-process-scenario-owner"),
    ("writer_process", "writer-process-boundary"),
    ("recovery_process", "recovery-process-boundary"),
    ("observer_process", "observer-process-boundary"),
    ("crash_matrix", "crash-seam-matrix"),
    ("oracle", "independent-recovery-oracle"),
    ("perturbation", "schedule-perturbation-axis"),
    ("corpus", "persisted-byte-mutation-corpus"),
];

pub(crate) fn expected_responsibility(path: &str) -> String {
    if let Some(responsibility) = path_specific_responsibility(path) {
        return responsibility.into();
    }
    if path == "crates/worth-store-recovery-runtime" {
        return "runtime-crate-boundary".into();
    }
    if path.ends_with("/Cargo.toml") {
        return "runtime-dependency-manifest".into();
    }
    if path.ends_with("/README.md") {
        return "runtime-contract-documentation".into();
    }
    if path.ends_with("/src/lib.rs") {
        return "narrow-public-facade".into();
    }
    if path.ends_with("/physical_store_recover.rs") {
        return "fresh-process-production-entry".into();
    }
    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".rs");
    if stem == "mod" {
        let parent = path.rsplit('/').nth(1).unwrap_or("module");
        return format!("{parent}-module-boundary");
    }
    STEM_RESPONSIBILITIES
        .iter()
        .find_map(|(candidate, responsibility)| (*candidate == stem).then_some(*responsibility))
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{stem}-semantic-owner"))
}

fn path_specific_responsibility(path: &str) -> Option<&'static str> {
    physics_or_store_responsibility(path)
        .or_else(|| phase_two_responsibility(path))
        .or_else(|| phase_four_responsibility(path))
        .or_else(|| phase_seven_responsibility(path))
        .or_else(|| performed_effect_responsibility(path))
}

fn physics_or_store_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-physics/src/source_precedence/admission.rs" => {
            Some("source-candidate-admission")
        }
        "crates/worth-store-recovery-physics/src/redo_replay/plan.rs" => {
            Some("immutable-redo-plan")
        }
        "crates/worth-store-recovery-physics/src/page_redo/eligibility.rs" => {
            Some("page-redo-eligibility")
        }
        "crates/worth-store-recovery-physics/src/page_redo/denial.rs" => Some("page-redo-denial"),
        "crates/worth-store/src/physical_runtime/recovery_construction/authority.rs" => {
            Some("recovery-construction-authority")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/binding.rs" => {
            Some("selected-checkpoint-freshness-source-basis-policy")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/binding/failure.rs" => {
            Some("partial-sampling-failure-evidence")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/binding/wal_payload.rs" => {
            Some("borrowed-wal-member-payload-decoding")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/port.rs" => {
            Some("freshness-sampling-port")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/authority.rs" => {
            Some("concrete-freshness-sampling-authority")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/registration.rs" => {
            Some("process-registered-recovery-session-authority")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/cleanup.rs" => {
            Some("published-root-cleanup-freshness-source-basis-policy")
        }
        _ => None,
    }
}

fn phase_two_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-runtime/src/entry/staging.rs" => {
            Some("typed-staging-counters-and-denial-evidence")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/staging/command.rs" => {
            Some("immutable-plan-to-artifact-command-lowering")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/staging/execution.rs" => {
            Some("exact-command-execution-and-close")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/staging/execution/counters.rs" => {
            Some("stage-honest-settlement-accounting")
        }
        "crates/worth-store-recovery-runtime/src/progression/planned/cancellation.rs" => {
            Some("plan-bound-staging-safe-point-authority")
        }
        "crates/worth-store-physical-backend/src/recovery_media/staging.rs" => {
            Some("confined-convergent-c4-staging-effect")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/staging.rs" => {
            Some("store-owned-redo-scheduler-reservation")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/discovery/observation.rs" => {
            Some("bounded-source-observation")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/discovery/observation/counters.rs" => {
            Some("discovery-counter-observation")
        }
        "crates/worth-store-recovery-runtime/src/entry/configuration.rs" => {
            Some("static-recovery-configuration-identity")
        }
        "crates/worth-store-recovery-runtime/src/entry/counters.rs" => {
            Some("entry-session-and-zero-effect-counters")
        }
        "crates/worth-store-recovery-runtime/src/entry/limits.rs" => {
            Some("finite-recovery-limit-admission")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/coordination.rs" => {
            Some("delegates-to-store-registered-session-coordination-authority")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/handoff.rs" => {
            Some("quiescent-recovered-handoff-sequencing")
        }
        "crates/worth-store-recovery-runtime/src/entry/publication.rs" => {
            Some("typed-publication-settlement-and-counters")
        }
        "crates/worth-store-recovery-runtime/src/entry/reopen.rs" => {
            Some("typed-fresh-reopen-failure-and-counters")
        }
        "crates/worth-store-recovery-runtime/src/handoff/recovered.rs" => {
            Some("sealed-physical-recovered-handoff")
        }
        "crates/worth-store-recovery-runtime/src/handoff/recovered_evidence.rs" => {
            Some("exact-selected-and-stage-honest-handoff-evidence")
        }
        "crates/worth-store-physical-backend/src/recovery_media/publication.rs" => {
            Some("scheduled-c4-root-protocol-and-namespace-publication")
        }
        "crates/worth-store-physical-backend/src/recovery_media/reopen.rs" => {
            Some("bounded-scheduled-fresh-reopen-read")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/publication.rs" => {
            Some("store-owned-publication-command-and-settlement")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/publication/execution.rs" => {
            Some("scheduled-publication-effect-sequencing")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/reopen.rs" => {
            Some("store-owned-fresh-reopen-command-and-denial")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/reopen/execution.rs" => {
            Some("scheduled-fresh-reopen-settlement")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/effect/reopen.rs" => {
            Some("performed-fresh-reopen-occurrence")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/mod.rs" => {
            Some("recovery-coordination-module-boundary")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/capacity.rs" => {
            Some("bounded-recovery-work-capacity")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/semantics.rs" => {
            Some("four-aspect-native-recovery-bases-and-exact-partitions")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/owner.rs" => {
            Some("store-signal-and-c5-scheduler-lifecycle-owner")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/effect.rs" => {
            Some("performed-recovery-effect-authority")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/staging/execution.rs" => {
            Some("scheduled-physical-and-signal-settlement")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/staging/execution/admission.rs" => {
            Some("exact-work-and-scheduler-admission")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/staging/execution/settlement.rs" => {
            Some("terminal-signal-completion-classification")
        }
        "crates/worth-store-physical-backend/src/recovery_media/mod.rs" => {
            Some("recovery-media-capability-boundary")
        }
        "crates/worth-store-physical-backend/src/recovery_media/qualification.rs" => {
            Some("existing-store-recovery-qualification")
        }
        "crates/worth-store-physical-backend/src/recovery_media/qualified.rs" => {
            Some("qualified-recovery-media-capability")
        }
        "crates/worth-store-physical-backend/src/recovery_media/profile.rs" => {
            Some("qualified-backend-profile-identity")
        }
        "crates/worth-store-physical-backend/src/recovery_media/generation.rs" => {
            Some("qualified-media-generation-identity")
        }
        "crates/worth-store-physical-backend/src/recovery_media/admitted.rs" => {
            Some("persisted-store-identity-admission")
        }
        "crates/worth-store-physical-backend/src/recovery_media/discovery.rs" => {
            Some("bounded-read-only-recovery-port")
        }
        "crates/worth-store-physical-backend/src/recovery_media/discovery/artifact.rs" => {
            Some("typed-recovery-artifact-address")
        }
        "crates/worth-store-physical-format/src/manifest/durable_root_routing/decode_limits.rs" => {
            Some("preallocation-routing-cardinality-contract")
        }
        "crates/worth-store-wal/src/artifact_store/segment_inventory/segment_inspection/denial.rs" => {
            Some("preallocation-wal-frame-limit-evidence")
        }
        _ => None,
    }
}

fn performed_effect_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-runtime/src/orchestration/publication/performed_root_replacement.rs" => {
            Some("performed-root-replacement-evidence")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/publication/performed_namespace_sync.rs" => {
            Some("performed-namespace-sync-evidence")
        }
        "crates/worth-store-recovery-runtime/src/orchestration/reopen/performed_independent_reopen.rs" => {
            Some("performed-independent-reopen-evidence")
        }
        "crates/worth-store-recovery-runtime/src/cleanup/performed_removal.rs" => {
            Some("performed-cleanup-removal-evidence")
        }
        _ => None,
    }
}
