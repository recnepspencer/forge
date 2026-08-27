use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StrategyCertificationBundle {
    pub(super) main_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    pub(super) feature_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    pub(super) replacement: ReplacementCertificationBundle,
    pub(super) merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    pub(super) merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    pub(super) aspect_overlap_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    pub(super) aspect_overlap_merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    pub(super) aspect_disjoint_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    pub(super) aspect_disjoint_merge_lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    pub(super) controller_sequence_merge_conflict: crate::merge::data::MergeConflictDigestBasis,
    pub(super) controller_sequence_merge_lowered_plan:
        crate::merge::data::MergeLoweredPlanDigestBasis,
    pub(super) main_replay: RelationalReplayOutcome,
    pub(super) feature_replay: RelationalReplayOutcome,
    pub(super) controller_sequence_noop: ControllerSequenceNoopEvidence,
    pub(super) missing_executor_replay: StrategyReplayMismatchEvidence,
    pub(super) failing_executor_replay: StrategyReplayMismatchEvidence,
    pub(super) branch_heads: StrategyBranchHeadEvidence,
    pub(super) visible_truth: StrategyVisibleTruthEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReplacementCertificationBundle {
    pub(super) replacement_commit_strategy_artifacts: StrategyCommitArtifactBundle,
    pub(super) replacement_replay: RelationalReplayOutcome,
    pub(super) replacement_lineage: ReplacementLineageEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ControllerSequenceNoopEvidence {
    pub(super) strategy_artifacts: StrategyCommitArtifactBundle,
    pub(super) changed_record_count: usize,
    pub(super) patch_record_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StrategyReplayMismatchEvidence {
    pub(super) strategy_surface_mismatch_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StrategyBranchHeadEvidence {
    pub(super) main: Option<RelationalCommitReceipt>,
    pub(super) feature: Option<RelationalCommitReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StrategyVisibleTruthEvidence {
    pub(super) entity_name: Option<String>,
    pub(super) branch_heads: StrategyBranchHeadEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReplacementLineageEvidence {
    pub(super) start_lineage: crate::facade::identity::LineageId,
    pub(super) end_lineage: crate::facade::identity::LineageId,
    pub(super) lineage_basis: crate::lineage::data::LineageDigestBasis,
    pub(super) event_batch_basis: crate::lineage::data::LineageEventBatchDigestBasis,
    pub(super) decision_log_basis: crate::lineage::data::LineageDecisionLogDigestBasis,
    pub(super) normalized_client_key_count: usize,
    pub(super) lineage_transition_count: usize,
}
