use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, ContinuityIdentityTag};
use crate::routing::{
    BridgeSubscriptionSliceIdentity, CanonicalSubscriptionSlices,
};

use super::{BridgeContinuityCounters, ResolvedLineageContinuity, ResolvedLineageContinuitySet};

pub type BridgeContinuityIdentity = BridgeIdentity<ContinuityIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityArtifact {
    route_identity: crate::routing::BridgeRouteIdentity,
    continuity_identity: BridgeContinuityIdentity,
    remapped_slices: CanonicalSubscriptionSlices,
    remapped_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    continuity_outcomes: Arc<[ResolvedLineageContinuity]>,
    continuity_resolution_digest: Arc<str>,
    counters: BridgeContinuityCounters,
}

impl BridgeContinuityArtifact {
    pub(crate) fn from_resolved(resolved: &ResolvedLineageContinuitySet) -> Self {
        let mut remapped_slices = resolved
            .continuity_entries()
            .iter()
            .flat_map(|entry| entry.successor_slices().iter().cloned())
            .collect::<Vec<_>>();
        remapped_slices.sort();
        remapped_slices.dedup();

        let remapped_slices = CanonicalSubscriptionSlices::new(remapped_slices);
        let remapped_basis = format!(
            "continuity-remapped-slices|route={}|resolution={}|slices={}",
            resolved.route_identity().as_str(),
            resolved.continuity_resolution_digest(),
            remapped_slices
                .slices()
                .iter()
                .map(|slice| {
                    format!(
                        "{}|{}|{}|{:?}|{:?}",
                        slice.entity_identity(),
                        slice.aspect_label(),
                        slice.surface_label(),
                        slice.slice_kind(),
                        slice.match_status(),
                    )
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        let remapped_digest = Sha256::digest(remapped_basis.as_bytes());
        let remapped_subscription_slice_identity = BridgeSubscriptionSliceIdentity::new(format!(
            "continuity-remapped-slices:sha256:{remapped_digest:x}"
        ));

        let artifact_basis = format!(
            "continuity-artifact|route={}|resolution={}|slice-identity={}",
            resolved.route_identity().as_str(),
            resolved.continuity_resolution_digest(),
            remapped_subscription_slice_identity.as_str(),
        );
        let artifact_digest = Sha256::digest(artifact_basis.as_bytes());
        let counters = resolved
            .counters()
            .with_digest_computations(2)
            .with_digest_input_bytes(remapped_basis.len() + artifact_basis.len())
            .with_sort_input_width(remapped_slices.len());

        Self {
            route_identity: resolved.route_identity().clone(),
            continuity_identity: BridgeContinuityIdentity::new(format!(
                "continuity-artifact:sha256:{artifact_digest:x}"
            )),
            remapped_slices,
            remapped_subscription_slice_identity,
            continuity_outcomes: Arc::from(resolved.continuity_entries().to_vec()),
            continuity_resolution_digest: Arc::from(resolved.continuity_resolution_digest()),
            counters,
        }
    }

    pub fn route_identity(&self) -> &crate::routing::BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn continuity_identity(&self) -> &BridgeContinuityIdentity {
        &self.continuity_identity
    }

    pub fn remapped_slices(&self) -> &CanonicalSubscriptionSlices {
        &self.remapped_slices
    }

    pub fn remapped_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.remapped_subscription_slice_identity
    }

    pub fn continuity_outcomes(&self) -> &[ResolvedLineageContinuity] {
        &self.continuity_outcomes
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeContinuityCounters {
        &self.counters
    }
}
