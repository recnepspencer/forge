use std::sync::Arc;

use forge_foundational::facade::AspectKey;
use sha2::{Digest, Sha256};

use crate::adapter::BridgeHistoricalLineageTopology;
use crate::error::BridgeContinuityError;
use crate::mapping::SubscriptionSliceKind;
use crate::routing::{
    BridgeRouteIdentity, BridgeSubscriptionSlice, BridgeSubscriptionSliceIdentity,
    FineGrainedMatchStatus,
};

use super::{
    BridgeContinuityClass, BridgeContinuityCounters, BridgeContinuityOutcomeClass,
    BridgeHistoricalLineagePacket, PriorSubscriptionSlice,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLineageContinuity {
    prior_slice_identity: BridgeSubscriptionSliceIdentity,
    outcome_class: BridgeContinuityOutcomeClass,
    successor_slices: Arc<[BridgeSubscriptionSlice]>,
    lineage_digest: Arc<str>,
}

impl ResolvedLineageContinuity {
    pub(crate) fn new(
        prior_slice_identity: BridgeSubscriptionSliceIdentity,
        outcome_class: BridgeContinuityOutcomeClass,
        successor_slices: Vec<BridgeSubscriptionSlice>,
        lineage_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            prior_slice_identity,
            outcome_class,
            successor_slices: Arc::from(successor_slices),
            lineage_digest: lineage_digest.into(),
        }
    }

    pub fn prior_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.prior_slice_identity
    }

    pub fn outcome_class(&self) -> BridgeContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn continuity_class(&self) -> Option<BridgeContinuityClass> {
        self.outcome_class.continued_class()
    }

    pub fn successor_slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.successor_slices
    }

    pub fn lineage_digest(&self) -> &str {
        self.lineage_digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLineageContinuitySet {
    route_identity: BridgeRouteIdentity,
    prior_subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    continuity_resolution_digest: Arc<str>,
    continuity_entries: Arc<[ResolvedLineageContinuity]>,
    counters: BridgeContinuityCounters,
}

impl ResolvedLineageContinuitySet {
    pub(crate) fn from_historical_packet(
        packet: &BridgeHistoricalLineagePacket,
    ) -> Result<Self, BridgeContinuityError> {
        let mut entries = Vec::with_capacity(packet.entries().len());
        let mut counters = BridgeContinuityCounters::from_request_set(
            packet.continuity_request_count(),
            packet.continuity_prior_slice_count(),
        );

        for entry in packet.entries() {
            let successor_record_keys = entry
                .lineage_authority()
                .canonical_resolved_record_keys()
                .iter()
                .map(|key| key.as_ref())
                .collect::<Vec<_>>();
            counters =
                counters.with_lineage_resolution_candidate_count(successor_record_keys.len());

            let (outcome_class, successor_slices) = classify_continuity(
                entry.prior_slice(),
                entry.lineage_authority().topology(),
                successor_record_keys.as_slice(),
            );
            counters = match outcome_class {
                BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
                    counters.with_single_successor()
                }
                BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
                    counters.with_split_successor()
                }
                BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                    counters.with_merge_like_successor()
                }
                BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
                    counters.with_rejection().with_ambiguity()
                }
                BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
                | BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass
                | BridgeContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
                    counters.with_rejection()
                }
            };

            entries.push(ResolvedLineageContinuity::new(
                entry
                    .prior_slice()
                    .prior_subscription_slice_identity()
                    .clone(),
                outcome_class,
                successor_slices,
                entry.lineage_authority().lineage_digest(),
            ));
        }

        let canonical_basis = format!(
            "resolved-lineage-continuity-set|route={}|slice-set={}|entry-count={}|entries={}",
            packet.prior_route_identity().as_str(),
            packet.prior_subscription_slice_identity().as_str(),
            entries.len(),
            entries
                .iter()
                .map(|entry| {
                    format!(
                        "{}:{:?}:{}:{}",
                        entry.prior_slice_identity().as_str(),
                        entry.outcome_class(),
                        entry.lineage_digest(),
                        entry
                            .successor_slices()
                            .iter()
                            .map(successor_slice_canonical_basis)
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                })
                .collect::<Vec<_>>()
                .join("|"),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        counters = counters
            .with_digest_computations(1)
            .with_digest_input_bytes(canonical_basis.len());

        Ok(Self {
            route_identity: packet.prior_route_identity().clone(),
            prior_subscription_slice_identity: packet.prior_subscription_slice_identity().clone(),
            continuity_resolution_digest: Arc::from(format!(
                "resolved-lineage-continuity:sha256:{digest:x}"
            )),
            continuity_entries: Arc::from(entries),
            counters,
        })
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn prior_subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.prior_subscription_slice_identity
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }

    pub fn continuity_entries(&self) -> &[ResolvedLineageContinuity] {
        &self.continuity_entries
    }

    pub fn counters(&self) -> &BridgeContinuityCounters {
        &self.counters
    }
}

fn classify_continuity(
    prior_slice: &PriorSubscriptionSlice,
    topology: BridgeHistoricalLineageTopology,
    successor_record_keys: &[&str],
) -> (BridgeContinuityOutcomeClass, Vec<BridgeSubscriptionSlice>) {
    match topology {
        BridgeHistoricalLineageTopology::NoAuthoritativeSuccessor => (
            BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor,
            Vec::new(),
        ),
        BridgeHistoricalLineageTopology::UnsupportedWithoutSuccessor => (
            BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass,
            Vec::new(),
        ),
        BridgeHistoricalLineageTopology::SingleSuccessor => (
            BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            vec![successor_slice_from_record_key(
                successor_record_keys[0],
                prior_slice.aspect_key().clone(),
                prior_slice.surface_label(),
                prior_slice.slice_kind(),
                prior_slice.match_status(),
            )],
        ),
        BridgeHistoricalLineageTopology::MergeLikeSuccessor => (
            BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor,
            vec![successor_slice_from_record_key(
                successor_record_keys[0],
                prior_slice.aspect_key().clone(),
                prior_slice.surface_label(),
                prior_slice.slice_kind(),
                prior_slice.match_status(),
            )],
        ),
        BridgeHistoricalLineageTopology::SplitSuccessors => (
            BridgeContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            successor_record_keys
                .iter()
                .map(|record_key| {
                    successor_slice_from_record_key(
                        record_key,
                        prior_slice.aspect_key().clone(),
                        prior_slice.surface_label(),
                        prior_slice.slice_kind(),
                        prior_slice.match_status(),
                    )
                })
                .collect(),
        ),
        BridgeHistoricalLineageTopology::AmbiguousSuccessor => (
            BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor,
            Vec::new(),
        ),
    }
}

fn successor_slice_from_record_key(
    record_key: &str,
    aspect_key: AspectKey,
    surface_label: &str,
    slice_kind: SubscriptionSliceKind,
    match_status: FineGrainedMatchStatus,
) -> BridgeSubscriptionSlice {
    BridgeSubscriptionSlice::new(
        record_key,
        aspect_key,
        surface_label,
        slice_kind,
        match_status,
    )
}

fn successor_slice_canonical_basis(slice: &BridgeSubscriptionSlice) -> String {
    format!(
        "{}|{}|{}|{:?}|{:?}",
        slice.entity_identity(),
        slice.aspect_label(),
        slice.surface_label(),
        slice.slice_kind(),
        slice.match_status(),
    )
}

#[cfg(test)]
mod tests {
    use super::classify_continuity;
    use crate::continuity::PriorSubscriptionSlice;
    use crate::mapping::SubscriptionSliceKind;
    use crate::routing::{BridgeSubscriptionSliceIdentity, FineGrainedMatchStatus};
    use forge_foundational::facade::AspectKey;

    #[test]
    fn resolution_rejects_no_authoritative_successor_when_lineage_is_empty() {
        let prior_slice = PriorSubscriptionSlice::from_parts(
            BridgeSubscriptionSliceIdentity::new("slice-set:test"),
            "entity:0:1:1",
            aspect_key("profile.name"),
            "name",
            SubscriptionSliceKind::SignalField,
            FineGrainedMatchStatus::Matched,
        );
        let (outcome, successor_slices) = classify_continuity(
            &prior_slice,
            crate::adapter::BridgeHistoricalLineageTopology::NoAuthoritativeSuccessor,
            &[],
        );

        assert_eq!(
            outcome,
            crate::continuity::BridgeContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
        );
        assert!(successor_slices.is_empty());
    }

    #[test]
    fn resolution_rejects_unsupported_when_lineage_exists_without_successor_records() {
        let prior_slice = PriorSubscriptionSlice::from_parts(
            BridgeSubscriptionSliceIdentity::new("slice-set:test"),
            "entity:0:1:1",
            aspect_key("profile.name"),
            "name",
            SubscriptionSliceKind::SignalField,
            FineGrainedMatchStatus::Matched,
        );
        let (outcome, successor_slices) = classify_continuity(
            &prior_slice,
            crate::adapter::BridgeHistoricalLineageTopology::UnsupportedWithoutSuccessor,
            &[],
        );

        assert_eq!(
            outcome,
            crate::continuity::BridgeContinuityOutcomeClass::RejectedUnsupportedContinuityClass
        );
        assert!(successor_slices.is_empty());
    }

    #[test]
    fn resolution_classifies_single_record_with_multiple_lineages_as_merge_like() {
        let prior_slice = PriorSubscriptionSlice::from_parts(
            BridgeSubscriptionSliceIdentity::new("slice-set:test"),
            "entity:0:1:1",
            aspect_key("profile.name"),
            "name",
            SubscriptionSliceKind::SignalField,
            FineGrainedMatchStatus::Matched,
        );
        let (outcome, successor_slices) = classify_continuity(
            &prior_slice,
            crate::adapter::BridgeHistoricalLineageTopology::MergeLikeSuccessor,
            &["entity:0:4:2"],
        );

        assert_eq!(
            outcome,
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        );
        assert_eq!(successor_slices.len(), 1);
    }

    #[test]
    fn resolution_rejects_competing_successor_sets_as_ambiguous() {
        let prior_slice = PriorSubscriptionSlice::from_parts(
            BridgeSubscriptionSliceIdentity::new("slice-set:test"),
            "entity:0:1:1",
            aspect_key("profile.name"),
            "name",
            SubscriptionSliceKind::SignalField,
            FineGrainedMatchStatus::Matched,
        );
        let (outcome, successor_slices) = classify_continuity(
            &prior_slice,
            crate::adapter::BridgeHistoricalLineageTopology::AmbiguousSuccessor,
            &["entity:0:4:2", "entity:0:5:2"],
        );

        assert_eq!(
            outcome,
            crate::continuity::BridgeContinuityOutcomeClass::RejectedAmbiguousSuccessor
        );
        assert!(successor_slices.is_empty());
    }

    fn aspect_key(value: &str) -> AspectKey {
        AspectKey::new(value).expect("valid continuity test aspect key")
    }
}
