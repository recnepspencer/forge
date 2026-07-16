mod authorization_consumption;
mod authorization_denial;
mod authorization_port;
mod lowered_plan_binding;
mod revocation_boundary;
mod staging_authorization_continuation;

pub use authorization_consumption::{
    AuthorizationConsumptionDenial, AuthorizationConsumptionReceipt,
};
pub use authorization_denial::{AuthorizationDenial, AuthorizationProviderFailure};
pub use authorization_port::{
    AuthorizationProviderDecision, ExternalOperatorAssertion, OperationalAuthorizationPort,
    OperationalAuthorizationRequest,
};
pub use lowered_plan_binding::AuthorizationReplayPolicy;
pub use revocation_boundary::AuthorizationRevocationObservation;
pub(crate) use staging_authorization_continuation::StagingAuthorizationContinuation;
pub use staging_authorization_continuation::{
    StagingAuthorizationContinuationDenial, StagingAuthorizationContinuationPort,
    StagingAuthorizationContinuationRequest,
};

pub(crate) use authorization_consumption::ConsumedOperationalPlan;
pub(crate) use authorization_consumption::{
    consume_authorization, record_recovery_staging_completion, recover_authorization_consumption,
};
pub(crate) use lowered_plan_binding::{
    authorize_lowered_plan, AuthorizedOperationalPlan, LoweredOperationalPlan,
};
