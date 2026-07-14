use worth_relational::facade::history::BranchId;

use crate::ordinary::workflow::WorthQueryWorkflowCounters;
use crate::runtime::{
    WorthQueryOrdinaryAuthorityAdmission, WorthQueryOrdinaryAuthorityFamily,
    WorthQueryRuntimeError, WorthQueryWorkspace,
};

use super::WorthQueryBranchMergeDeclaration;

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryBranchMergeContext {
    pub(crate) authority: WorthQueryOrdinaryAuthorityAdmission,
    pub(crate) target_branch: String,
    pub(crate) source_branch: String,
}

#[derive(Debug)]
pub struct WorthQueryBranchMergeContextStop {
    error: WorthQueryRuntimeError,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryBranchMergeContextStop {
    pub fn error(&self) -> &WorthQueryRuntimeError {
        &self.error
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub fn next_action(&self) -> super::WorthQueryBranchMergeNextAction {
        super::WorthQueryBranchMergeNextAction::ProvideCurrentBranchAuthority
    }
}

pub fn branch_merge(
    workspace: &WorthQueryWorkspace,
    declaration: &WorthQueryBranchMergeDeclaration,
) -> Result<WorthQueryBranchMergeContext, WorthQueryBranchMergeContextStop> {
    let target_branch = declaration.target_branch().to_string();
    let source_branch = declaration.source_branch().to_string();
    let authority = workspace
        .capture_ordinary_merge_authority(
            BranchId(target_branch.clone()),
            BranchId(source_branch.clone()),
        )
        .map_err(|error| WorthQueryBranchMergeContextStop {
            error,
            counters: WorthQueryWorkflowCounters::context_checked(),
        })?;
    debug_assert_eq!(
        authority.family(),
        WorthQueryOrdinaryAuthorityFamily::BranchMerge
    );
    Ok(WorthQueryBranchMergeContext {
        authority,
        target_branch,
        source_branch,
    })
}
