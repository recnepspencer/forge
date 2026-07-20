use crate::{WorthServerProductOperationBaseDigest, WorthServerProductOperationSuccess};

use super::{WorthServerAdmittedDurableProductMutation, WorthServerProductAuthorityScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDurableProductMutationCompletion {
    operation_name: String,
    request_digest: String,
    authority_scope: WorthServerProductAuthorityScope,
    success: WorthServerProductOperationSuccess,
    next_basis: WorthServerProductOperationBaseDigest,
    product_commit_digest: String,
    canonical_digest: String,
}

impl WorthServerDurableProductMutationCompletion {
    pub fn new(
        attempt: &WorthServerAdmittedDurableProductMutation,
        success: WorthServerProductOperationSuccess,
        next_basis: WorthServerProductOperationBaseDigest,
        product_commit_digest: impl Into<String>,
    ) -> Result<Self, String> {
        let product_commit_digest = product_commit_digest.into().trim().to_string();
        if product_commit_digest.is_empty() {
            return Err("durable product completion requires a product commit digest".to_string());
        }
        if success.result_artifact().contract() != attempt.result_contract() {
            return Err(
                "durable product completion result contract does not match the admitted attempt"
                    .to_string(),
            );
        }
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-durable-product-completion-v2",
        )
        .field("operation", attempt.operation_name())
        .field("request", attempt.request_digest())
        .field("scope", attempt.authority_scope().value())
        .field("result", success.result_artifact().artifact_digest())
        .field("next_basis", next_basis.value())
        .field("commit", &product_commit_digest)
        .finish();
        Ok(Self {
            operation_name: attempt.operation_name().to_string(),
            request_digest: attempt.request_digest().to_string(),
            authority_scope: attempt.authority_scope().clone(),
            success,
            next_basis,
            product_commit_digest,
            canonical_digest,
        })
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn authority_scope(&self) -> &WorthServerProductAuthorityScope {
        &self.authority_scope
    }

    pub fn success(&self) -> &WorthServerProductOperationSuccess {
        &self.success
    }

    pub fn next_basis(&self) -> &WorthServerProductOperationBaseDigest {
        &self.next_basis
    }

    pub fn product_commit_digest(&self) -> &str {
        &self.product_commit_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn matches_attempt(
        &self,
        attempt: &WorthServerAdmittedDurableProductMutation,
    ) -> bool {
        self.operation_name == attempt.operation_name()
            && self.request_digest == attempt.request_digest()
            && &self.authority_scope == attempt.authority_scope()
            && self.success.result_artifact().contract() == attempt.result_contract()
    }

    pub(crate) fn matches_recovery(
        &self,
        recovery: &super::WorthServerDurableProductMutationRecoveryHandle,
        result_contract: &crate::WorthServerProductResultContract,
    ) -> bool {
        self.operation_name == recovery.operation_name()
            && self.request_digest == recovery.request_digest()
            && &self.authority_scope == recovery.authority_scope()
            && self.success.result_artifact().contract() == result_contract
    }
}
