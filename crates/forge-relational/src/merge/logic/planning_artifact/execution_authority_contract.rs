use crate::merge::data::{
    MergeExecutionAuthorityContract, MergeExecutionAuthorizationRule,
    MergeExecutionConsumptionRule, MergeExecutionDecisionSurface,
};

pub(super) fn lowered_artifact_execution_authority_contract() -> MergeExecutionAuthorityContract {
    MergeExecutionAuthorityContract {
        decision_surface: MergeExecutionDecisionSurface::LoweredRecordDecisionOnly,
        identity_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        conflict_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        policy_authority: MergeExecutionConsumptionRule::ConsumeCanonicalLoweredArtifactOnly,
        value_authorization:
            MergeExecutionAuthorizationRule::MustNotWidenBeyondAuthorizedAspectValueSurface,
    }
}
