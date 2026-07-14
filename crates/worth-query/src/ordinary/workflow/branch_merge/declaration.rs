use crate::ordinary::WorthQueryOrdinaryInspectionPolicy;
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};

use super::{WorthQueryBranchMergeContext, WorthQueryBranchMergeRequest};
use crate::ordinary::workflow::WorthQueryWorkflowFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBranchMergeDeclarationDenialKind {
    EmptyTargetBranch,
    EmptySourceBranch,
    SameBranch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBranchMergeDeclarationStop {
    denial_kind: WorthQueryBranchMergeDeclarationDenialKind,
    message: &'static str,
}

impl WorthQueryBranchMergeDeclarationStop {
    pub fn denial_kind(&self) -> WorthQueryBranchMergeDeclarationDenialKind {
        self.denial_kind
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBranchMergeDeclarationIdentity {
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryBranchMergeDeclarationIdentity {
    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn as_str(&self) -> &str {
        self.identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBranchMergeDeclaration {
    identity: WorthQueryBranchMergeDeclarationIdentity,
    target_branch: String,
    source_branch: String,
    inspection_policy: WorthQueryOrdinaryInspectionPolicy,
}

impl WorthQueryBranchMergeDeclaration {
    pub fn identity(&self) -> &WorthQueryBranchMergeDeclarationIdentity {
        &self.identity
    }

    pub fn family(&self) -> WorthQueryWorkflowFamily {
        WorthQueryWorkflowFamily::BranchMerge
    }

    pub fn target_branch(&self) -> &str {
        &self.target_branch
    }

    pub fn source_branch(&self) -> &str {
        &self.source_branch
    }

    pub fn with_rich_inspection(mut self) -> Self {
        self.inspection_policy = WorthQueryOrdinaryInspectionPolicy::Rich;
        self
    }

    pub fn using(self, context: WorthQueryBranchMergeContext) -> WorthQueryBranchMergeRequest {
        WorthQueryBranchMergeRequest {
            declaration: self,
            context,
        }
    }

    pub(crate) fn inspection_policy(&self) -> WorthQueryOrdinaryInspectionPolicy {
        self.inspection_policy
    }
}

pub fn declare_branch_merge(
    target_branch: impl Into<String>,
    source_branch: impl Into<String>,
) -> Result<WorthQueryBranchMergeDeclaration, WorthQueryBranchMergeDeclarationStop> {
    let target_branch = target_branch.into().trim().to_string();
    let source_branch = source_branch.into().trim().to_string();
    if target_branch.is_empty() {
        return Err(stop(
            WorthQueryBranchMergeDeclarationDenialKind::EmptyTargetBranch,
            "branch merge target may not be empty",
        ));
    }
    if source_branch.is_empty() {
        return Err(stop(
            WorthQueryBranchMergeDeclarationDenialKind::EmptySourceBranch,
            "branch merge source may not be empty",
        ));
    }
    if target_branch == source_branch {
        return Err(stop(
            WorthQueryBranchMergeDeclarationDenialKind::SameBranch,
            "branch merge target and source must be distinct",
        ));
    }
    let identity = WorthQueryBranchMergeDeclarationIdentity {
        identity: WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::WorkflowMutationLowering,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("role"),
            "ordinary-branch-merge-declaration",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("family"),
            WorthQueryWorkflowFamily::BranchMerge.as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("target_branch"), &target_branch)
        .field_shape(WorthQueryEvidenceTag::new("source_branch"), &source_branch)
        .seal(),
    };
    Ok(WorthQueryBranchMergeDeclaration {
        identity,
        target_branch,
        source_branch,
        inspection_policy: WorthQueryOrdinaryInspectionPolicy::OperationalOnly,
    })
}

fn stop(
    denial_kind: WorthQueryBranchMergeDeclarationDenialKind,
    message: &'static str,
) -> WorthQueryBranchMergeDeclarationStop {
    WorthQueryBranchMergeDeclarationStop {
        denial_kind,
        message,
    }
}
