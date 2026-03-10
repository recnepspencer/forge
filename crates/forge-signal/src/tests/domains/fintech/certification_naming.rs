pub(super) mod workflow_names {
    pub(crate) const HOSTILE_BRANCH_REPLAY_AUDIT: &str = "hostile-branch-replay-audit";
}

pub(super) mod artifact_aliases {
    pub(crate) const MAIN_BRANCH: &str = "main";
    pub(crate) const ANALYSIS_BRANCH: &str = "analysis";
    pub(crate) const CORRECTION_BRANCH: &str = "correction";
    pub(crate) const MAIN_SNAPSHOT: &str = "main_snapshot";
    pub(crate) const ANALYSIS_SNAPSHOT: &str = "analysis_snapshot";
    pub(crate) const BASELINE_AUDIT: &str = "baseline_audit";
    pub(crate) const ANALYSIS_AUDIT: &str = "analysis_audit";
    pub(crate) const RESTORED_ANALYSIS_AUDIT: &str = "restored_analysis_audit";
    pub(crate) const RESTORED_MAIN_AUDIT: &str = "restored_main_audit";
    pub(crate) const MAIN_REPLAY: &str = "main_replay";
    pub(crate) const ANALYSIS_REPLAY_BEFORE: &str = "analysis_replay_before";
    pub(crate) const ANALYSIS_REPLAY_AFTER: &str = "analysis_replay_after";
    pub(crate) const ANALYSIS_AROUND_SNAPSHOT: &str = "analysis_around_snapshot";
    pub(crate) const CORRECTION_REPLAY: &str = "correction_replay";
    pub(crate) const CORRECTION_LINEAGE: &str = "correction_lineage";
}

pub(super) mod invariant_names {
    pub(crate) const ANALYSIS_RESTORE_MATCHES: &str = "audit_eq:analysis_audit:restored_analysis_audit";
    pub(crate) const MAIN_RESTORE_MATCHES: &str = "audit_eq:baseline_audit:restored_main_audit";
    pub(crate) const ANALYSIS_REPLAY_HAS_ROLLBACK: &str =
        "replay_has_kind:analysis_replay_after:TransactionRolledBack";
    pub(crate) const MAIN_REPLAY_BRANCH_LOCAL: &str = "replay_branch_local:main_replay:main";
    pub(crate) const CORRECTION_REPLAY_HAS_BRANCH_SWITCH: &str =
        "replay_has_kind:correction_replay:BranchSwitched";
    pub(crate) const CORRECTION_LINEAGE_HAS_RECOVERY: &str =
        "lineage_has_any:correction_lineage:Replaced,Refreshed,Restored";
    pub(crate) const MAIN_BRANCH_HEAD_MATCHES: &str =
        "branch_head_matches_snapshot:main:main_snapshot";
    pub(crate) const ANALYSIS_REPLAY_MENTIONS_SNAPSHOT: &str =
        "replay_mentions_snapshot:analysis_around_snapshot:analysis_snapshot";
}
