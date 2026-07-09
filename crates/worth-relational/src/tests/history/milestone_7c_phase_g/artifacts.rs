#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeExecutionCertificationArtifacts {
    pub(super) merge_execution_digest: String,
    pub(super) merge_execution_diagnostics_digest: String,
    pub(super) visible_entity_count: usize,
    pub(super) visible_relation_count: usize,
    pub(super) replay_verified: bool,
    pub(super) recovery_envelope_matches: bool,
    pub(super) recovery_truth_matches: bool,
    pub(super) branch_heads_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuthoritativeMergeExecutionCertificationSuite {
    pub(super) exact_shared: MergeExecutionCertificationArtifacts,
    pub(super) source_only_addition: MergeExecutionCertificationArtifacts,
    pub(super) prefer_richer_reconcile: MergeExecutionCertificationArtifacts,
}
