use crate::basis_lifecycle::basis_lifecycle;
use crate::effect_lifecycle::{
    admit_effect_intent, evaluate_effect_eligibility, normalize_raw_effect_intent,
    scope_admitted_effect_plan, EffectDiagnosticsMaterialization, EffectDiagnosticsRequest,
    EffectEligibility, EffectEligibilityOutcome, EffectExecutionReceipt,
    ExecutedEffectAuthorityArtifact, ExecutedEffectPlan, RawEffectIntent,
};
use crate::workflow::{
    synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity,
    MergeLoweringInput, WorkflowAuthorityTargetFamily, WorkflowBindingScopeField,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy,
};
use crate::WorthQueryEvidenceIdentity;

use super::super::{WorthQueryRuntime, WorthQueryValidatedMergeAuthority};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryOrdinaryMergeFailureStage {
    Authority,
    Basis,
    Intent,
    Eligibility,
    Lowering,
    RelationalExecution,
}

pub(crate) struct WorthQueryOrdinaryMergeExecutionError {
    stage: WorthQueryOrdinaryMergeFailureStage,
    message: String,
}

impl WorthQueryOrdinaryMergeExecutionError {
    fn new(stage: WorthQueryOrdinaryMergeFailureStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    pub(crate) fn stage(&self) -> WorthQueryOrdinaryMergeFailureStage {
        self.stage
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}

pub(crate) struct WorthQueryLowerRuntimeMergeExecution {
    admitted_effect_identity: WorthQueryEvidenceIdentity,
    lowered_plan_identity: WorthQueryEvidenceIdentity,
    receipt: EffectExecutionReceipt,
    diagnostics: Option<EffectDiagnosticsMaterialization>,
}

impl WorthQueryLowerRuntimeMergeExecution {
    pub(crate) fn admitted_effect_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admitted_effect_identity
    }

    pub(crate) fn lowered_plan_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.lowered_plan_identity
    }

    pub(crate) fn receipt(&self) -> &EffectExecutionReceipt {
        &self.receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        EffectExecutionReceipt,
        Option<EffectDiagnosticsMaterialization>,
    ) {
        (self.receipt, self.diagnostics)
    }
}

impl WorthQueryRuntime {
    pub(crate) fn execute_ordinary_merge(
        &mut self,
        authority: WorthQueryValidatedMergeAuthority,
        declaration_identity: &WorthQueryEvidenceIdentity,
        target_branch: worth_relational::facade::history::BranchId,
        source_branch: worth_relational::facade::history::BranchId,
        materialize_inspection: bool,
    ) -> Result<WorthQueryLowerRuntimeMergeExecution, WorthQueryOrdinaryMergeExecutionError> {
        let merge_authority = authority.backend_authority();
        if merge_authority.target_branch() != &target_branch
            || merge_authority.source_branch() != &source_branch
        {
            return Err(WorthQueryOrdinaryMergeExecutionError::new(
                WorthQueryOrdinaryMergeFailureStage::Authority,
                "ordinary branch-merge declaration does not match its captured authority",
            ));
        }
        let scope = WorkflowBindingScopeField::Identity(declaration_identity);
        let binding =
            synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
                "ordinary-declarative-branch-merge",
                &scope,
                authority.snapshot_identity().clone(),
                target_branch.clone(),
            );
        let basis = basis_lifecycle()
            .branch_head(&target_branch.0, true)
            .prepare_mutation()
            .map_err(|error| {
                WorthQueryOrdinaryMergeExecutionError::new(
                    WorthQueryOrdinaryMergeFailureStage::Basis,
                    format!("{error:?}"),
                )
            })?;
        let request = WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        );
        let normalized = normalize_raw_effect_intent(
            &basis.into(),
            RawEffectIntent::Merge {
                binding,
                request,
                input: MergeLoweringInput::reconcile_into_target(target_branch, source_branch),
            },
        )
        .map_err(|denial| {
            WorthQueryOrdinaryMergeExecutionError::new(
                WorthQueryOrdinaryMergeFailureStage::Intent,
                denial.message(),
            )
        })?;
        let eligibility = admitted_eligibility(evaluate_effect_eligibility(normalized))?;
        let admitted = admit_effect_intent(eligibility);
        let admitted_effect_identity = admitted.admitted_identity().clone();
        let lowered = scope_admitted_effect_plan(admitted)
            .lower()
            .map_err(|denial| {
                WorthQueryOrdinaryMergeExecutionError::new(
                    WorthQueryOrdinaryMergeFailureStage::Lowering,
                    denial.message(),
                )
            })?;
        let lowered_plan_identity = lowered.lowered_effect_execution_plan_identity().clone();
        let declaration = lowered.as_merge().cloned().ok_or_else(|| {
            WorthQueryOrdinaryMergeExecutionError::new(
                WorthQueryOrdinaryMergeFailureStage::Lowering,
                "branch-merge effect lowering produced a non-merge artifact",
            )
        })?;
        let outcome = self
            .backend
            .execute_query_merge(merge_authority, &declaration)
            .map_err(|(kind, message)| {
                WorthQueryOrdinaryMergeExecutionError::new(
                    WorthQueryOrdinaryMergeFailureStage::RelationalExecution,
                    format!("{}: {message}", kind.as_str()),
                )
            })?;
        let receipt =
            ExecutedEffectPlan::new(lowered, ExecutedEffectAuthorityArtifact::Merge(outcome), 1)
                .receipt();
        let diagnostics = materialize_inspection
            .then(|| receipt.materialize_diagnostics(EffectDiagnosticsRequest::forensic()));
        Ok(WorthQueryLowerRuntimeMergeExecution {
            admitted_effect_identity,
            lowered_plan_identity,
            receipt,
            diagnostics,
        })
    }
}

fn admitted_eligibility(
    outcome: EffectEligibilityOutcome,
) -> Result<EffectEligibility, WorthQueryOrdinaryMergeExecutionError> {
    match outcome {
        EffectEligibilityOutcome::Admitted(eligibility) => Ok(eligibility),
        EffectEligibilityOutcome::Advisory(outcome) => {
            Err(eligibility_error(outcome.decision_trace().message()))
        }
        EffectEligibilityOutcome::Denied(outcome) => {
            Err(eligibility_error(outcome.decision_trace().message()))
        }
        EffectEligibilityOutcome::RebindRequired(outcome) => {
            Err(eligibility_error(outcome.decision_trace().message()))
        }
        EffectEligibilityOutcome::Deferred(outcome) => {
            Err(eligibility_error(outcome.decision_trace().message()))
        }
    }
}

fn eligibility_error(message: &str) -> WorthQueryOrdinaryMergeExecutionError {
    WorthQueryOrdinaryMergeExecutionError::new(
        WorthQueryOrdinaryMergeFailureStage::Eligibility,
        message,
    )
}
