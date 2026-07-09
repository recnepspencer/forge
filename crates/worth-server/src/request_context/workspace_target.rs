#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthServerWorkspaceTarget {
    tenant_id: String,
    workspace_id: String,
}

impl WorthServerWorkspaceTarget {
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

    pub fn canonical_label(&self) -> String {
        format!("{}:{}", self.tenant_id, self.workspace_id)
    }

    pub fn workspace_digest(&self) -> String {
        format!(
            "worth-server-workspace-target-v1:{}",
            self.canonical_label()
        )
    }
}
