use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_runtime_bridge::facade::{
    AdmittedBridgeSubscription, BridgeAspectRegistration, BridgeAspectRegistrationId,
    BridgeContinuityAuthorityBasis, BridgeDeliveredContinuityResult, BridgeDeliveryReceipt,
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeLineageContext,
    BridgeLineageSourceError, BridgeMappingId, BridgeMappingRegistration, BridgeProducerMetadata,
    BridgeSignalInvalidationDelivery, CoarseRoutingMode, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, MappingSelector, NormalizedSubscriptionSliceIntent,
    RawCommittedPatchEnvelope, RelationalBridgeSourceError, RuntimeBridge, RuntimeBridgeBuilder,
    SignalBridgeSinkError, SignalInvalidationScope, SliceFallbackPolicy, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, SubscriptionSliceKind,
    TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};

pub(crate) const PHASE_SIX_MAIN_BRANCH: &str = "main";
const COMMIT_A: &str = "commit-a";
const SNAPSHOT_A: &str = "snapshot-a";

#[derive(Debug, Clone, Default)]
struct TestRelationalState {
    committed_patches: BTreeMap<String, RawCommittedPatchEnvelope>,
    branch_heads: BTreeMap<String, String>,
    snapshots: BTreeMap<String, Vec<SnapshotReadRecord>>,
}

#[derive(Debug, Clone, Default)]
struct TestRelationalSource {
    state: Arc<RwLock<TestRelationalState>>,
}

impl TestRelationalSource {
    fn insert_committed_patch(&self, patch: RawCommittedPatchEnvelope) {
        let mut state = self
            .state
            .write()
            .expect("fixture bridge source lock poisoned");
        state.branch_heads.insert(
            patch.branch_identity().as_str().to_string(),
            patch.commit_identity().as_str().to_string(),
        );
        state
            .committed_patches
            .insert(patch.commit_identity().as_str().to_string(), patch);
    }

    fn insert_snapshot(&self, snapshot_identity: &str, records: Vec<SnapshotReadRecord>) {
        self.state
            .write()
            .expect("fixture bridge source lock poisoned")
            .snapshots
            .insert(snapshot_identity.to_string(), records);
    }
}

impl CommittedPatchSource for TestRelationalSource {
    fn load_committed_patch(
        &self,
        request: forge_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("fixture bridge source lock poisoned")
            .committed_patches
            .get(request.commit_identity())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no committed patch registered for `{}`",
                    request.commit_identity()
                ))
            })
    }
}

impl SnapshotReadSource for TestRelationalSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        let records = self
            .state
            .read()
            .expect("fixture bridge source lock poisoned")
            .snapshots
            .get(identity.as_str())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no snapshot registered for `{}`",
                    identity.as_str()
                ))
            })?;
        Ok(Box::new(TestSnapshotReader {
            snapshot_identity: identity.clone(),
            records,
        }))
    }
}

impl TruthBranchHeadSource for TestRelationalSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let state = self
            .state
            .read()
            .expect("fixture bridge source lock poisoned");
        let commit_identity = state
            .branch_heads
            .get(branch_identity.as_str())
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no branch head registered for `{}`",
                    branch_identity.as_str()
                ))
            })?;
        state
            .committed_patches
            .get(commit_identity)
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "branch head `{}` for `{}` had no patch envelope",
                    commit_identity,
                    branch_identity.as_str()
                ))
            })
    }
}

#[derive(Debug, Clone)]
struct TestSnapshotReader {
    snapshot_identity: TruthSnapshotIdentity,
    records: Vec<SnapshotReadRecord>,
}

impl TruthSnapshotReader for TestSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot_identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        let lookup = self
            .records
            .iter()
            .map(|record| (record.request_key().to_string(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let records = request
            .reads()
            .iter()
            .filter_map(|read| lookup.get(read.request_key()).cloned())
            .collect::<Vec<_>>();
        Ok(SnapshotReadPacketResult::new(
            self.snapshot_identity.clone(),
            records,
        ))
    }
}

#[derive(Debug, Clone, Default)]
struct NoopSignalSink;

impl InvalidationSink for NoopSignalSink {
    fn deliver_invalidation(
        &self,
        _delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            1,
            TruthSnapshotIdentity::new(SNAPSHOT_A),
        ))
    }
}

#[derive(Debug, Clone, Default)]
struct FixedLineageSource;

impl ContinuityLineageSource for FixedLineageSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        BridgeHistoricalLineageAuthority::try_new(
            request.authority_basis().clone(),
            vec![Arc::from("lineage:successor")],
            vec![Arc::from("entity:0:4:2")],
            vec![1],
        )
    }
}

pub(crate) fn observation_runtime() -> RuntimeBridge {
    build_runtime(base_source(), false)
}

pub(crate) fn continuity_runtime() -> RuntimeBridge {
    build_runtime(base_source_with_field_slice(), true)
}

pub(crate) fn subscription_runtime() -> RuntimeBridge {
    build_runtime(base_source(), false)
}

pub(crate) fn detail_subscription(runtime: &RuntimeBridge) -> AdmittedBridgeSubscription {
    let declaration = runtime
        .declare_subscription(
            forge_runtime_bridge::facade::BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("detail slice intent should validate")],
            forge_runtime_bridge::facade::BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("detail declaration should admit");
    runtime
        .admit_subscription(
            &declaration,
            forge_runtime_bridge::facade::BridgeSubscriptionBasisRequest::branch_head(
                TruthBranchIdentity::new(PHASE_SIX_MAIN_BRANCH),
            ),
        )
        .expect("branch-head subscription basis should admit")
}

pub(crate) fn delivered_continuity(runtime: &RuntimeBridge) -> BridgeDeliveredContinuityResult {
    let route = runtime
        .plan_committed_patch_with_mapping_context(
            forge_runtime_bridge::facade::BridgeRouteRequest::for_commit(COMMIT_A),
            forge_runtime_bridge::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new(PHASE_SIX_MAIN_BRANCH),
                    TruthSnapshotIdentity::new(SNAPSHOT_A),
                )),
            ),
        )
        .expect("continuity route should plan");
    runtime
        .deliver_invalidation(route)
        .expect("continuity route should deliver");
    let route_record = runtime
        .diagnostics()
        .last_route_record()
        .expect("continuity route record should be retained");
    runtime
        .deliver_continuity(&route_record)
        .expect("continuity should deliver")
}

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        SignalInvalidationScope::new("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn field_aspect_registration() -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new("profile-name-field"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )
}

fn build_runtime(source: TestRelationalSource, with_continuity: bool) -> RuntimeBridge {
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(NoopSignalSink)
        .register_mapping(registration());
    let builder = if with_continuity {
        builder
            .with_continuity_lineage_source(FixedLineageSource)
            .register_aspect_mapping(field_aspect_registration())
    } else {
        builder
    };
    builder
        .build()
        .expect("fixture bridge runtime should build")
}

fn base_source() -> TestRelationalSource {
    let source = TestRelationalSource::default();
    source.insert_committed_patch(committed_patch(
        PHASE_SIX_MAIN_BRANCH,
        COMMIT_A,
        SNAPSHOT_A,
        "name",
    ));
    source.insert_snapshot(SNAPSHOT_A, snapshot_records("user:profile", "alice"));
    source
}

fn base_source_with_field_slice() -> TestRelationalSource {
    let source = TestRelationalSource::default();
    source.insert_committed_patch(committed_patch(
        PHASE_SIX_MAIN_BRANCH,
        COMMIT_A,
        SNAPSHOT_A,
        "name",
    ));
    source.insert_snapshot(
        SNAPSHOT_A,
        snapshot_records("user:profile:signal-field:name", "alice"),
    );
    source
}

fn committed_patch(
    branch: &str,
    commit: &str,
    snapshot: &str,
    surface: &str,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new_with_metadata(
        BridgeProducerMetadata::bridge_harness_fixture(),
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(format!("patch-{commit}")),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new(branch),
        vec![forge_runtime_bridge::facade::BridgeCommittedPatchItem::new(
            "user", "profile", surface,
        )],
    )
}

fn snapshot_records(key: &str, value: &str) -> Vec<SnapshotReadRecord> {
    vec![SnapshotReadRecord::new(key, value.as_bytes().to_vec())]
}
