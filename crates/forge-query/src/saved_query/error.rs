#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SavedQueryFailureClass {
    DurableClaimDenied,
    FreezeInvariantRejected,
    IllegalSemanticDrift,
    TemporalAsyncSurfaceDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryError {
    failure_class: SavedQueryFailureClass,
    message: String,
}

impl SavedQueryError {
    pub(crate) fn durable_claim_denied(message: impl Into<String>) -> Self {
        Self {
            failure_class: SavedQueryFailureClass::DurableClaimDenied,
            message: message.into(),
        }
    }

    pub(crate) fn freeze_invariant_rejected(message: impl Into<String>) -> Self {
        Self {
            failure_class: SavedQueryFailureClass::FreezeInvariantRejected,
            message: message.into(),
        }
    }

    pub(crate) fn temporal_async_surface_deferred(message: impl Into<String>) -> Self {
        Self {
            failure_class: SavedQueryFailureClass::TemporalAsyncSurfaceDeferred,
            message: message.into(),
        }
    }

    pub fn failure_class(&self) -> &SavedQueryFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
