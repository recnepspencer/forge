#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDurableProductMutationRecoveryHandle {
    operation_name: String,
    tenant_id: String,
    workspace_id: String,
    authority_scope: super::WorthServerProductAuthorityScope,
    recovery_identity: String,
    request_digest: String,
    canonical_digest: String,
}

impl WorthServerDurableProductMutationRecoveryHandle {
    pub fn for_attempt(
        attempt: &super::WorthServerAdmittedDurableProductMutation,
        recovery_identity: impl Into<String>,
    ) -> Result<Self, String> {
        let operation_name = attempt.operation_name().to_string();
        let tenant_id = attempt.tenant_id().to_string();
        let workspace_id = attempt.workspace_id().to_string();
        let authority_scope = attempt.authority_scope().clone();
        let recovery_identity = recovery_identity.into().trim().to_string();
        let request_digest = attempt.request_digest().to_string();
        if recovery_identity.is_empty() {
            return Err("durable product recovery handles require a recovery identity".to_string());
        }
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-durable-product-recovery-v3",
        )
        .field("operation", &operation_name)
        .field("tenant", &tenant_id)
        .field("workspace", &workspace_id)
        .field("scope", authority_scope.value())
        .field("recovery", &recovery_identity)
        .field("request", &request_digest)
        .finish();
        Ok(Self {
            operation_name,
            tenant_id,
            workspace_id,
            authority_scope,
            recovery_identity,
            request_digest,
            canonical_digest,
        })
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    pub fn authority_scope(&self) -> &super::WorthServerProductAuthorityScope {
        &self.authority_scope
    }

    pub fn recovery_identity(&self) -> &str {
        &self.recovery_identity
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
