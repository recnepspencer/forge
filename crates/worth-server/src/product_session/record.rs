use super::{
    WorthServerProductSessionExpiryPosture, WorthServerProductSessionIdentity,
    WorthServerProductSessionLifecycle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductSession {
    identity: WorthServerProductSessionIdentity,
    lifecycle: WorthServerProductSessionLifecycle,
    expiry_posture: WorthServerProductSessionExpiryPosture,
    operation_name: String,
    tenant_id: String,
    workspace_id: String,
    branch_label: String,
    basis_digest: Option<String>,
    canonical_digest: String,
}

impl WorthServerProductSession {
    pub(crate) fn new(
        identity: WorthServerProductSessionIdentity,
        lifecycle: WorthServerProductSessionLifecycle,
        expiry_posture: WorthServerProductSessionExpiryPosture,
        operation_name: impl Into<String>,
        tenant_id: impl Into<String>,
        workspace_id: impl Into<String>,
        branch_label: impl Into<String>,
        basis_digest: Option<String>,
    ) -> Self {
        let operation_name = operation_name.into();
        let tenant_id = tenant_id.into();
        let workspace_id = workspace_id.into();
        let branch_label = branch_label.into();
        let canonical_digest = format!(
            "worth-server-product-session-v1|identity={}|lifecycle={}|expiry={:?}|operation={}|tenant={}|workspace={}|branch={}|basis={}",
            identity.as_str(),
            lifecycle.as_str(),
            expiry_posture,
            operation_name,
            tenant_id,
            workspace_id,
            branch_label,
            basis_digest.as_deref().unwrap_or("none"),
        );
        Self {
            identity,
            lifecycle,
            expiry_posture,
            operation_name,
            tenant_id,
            workspace_id,
            branch_label,
            basis_digest,
            canonical_digest,
        }
    }

    pub fn identity(&self) -> &WorthServerProductSessionIdentity {
        &self.identity
    }

    pub fn lifecycle(&self) -> WorthServerProductSessionLifecycle {
        self.lifecycle
    }

    pub fn expiry_posture(&self) -> &WorthServerProductSessionExpiryPosture {
        &self.expiry_posture
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn branch_label(&self) -> &str {
        &self.branch_label
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub(crate) fn workspace_id(&self) -> &str {
        &self.workspace_id
    }
}
