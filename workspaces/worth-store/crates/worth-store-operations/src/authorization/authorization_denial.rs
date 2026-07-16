#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationProviderFailure {
    Unsupported,
    Unavailable,
    Timeout,
    UnsupportedAssertion,
    InvalidProofOfPossession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationDenial {
    Provider(AuthorizationProviderFailure),
    ProviderDenied { reason_code: String },
    PlanBindingMismatch,
    AuthorityIdentityMismatch,
    AssertionExpired,
    AuthorizationExpired,
    AuthorizationRevoked,
    InvalidValidityWindow,
    InvalidProviderDecision,
}
