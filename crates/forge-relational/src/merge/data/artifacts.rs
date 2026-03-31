use serde::{Deserialize, Serialize};

use crate::history::data::CommitId;
use crate::identity::data::KindId;
use crate::merge::data::{
    AspectComparisonState, AspectMergePolicyDeclaration, AspectMergePolicyKind,
    AuthorizedAspectValueSurface, CausalAnnotationSummary, ConflictClassificationSummary,
    IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope, IdentityDiscoverySummary,
    LoweredAspectAction, LoweredAspectDenialIntent, LoweredAspectExecutionIntent,
    LoweredMergeAction, LoweredMergeBlockedReason, LoweredMergePlanSummary,
    LoweredMergeRejectedReason, LoweredRecordDecisionKind, LoweredRecordDenialKind,
    LoweredRecordExecutionIntentKind, MergeAncestrySummary, MergeConflictClass,
    MergeExecutableClass, MergeExecutionReadiness, MergePlanningDecisionLog,
    MergePlanningDecisionLogDigestBasis, MergePlanningRequest, MergePolicyDecisionBoundary,
    MergePolicyOwnershipClass, MergePolicyProofBoundary, MergePolicyResolutionSummary,
    MergeResolutionClass, MergeVisibilityEvidence, RelationConflictEvidence,
    ResolvedAspectMergePolicy, ResolvedMergeBase, StrategyConflictClass,
};
use crate::schema::data::{
    AspectPlanRevision, RelationIntegrityPlanRevision, RelationPayloadClass, SchemaId,
    SchemaVersionId,
};
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningSummary {
    pub request_summary: String,
    pub ancestry_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeRequestDigestBasis {
    pub target_branch: crate::history::data::BranchId,
    pub source_branch: crate::history::data::BranchId,
    pub merge_intent: crate::merge::data::MergeIntent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeBaseDigestBasis {
    pub rule: crate::merge::data::MergeBaseSelectionRule,
    pub commit_id: CommitId,
    pub supporting_left_ancestors: std::sync::Arc<[CommitId]>,
    pub supporting_right_ancestors: std::sync::Arc<[CommitId]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeIdentityDigestBasis {
    pub effective_declarations: std::sync::Arc<[IdentityBasisDeclaration]>,
    pub candidate_scopes: std::sync::Arc<[Option<IdentityBasisScope>]>,
    pub candidate_sources: std::sync::Arc<[RecordRef]>,
    pub candidate_targets: std::sync::Arc<[Option<RecordRef>]>,
    pub candidate_bases: std::sync::Arc<[IdentityBasisKind]>,
    pub candidate_match_classes: std::sync::Arc<[crate::merge::data::IdentityMatchClass]>,
    pub candidate_reasons: std::sync::Arc<[crate::merge::data::IdentityResolutionReason]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeCausalDigestBasis {
    pub records: std::sync::Arc<[RecordRef]>,
    pub dispositions: std::sync::Arc<[crate::merge::data::MergeRecordCausalDisposition]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeConflictDigestBasis {
    pub records: std::sync::Arc<[RecordRef]>,
    pub classes: std::sync::Arc<[MergeConflictClass]>,
    pub validated_schema_correspondence: std::sync::Arc<[bool]>,
    pub strategy_conflict_classes: std::sync::Arc<[Option<StrategyConflictClass>]>,
    pub source_strategy_descriptors:
        std::sync::Arc<[std::sync::Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    pub target_strategy_descriptors:
        std::sync::Arc<[std::sync::Arc<[crate::commit_strategies::data::StrategyMergeDescriptor]>]>,
    pub relation_evidence: std::sync::Arc<[Option<RelationConflictEvidence>]>,
    pub source_visibility_evidence: std::sync::Arc<[MergeVisibilityEvidence]>,
    pub target_visibility_evidence: std::sync::Arc<[MergeVisibilityEvidence]>,
    pub base_visibility_evidence: std::sync::Arc<[MergeVisibilityEvidence]>,
    pub aspect_evidence_keys:
        std::sync::Arc<[std::sync::Arc<[crate::publication::patch::data::AspectKey]>]>,
    pub aspect_evidence_comparisons:
        std::sync::Arc<[std::sync::Arc<[crate::merge::data::AspectComparisonState]>]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePolicyDigestBasis {
    pub records: std::sync::Arc<[RecordRef]>,
    pub proof_boundaries: std::sync::Arc<[MergePolicyProofBoundary]>,
    pub applied_policies: std::sync::Arc<[std::sync::Arc<[ResolvedAspectMergePolicy]>]>,
    pub aspect_rows: std::sync::Arc<[std::sync::Arc<[MergePolicyAspectDigestRow]>]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeLoweredPlanDigestBasis {
    pub records: std::sync::Arc<[RecordRef]>,
    pub readiness: std::sync::Arc<[MergeExecutionReadiness]>,
    pub resolution_classes: std::sync::Arc<[MergeResolutionClass]>,
    pub executable_classes: std::sync::Arc<[Option<MergeExecutableClass>]>,
    pub record_decisions: std::sync::Arc<[LoweredRecordDecisionKind]>,
    pub lowered_actions: std::sync::Arc<[Option<LoweredMergeAction>]>,
    pub blocked_reasons: std::sync::Arc<[Option<LoweredMergeBlockedReason>]>,
    pub rejected_reasons: std::sync::Arc<[Option<LoweredMergeRejectedReason>]>,
    pub execution_bundle_kinds: std::sync::Arc<[Option<LoweredRecordExecutionIntentKind>]>,
    pub denial_bundle_kinds: std::sync::Arc<[Option<LoweredRecordDenialKind>]>,
    pub aspect_rows: std::sync::Arc<[std::sync::Arc<[MergeLoweredAspectDigestRow]>]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeExecutionDecisionSurface {
    LoweredRecordDecisionOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeExecutionConsumptionRule {
    ConsumeCanonicalLoweredArtifactOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeExecutionAuthorizationRule {
    MustNotWidenBeyondAuthorizedAspectValueSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionAuthorityContract {
    pub decision_surface: MergeExecutionDecisionSurface,
    pub identity_authority: MergeExecutionConsumptionRule,
    pub conflict_authority: MergeExecutionConsumptionRule,
    pub policy_authority: MergeExecutionConsumptionRule,
    pub value_authorization: MergeExecutionAuthorizationRule,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePolicyAspectDigestRow {
    pub aspect_key: crate::publication::patch::data::AspectKey,
    pub comparison: AspectComparisonState,
    pub applied_policy: Option<AspectMergePolicyKind>,
    pub policy_ownership: Option<MergePolicyOwnershipClass>,
    pub decision_boundary: MergePolicyDecisionBoundary,
    pub resolved_value_strategy: Option<crate::merge::data::MergeResolvedAspectValueStrategy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeLoweredAspectDigestRow {
    pub aspect_key: crate::publication::patch::data::AspectKey,
    pub readiness: MergeExecutionReadiness,
    pub lowered_action: Option<LoweredAspectAction>,
    pub authorized_values: Option<AuthorizedAspectValueSurface>,
    pub execution_intent: Option<LoweredAspectExecutionIntent>,
    pub resolved_value_strategy: Option<crate::merge::data::MergeResolvedAspectValueStrategy>,
    pub denial_intent: Option<LoweredAspectDenialIntent>,
    pub blocked_reason: Option<LoweredMergeBlockedReason>,
    pub rejected_reason: Option<LoweredMergeRejectedReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MergeSchemaKindClass {
    Entity,
    Relation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeSchemaKindSemanticSnapshot {
    pub kind_class: MergeSchemaKindClass,
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub aspect_plan_revision: AspectPlanRevision,
    pub identity_declarations: Vec<IdentityBasisDeclaration>,
    pub merge_policy_declarations: Vec<AspectMergePolicyDeclaration>,
    pub relation_payload_class: Option<RelationPayloadClass>,
    pub relation_integrity_plan_revision: Option<RelationIntegrityPlanRevision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeSchemaSnapshotDigestBasis {
    pub authoritative_schema_id: Option<SchemaId>,
    pub authoritative_schema_version_id: Option<SchemaVersionId>,
    pub registry_digest: String,
    pub touched_kinds: std::sync::Arc<[MergeSchemaKindSemanticSnapshot]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeArtifactDigestBasis {
    pub request: MergeRequestDigestBasis,
    pub schema: MergeSchemaSnapshotDigestBasis,
    pub execution_contract: MergeExecutionAuthorityContract,
    pub merge_base: MergeBaseDigestBasis,
    pub identity: MergeIdentityDigestBasis,
    pub causal: MergeCausalDigestBasis,
    pub conflict: MergeConflictDigestBasis,
    pub policy: MergePolicyDigestBasis,
    pub lowered_plan: MergeLoweredPlanDigestBasis,
    pub decision_log: MergePlanningDecisionLogDigestBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningArtifactCore {
    pub request: MergePlanningRequest,
    pub schema_snapshot: MergeSchemaSnapshotDigestBasis,
    pub execution_authority_contract: MergeExecutionAuthorityContract,
    pub merge_base: ResolvedMergeBase,
    pub ancestry: MergeAncestrySummary,
    pub identity_discovery: IdentityDiscoverySummary,
    pub conflict_classification: ConflictClassificationSummary,
    pub causal_annotation: CausalAnnotationSummary,
    pub policy_resolution: MergePolicyResolutionSummary,
    pub lowered_plan: LoweredMergePlanSummary,
    pub decision_log: MergePlanningDecisionLog,
    pub digest_basis: MergeArtifactDigestBasis,
    pub decision_log_digest_basis: MergePlanningDecisionLogDigestBasis,
    pub summary: MergePlanningSummary,
}
