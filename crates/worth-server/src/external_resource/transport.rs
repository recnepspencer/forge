use super::WorthServerLoweredExternalResourcePlan;

pub trait WorthServerExternalResourceTransport: std::fmt::Debug + Send + Sync {
    fn execute(
        &self,
        plan: &WorthServerLoweredExternalResourcePlan,
    ) -> WorthServerExternalResourceTransportOutcome;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerExternalResourceTransportOutcome {
    Responded(WorthServerExternalResourceTransportResponse),
    RejectedBeforeAttempt { reason_key: String },
    Denied { reason_key: String },
    TimedOut,
    Unavailable,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalResourceTransportResponse {
    body: Vec<u8>,
    transport_evidence_identity: String,
}

impl WorthServerExternalResourceTransportResponse {
    pub fn new(
        body: Vec<u8>,
        transport_evidence_identity: impl Into<String>,
    ) -> Result<Self, String> {
        let transport_evidence_identity = transport_evidence_identity.into();
        if transport_evidence_identity.trim().is_empty() {
            return Err("transport evidence identity must not be blank".to_string());
        }
        Ok(Self {
            body,
            transport_evidence_identity,
        })
    }

    pub(crate) fn body(&self) -> &[u8] {
        &self.body
    }

    pub(crate) fn transport_evidence_identity(&self) -> &str {
        &self.transport_evidence_identity
    }
}
