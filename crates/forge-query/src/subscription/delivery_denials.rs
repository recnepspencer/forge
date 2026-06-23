use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::delivery_error::{QueryDeliveryDenialKind, QueryDeliveryError};
use super::maintenance_delta::QuerySubscriptionMaintenanceDelta;
use super::ActiveSubscriptionCounters;

fn raw_delivery_denial_source_identity(source_label: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::SubscriptionActivationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_raw_delivery_denial_source_v1",
        )
        .field_value(ForgeQueryEvidenceTag::new("source"), source_label)
        .seal()
}

pub fn deny_raw_cdc_delivery_fallback(
    source_label: impl AsRef<str>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_cdc_delivery_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawCdcFallbackDenied,
        "raw CDC cannot be consumed as active query delivery",
        raw_delivery_denial_source_identity(source_label.as_ref()),
        counters,
    ))
}

pub fn deny_raw_bridge_invalidation_delivery(
    source_label: impl AsRef<str>,
) -> Result<QuerySubscriptionMaintenanceDelta, QueryDeliveryError> {
    let mut counters = ActiveSubscriptionCounters::default();
    counters.raw_bridge_invalidation_denial_count = 1;
    Err(QueryDeliveryError::new(
        QueryDeliveryDenialKind::RawBridgeInvalidationDenied,
        "raw bridge invalidation must lower into a query maintenance delta first",
        raw_delivery_denial_source_identity(source_label.as_ref()),
        counters,
    ))
}
