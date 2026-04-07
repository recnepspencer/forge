use std::sync::Arc;

use crate::continuity::{
    BridgeContinuityArtifact, BridgeContinuityCounters, BridgeContinuityIdentity,
    ResolvedLineageContinuity,
};
use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::routing::{BridgeRouteIdentity, CanonicalSubscriptionSlices};
use crate::snapshot::TruthSnapshotIdentity;

use super::BridgeRouteRecord;

pub const BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.continuity-record.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalContinuityRecord {
    schema_version: Arc<str>,
    route_record: BridgeRouteRecord,
    continuity_request_digest: Arc<str>,
    continuity_resolution_digest: Arc<str>,
    continuity_artifact_identity: BridgeContinuityIdentity,
    remapped_subscription_slice_identity: crate::routing::BridgeSubscriptionSliceIdentity,
    remapped_slices: CanonicalSubscriptionSlices,
    continuity_outcomes: Arc<[ResolvedLineageContinuity]>,
    counters: BridgeContinuityCounters,
}

impl BridgeCanonicalContinuityRecord {
    pub(crate) fn new(
        route_record: BridgeRouteRecord,
        continuity_request_digest: impl Into<Arc<str>>,
        continuity_resolution_digest: impl Into<Arc<str>>,
        continuity_artifact_identity: BridgeContinuityIdentity,
        remapped_subscription_slice_identity: crate::routing::BridgeSubscriptionSliceIdentity,
        remapped_slices: CanonicalSubscriptionSlices,
        continuity_outcomes: Arc<[ResolvedLineageContinuity]>,
        counters: BridgeContinuityCounters,
    ) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1),
            route_record,
            continuity_request_digest: continuity_request_digest.into(),
            continuity_resolution_digest: continuity_resolution_digest.into(),
            continuity_artifact_identity,
            remapped_subscription_slice_identity,
            remapped_slices,
            continuity_outcomes,
            counters,
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn with_schema_version_for_test(
        mut self,
        schema_version: impl Into<Arc<str>>,
    ) -> Self {
        self.schema_version = schema_version.into();
        self
    }

    pub(crate) fn decode(&self) -> Result<Self, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical continuity record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::replay(
                self.route_identity().clone(),
                self.source_snapshot().clone(),
            )));
        }

        Ok(self.clone())
    }

    pub fn route_record(&self) -> &BridgeRouteRecord {
        &self.route_record
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.route_record.route_identity()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.route_record.source_snapshot()
    }

    pub fn source_branch(&self) -> &crate::input::envelope::TruthBranchIdentity {
        self.route_record.source_branch()
    }

    pub fn continuity_request_digest(&self) -> &str {
        self.continuity_request_digest.as_ref()
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }

    pub fn continuity_artifact_identity(&self) -> &BridgeContinuityIdentity {
        &self.continuity_artifact_identity
    }

    pub fn remapped_subscription_slice_identity(
        &self,
    ) -> &crate::routing::BridgeSubscriptionSliceIdentity {
        &self.remapped_subscription_slice_identity
    }

    pub fn remapped_slices(&self) -> &CanonicalSubscriptionSlices {
        &self.remapped_slices
    }

    pub fn continuity_outcomes(&self) -> &[ResolvedLineageContinuity] {
        &self.continuity_outcomes
    }

    pub fn counters(&self) -> &BridgeContinuityCounters {
        &self.counters
    }
}

pub type BridgeContinuityReplaySummary = BridgeContinuityArtifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityExplanation {
    route_identity: BridgeRouteIdentity,
    source_snapshot: TruthSnapshotIdentity,
    source_branch: crate::input::envelope::TruthBranchIdentity,
    continuity_request_digest: Arc<str>,
    continuity_resolution_digest: Arc<str>,
    continuity_artifact_identity: BridgeContinuityIdentity,
    remapped_subscription_slice_identity: crate::routing::BridgeSubscriptionSliceIdentity,
    remapped_slices: CanonicalSubscriptionSlices,
    continuity_outcomes: Vec<ResolvedLineageContinuity>,
    counters: BridgeContinuityCounters,
}

impl BridgeContinuityExplanation {
    pub(crate) fn from_canonical_record(record: &BridgeCanonicalContinuityRecord) -> Self {
        Self {
            route_identity: record.route_identity().clone(),
            source_snapshot: record.source_snapshot().clone(),
            source_branch: record.source_branch().clone(),
            continuity_request_digest: Arc::from(record.continuity_request_digest()),
            continuity_resolution_digest: Arc::from(record.continuity_resolution_digest()),
            continuity_artifact_identity: record.continuity_artifact_identity().clone(),
            remapped_subscription_slice_identity: record.remapped_subscription_slice_identity().clone(),
            remapped_slices: record.remapped_slices().clone(),
            continuity_outcomes: record.continuity_outcomes().to_vec(),
            counters: *record.counters(),
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub fn source_branch(&self) -> &crate::input::envelope::TruthBranchIdentity {
        &self.source_branch
    }

    pub fn continuity_request_digest(&self) -> &str {
        self.continuity_request_digest.as_ref()
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        self.continuity_resolution_digest.as_ref()
    }

    pub fn continuity_artifact_identity(&self) -> &BridgeContinuityIdentity {
        &self.continuity_artifact_identity
    }

    pub fn remapped_subscription_slice_identity(
        &self,
    ) -> &crate::routing::BridgeSubscriptionSliceIdentity {
        &self.remapped_subscription_slice_identity
    }

    pub fn remapped_slices(&self) -> &CanonicalSubscriptionSlices {
        &self.remapped_slices
    }

    pub fn continuity_outcomes(&self) -> &[ResolvedLineageContinuity] {
        &self.continuity_outcomes
    }

    pub fn counters(&self) -> &BridgeContinuityCounters {
        &self.counters
    }
}
