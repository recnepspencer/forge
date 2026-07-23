use super::{
    WorthServerExternalResourceIntent, WorthServerExternalResourcePlanDenial,
    WorthServerExternalResourceTransport, WorthServerExternalResourceTransportOutcome,
    WorthServerLoweredExternalResourcePlan,
};

#[derive(Debug)]
pub struct WorthServerExternalResourceExecutionBoundary<'a> {
    transport: &'a dyn WorthServerExternalResourceTransport,
}

impl<'a> WorthServerExternalResourceExecutionBoundary<'a> {
    pub fn using(transport: &'a dyn WorthServerExternalResourceTransport) -> Self {
        Self { transport }
    }

    pub fn plan(
        &self,
        intent: WorthServerExternalResourceIntent,
    ) -> Result<WorthServerLoweredExternalResourcePlan, WorthServerExternalResourcePlanDenial> {
        WorthServerLoweredExternalResourcePlan::lower(intent)
    }

    pub fn execute(
        &self,
        plan: WorthServerLoweredExternalResourcePlan,
    ) -> WorthServerExternalResourceExecutionOutcome {
        let request_bytes = plan.request_body().len();
        match self.transport.execute(&plan) {
            WorthServerExternalResourceTransportOutcome::Responded(response) => {
                let response_bytes = response.body().len();
                if response_bytes > plan.budget().max_response_bytes() {
                    return WorthServerExternalResourceExecutionOutcome::Denied(
                        WorthServerExternalResourceExecutionDenial::new(
                            WorthServerExternalResourceExecutionDenialCode::ResponseBudgetExceeded,
                            "external resource response exceeded its admitted byte budget",
                            counters(1, request_bytes, response_bytes),
                        ),
                    );
                }
                WorthServerExternalResourceExecutionOutcome::Completed(
                    WorthServerCompletedExternalResourceExecution::new(
                        plan,
                        response.body().to_vec(),
                        response.transport_evidence_identity().to_string(),
                        counters(1, request_bytes, response_bytes),
                    ),
                )
            }
            WorthServerExternalResourceTransportOutcome::RejectedBeforeAttempt { reason_key } => {
                WorthServerExternalResourceExecutionOutcome::Denied(
                    WorthServerExternalResourceExecutionDenial::new(
                        WorthServerExternalResourceExecutionDenialCode::ProviderAdmissionDenied,
                        reason_key,
                        counters(0, request_bytes, 0),
                    ),
                )
            }
            WorthServerExternalResourceTransportOutcome::Denied { reason_key } => {
                WorthServerExternalResourceExecutionOutcome::Denied(
                    WorthServerExternalResourceExecutionDenial::new(
                        WorthServerExternalResourceExecutionDenialCode::ProviderDenied,
                        reason_key,
                        counters(1, request_bytes, 0),
                    ),
                )
            }
            WorthServerExternalResourceTransportOutcome::TimedOut => {
                WorthServerExternalResourceExecutionOutcome::Denied(
                    WorthServerExternalResourceExecutionDenial::new(
                        WorthServerExternalResourceExecutionDenialCode::TimedOut,
                        "external resource execution timed out",
                        counters(1, request_bytes, 0),
                    ),
                )
            }
            WorthServerExternalResourceTransportOutcome::Unavailable => {
                WorthServerExternalResourceExecutionOutcome::Denied(
                    WorthServerExternalResourceExecutionDenial::new(
                        WorthServerExternalResourceExecutionDenialCode::Unavailable,
                        "external resource provider was unavailable",
                        counters(1, request_bytes, 0),
                    ),
                )
            }
            WorthServerExternalResourceTransportOutcome::Failed => {
                WorthServerExternalResourceExecutionOutcome::Failed(
                    WorthServerExternalResourceExecutionFailure::new(counters(1, request_bytes, 0)),
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerCompletedExternalResourceExecution {
    plan: WorthServerLoweredExternalResourcePlan,
    response_body: Vec<u8>,
    transport_evidence_identity: String,
    counters: WorthServerExternalResourceExecutionCounters,
}

impl WorthServerCompletedExternalResourceExecution {
    fn new(
        plan: WorthServerLoweredExternalResourcePlan,
        response_body: Vec<u8>,
        transport_evidence_identity: String,
        counters: WorthServerExternalResourceExecutionCounters,
    ) -> Self {
        Self {
            plan,
            response_body,
            transport_evidence_identity,
            counters,
        }
    }

    pub fn plan(&self) -> &WorthServerLoweredExternalResourcePlan {
        &self.plan
    }

    pub fn response_body(&self) -> &[u8] {
        &self.response_body
    }

    pub fn transport_evidence_identity(&self) -> &str {
        &self.transport_evidence_identity
    }

    pub fn counters(&self) -> WorthServerExternalResourceExecutionCounters {
        self.counters
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthServerLoweredExternalResourcePlan,
        String,
        WorthServerExternalResourceExecutionCounters,
    ) {
        (self.plan, self.transport_evidence_identity, self.counters)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceExecutionCounters {
    transport_attempts: usize,
    request_bytes: usize,
    response_bytes: usize,
}

impl WorthServerExternalResourceExecutionCounters {
    pub fn transport_attempts(&self) -> usize {
        self.transport_attempts
    }

    pub fn request_bytes(&self) -> usize {
        self.request_bytes
    }

    pub fn response_bytes(&self) -> usize {
        self.response_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerExternalResourceExecutionDenialCode {
    ProviderAdmissionDenied,
    ProviderDenied,
    TimedOut,
    Unavailable,
    ResponseBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceExecutionDenial {
    code: WorthServerExternalResourceExecutionDenialCode,
    detail: String,
    counters: WorthServerExternalResourceExecutionCounters,
}

impl WorthServerExternalResourceExecutionDenial {
    fn new(
        code: WorthServerExternalResourceExecutionDenialCode,
        detail: impl Into<String>,
        counters: WorthServerExternalResourceExecutionCounters,
    ) -> Self {
        Self {
            code,
            detail: detail.into(),
            counters,
        }
    }

    pub fn code(&self) -> WorthServerExternalResourceExecutionDenialCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> WorthServerExternalResourceExecutionCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceExecutionFailure {
    counters: WorthServerExternalResourceExecutionCounters,
}

impl WorthServerExternalResourceExecutionFailure {
    fn new(counters: WorthServerExternalResourceExecutionCounters) -> Self {
        Self { counters }
    }

    pub fn counters(&self) -> WorthServerExternalResourceExecutionCounters {
        self.counters
    }
}

#[derive(Clone, Debug)]
pub enum WorthServerExternalResourceExecutionOutcome {
    Completed(WorthServerCompletedExternalResourceExecution),
    Denied(WorthServerExternalResourceExecutionDenial),
    Failed(WorthServerExternalResourceExecutionFailure),
}

fn counters(
    transport_attempts: usize,
    request_bytes: usize,
    response_bytes: usize,
) -> WorthServerExternalResourceExecutionCounters {
    WorthServerExternalResourceExecutionCounters {
        transport_attempts,
        request_bytes,
        response_bytes,
    }
}
