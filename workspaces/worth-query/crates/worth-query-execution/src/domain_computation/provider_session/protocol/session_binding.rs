use std::sync::Arc;

use super::WorthQueryProviderSessionToken;
use crate::execution_digest::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQuerySessionBinding {
    identity: Arc<str>,
    token_identity: Arc<str>,
    token_generation: u64,
    provider_identity: Arc<str>,
    provider_generation: u64,
}

impl WorthQuerySessionBinding {
    pub(super) fn seal(
        token: &WorthQueryProviderSessionToken,
        contract: &super::WorthQueryProviderExecutionPlanContract,
    ) -> Self {
        let identity = hash_parts(&[
            "worth_query_provider_session_binding_v1".to_owned(),
            token.identity().to_owned(),
            token.generation().to_string(),
            contract.provider_identity().to_owned(),
            contract.provider_generation().to_string(),
            contract.identity().to_owned(),
            contract.basis_identity().to_owned(),
            contract.snapshot_identity().to_owned(),
            contract.admitted_session_identity().to_owned(),
            contract.resource_attempt_identity().to_owned(),
        ]);
        Self {
            identity: identity.into(),
            token_identity: token.identity().into(),
            token_generation: token.generation(),
            provider_identity: contract.provider_identity().into(),
            provider_generation: contract.provider_generation(),
        }
    }

    pub(crate) fn canonical_identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn token_identity(&self) -> &str {
        &self.token_identity
    }

    pub(crate) fn token_generation(&self) -> u64 {
        self.token_generation
    }

    #[expect(dead_code, reason = "Phase 11 binds provider identity into commit")]
    pub(crate) fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    #[expect(dead_code, reason = "Phase 11 binds provider generation into commit")]
    pub(crate) fn provider_generation(&self) -> u64 {
        self.provider_generation
    }
}
