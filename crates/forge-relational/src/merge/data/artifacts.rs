use serde::{Deserialize, Serialize};

use crate::history::data::CommitId;
use crate::identity::data::KindId;
use crate::merge::data::{
    merge_inspection_artifact_digest, merge_inspection_lowered_plan_digest,
    merge_inspection_row_digest, AspectComparisonState, AspectMergePolicyDeclaration,
    AspectMergePolicyKind, AuthorizedAspectValueSurface, CausalAnnotationSummary,
    ConflictClassificationSummary, IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope,
    IdentityDiscoverySummary, LoweredAspectAction, LoweredAspectDenialIntent,
    LoweredAspectExecutionIntent, LoweredMergeAction, LoweredMergeBlockedReason,
    LoweredMergePlanSummary, LoweredMergeRejectedReason, LoweredRecordDecisionKind,
    LoweredRecordDenialKind, LoweredRecordExecutionIntentKind, MergeAncestrySummary,
    MergeConflictClass, MergeExecutableClass, MergeExecutionReadiness, MergePlanningDecisionLog,
    MergePlanningDecisionLogDigestBasis, MergePlanningRequest, MergePolicyDecisionBoundary,
    MergePolicyOwnershipClass, MergePolicyProofBoundary, MergePolicyResolutionSummary,
    MergeResolutionClass, MergeVisibilityEvidence, RelationConflictEvidence,
    ResolvedAspectMergePolicy, ResolvedMergeBase, StrategyConflictClass,
};
use crate::schema::data::{
    AspectPlanRevision, RelationIntegrityPlanRevision, SchemaId, SchemaVersionId,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalMergeInspectionAdmission {
    ExecutionAdmissible,
    ExecutionDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalMergeInspectionInput {
    request: crate::merge::data::MergeExecutionRequest,
    lowered_plan: LoweredMergePlanSummary,
}

impl RelationalMergeInspectionInput {
    fn from_planning_artifact(artifact: &MergePlanningArtifactCore) -> Self {
        Self {
            request: crate::merge::data::MergeExecutionRequest::from(artifact.request.clone()),
            lowered_plan: artifact.lowered_plan.clone(),
        }
    }

    pub fn request(&self) -> &crate::merge::data::MergeExecutionRequest {
        &self.request
    }

    pub fn lowered_plan(&self) -> &LoweredMergePlanSummary {
        &self.lowered_plan
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalMergeInspectionRow {
    record: RecordRef,
    target_record: Option<RecordRef>,
    classification: MergeConflictClass,
    resolution_class: MergeResolutionClass,
    readiness: MergeExecutionReadiness,
    decision_kind: LoweredRecordDecisionKind,
    blocked_reason: Option<LoweredMergeBlockedReason>,
    rejected_reason: Option<LoweredMergeRejectedReason>,
    admission: RelationalMergeInspectionAdmission,
    row_digest: String,
}

impl RelationalMergeInspectionRow {
    pub fn record(&self) -> &RecordRef {
        &self.record
    }

    pub fn target_record(&self) -> Option<&RecordRef> {
        self.target_record.as_ref()
    }

    pub fn classification(&self) -> &MergeConflictClass {
        &self.classification
    }

    pub fn resolution_class(&self) -> &MergeResolutionClass {
        &self.resolution_class
    }

    pub fn readiness(&self) -> &MergeExecutionReadiness {
        &self.readiness
    }

    pub fn decision_kind(&self) -> LoweredRecordDecisionKind {
        self.decision_kind
    }

    pub fn blocked_reason(&self) -> Option<LoweredMergeBlockedReason> {
        self.blocked_reason
    }

    pub fn rejected_reason(&self) -> Option<LoweredMergeRejectedReason> {
        self.rejected_reason
    }

    pub fn admission(&self) -> RelationalMergeInspectionAdmission {
        self.admission
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalMergeInspectionArtifact {
    request: crate::merge::data::MergeExecutionRequest,
    lowered_plan_digest: String,
    rows: std::sync::Arc<[RelationalMergeInspectionRow]>,
    artifact_digest: String,
}

impl RelationalMergeInspectionArtifact {
    pub fn from_input(input: RelationalMergeInspectionInput) -> Self {
        let RelationalMergeInspectionInput {
            request,
            lowered_plan,
        } = input;
        let rows = lowered_plan
            .records
            .iter()
            .map(RelationalMergeInspectionRow::from_lowered_record)
            .collect::<Vec<_>>();
        let lowered_plan_digest = merge_inspection_lowered_plan_digest(
            &request,
            &rows,
            lowered_plan.record_count,
            lowered_plan.blocked_count,
            lowered_plan.rejected_count,
        );
        let artifact_digest =
            merge_inspection_artifact_digest(&request, &lowered_plan_digest, &rows);

        Self {
            request,
            lowered_plan_digest,
            rows: std::sync::Arc::from(rows),
            artifact_digest,
        }
    }

    pub fn request(&self) -> &crate::merge::data::MergeExecutionRequest {
        &self.request
    }

    pub fn lowered_plan_digest(&self) -> &str {
        &self.lowered_plan_digest
    }

    pub fn rows(&self) -> &[RelationalMergeInspectionRow] {
        &self.rows
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

impl MergePlanningArtifactCore {
    pub fn inspection_input(&self) -> RelationalMergeInspectionInput {
        RelationalMergeInspectionInput::from_planning_artifact(self)
    }
}

impl RelationalMergeInspectionRow {
    fn from_lowered_record(record: &crate::merge::data::LoweredMergePlanRecord) -> Self {
        let decision_kind = match &record.record_decision {
            crate::merge::data::LoweredRecordDecision::Execute(_) => {
                LoweredRecordDecisionKind::Execute
            }
            crate::merge::data::LoweredRecordDecision::Block(_) => LoweredRecordDecisionKind::Block,
            crate::merge::data::LoweredRecordDecision::Reject(_) => {
                LoweredRecordDecisionKind::Reject
            }
        };
        let admission = match decision_kind {
            LoweredRecordDecisionKind::Execute => {
                RelationalMergeInspectionAdmission::ExecutionAdmissible
            }
            LoweredRecordDecisionKind::Block | LoweredRecordDecisionKind::Reject => {
                RelationalMergeInspectionAdmission::ExecutionDenied
            }
        };
        let row_digest = merge_inspection_row_digest(
            &record.record,
            record.target_record.as_ref(),
            &record.classification,
            &record.resolution_class,
            &record.readiness,
            decision_kind,
            record.blocked_reason,
            record.rejected_reason,
            admission,
        );

        Self {
            record: record.record.clone(),
            target_record: record.target_record.clone(),
            classification: record.classification,
            resolution_class: record.resolution_class,
            readiness: record.readiness,
            decision_kind,
            blocked_reason: record.blocked_reason,
            rejected_reason: record.rejected_reason,
            admission,
            row_digest,
        }
    }
}
