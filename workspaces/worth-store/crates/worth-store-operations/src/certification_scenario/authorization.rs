use crate::{
    AuthorizationProviderDecision, AuthorizationProviderFailure, ExternalOperatorAssertion,
    OperationalAuthorizationPort, OperationalAuthorizationRequest,
};

pub struct ExactScenarioAuthorizationPort;

impl OperationalAuthorizationPort for ExactScenarioAuthorizationPort {
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

pub fn certification_operator_assertion() -> ExternalOperatorAssertion {
    ExternalOperatorAssertion::admit(
        "certification-identity-provider",
        "destructive-operation-approval",
        b"signed-certification-operator-assertion",
        [0xb2; 32],
        10,
        100,
    )
    .expect("certification operator assertion")
}
