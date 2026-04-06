use std::sync::Arc;

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::routing::BridgeRouteOutcomeReference;

use super::BridgeRouteRecord;

pub const BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2: &str = "forge-runtime-bridge.route-record.v2";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalRouteRecord {
    schema_version: Arc<str>,
    route_record: BridgeRouteRecord,
}

impl BridgeCanonicalRouteRecord {
    pub(crate) fn from_route_record(route_record: BridgeRouteRecord) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2),
            route_record,
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

    pub(crate) fn decode(&self) -> Result<BridgeRouteRecord, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical route record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V2
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.route_record.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeReplayArtifact {
    outcome: BridgeRouteOutcomeReference,
}

impl BridgeReplayArtifact {
    pub(crate) fn from_route_record(record: BridgeRouteRecord) -> Self {
        Self {
            outcome: BridgeRouteOutcomeReference::new(
                record.route_identity().clone(),
                record.invalidation_identity().clone(),
                crate::routing::BridgeRouteSourceSummary::new(
                    record.source_commit().clone(),
                    record.source_patch().clone(),
                    record.source_snapshot().clone(),
                ),
                record.subscription_slice_identity().clone(),
            ),
        }
    }

    pub(crate) fn new(outcome: BridgeRouteOutcomeReference) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &BridgeRouteOutcomeReference {
        &self.outcome
    }

    pub fn route_identity(&self) -> &crate::routing::BridgeRouteIdentity {
        self.outcome.route_identity()
    }

    pub fn invalidation_identity(&self) -> &crate::routing::BridgeInvalidationIdentity {
        self.outcome.invalidation_identity()
    }

    pub fn subscription_slice_identity(&self) -> &crate::routing::BridgeSubscriptionSliceIdentity {
        self.outcome.subscription_slice_identity()
    }

    pub fn source_commit(&self) -> &crate::input::envelope::TruthCommitIdentity {
        self.outcome.source_commit()
    }

    pub fn source_patch(&self) -> &crate::input::envelope::TruthPatchIdentity {
        self.outcome.source_patch()
    }

    pub fn source_snapshot(&self) -> &crate::snapshot::TruthSnapshotIdentity {
        self.outcome.source_snapshot()
    }
}
pub type BridgeReplayRecord = BridgeReplayArtifact;
pub type BridgeReplaySummary = BridgeReplayArtifact;
