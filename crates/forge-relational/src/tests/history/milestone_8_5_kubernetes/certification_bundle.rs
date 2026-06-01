use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesIntentCertificationBundle {
    pub(super) overlap_conflict: KubernetesPlanningEvidence,
    pub(super) narrowed_non_conflict: KubernetesPlanningEvidence,
    pub(super) rebroadened_conflict: KubernetesPlanningEvidence,
    pub(super) revalidated_shared_truth: KubernetesPlanningEvidence,
    pub(super) revalidation_noop: KubernetesNoopEvidence,
    pub(super) broad_intent_replay: RelationalReplayOutcome,
    pub(super) first_converge_replay: RelationalReplayOutcome,
    pub(super) rebroadened_intent_replay: RelationalReplayOutcome,
    pub(super) revalidation_noop_replay: RelationalReplayOutcome,
    pub(super) branch_heads: KubernetesBranchHeadEvidence,
    pub(super) visible_truth: KubernetesVisibleTruthEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesPlanningEvidence {
    pub(super) conflict: KubernetesConflictEvidence,
    pub(super) lowered_plan: crate::merge::data::MergeLoweredPlanDigestBasis,
    pub(super) decision_log: crate::merge::data::MergePlanningDecisionLogDigestBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesConflictEvidence {
    pub(super) records: Arc<[crate::facade::transactions::RecordRef]>,
    pub(super) classes: Arc<[crate::merge::data::MergeConflictClass]>,
    pub(super) validated_schema_correspondence: Arc<[bool]>,
    pub(super) strategy_conflict_classes: Arc<[Option<crate::merge::data::StrategyConflictClass>]>,
    pub(super) source_strategy_descriptors:
        Arc<[Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    pub(super) target_strategy_descriptors:
        Arc<[Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    pub(super) relation_evidence: Arc<[Option<crate::merge::data::RelationConflictEvidence>]>,
    pub(super) aspect_evidence_keys: Arc<[Arc<[forge_foundational::facade::AspectKey]>]>,
    pub(super) aspect_evidence_comparisons: Arc<[Arc<[crate::merge::data::AspectComparisonState]>]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesNoopEvidence {
    pub(super) strategy_artifacts: StrategyCommitArtifactBundle,
    pub(super) changed_record_count: usize,
    pub(super) patch_record_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesBranchHeadEvidence {
    pub(super) main: Option<CommitReference>,
    pub(super) controller: Option<CommitReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KubernetesVisibleTruthEvidence {
    pub(super) entity_name: Option<String>,
    pub(super) replicas_canonical_bytes: Option<Vec<u8>>,
}
