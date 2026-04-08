use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::adapter::BridgeHistoricalLineageAuthority;
use crate::routing::{BridgeRouteIdentity, BridgeSubscriptionSliceIdentity};

use super::{BridgeContinuityAuthorityBasis, BridgeEligibleContinuityRequestSet, PriorSubscriptionSlice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalLineagePacketEntry {
    request_key: Arc<str>,
    prior_slice: PriorSubscriptionSlice,
    lineage_authority: BridgeHistoricalLineageAuthority,
}

impl BridgeHistoricalLineagePacketEntry {
    pub(crate) fn new(
        request_key: impl Into<Arc<str>>,
        prior_slice: PriorSubscriptionSlice,
        lineage_authority: BridgeHistoricalLineageAuthority,
    ) -> Self {
        Self {
            request_key: request_key.into(),
            prior_slice,
            lineage_authority,
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn prior_slice(&self) -> &PriorSubscriptionSlice {
        &self.prior_slice
    }

    pub fn lineage_authority(&self) -> &BridgeHistoricalLineageAuthority {
        &self.lineage_authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalLineagePacket {
    prior_route_identity: BridgeRouteIdentity,
    authority_basis: BridgeContinuityAuthorityBasis,
    prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    continuity_request_count: usize,
    continuity_prior_slice_count: usize,
    entries: Arc<[BridgeHistoricalLineagePacketEntry]>,
    digest: Arc<str>,
}

impl BridgeHistoricalLineagePacket {
    pub(crate) fn from_entries(
        requests: &BridgeEligibleContinuityRequestSet,
        entries: Vec<BridgeHistoricalLineagePacketEntry>,
    ) -> Self {
        let canonical_basis = format!(
            "historical-lineage-packet|route={}|slice-set={}|authority={}|entry-count={}|entries={}",
            requests.prior_route_identity().as_str(),
            requests.prior_subscription_slice_identity().as_str(),
            requests.authority_basis().digest(),
            entries.len(),
            entries
                .iter()
                .map(|entry| format!("{}:{}", entry.request_key(), entry.lineage_authority().lineage_digest()))
                .collect::<Vec<_>>()
                .join(","),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            prior_route_identity: requests.prior_route_identity().clone(),
            authority_basis: requests.authority_basis().clone(),
            prior_subscription_slice_identity: requests.prior_subscription_slice_identity().clone(),
            continuity_request_count: requests.requests().len(),
            continuity_prior_slice_count: requests.prior_slice_count(),
            entries: Arc::from(entries),
            digest: Arc::from(format!("historical-lineage-packet:sha256:{digest:x}")),
        }
    }

    pub fn prior_route_identity(&self) -> &BridgeRouteIdentity {
        &self.prior_route_identity
    }

    pub fn authority_basis(&self) -> &BridgeContinuityAuthorityBasis {
        &self.authority_basis
    }

    pub fn prior_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.prior_subscription_slice_identity
    }

    pub fn continuity_request_count(&self) -> usize {
        self.continuity_request_count
    }

    pub fn continuity_prior_slice_count(&self) -> usize {
        self.continuity_prior_slice_count
    }

    pub fn entries(&self) -> &[BridgeHistoricalLineagePacketEntry] {
        &self.entries
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
