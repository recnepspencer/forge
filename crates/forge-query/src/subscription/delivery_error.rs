use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::active_counters::ActiveSubscriptionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryDeliveryDenialKind {
    DeliveryWindowBudgetExceeded,
    RawCdcFallbackDenied,
    RawBridgeInvalidationDenied,
    WorkPacketWindowMismatch,
    WorkPacketDeltaMismatch,
    MissingPreviousValueEvidence,
    StaleTemporalBasis,
    DenseRefreshDenied,
    AllocationPostureForbidden,
}

impl QueryDeliveryDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeliveryWindowBudgetExceeded => "delivery_window_budget_exceeded",
            Self::RawCdcFallbackDenied => "raw_cdc_fallback_denied",
            Self::RawBridgeInvalidationDenied => "raw_bridge_invalidation_denied",
            Self::WorkPacketWindowMismatch => "work_packet_window_mismatch",
            Self::WorkPacketDeltaMismatch => "work_packet_delta_mismatch",
            Self::MissingPreviousValueEvidence => "missing_previous_value_evidence",
            Self::StaleTemporalBasis => "stale_temporal_basis",
            Self::DenseRefreshDenied => "dense_refresh_denied",
            Self::AllocationPostureForbidden => "allocation_posture_forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDeliveryError {
    denial_kind: QueryDeliveryDenialKind,
    message: String,
    pub(in crate::subscription) source_identity: ForgeQueryEvidenceIdentity,
    counters: ActiveSubscriptionCounters,
}

impl QueryDeliveryError {
    pub(super) fn new(
        denial_kind: QueryDeliveryDenialKind,
        message: impl Into<String>,
        source_identity: ForgeQueryEvidenceIdentity,
        counters: ActiveSubscriptionCounters,
    ) -> Self {
        Self {
            denial_kind,
            message: message.into(),
            source_identity,
            counters,
        }
    }

    pub fn denial_kind(&self) -> &QueryDeliveryDenialKind {
        &self.denial_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn counters(&self) -> &ActiveSubscriptionCounters {
        &self.counters
    }
}
