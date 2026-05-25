use forge_proof::TransitionOutcome;

use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
};
use super::test_support::{
    admitted_plan_target, admitted_plan_target_parts, declaration_target, ready, success,
};
use super::{
    materialize_intent_admission_support_traceability_report,
    materialize_query_workflow_declaration, ForgeQuerySupportContributionAuthoring,
    ForgeQuerySupportContributionPayload, ForgeQuerySupportContributionPosture,
    ForgeQueryWorkflowContributionAuthoring, ForgeQueryWorkflowContributionPayload,
    ForgeQueryWorkflowContributionPosture, ForgeQueryWorkflowRuntimeBindingSemantics,
    ForgeQueryWorkflowRuntimeSemantics,
};

#[test]
fn support_traceability_materializer_builds_domain_scoped_report() {
    let report = success(materialize_intent_admission_support_traceability_report(
        ready_support(ForgeQuerySupportContributionAuthoring::declaration_support(
            "spatial.arbitration.support",
            "multiple candidates remain admissible",
        )),
    ));

    assert_eq!(report.rows().len(), 1);
    let row = &report.rows()[0];
    assert_eq!(row.lane(), "domain_support");
    assert_eq!(row.family(), "authoritative-user-intent");
    assert_eq!(row.entrypoint(), "ForgeQueryRuntime::execute_intent");
    assert_eq!(
        row.support_detail(),
        "spatial.arbitration.support:multiple candidates remain admissible"
    );
    assert!(row.target_binding_digest().is_some());
    assert_eq!(row.request_digest(), Some("test.request"));
    assert_eq!(row.eligibility_digest(), Some("test.eligibility"));
    assert_eq!(row.decision_digest(), Some("test.decision"));
}

#[test]
fn equivalent_support_meaning_materializes_same_traceability_digest() {
    let authored = success(materialize_intent_admission_support_traceability_report(
        ready_support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "spatial.traceability",
                "query admission support is declaration-scoped",
            ),
        ),
    ));
    let direct = success(materialize_intent_admission_support_traceability_report(
        ready(
            super::proof_integration::create_requested_domain_capability_contribution(
                ForgeQueryAdmittedPlanBoundContributionTarget::from_digest("plan-support"),
                ForgeQuerySupportContributionPayload::new(
                    ForgeQuerySupportContributionPosture::DeclarationTraceability,
                    "spatial.traceability",
                    "query admission support is declaration-scoped",
                ),
            ),
        ),
    ));

    assert_eq!(
        authored.decision_support_traceability_digest(),
        direct.decision_support_traceability_digest()
    );
}

#[test]
fn support_traceability_digest_changes_when_admitted_plan_scope_changes() {
    let left = success(materialize_intent_admission_support_traceability_report(
        ready(
            super::proof_integration::create_requested_domain_capability_contribution(
                admitted_plan_target_parts(
                    "plan-support-left",
                    "request-left",
                    "eligibility-left",
                    "decision-left",
                ),
                ForgeQuerySupportContributionPayload::new(
                    ForgeQuerySupportContributionPosture::DeclarationSupport,
                    "spatial.traceability",
                    "support stays attached to the admitted plan",
                ),
            ),
        ),
    ));
    let right = success(materialize_intent_admission_support_traceability_report(
        ready(
            super::proof_integration::create_requested_domain_capability_contribution(
                admitted_plan_target_parts(
                    "plan-support-right",
                    "request-right",
                    "eligibility-right",
                    "decision-right",
                ),
                ForgeQuerySupportContributionPayload::new(
                    ForgeQuerySupportContributionPosture::DeclarationSupport,
                    "spatial.traceability",
                    "support stays attached to the admitted plan",
                ),
            ),
        ),
    ));

    assert_ne!(
        left.decision_support_traceability_digest(),
        right.decision_support_traceability_digest()
    );
}

#[test]
fn workflow_materializer_builds_preview_and_runtime_declarations() {
    let preview = success(materialize_query_workflow_declaration(ready_workflow(
        ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            "spatial.preview.only",
            "preview remains read-only",
            "preview-session:42",
        ),
    )));
    let confirmation = success(materialize_query_workflow_declaration(ready_workflow(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection(
            "spatial.confirmation.required",
            "authoritative confirmation is required before writeback",
            "runtime-snapshot:42",
        ),
    )));

    assert_eq!(
        preview.request().declaration_family(),
        &crate::workflow::WorkflowDeclarationFamily::ConflictInspectionNarrow
    );
    assert_eq!(
        preview.binding().basis_family(),
        &crate::workflow::WorkflowBasisFamily::PreviewFoundation
    );
    assert_eq!(
        preview.binding().preview_evaluation_class(),
        Some(&crate::workflow::WorkflowPreviewEvaluationClass::ReadOnly)
    );

    assert_eq!(
        confirmation.binding().basis_family(),
        &crate::workflow::WorkflowBasisFamily::RuntimePreflight
    );
    assert_eq!(
        confirmation.binding().runtime_snapshot_token(),
        Some("runtime-snapshot:42")
    );
}

#[test]
fn workflow_materializer_accepts_admitted_plan_targets() {
    let preview = success(materialize_query_workflow_declaration(ready_workflow_plan(
        ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
            "spatial.preview.lowering",
            "promotion-eligible preview can lower a bounded mutation workflow",
            "preview-session:77",
        ),
        admitted_plan_target_parts(
            "plan-workflow-preview",
            "request-preview",
            "eligibility-preview",
            "decision-preview",
        ),
    )));
    let confirmation = success(materialize_query_workflow_declaration(ready_workflow_plan(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection(
            "spatial.confirmation.runtime",
            "authoritative confirmation requires runtime preflight context",
            "runtime-snapshot:77",
        ),
        admitted_plan_target_parts(
            "plan-workflow-runtime",
            "request-runtime",
            "eligibility-runtime",
            "decision-runtime",
        ),
    )));

    assert_eq!(
        preview.binding().basis_family(),
        &crate::workflow::WorkflowBasisFamily::PreviewFoundation
    );
    assert_eq!(
        preview.binding().preview_evaluation_class(),
        Some(&crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible)
    );
    assert_eq!(
        preview.request().authority_target_family(),
        &crate::workflow::WorkflowAuthorityTargetFamily::RelationalMutation
    );
    assert_eq!(
        confirmation.binding().basis_family(),
        &crate::workflow::WorkflowBasisFamily::RuntimePreflight
    );
    assert_eq!(
        confirmation.binding().runtime_snapshot_token(),
        Some("runtime-snapshot:77")
    );
}

#[test]
fn discard_required_workflow_materializer_uses_promotion_eligible_preview_binding() {
    let discard = success(materialize_query_workflow_declaration(ready_workflow(
        ForgeQueryWorkflowContributionAuthoring::discard_required_query_inspection(
            "spatial.preview.discard",
            "discard-required preview semantics should now carry promotable preview authority at the workflow seam",
            "preview-session:discard",
        ),
    )));

    assert_eq!(
        discard.binding().basis_family(),
        &crate::workflow::WorkflowBasisFamily::PreviewFoundation
    );
    assert_eq!(
        discard.binding().preview_evaluation_class(),
        Some(&crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible)
    );
    assert_eq!(
        discard.request().authority_target_family(),
        &crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection
    );
    assert_eq!(
        discard.binding().preview_request_family(),
        Some(&crate::preview::PreviewWorkflowFoundationRequest::deferred_mutation_writeback())
    );
}

#[test]
fn workflow_materialization_digest_changes_when_declaration_scope_changes() {
    let left = success(materialize_query_workflow_declaration(ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            declaration_target("intent-workflow-left"),
            ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
                ForgeQueryWorkflowContributionPosture::PreviewOnly,
                "spatial.preview.only",
                "preview remains read-only",
                Some(ForgeQueryWorkflowRuntimeSemantics::new(
                    ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                        "preview-session:42",
                        crate::workflow::WorkflowPreviewEvaluationClass::ReadOnly,
                    ),
                    crate::workflow::WorkflowDeclarationFamily::ConflictInspectionNarrow,
                    crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection,
                    crate::workflow::WorkflowCostClass::InspectionNarrow,
                    crate::workflow::WorkflowBudgetClass::InspectionBounded,
                    crate::workflow::WorkflowFreshnessPolicy::ExactBasis,
                )),
            ),
        ),
    )));
    let right = success(materialize_query_workflow_declaration(ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            declaration_target("intent-workflow-right"),
            ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
                ForgeQueryWorkflowContributionPosture::PreviewOnly,
                "spatial.preview.only",
                "preview remains read-only",
                Some(ForgeQueryWorkflowRuntimeSemantics::new(
                    ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                        "preview-session:42",
                        crate::workflow::WorkflowPreviewEvaluationClass::ReadOnly,
                    ),
                    crate::workflow::WorkflowDeclarationFamily::ConflictInspectionNarrow,
                    crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection,
                    crate::workflow::WorkflowCostClass::InspectionNarrow,
                    crate::workflow::WorkflowBudgetClass::InspectionBounded,
                    crate::workflow::WorkflowFreshnessPolicy::ExactBasis,
                )),
            ),
        ),
    )));

    assert_ne!(
        left.report().declaration_digest(),
        right.report().declaration_digest()
    );
    assert_ne!(left.binding().digest(), right.binding().digest());
}

#[test]
fn workflow_materialization_digest_changes_when_admitted_plan_scope_changes() {
    let left = success(materialize_query_workflow_declaration(ready_workflow_plan(
        ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            "spatial.preview.only",
            "preview remains read-only",
            "preview-session:42",
        ),
        admitted_plan_target_parts(
            "plan-workflow-left",
            "request-left",
            "eligibility-left",
            "decision-left",
        ),
    )));
    let right = success(materialize_query_workflow_declaration(ready_workflow_plan(
        ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            "spatial.preview.only",
            "preview remains read-only",
            "preview-session:42",
        ),
        admitted_plan_target_parts(
            "plan-workflow-right",
            "request-right",
            "eligibility-right",
            "decision-right",
        ),
    )));

    assert_ne!(
        left.report().declaration_digest(),
        right.report().declaration_digest()
    );
    assert_ne!(left.binding().digest(), right.binding().digest());
}

#[test]
fn workflow_materializer_denies_missing_or_inconsistent_runtime_semantics() {
    let missing = materialize_query_workflow_declaration(ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            declaration_target("intent-workflow"),
            ForgeQueryWorkflowContributionPayload::new(
                ForgeQueryWorkflowContributionPosture::PreviewOnly,
                "spatial.preview.only",
                "preview remains read-only",
            ),
        ),
    ));
    let inconsistent = materialize_query_workflow_declaration(ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            declaration_target("intent-workflow"),
            ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
                ForgeQueryWorkflowContributionPosture::PreviewOnly,
                "spatial.preview.only",
                "preview remains read-only",
                Some(ForgeQueryWorkflowRuntimeSemantics::new(
                    ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                        "preview-session:42",
                        crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible,
                    ),
                    crate::workflow::WorkflowDeclarationFamily::MutationLoweringNarrow,
                    crate::workflow::WorkflowAuthorityTargetFamily::RelationalMutation,
                    crate::workflow::WorkflowCostClass::MutationLoweringNarrow,
                    crate::workflow::WorkflowBudgetClass::AuthorityTargetBounded,
                    crate::workflow::WorkflowFreshnessPolicy::AllowExplicitRebind,
                )),
            ),
        ),
    ));

    assert!(matches!(
        missing,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
    assert!(matches!(
        inconsistent,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics
    ));
}

fn ready_support(
    authoring: ForgeQuerySupportContributionAuthoring,
) -> super::ForgeQueryMaterializationReadySupportContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(authoring.bind_to_admitted_plan_target(admitted_plan_target("plan-support")))
}

fn ready_workflow(
    authoring: ForgeQueryWorkflowContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyWorkflowContribution<
    ForgeQueryDeclarationBoundContributionTarget,
> {
    ready(authoring.bind_to_declaration_target(declaration_target("intent-workflow")))
}

fn ready_workflow_plan(
    authoring: ForgeQueryWorkflowContributionAuthoring,
    target: ForgeQueryAdmittedPlanBoundContributionTarget,
) -> super::ForgeQueryMaterializationReadyWorkflowContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(authoring.bind_to_admitted_plan_target(target))
}
