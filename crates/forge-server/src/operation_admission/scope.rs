#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationScope {
    Workspace {
        tenant_id: String,
        workspace_id: String,
    },
    WorkspaceBranch {
        tenant_id: String,
        workspace_id: String,
        branch_label: String,
    },
    ProductDraft {
        tenant_id: String,
        workspace_id: String,
        product_session_identity: String,
        draft_scope: String,
    },
    SyncLease {
        tenant_id: String,
        workspace_id: String,
        branch_label: String,
        lease_target: String,
    },
}

impl ForgeServerOperationScope {
    pub(crate) fn workspace(tenant_id: impl Into<String>, workspace_id: impl Into<String>) -> Self {
        Self::Workspace {
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
        }
    }

    pub(crate) fn workspace_branch(
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
        branch_label: impl Into<String>,
    ) -> Self {
        Self::WorkspaceBranch {
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
            branch_label: branch_label.into(),
        }
    }

    pub(crate) fn product_draft(
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
        product_session_identity: impl Into<String>,
        draft_scope: impl Into<String>,
    ) -> Self {
        Self::ProductDraft {
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
            product_session_identity: product_session_identity.into(),
            draft_scope: draft_scope.into(),
        }
    }

    pub(crate) fn sync_lease(
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
        branch_label: impl Into<String>,
        lease_target: impl Into<String>,
    ) -> Self {
        Self::SyncLease {
            tenant_id: tenant_id.into(),
            workspace_id: workspace_id.into(),
            branch_label: branch_label.into(),
            lease_target: lease_target.into(),
        }
    }

    pub(crate) fn canonical_digest(&self) -> String {
        match self {
            Self::Workspace {
                tenant_id,
                workspace_id,
            } => format!(
                "forge-server-operation-scope-v1|kind=workspace|tenant={tenant_id}|workspace={workspace_id}"
            ),
            Self::WorkspaceBranch {
                tenant_id,
                workspace_id,
                branch_label,
            } => format!(
                "forge-server-operation-scope-v1|kind=workspace-branch|tenant={tenant_id}|workspace={workspace_id}|branch={branch_label}"
            ),
            Self::ProductDraft {
                tenant_id,
                workspace_id,
                product_session_identity,
                draft_scope,
            } => format!(
                "forge-server-operation-scope-v1|kind=product-draft|tenant={tenant_id}|workspace={workspace_id}|session={product_session_identity}|draft={draft_scope}"
            ),
            Self::SyncLease {
                tenant_id,
                workspace_id,
                branch_label,
                lease_target,
            } => format!(
                "forge-server-operation-scope-v1|kind=sync-lease|tenant={tenant_id}|workspace={workspace_id}|branch={branch_label}|target={lease_target}"
            ),
        }
    }

    pub(crate) fn breadth(&self) -> usize {
        match self {
            Self::Workspace { .. } => 2,
            Self::WorkspaceBranch { .. } => 3,
            Self::ProductDraft { .. } | Self::SyncLease { .. } => 4,
        }
    }
}
