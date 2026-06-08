#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeServerWorkspaceTarget {
    tenant_id: String,
    workspace_id: String,
}

impl ForgeServerWorkspaceTarget {
    pub(crate) fn new(tenant_id: String, workspace_id: String) -> Self {
        Self {
            tenant_id,
            workspace_id,
        }
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }
}
