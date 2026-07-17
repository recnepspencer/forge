use worth_store_operations::{
    AuthorizationProviderDecision, AuthorizationProviderFailure, ExternalOperatorAssertion,
    OperationalAuthorizationPort, OperationalAuthorizationRequest,
};

#[derive(Debug)]
pub(super) struct ExactAuthorization;

impl OperationalAuthorizationPort for ExactAuthorization {
    fn authorize(
        &self,
        request: OperationalAuthorizationRequest<'_>,
        assertion: &ExternalOperatorAssertion,
    ) -> Result<AuthorizationProviderDecision, AuthorizationProviderFailure> {
        Ok(AuthorizationProviderDecision::authorized(
            request.plan_fingerprint(),
            request.plan_fingerprint(),
            assertion.proof_of_possession_binding(),
            request.requested_at(),
            request.expires_at(),
        ))
    }
}

pub(super) fn operator_assertion() -> ExternalOperatorAssertion {
    ExternalOperatorAssertion::admit(
        "s10-test-provider",
        "operational-recovery-approval",
        b"signed-operational-recovery-approval",
        [0xA2; 32],
        10,
        100,
    )
    .unwrap()
}
