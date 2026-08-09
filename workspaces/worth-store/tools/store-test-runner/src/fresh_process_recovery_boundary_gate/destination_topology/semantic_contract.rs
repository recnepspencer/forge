pub(super) fn expected_phase(path: &str) -> &'static str {
    if path.starts_with("crates/worth-store-recovery-runtime") {
        return runtime_phase(path);
    }
    if path.contains("worth-store-physical-backend/src/recovery_media/") {
        return "phase-2";
    }
    if path.contains("worth-store-recovery-physics/src/source_precedence/")
        || path.contains("worth-store-recovery-physics/src/wal_prefix/")
    {
        return "phase-3";
    }
    if path.contains("worth-store-recovery-physics/src/page_redo/") {
        return if path.ends_with("/transition.rs") {
            "phase-5"
        } else {
            "phase-4"
        };
    }
    if path.contains("worth-store-recovery-physics/") {
        return "phase-4";
    }
    if path.contains("worth-store/src/physical_runtime/recovery_freshness/") {
        return if path.ends_with("/cleanup.rs") {
            "phase-7"
        } else if path.ends_with("/binding.rs") {
            "phase-4"
        } else {
            "phase-2"
        };
    }
    if path.contains("worth-store/src/physical_runtime/recovery_construction/")
        || path.contains("worth-store-offline-verifier/")
    {
        return "phase-6";
    }
    "phase-9"
}

fn runtime_phase(path: &str) -> &'static str {
    if path.contains("/entry/")
        || path.ends_with("/orchestration/coordination.rs")
        || runtime_scaffold(path)
    {
        return "phase-2";
    }
    if path.ends_with("/handoff/mod.rs")
        || path.ends_with("/progression/discovered.rs")
        || path.ends_with("/progression/selected.rs")
        || path.ends_with("/orchestration/discovery.rs")
        || path.ends_with("/handoff/unsupported_scope.rs")
    {
        return "phase-3";
    }
    if path.ends_with("/progression/planned.rs")
        || path.ends_with("/orchestration/planning.rs")
        || path.ends_with("/handoff/operation_fates.rs")
    {
        return "phase-4";
    }
    if path.ends_with("/progression/staged.rs")
        || path.ends_with("/orchestration/staging.rs")
        || path.contains("/orchestration/staging/")
    {
        return "phase-5";
    }
    if path.contains("/cleanup/") || path.ends_with("/handoff/cleanup_posture.rs") {
        return "phase-7";
    }
    "phase-6"
}

fn runtime_scaffold(path: &str) -> bool {
    !path.contains("/src/progression/")
        && !path.contains("/src/orchestration/")
        && !path.contains("/src/handoff/")
        && !path.contains("/src/cleanup/")
        && !path.contains("/src/observation/")
        || path.ends_with("/progression/mod.rs")
        || path.ends_with("/progression/admitted.rs")
        || path.ends_with("/orchestration/mod.rs")
}
const STEM_RESPONSIBILITIES: &[(&str, &str)] = &[
    ("request", "persisted-input-request-declaration"),
    ("admission", "fresh-process-entry-admission"),
    ("authority", "concrete-platform-authority"),
    ("authority_binding", "exact-entry-binding-axes"),
    ("session", "linear-recovery-session"),
    ("outcome", "top-level-outcome-vocabulary"),
    ("admitted", "admitted-recovery-world"),
    ("discovered", "bounded-candidate-discovery"),
    ("selected", "deterministic-source-selection"),
    ("planned", "immutable-recovery-plan"),
    ("staged", "closed-staging-generation"),
    ("published", "namespace-durable-publication"),
    ("reopened", "independent-reopen-proof"),
    ("performed_effect", "concrete-effect-substrate-wrappers"),
    ("discovery", "cross-owner-discovery-sequencing"),
    ("planning", "cross-owner-planning-sequencing"),
    ("staging", "cross-owner-staging-sequencing"),
    ("publication", "cross-owner-publication-sequencing"),
    ("reopen", "cross-owner-reopen-sequencing"),
    ("operation_fates", "operation-fate-handoff"),
    ("unsupported_scope", "unsupported-scope-handoff"),
    ("cleanup_posture", "cleanup-posture-handoff"),
    ("plan", "post-publication-cleanup-plan"),
    ("eligibility", "owner-sampled-cleanup-eligibility"),
    ("execution", "one-artifact-cleanup-execution"),
    ("counters", "cheap-recovery-counter-facts"),
    ("protocol", "foundational-report-protocol"),
    ("report", "typed-recovery-report"),
    ("candidate", "source-candidate-meaning"),
    ("current_previous_root", "current-previous-root-precedence"),
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

pub(super) fn expected_responsibility(path: &str) -> String {
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
        "crates/worth-store/src/physical_runtime/recovery_freshness/port.rs" => {
            Some("freshness-sampling-port")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/authority.rs" => {
            Some("concrete-freshness-sampling-authority")
        }
        "crates/worth-store/src/physical_runtime/recovery_freshness/cleanup.rs" => {
            Some("published-root-cleanup-freshness-source-basis-policy")
        }
        _ => None,
    }
}

fn phase_two_responsibility(path: &str) -> Option<&'static str> {
    match path {
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
            Some("fresh-signal-and-bounded-c5-scheduler-instance")
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
        _ => None,
    }
}

fn performed_effect_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-runtime/src/orchestration/staging/performed_write.rs" => {
            Some("performed-staging-write-evidence")
        }
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
