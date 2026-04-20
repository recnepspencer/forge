use super::*;
use crate::identity::hash_parts;
use forge_relational::facade::commit_strategies::{
    CommitStrategySemanticName, RawStrategyCommitRequest, StrategyCallerProvenance,
    StrategyRequestOrigin,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::EntityId;
use forge_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use forge_runtime_bridge::facade::{
    BridgeRequestKind, BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity,
    BridgeWritebackEffectClass, BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackStrategyClass,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MutationIntentFamily {
    IntentReconciliation,
}

impl MutationIntentFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentReconciliation => "intent_reconciliation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationalStrategyTarget {
    IntentReconciliation,
}

impl RelationalStrategyTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntentReconciliation => "intent_reconciliation",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MutationLoweringInput {
    IntentReconciliation {
        entity_id: EntityId,
        desired_payload: serde_json::Value,
    },
}

impl MutationLoweringInput {
    pub fn family(&self) -> MutationIntentFamily {
        match self {
            Self::IntentReconciliation { .. } => MutationIntentFamily::IntentReconciliation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeWorkflowIntent {
    ReconcileIntoTarget,
}

impl MergeWorkflowIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReconcileIntoTarget => "reconcile_into_target",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MergeAuthorityTarget {
    PairwiseExecution,
}

impl MergeAuthorityTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PairwiseExecution => "pairwise_execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergeLoweringInput {
    intent: MergeWorkflowIntent,
    target_branch: BranchId,
    source_branch: BranchId,
}

impl MergeLoweringInput {
    pub fn reconcile_into_target(target_branch: BranchId, source_branch: BranchId) -> Self {
        Self {
            intent: MergeWorkflowIntent::ReconcileIntoTarget,
            target_branch,
            source_branch,
        }
    }

    pub fn intent(&self) -> &MergeWorkflowIntent {
        &self.intent
    }

    pub fn target_branch(&self) -> &BranchId {
        &self.target_branch
    }

    pub fn source_branch(&self) -> &BranchId {
        &self.source_branch
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WritebackDeclarationFamily {
    ProjectedStateDiff,
}

impl WritebackDeclarationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProjectedStateDiff => "projected_state_diff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WritebackLoweringInput {
    family: WritebackDeclarationFamily,
}

impl WritebackLoweringInput {
    pub fn projected_state_diff() -> Self {
        Self {
            family: WritebackDeclarationFamily::ProjectedStateDiff,
        }
    }

    pub fn family(&self) -> &WritebackDeclarationFamily {
        &self.family
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowFreshnessBinding {
    RuntimeBasisExact,
    PreviewSessionBound,
    PreviewPromotionBound,
    BridgeAuthorityRebindRequired,
}

impl WorkflowFreshnessBinding {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeBasisExact => "runtime_basis_exact",
            Self::PreviewSessionBound => "preview_session_bound",
            Self::PreviewPromotionBound => "preview_promotion_bound",
            Self::BridgeAuthorityRebindRequired => "bridge_authority_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowStalenessClass {
    ExactBasisPreserved,
    AuthorityValidationRequired,
    StaleDenied,
    ExplicitRebindRequired,
}

impl WorkflowStalenessClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactBasisPreserved => "exact_basis_preserved",
            Self::AuthorityValidationRequired => "authority_validation_required",
            Self::StaleDenied => "stale_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowLoweringFailureClass {
    InvalidWorkflowDeclarationFamily,
    UnsupportedMergeFamily,
    UnsupportedRelationalStrategyTarget,
    UnsupportedWritebackFamily,
    InvalidMergeBranchPairing,
    UnsupportedWritebackCausality,
    StaleWorkflowDenied,
    ExplicitRebindRequired,
    LoweringSerializationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowLoweringError {
    failure_class: WorkflowLoweringFailureClass,
    message: &'static str,
    staleness_class: WorkflowStalenessClass,
    counters: WorkflowLoweringCounters,
}

impl WorkflowLoweringError {
    fn new(
        failure_class: WorkflowLoweringFailureClass,
        message: &'static str,
        staleness_class: WorkflowStalenessClass,
        counters: WorkflowLoweringCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            staleness_class,
            counters,
        }
    }

    pub fn failure_class(&self) -> &WorkflowLoweringFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn staleness_class(&self) -> &WorkflowStalenessClass {
        &self.staleness_class
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WritebackCausalityBinding {
    binding_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    request_kind: BridgeRequestKind,
    causality_digest: String,
}

impl WritebackCausalityBinding {
    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredMutationIntentDeclaration {
    declaration: QueryWorkflowDeclaration,
    mutation_family: MutationIntentFamily,
    strategy_target: RelationalStrategyTarget,
    strategy_request: RawStrategyCommitRequest,
    freshness_binding: WorkflowFreshnessBinding,
    staleness_class: WorkflowStalenessClass,
    lowering_digest: String,
    counters: WorkflowLoweringCounters,
}

impl LoweredMutationIntentDeclaration {
    pub fn declaration(&self) -> &QueryWorkflowDeclaration {
        &self.declaration
    }

    pub fn mutation_family(&self) -> &MutationIntentFamily {
        &self.mutation_family
    }

    pub fn strategy_target(&self) -> &RelationalStrategyTarget {
        &self.strategy_target
    }

    pub fn strategy_request(&self) -> &RawStrategyCommitRequest {
        &self.strategy_request
    }

    pub fn freshness_binding(&self) -> &WorkflowFreshnessBinding {
        &self.freshness_binding
    }

    pub fn staleness_class(&self) -> &WorkflowStalenessClass {
        &self.staleness_class
    }

    pub fn lowering_digest(&self) -> &str {
        &self.lowering_digest
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredMergeWorkflowDeclaration {
    declaration: QueryWorkflowDeclaration,
    merge_intent: MergeWorkflowIntent,
    authority_target: MergeAuthorityTarget,
    merge_request: MergeExecutionRequest,
    freshness_binding: WorkflowFreshnessBinding,
    staleness_class: WorkflowStalenessClass,
    lowering_digest: String,
    counters: WorkflowLoweringCounters,
}

impl LoweredMergeWorkflowDeclaration {
    pub fn declaration(&self) -> &QueryWorkflowDeclaration {
        &self.declaration
    }

    pub fn merge_intent(&self) -> &MergeWorkflowIntent {
        &self.merge_intent
    }

    pub fn authority_target(&self) -> &MergeAuthorityTarget {
        &self.authority_target
    }

    pub fn merge_request(&self) -> &MergeExecutionRequest {
        &self.merge_request
    }

    pub fn freshness_binding(&self) -> &WorkflowFreshnessBinding {
        &self.freshness_binding
    }

    pub fn staleness_class(&self) -> &WorkflowStalenessClass {
        &self.staleness_class
    }

    pub fn lowering_digest(&self) -> &str {
        &self.lowering_digest
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryWritebackDeclaration {
    declaration: QueryWorkflowDeclaration,
    family: WritebackDeclarationFamily,
    causality_binding: WritebackCausalityBinding,
    bridge_declaration: BridgeWritebackDeclaration,
    freshness_binding: WorkflowFreshnessBinding,
    staleness_class: WorkflowStalenessClass,
    lowering_digest: String,
    counters: WorkflowLoweringCounters,
}

impl QueryWritebackDeclaration {
    pub fn declaration(&self) -> &QueryWorkflowDeclaration {
        &self.declaration
    }

    pub fn family(&self) -> &WritebackDeclarationFamily {
        &self.family
    }

    pub fn causality_binding(&self) -> &WritebackCausalityBinding {
        &self.causality_binding
    }

    pub fn bridge_declaration(&self) -> &BridgeWritebackDeclaration {
        &self.bridge_declaration
    }

    pub fn freshness_binding(&self) -> &WorkflowFreshnessBinding {
        &self.freshness_binding
    }

    pub fn staleness_class(&self) -> &WorkflowStalenessClass {
        &self.staleness_class
    }

    pub fn lowering_digest(&self) -> &str {
        &self.lowering_digest
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

pub fn lower_mutation_intent_declaration(
    declaration: &QueryWorkflowDeclaration,
    input: MutationLoweringInput,
) -> Result<LoweredMutationIntentDeclaration, WorkflowLoweringError> {
    ensure_workflow_family(
        declaration,
        WorkflowDeclarationFamily::MutationLoweringNarrow,
    )?;
    let freshness_binding = mutation_freshness_binding(
        declaration.binding(),
        declaration.request().freshness_policy(),
    )?;
    let mutation_family = input.family();
    let strategy_target = match mutation_family {
        MutationIntentFamily::IntentReconciliation => {
            RelationalStrategyTarget::IntentReconciliation
        }
    };
    let (strategy_name, input_bytes) = match input {
        MutationLoweringInput::IntentReconciliation {
            entity_id,
            desired_payload,
        } => (
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            serde_json::to_vec(&serde_json::json!({
                "entity_id": entity_id,
                "desired_payload": desired_payload,
            }))
            .map_err(|_| {
                WorkflowLoweringError::new(
                    WorkflowLoweringFailureClass::LoweringSerializationFailed,
                    "mutation lowering could not serialize the query-owned strategy input",
                    WorkflowStalenessClass::ExactBasisPreserved,
                    lowering_denial_counters(1, LoweringDenialClass::General),
                )
            })?,
        ),
    };

    let request = RawStrategyCommitRequest::new(
        strategy_name,
        input_bytes,
        StrategyCallerProvenance {
            request_origin: StrategyRequestOrigin::Api,
            actor_identity: Some("forge-query".to_string()),
            correlation_id: Some(declaration.report().declaration_digest().to_string()),
        },
    );
    let lowering_digest = hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("mutation_family:{}", mutation_family.as_str()),
        format!("strategy_target:{}", strategy_target.as_str()),
        format!("freshness:{}", freshness_binding.as_str()),
        format!(
            "request_origin:{:?}",
            request.caller_provenance().request_origin
        ),
    ]);

    Ok(LoweredMutationIntentDeclaration {
        declaration: declaration.clone(),
        mutation_family,
        strategy_target,
        strategy_request: request,
        freshness_binding,
        staleness_class: WorkflowStalenessClass::ExactBasisPreserved,
        lowering_digest,
        counters: mutation_lowering_success_counters(1),
    })
}

pub fn lower_merge_workflow_declaration(
    declaration: &QueryWorkflowDeclaration,
    input: MergeLoweringInput,
) -> Result<LoweredMergeWorkflowDeclaration, WorkflowLoweringError> {
    ensure_merge_workflow_family(declaration)?;
    if input.target_branch() == input.source_branch() {
        return Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::InvalidMergeBranchPairing,
            "merge lowering requires distinct target and source branches",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(1, LoweringDenialClass::MergeDenied),
        ));
    }

    let freshness_binding = merge_freshness_binding(declaration.binding());
    if matches!(
        freshness_binding,
        WorkflowFreshnessBinding::PreviewSessionBound
            | WorkflowFreshnessBinding::PreviewPromotionBound
    ) && declaration.request().freshness_policy() == &WorkflowFreshnessPolicy::ExactBasis
    {
        return Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::StaleWorkflowDenied,
            "preview-based merge lowering cannot preserve exact basis without authoritative freshness revalidation",
            WorkflowStalenessClass::StaleDenied,
            lowering_denial_counters(1, LoweringDenialClass::StaleDenied),
        ));
    }
    let staleness_class = match freshness_binding {
        WorkflowFreshnessBinding::RuntimeBasisExact => {
            WorkflowStalenessClass::AuthorityValidationRequired
        }
        WorkflowFreshnessBinding::PreviewSessionBound
        | WorkflowFreshnessBinding::PreviewPromotionBound => {
            WorkflowStalenessClass::AuthorityValidationRequired
        }
        WorkflowFreshnessBinding::BridgeAuthorityRebindRequired => {
            WorkflowStalenessClass::ExplicitRebindRequired
        }
    };
    let merge_request = MergeExecutionRequest {
        target_branch: input.target_branch().clone(),
        source_branch: input.source_branch().clone(),
        merge_intent: MergeIntent::ReconcileIntoTarget,
    };
    let lowering_digest = hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("merge_intent:{}", input.intent().as_str()),
        format!("target_branch:{}", input.target_branch().0),
        format!("source_branch:{}", input.source_branch().0),
        format!("freshness:{}", freshness_binding.as_str()),
        format!("staleness:{}", staleness_class.as_str()),
    ]);

    Ok(LoweredMergeWorkflowDeclaration {
        declaration: declaration.clone(),
        merge_intent: input.intent().clone(),
        authority_target: MergeAuthorityTarget::PairwiseExecution,
        merge_request,
        freshness_binding,
        staleness_class,
        lowering_digest,
        counters: merge_lowering_success_counters(1),
    })
}

pub fn lower_query_writeback_declaration(
    declaration: &QueryWorkflowDeclaration,
    input: WritebackLoweringInput,
) -> Result<QueryWritebackDeclaration, WorkflowLoweringError> {
    ensure_writeback_workflow_family(declaration)?;
    let request_kind = writeback_request_kind(
        declaration.binding(),
        declaration.request().freshness_policy(),
    )?;
    let (family_kind, effect_class, strategy_class, idempotence_class) = match input.family() {
        WritebackDeclarationFamily::ProjectedStateDiff => (
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
    };
    let freshness_binding = WorkflowFreshnessBinding::RuntimeBasisExact;
    let causality_digest = hash_parts(&[
        format!("binding:{}", declaration.binding().digest()),
        format!(
            "basis_family:{}",
            declaration.binding().basis_family().as_str()
        ),
        format!("basis_digest:{}", declaration.binding().basis_digest()),
        format!("request_kind:{request_kind:?}"),
    ]);
    let bridge_declaration = BridgeWritebackDeclaration::writeback_capable(
        BridgeWritebackDeclarationIdentity::new(format!(
            "forge-query:{}",
            declaration.report().declaration_digest()
        )),
        request_kind,
        family_kind,
        effect_class,
        strategy_class,
        hash_parts(&[
            format!("workflow:{}", declaration.report().declaration_digest()),
            format!("family:{}", input.family().as_str()),
            format!("causality:{causality_digest}"),
        ]),
        idempotence_class,
    );
    let lowering_digest = hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("writeback_family:{}", input.family().as_str()),
        format!("bridge_declaration:{}", bridge_declaration.digest()),
        format!("causality:{causality_digest}"),
    ]);

    Ok(QueryWritebackDeclaration {
        declaration: declaration.clone(),
        family: input.family().clone(),
        causality_binding: WritebackCausalityBinding {
            binding_digest: declaration.binding().digest().to_string(),
            basis_family: declaration.binding().basis_family().clone(),
            basis_digest: declaration.binding().basis_digest().to_string(),
            request_kind,
            causality_digest,
        },
        bridge_declaration,
        freshness_binding,
        staleness_class: WorkflowStalenessClass::AuthorityValidationRequired,
        lowering_digest,
        counters: writeback_lowering_success_counters(1),
    })
}

fn ensure_workflow_family(
    declaration: &QueryWorkflowDeclaration,
    expected: WorkflowDeclarationFamily,
) -> Result<(), WorkflowLoweringError> {
    if declaration.request().declaration_family() == &expected {
        Ok(())
    } else {
        Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::InvalidWorkflowDeclarationFamily,
            "workflow lowering entrypoints may only lower their matching declaration family",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(0, LoweringDenialClass::General),
        ))
    }
}

fn ensure_merge_workflow_family(
    declaration: &QueryWorkflowDeclaration,
) -> Result<(), WorkflowLoweringError> {
    if declaration.request().declaration_family() == &WorkflowDeclarationFamily::MergeLoweringNarrow
    {
        Ok(())
    } else {
        Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedMergeFamily,
            "merge lowering entrypoints may only lower admitted merge workflow declarations",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(0, LoweringDenialClass::MergeDenied),
        ))
    }
}

fn ensure_writeback_workflow_family(
    declaration: &QueryWorkflowDeclaration,
) -> Result<(), WorkflowLoweringError> {
    if declaration.request().declaration_family()
        == &WorkflowDeclarationFamily::WritebackLoweringNarrow
    {
        Ok(())
    } else {
        Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedWritebackFamily,
            "writeback lowering entrypoints may only lower admitted writeback workflow declarations",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(0, LoweringDenialClass::WritebackDenied),
        ))
    }
}

fn mutation_freshness_binding(
    binding: &WorkflowContextBinding,
    freshness_policy: &WorkflowFreshnessPolicy,
) -> Result<WorkflowFreshnessBinding, WorkflowLoweringError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(WorkflowFreshnessBinding::RuntimeBasisExact),
        WorkflowBasisFamily::PreviewFoundation
        | WorkflowBasisFamily::PreviewPromotionComparison => {
            let (failure_class, message, staleness_class, denial_class) = if freshness_policy
                == &WorkflowFreshnessPolicy::ExactBasis
            {
                (
                        WorkflowLoweringFailureClass::StaleWorkflowDenied,
                        "mutation lowering cannot preserve exact basis from preview workflow contexts without authoritative revalidation",
                        WorkflowStalenessClass::StaleDenied,
                        LoweringDenialClass::StaleDenied,
                    )
            } else {
                (
                        WorkflowLoweringFailureClass::ExplicitRebindRequired,
                        "mutation lowering remains runtime-basis only until relational authority rebind is explicit",
                        WorkflowStalenessClass::ExplicitRebindRequired,
                        LoweringDenialClass::ExplicitRebind,
                    )
            };
            Err(WorkflowLoweringError::new(
                failure_class,
                message,
                staleness_class,
                lowering_denial_counters(1, denial_class),
            ))
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedRelationalStrategyTarget,
            "correspondence/historical contexts cannot lower query-owned mutation intent",
            WorkflowStalenessClass::ExplicitRebindRequired,
            lowering_denial_counters(1, LoweringDenialClass::AmbientBasisFallback),
        )),
    }
}

fn merge_freshness_binding(binding: &WorkflowContextBinding) -> WorkflowFreshnessBinding {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => WorkflowFreshnessBinding::RuntimeBasisExact,
        WorkflowBasisFamily::PreviewFoundation => WorkflowFreshnessBinding::PreviewSessionBound,
        WorkflowBasisFamily::PreviewPromotionComparison => {
            WorkflowFreshnessBinding::PreviewPromotionBound
        }
        WorkflowBasisFamily::CorrespondenceHistorical => {
            WorkflowFreshnessBinding::BridgeAuthorityRebindRequired
        }
    }
}

fn writeback_request_kind(
    binding: &WorkflowContextBinding,
    freshness_policy: &WorkflowFreshnessPolicy,
) -> Result<BridgeRequestKind, WorkflowLoweringError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(BridgeRequestKind::Authoritative),
        WorkflowBasisFamily::PreviewFoundation
        | WorkflowBasisFamily::PreviewPromotionComparison => {
            let (failure_class, message, staleness_class, denial_class) = if freshness_policy
                == &WorkflowFreshnessPolicy::ExactBasis
            {
                (
                        WorkflowLoweringFailureClass::StaleWorkflowDenied,
                        "query-triggered writeback cannot preserve exact basis from preview workflow contexts without authoritative revalidation",
                        WorkflowStalenessClass::StaleDenied,
                        LoweringDenialClass::StaleDenied,
                    )
            } else {
                (
                        WorkflowLoweringFailureClass::ExplicitRebindRequired,
                        "query-triggered writeback requires authoritative rebind before bridge lowering",
                        WorkflowStalenessClass::ExplicitRebindRequired,
                        LoweringDenialClass::WritebackExplicitRebind,
                    )
            };
            Err(WorkflowLoweringError::new(
                failure_class,
                message,
                staleness_class,
                lowering_denial_counters(1, denial_class),
            ))
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedWritebackCausality,
            "correspondence/historical workflow contexts cannot mint bridge writeback causality",
            WorkflowStalenessClass::ExplicitRebindRequired,
            lowering_denial_counters(1, LoweringDenialClass::AmbientBasisFallback),
        )),
    }
}

fn lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_declaration_count: 1,
        workflow_lowering_count: 1,
        workflow_mutation_lowering_count: 0,
        workflow_merge_lowering_count: 0,
        workflow_lowering_width: width,
        workflow_lowering_denial_count: 0,
        workflow_merge_denial_count: 0,
        workflow_writeback_declaration_count: 0,
        workflow_writeback_denial_count: 0,
        workflow_writeback_causality_binding_count: 0,
        workflow_staleness_check_count: 1,
        workflow_stale_denial_count: 0,
        workflow_lowering_staleness_denial_count: 0,
        workflow_explicit_rebind_required_count: 0,
        workflow_authority_override_denial_count: 0,
        workflow_ambient_basis_fallback_denial_count: 0,
        workflow_replay_bundle_count: 0,
        workflow_budget_cross_count: 0,
        workflow_work_avoided_by_query_lowering_count: width,
        workflow_executor_rediscovery_count: 0,
    }
}

fn mutation_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_mutation_lowering_count: 1,
        ..lowering_success_counters(width)
    }
}

fn merge_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_merge_lowering_count: 1,
        ..lowering_success_counters(width)
    }
}

fn writeback_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_writeback_declaration_count: 1,
        workflow_writeback_causality_binding_count: 1,
        ..lowering_success_counters(width)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoweringDenialClass {
    General,
    MergeDenied,
    WritebackDenied,
    StaleDenied,
    ExplicitRebind,
    WritebackExplicitRebind,
    AmbientBasisFallback,
}

fn lowering_denial_counters(
    width: usize,
    denial_class: LoweringDenialClass,
) -> WorkflowLoweringCounters {
    let is_rebind = matches!(
        denial_class,
        LoweringDenialClass::ExplicitRebind | LoweringDenialClass::WritebackExplicitRebind
    );
    let is_stale_denial = matches!(denial_class, LoweringDenialClass::StaleDenied);
    let is_merge_denial = matches!(denial_class, LoweringDenialClass::MergeDenied);
    let is_writeback_denial = matches!(
        denial_class,
        LoweringDenialClass::WritebackDenied | LoweringDenialClass::WritebackExplicitRebind
    );
    // This remains intentional defense in depth even though correspondence and
    // historical workflow contexts are currently denied during binding. If a
    // future admitted lane reaches lowering without first rebinding authority,
    // the lowering counters still preserve the ambient-basis-fallback denial.
    let is_ambient_basis_fallback =
        matches!(denial_class, LoweringDenialClass::AmbientBasisFallback);
    let budget_cross = usize::from(is_rebind || is_ambient_basis_fallback || is_stale_denial);

    WorkflowLoweringCounters {
        workflow_declaration_count: 1,
        workflow_lowering_count: 1,
        workflow_mutation_lowering_count: 0,
        workflow_merge_lowering_count: 0,
        workflow_lowering_width: width,
        workflow_lowering_denial_count: 1,
        workflow_merge_denial_count: usize::from(is_merge_denial),
        workflow_writeback_declaration_count: 0,
        workflow_writeback_denial_count: usize::from(is_writeback_denial),
        workflow_writeback_causality_binding_count: 0,
        workflow_staleness_check_count: 1,
        workflow_stale_denial_count: usize::from(is_stale_denial),
        workflow_lowering_staleness_denial_count: usize::from(
            is_stale_denial || is_rebind || is_ambient_basis_fallback,
        ),
        workflow_explicit_rebind_required_count: usize::from(is_rebind),
        workflow_authority_override_denial_count: 0,
        workflow_ambient_basis_fallback_denial_count: usize::from(is_ambient_basis_fallback),
        workflow_replay_bundle_count: 0,
        workflow_budget_cross_count: budget_cross,
        workflow_work_avoided_by_query_lowering_count: width,
        workflow_executor_rediscovery_count: 0,
    }
}
