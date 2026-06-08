use super::delivery_error::{QueryDeliveryDenialKind, QueryDeliveryError};
use super::maintenance_delta::QuerySubscriptionMaintenanceDelta;
use super::ActiveSubscriptionCounters;

pub fn deny_raw_cdc_delivery_fallback(
    source_digest: impl Into<String>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_cdc_delivery_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawCdcFallbackDenied,
        "raw CDC cannot be consumed as active query delivery",
        source_digest,
        counters,
    ))
}

pub fn deny_raw_bridge_invalidation_delivery(
    source_digest: impl Into<String>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_bridge_invalidation_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawBridgeInvalidationDenied,
        "raw bridge invalidation must lower into a query maintenance delta first",
        source_digest,
        counters,
    ))
}
