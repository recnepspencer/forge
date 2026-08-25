use worth_relational::facade::transactions::MergeExecutionOutcome;

use super::super::*;
use super::SharedState;

pub(super) fn capture_merge_authority(
    state: &SharedState,
    target_branch: &crate::runtime::WorthQueryAdmittedBranchName,
    source_branch: &crate::runtime::WorthQueryAdmittedBranchName,
) -> Result<WorthQueryBackendMergeAuthority, WorthQueryWorkspaceError> {
    let state = state.borrow();
    let runtime = state.relational_runtime.as_ref().ok_or_else(|| {
        WorthQueryWorkspaceError::new("stateful bridge fixture has no relational merge authority")
    })?;
    WorthQueryBackendMergeAuthority::capture(runtime, target_branch, source_branch)
}

pub(super) fn validate_merge_authority(
    state: &SharedState,
    authority: &WorthQueryBackendMergeAuthority,
) -> Result<(), WorthQueryWorkspaceError> {
    let state = state.borrow();
    let runtime = state.relational_runtime.as_ref().ok_or_else(|| {
        WorthQueryWorkspaceError::new("stateful bridge fixture has no relational merge authority")
    })?;
    authority.validate_against(runtime)
}

pub(super) fn execute_merge(
    state: &SharedState,
    authority: &WorthQueryBackendMergeAuthority,
    declaration: &crate::workflow::LoweredMergeWorkflowDeclaration,
) -> Result<MergeExecutionOutcome, crate::effect_lifecycle::RelationalEffectExecutionFailure> {
    let mut state = state.borrow_mut();
    let runtime = state.relational_runtime.as_mut().ok_or_else(|| {
        (
            crate::effect_lifecycle::EffectExecutionDenialKind::MissingRelationalAuthority,
            "stateful bridge fixture has no relational merge authority".to_string(),
        )
    })?;
    if declaration.merge_request().target_branch() != authority.target_branch()
        || declaration.merge_request().source_branch() != authority.source_branch()
    {
        return Err((
            crate::effect_lifecycle::EffectExecutionDenialKind::AuthorityOverrideRejected,
            "lowered merge request does not match fixture authority".to_string(),
        )
            .into());
    }
    crate::effect_lifecycle::execute_lowered_merge(runtime, declaration)
}
