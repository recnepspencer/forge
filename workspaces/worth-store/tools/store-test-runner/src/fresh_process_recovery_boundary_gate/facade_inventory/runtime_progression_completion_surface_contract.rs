pub(super) const RUNTIME_PROGRESSION_COMPLETION_DESTINATION_SURFACES: &[(&str, &str, &str)] = &[
    ("RecoveryCompletion", "progression/completion", "phase-8"),
    (
        "RecoveryCompletion::admitted_page_lsn_frontier",
        "progression/completion",
        "phase-8",
    ),
    (
        "RecoveryCompletion::recovered_root",
        "progression/completion",
        "phase-8",
    ),
    (
        "RecoveryCompletion::replayed_frames",
        "progression/completion",
        "phase-8",
    ),
    (
        "RecoveryCompletion::source_candidate_count",
        "progression/completion",
        "phase-8",
    ),
    (
        "RecoveryCompletion::source_decision_digest",
        "progression/completion",
        "phase-8",
    ),
    (
        "RecoveryCompletionDenial",
        "progression/completion",
        "phase-8",
    ),
    ("complete_recovery", "progression/completion", "phase-8"),
];
