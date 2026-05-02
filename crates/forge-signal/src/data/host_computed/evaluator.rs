use crate::data::error::SignalError;

use super::request::HostComputedEvaluationRequest;
use super::response::HostComputedEvaluationResponse;

/// Narrow core-owned boundary for host-computed evaluation adapters.
pub trait HostComputedEvaluator {
    type Context;

    fn evaluate(
        &mut self,
        request: &HostComputedEvaluationRequest,
        ctx: &mut Self::Context,
    ) -> Result<HostComputedEvaluationResponse, SignalError>;
}
