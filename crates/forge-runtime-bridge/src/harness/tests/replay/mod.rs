use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeHistoricalResolvedLineageIdentity, BridgeHistoricalResolvedRecordIdentity,
    BridgeLineageContext, BridgeLineageSourceError, BridgeRouteRequest, ContinuityLineageSource,
    SubscriptionSliceKind, TruthDeltaSurfaceKind, BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1,
};
use forge_harness::facade::{ExecutionProfile, ExecutionRequest, ReplayRequest, ScenarioPlan};
use forge_harness::runtime::{HarnessAdapter, ReplayHarnessAdapter};

use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, committed_patch_items,
    field_aspect_registration, field_aspect_registration_with_kind, field_slice_snapshot,
    registration, snapshot,
};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
};

#[derive(Debug, Clone, Default)]
struct ReplaySingleSuccessorLineageSource;

impl ContinuityLineageSource for ReplaySingleSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![BridgeHistoricalResolvedLineageIdentity::new(
                "lineage:replay-successor",
            )],
            vec![BridgeHistoricalResolvedRecordIdentity::new("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct ReplayDriftedSuccessorLineageSource;

impl ContinuityLineageSource for ReplayDriftedSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![BridgeHistoricalResolvedLineageIdentity::new(
                "lineage:replay-successor",
            )],
            vec![BridgeHistoricalResolvedRecordIdentity::new("entity:0:9:2")],
            vec![7],
        )
    }
}

mod bulk;
mod continuity;
mod route;
