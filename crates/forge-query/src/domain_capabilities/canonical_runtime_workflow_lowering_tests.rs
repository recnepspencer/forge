use forge_proof::TransitionOutcome;
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId};
use forge_runtime_bridge::facade::BridgeRequestKind;
use serde_json::json;

use super::test_support::{
    admitted_plan_target_parts, declaration_target, ready, ready_payload, success,
};
use super::{
    materialize_lowered_merge_workflow_declaration,
    materialize_lowered_mutation_intent_declaration, materialize_query_writeback_lowering,
    ForgeQueryDomainCapabilityProgressionDenialKind, ForgeQueryWorkflowContributionAuthoring,
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
    ForgeQueryWorkflowLoweringSemantics, ForgeQueryWorkflowRuntimeBindingSemantics,
    ForgeQueryWorkflowRuntimeSemantics,
};

#[test]
fn mutation_lowering_materializer_builds_runtime_lowered_declaration() {
    let lowered = success(materialize_lowered_mutation_intent_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_mutation_reconciliation(
            "spatial.workflow.mutation",
            "runtime preflight should lower a relational mutation intent",
            "runtime-snapshot:42",
            "authority-binding:42",
            EntityId::new(PartitionId(1), 41, 0),
            json!({"name":"after"}),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-mutation")),
    )));

    assert_eq!(
        lowered.strategy_request().strategy_name().as_str(),
        "strategy.intent.reconcile"
    );
    assert_eq!(
        lowered.authority_binding().binding_digest(),
        "authority-binding:42"
    );
    assert_eq!(
        lowered.authority_binding().runtime_snapshot_token(),
        Some("runtime-snapshot:42")
    );
    assert_eq!(lowered.counters().workflow_mutation_lowering_count(), 1);
}

#[test]
fn merge_lowering_materializer_builds_preview_bound_lowering() {
    let lowered = success(materialize_lowered_merge_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::promotion_eligible_merge_reconciliation(
            "spatial.workflow.merge",
            "preview promotion should lower a merge workflow declaration",
            crate::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:77"),
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        )
        .bind_to_admitted_plan_target(admitted_plan_target_parts(
            "plan-workflow-merge",
            "request-merge",
            "eligibility-merge",
            "decision-merge",
        )),
    )));

    assert_eq!(
        lowered.merge_request().target_branch,
        BranchId("main".to_string())
    );
    assert_eq!(
        lowered.merge_request().source_branch,
        BranchId("candidate".to_string())
    );
    assert_eq!(
        lowered.freshness_binding().as_str(),
        "preview_session_bound"
    );
    assert_eq!(
        lowered.staleness_class().as_str(),
        "authority_validation_required"
    );
}

#[test]
fn writeback_lowering_materializer_builds_bridge_writeback_declaration() {
    let lowered = success(materialize_query_writeback_lowering(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_writeback_projected_state_diff(
            "spatial.workflow.writeback",
            "runtime preflight should lower a bridge writeback declaration",
            "runtime-snapshot:58",
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-writeback")),
    )));

    assert_eq!(
        lowered.causality_binding().request_kind(),
        BridgeRequestKind::Authoritative
    );
    assert_eq!(
        lowered.bridge_declaration().request_kind(),
        BridgeRequestKind::Authoritative
    );
    assert_eq!(lowered.counters().workflow_writeback_declaration_count(), 1);
}

#[test]
fn workflow_lowering_materializer_denies_missing_lowering_semantics() {
    let denied = materialize_lowered_mutation_intent_declaration(ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            declaration_target("intent-workflow-missing-lowering"),
            ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
                ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
                "spatial.workflow.mutation",
                "missing lowering semantics should deny",
                Some(ForgeQueryWorkflowRuntimeSemantics::new(
                    ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                        "runtime-snapshot:42",
                    ),
                    crate::workflow::WorkflowDeclarationFamily::MutationLoweringNarrow,
                    crate::workflow::WorkflowAuthorityTargetFamily::RelationalMutation,
                    crate::workflow::WorkflowCostClass::MutationLoweringNarrow,
                    crate::workflow::WorkflowBudgetClass::AuthorityTargetBounded,
                    crate::workflow::WorkflowFreshnessPolicy::ExactBasis,
                )),
            ),
        ),
    ));

    assert!(matches!(
        denied,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn workflow_lowering_materializer_preserves_runtime_stale_posture() {
    let stale = materialize_query_writeback_lowering(ready_payload(
        declaration_target("intent-workflow-writeback-stale"),
        ForgeQueryWorkflowContributionPayload::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
            "spatial.workflow.writeback.stale",
            "preview promotion writeback should preserve stale posture at lowering time",
            Some(ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    crate::facade::runtime::BridgePreviewSessionIdentity::new(
                        "preview-session:88",
                    ),
                    crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                crate::workflow::WorkflowDeclarationFamily::WritebackLoweringNarrow,
                crate::workflow::WorkflowAuthorityTargetFamily::BridgeWriteback,
                crate::workflow::WorkflowCostClass::WritebackLoweringNarrow,
                crate::workflow::WorkflowBudgetClass::AuthorityTargetBounded,
                crate::workflow::WorkflowFreshnessPolicy::ExactBasis,
            )),
            Some(ForgeQueryWorkflowLoweringSemantics::writeback(
                crate::workflow::WritebackLoweringInput::projected_state_diff(),
            )),
        ),
    ));

    match stale {
        TransitionOutcome::Stale(stale) => {
            assert_eq!(stale.category(), "workflow-preview");
            assert_eq!(
                stale.bound_target_digest(),
                "intent-workflow-writeback-stale"
            );
        }
        other => panic!("expected stale outcome, got {other:?}"),
    }
}

#[test]
fn discard_required_writeback_lowering_preserves_preview_stale_posture() {
    let stale = materialize_query_writeback_lowering(ready(
        ForgeQueryWorkflowContributionAuthoring::discard_required_writeback_projected_state_diff(
            "spatial.workflow.writeback.discard",
            "discard-required preview writeback should stay stale until authoritative revalidation",
            crate::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:91"),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-writeback-discard")),
    ));

    match stale {
        TransitionOutcome::Stale(stale) => {
            assert_eq!(stale.category(), "workflow-preview");
            assert_eq!(
                stale.bound_target_digest(),
                "intent-workflow-writeback-discard"
            );
        }
        other => panic!("expected stale outcome, got {other:?}"),
    }
}

#[test]
fn workflow_lowering_materializer_preserves_runtime_rebind_posture() {
    let rebind = materialize_query_writeback_lowering(ready_payload(
        declaration_target("intent-workflow-writeback-rebind"),
        ForgeQueryWorkflowContributionPayload::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
            "spatial.workflow.writeback.rebind",
            "preview-scoped writeback should preserve explicit rebind posture",
            Some(ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    crate::facade::runtime::BridgePreviewSessionIdentity::new(
                        "preview-session:89",
                    ),
                    crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                crate::workflow::WorkflowDeclarationFamily::WritebackLoweringNarrow,
                crate::workflow::WorkflowAuthorityTargetFamily::BridgeWriteback,
                crate::workflow::WorkflowCostClass::WritebackLoweringNarrow,
                crate::workflow::WorkflowBudgetClass::AuthorityTargetBounded,
                crate::workflow::WorkflowFreshnessPolicy::AllowExplicitRebind,
            )),
            Some(ForgeQueryWorkflowLoweringSemantics::writeback(
                crate::workflow::WritebackLoweringInput::projected_state_diff(),
            )),
        ),
    ));

    match rebind {
        TransitionOutcome::RebindRequired(rebind) => {
            assert_eq!(rebind.category(), "workflow-preview");
            assert_eq!(
                rebind.bound_target_digest(),
                "intent-workflow-writeback-rebind"
            );
        }
        other => panic!("expected rebind-required outcome, got {other:?}"),
    }
}
