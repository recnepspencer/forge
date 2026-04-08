use forge_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeDiagnosticsTier, BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeLineageContext, BridgeLineageSourceError, BridgeMappingContext, BridgeRouteRequest,
    BridgeBulkDecisionRecordKind, BridgeBulkPlanningFailureKind, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgeParallelLegalityClass, BridgeParallelLegalityReason,
    BridgeParallelProfitabilityClass, BridgeParallelProfitabilityReason, BridgePreparationMode,
    BridgeRuntimePolicy, ContinuityLineageSource, FineGrainedMatchStatus, SubscriptionSliceKind,
    TruthSnapshotIdentity,
};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::{BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime, committed_patch, committed_patch_items, field_aspect_registration,
    field_slice_snapshot, registration, snapshot, surface_fallback_registration,
};

#[derive(Debug, Clone, Default)]
struct TestContinuityLineageSource;

impl ContinuityLineageSource for TestContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:test-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestMismatchedAuthorityLineageSource;

impl ContinuityLineageSource for TestMismatchedAuthorityLineageSource {
    fn historical_lineage(
        &self,
        _request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            BridgeContinuityAuthorityBasis::new(
                crate::facade::TruthBranchIdentity::new("wrong-branch"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![Arc::from("lineage:test-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestUnsupportedContinuityLineageSource;

impl ContinuityLineageSource for TestUnsupportedContinuityLineageSource {
    fn historical_lineage(
        &self,
        _request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        Err(crate::facade::BridgeLineageSourceError::new(
            crate::facade::BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
            "unsupported continuity class",
        ))
    }
}

#[derive(Debug, Clone, Default)]
struct TestSplitContinuityLineageSource;

impl ContinuityLineageSource for TestSplitContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-split-a"),
                Arc::from("lineage:test-split-b"),
            ],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![7, 8],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestMergeLikeContinuityLineageSource;

impl ContinuityLineageSource for TestMergeLikeContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-merge-a"),
                Arc::from("lineage:test-merge-b"),
            ],
            vec![Arc::from("entity:0:9:3")],
            vec![7, 8],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestAmbiguousContinuityLineageSource;

impl ContinuityLineageSource for TestAmbiguousContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![
                Arc::from("lineage:test-ambiguous-a"),
                Arc::from("lineage:test-ambiguous-b"),
                Arc::from("lineage:test-ambiguous-c"),
            ],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![7, 8, 9],
        )
    }
}

#[derive(Debug, Clone, Default)]
struct TestNoAuthoritativeSuccessorLineageSource;

impl ContinuityLineageSource for TestNoAuthoritativeSuccessorLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            Vec::new(),
            Vec::new(),
            vec![7],
        )
    }
}

#[derive(Debug, Clone)]
struct CountingContinuityLineageSource {
    call_count: Arc<AtomicUsize>,
}

impl CountingContinuityLineageSource {
    fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }
}

impl ContinuityLineageSource for CountingContinuityLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:test-successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![7],
        )
    }
}


mod route_identity;
mod bulk_workload;
mod packet_reduction;
mod continuity;
