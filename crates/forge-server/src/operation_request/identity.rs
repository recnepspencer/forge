use crate::ForgeServerOperationFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationIdentity {
    operation_family: ForgeServerOperationFamily,
    tenant_id: String,
    workspace_id: String,
    target_identity: String,
    operation_name: String,
    basis_digest: Option<String>,
    idempotency_key: Option<String>,
    product_session_identity: Option<String>,
    payload_identity: Option<String>,
    canonical_digest: String,
}

impl ForgeServerOperationIdentity {
    pub(crate) fn new(
        operation_family: ForgeServerOperationFamily,
        tenant_id: String,
        workspace_id: String,
        target_identity: String,
        operation_name: String,
        basis_digest: Option<String>,
        idempotency_key: Option<String>,
        product_session_identity: Option<String>,
        payload_identity: Option<String>,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-operation-identity-v1|family={}|tenant={tenant_id}|workspace={workspace_id}|target={target_identity}|operation={operation_name}|basis={}|idempotency={}|product_session={}|payload={}",
            operation_family.as_str(),
            basis_digest.as_deref().unwrap_or("none"),
            idempotency_key.as_deref().unwrap_or("none"),
            product_session_identity.as_deref().unwrap_or("none"),
            payload_identity.as_deref().unwrap_or("none"),
        );
        Self {
            operation_family,
            tenant_id,
            workspace_id,
            target_identity,
            operation_name,
            basis_digest,
            idempotency_key,
            product_session_identity,
            payload_identity,
            canonical_digest,
        }
    }

    pub fn operation_family(&self) -> ForgeServerOperationFamily {
        self.operation_family
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    pub fn product_session_identity(&self) -> Option<&str> {
        self.product_session_identity.as_deref()
    }

    pub fn payload_identity(&self) -> Option<&str> {
        self.payload_identity.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn with_basis_digest(&self, basis_digest: Option<&str>) -> Self {
        Self::new(
            self.operation_family,
            self.tenant_id.clone(),
            self.workspace_id.clone(),
            self.target_identity.clone(),
            self.operation_name.clone(),
            basis_digest.map(str::to_string),
            self.idempotency_key.clone(),
            self.product_session_identity.clone(),
            self.payload_identity.clone(),
        )
    }
}
