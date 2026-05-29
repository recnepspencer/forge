use crate::effect_lifecycle::{
    scope_admitted_effect_plan, EffectAuthorityOwner, EffectConflictFootprint, EffectFamily,
    EffectInvariantScope, EffectPermittedLoweringFamily, EffectPolicyPosture, EffectPreviewPosture,
    EffectStrategyIdentityTarget,
};

use super::support::{
    admitted_branch_merge_effect, admitted_mutation_effect, admitted_tenant_writeback_effect,
};

#[test]
fn admitted_mutation_scopes_into_one_authority_plan() {
    let plan = scope_admitted_effect_plan(admitted_mutation_effect());

    assert_eq!(plan.family(), EffectFamily::Mutation);
    assert_eq!(
        plan.authority_owner(),
        EffectAuthorityOwner::ForgeRelational
    );
    assert_eq!(
        plan.invariant_scope(),
        EffectInvariantScope::EntityScopedMutation
    );
    assert_eq!(
        plan.preview_posture(),
        EffectPreviewPosture::NotPreviewBound
    );
    assert_eq!(plan.policy_posture(), EffectPolicyPosture::Unmasked);
    assert_eq!(
        plan.permitted_lowering_family(),
        EffectPermittedLoweringFamily::MutationIntentDeclaration
    );
    assert_eq!(
        plan.conflict_footprint(),
        EffectConflictFootprint::EntityMutation
    );
    assert_eq!(
        plan.strategy_identity_target(),
        EffectStrategyIdentityTarget::NativeStrategyCommitRequest
    );
    assert_eq!(plan.counters().authority_scoped_plan_count(), 1);
    assert_eq!(
        plan.counters().effect_support_row_count(),
        plan.admitted()
            .normalized()
            .counters()
            .effect_support_row_count()
    );
    assert_eq!(
        plan.basis_lane().as_str(),
        plan.admitted()
            .workflow_declaration()
            .binding()
            .basis_family()
            .as_str()
    );
    assert!(!plan.plan_digest().is_empty());
}

#[test]
fn plans_make_policy_scope_and_family_distinctions_inspectable_without_lowering() {
    let branch_merge = scope_admitted_effect_plan(admitted_branch_merge_effect());
    let tenant_writeback = scope_admitted_effect_plan(admitted_tenant_writeback_effect());

    assert_eq!(
        branch_merge.preview_posture(),
        EffectPreviewPosture::NotPreviewBound
    );
    assert_eq!(
        branch_merge.permitted_lowering_family(),
        EffectPermittedLoweringFamily::MergeWorkflowDeclaration
    );
    assert_eq!(
        branch_merge.authority_owner(),
        EffectAuthorityOwner::ForgeRelational
    );
    assert_eq!(
        branch_merge.conflict_footprint(),
        EffectConflictFootprint::BranchMerge
    );
    assert_eq!(
        tenant_writeback.policy_posture(),
        EffectPolicyPosture::TenantScoped
    );
    assert_eq!(
        tenant_writeback.permitted_lowering_family(),
        EffectPermittedLoweringFamily::QueryWritebackDeclaration
    );
    assert_eq!(
        tenant_writeback.authority_owner(),
        EffectAuthorityOwner::ForgeRuntimeBridge
    );
    assert_eq!(
        tenant_writeback.conflict_footprint(),
        EffectConflictFootprint::BridgeWriteback
    );
    assert_eq!(
        tenant_writeback.strategy_identity_target(),
        EffectStrategyIdentityTarget::BridgeWritebackDeclaration
    );
}
