use crate::authorization::OperationalAuthorizationRequest;
use crate::{
    AuthorizationProviderDecision, AuthorizationProviderFailure, ExternalOperatorAssertion,
    OperationalAuthorizationPort,
};

pub(crate) struct ExactAuthorizationPort {
    pub(crate) substitute_plan: Option<[u8; 32]>,
}

impl OperationalAuthorizationPort for ExactAuthorizationPort {
    fn authorize(
        &self,
        request: OperationalAuthorizationRequest<'_>,
        assertion: &ExternalOperatorAssertion,
    ) -> Result<AuthorizationProviderDecision, AuthorizationProviderFailure> {
        Ok(AuthorizationProviderDecision::authorized(
            request.plan_fingerprint(),
            self.substitute_plan
                .unwrap_or_else(|| request.plan_fingerprint()),
            assertion.proof_of_possession_binding(),
            request.requested_at(),
            request.expires_at(),
        ))
    }
}

pub(crate) fn operator_assertion() -> ExternalOperatorAssertion {
    ExternalOperatorAssertion::admit(
        "test-identity-provider",
        "destructive-operation-approval",
        b"signed-operator-assertion",
        [0xb2; 32],
        10,
        100,
    )
    .expect("operator assertion")
}
