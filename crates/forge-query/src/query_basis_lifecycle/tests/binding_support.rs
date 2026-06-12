use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId,
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeContinuityAuthorityBasis, BridgeDeliveryReceipt,
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
    BridgeHistoricalResolvedLineageIdentity, BridgeHistoricalResolvedRecordIdentity,
    BridgeLineageContext, BridgeLineageSourceError, BridgeMappingId, BridgeMappingRegistration,
    BridgeProducerMetadata, BridgeSignalInvalidationDelivery, CoarseRoutingMode,
    CommittedPatchSource, ContinuityLineageSource, InvalidationSink, MappingSelector,
    NormalizedSubscriptionSliceIntent, RelationalBridgeSourceError, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SliceWideningPolicy,
    SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadSource, SubscriptionSliceKind, TruthBranchHeadSource, TruthBranchIdentity,
    TruthCommitIdentity, TruthDeltaSurfaceKind, TruthPatchIdentity, TruthPatchScope,
    TruthPatchTargetSelector, TruthSnapshotIdentity, TruthSnapshotReader,
};

pub const MAIN_BRANCH: &str = "main";
pub const OTHER_BRANCH: &str = "other";
pub const COMMIT_A: &str = "commit-a";
pub const SNAPSHOT_A: &str = "snapshot-a";

#[derive(Debug, Clone, Default)]
struct TestRelationalState {
    committed_patches: BTreeMap<String, BridgeCommittedPatchEnvelope>,
    branch_heads: BTreeMap<String, String>,
    snapshots: BTreeMap<String, Vec<SnapshotReadRecord>>,
}

#[derive(Debug, Clone, Default)]
struct TestRelationalSource {
    state: Arc<RwLock<TestRelationalState>>,
}

impl TestRelationalSource {
    fn insert_committed_patch(&self, patch: BridgeCommittedPatchEnvelope) {
        let mut state = self
            .state
            .write()
            .expect("test bridge source lock poisoned");
        state.branch_heads.insert(
            patch
                .branch_identity()
                .evidence_identity()
                .as_str()
                .to_string(),
            patch
                .commit_identity()
                .evidence_identity()
                .as_str()
                .to_string(),
        );
        state.committed_patches.insert(
            patch
                .commit_identity()
                .evidence_identity()
                .as_str()
                .to_string(),
            patch,
        );
    }

    fn insert_snapshot(&self, snapshot_identity: &str, records: Vec<SnapshotReadRecord>) {
        self.state
            .write()
            .expect("test bridge source lock poisoned")
            .snapshots
            .insert(snapshot_identity.to_string(), records);
    }
}

impl CommittedPatchSource for TestRelationalSource {
    fn load_committed_patch(
        &self,
        request: forge_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("test bridge source lock poisoned")
            .committed_patches
            .get(request.commit_identity().evidence_identity().as_str())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no committed patch registered for `{}`",
                    request.commit_identity().evidence_identity().as_str()
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
            .expect("test bridge source lock poisoned")
            .snapshots
            .get(identity.evidence_identity().as_str())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no snapshot registered for `{}`",
                    identity.evidence_identity().as_str()
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let state = self.state.read().expect("test bridge source lock poisoned");
        let commit_identity = state
            .branch_heads
            .get(branch_identity.evidence_identity().as_str())
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no branch head registered for `{}`",
                    branch_identity.evidence_identity().as_str()
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
                    branch_identity.evidence_identity().as_str()
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
        let fixture_value = self
            .records
            .first()
            .and_then(SnapshotReadRecord::scalar_aspect_value)
            .cloned()
            .unwrap_or_else(|| AspectValue::String("unknown".into()));
        let records = request
            .reads()
            .iter()
            .map(|read| SnapshotReadRecord::for_request(read, fixture_value.clone()))
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
            TruthSnapshotIdentity::from_bridge_harness_label(SNAPSHOT_A),
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
            vec![
                BridgeHistoricalResolvedLineageIdentity::from_bridge_harness_label(
                    "lineage:successor",
                ),
            ],
            vec![BridgeHistoricalResolvedRecordIdentity::from_bridge_harness_label("entity:0:4:2")],
            vec![1],
        )
    }
}

pub fn observation_runtime() -> RuntimeBridge {
    let source = base_source();
    build_runtime(source, false)
}

pub fn continuity_runtime() -> RuntimeBridge {
    let source = base_source_with_field_slice();
    build_runtime(source, true)
}

pub fn subscription_runtime() -> RuntimeBridge {
    let source = base_source();
    build_runtime(source, false)
}

pub fn detail_subscription(
    runtime: &RuntimeBridge,
) -> forge_runtime_bridge::facade::AdmittedBridgeSubscription {
    let declaration = runtime
        .declare_subscription(
            forge_runtime_bridge::facade::BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                aspect_key("profile"),
                field_key("name"),
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
                TruthBranchIdentity::from_bridge_harness_label(MAIN_BRANCH),
            ),
        )
        .expect("branch-head subscription basis should admit")
}

pub fn delivered_continuity(
    runtime: &RuntimeBridge,
) -> forge_runtime_bridge::facade::BridgeDeliveredContinuityResult {
    let route = runtime
        .plan_committed_patch_with_mapping_context(
            forge_runtime_bridge::facade::BridgeRouteRequest::for_commit(
                TruthCommitIdentity::from_bridge_harness_label(COMMIT_A),
            ),
            forge_runtime_bridge::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::from_bridge_harness_label(MAIN_BRANCH),
                    TruthSnapshotIdentity::from_bridge_harness_label(SNAPSHOT_A),
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
        BridgeMappingId::from_stable_name("profile-name"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            AspectKeySelector::exact(aspect_key("profile")),
            TruthPatchTargetSelector::entity_field(field_key("name")),
        ),
        SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
        SignalInvalidationScope::from_stable_name("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn field_aspect_registration() -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::from_stable_name("profile-name-field"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            AspectKeySelector::exact(aspect_key("profile")),
            TruthPatchTargetSelector::entity_field(field_key("name")),
        ),
        SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::Disallow,
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
    builder.build().expect("bridge runtime should build")
}

fn base_source() -> TestRelationalSource {
    let source = TestRelationalSource::default();
    let patch = committed_patch(MAIN_BRANCH, COMMIT_A, SNAPSHOT_A, "name");
    source.insert_snapshot(
        patch.snapshot_identity().evidence_identity().as_str(),
        snapshot_records("user:profile", "alice"),
    );
    source.insert_committed_patch(patch);
    source
}

fn base_source_with_field_slice() -> TestRelationalSource {
    let source = TestRelationalSource::default();
    let patch = committed_patch(MAIN_BRANCH, COMMIT_A, SNAPSHOT_A, "name");
    source.insert_snapshot(
        patch.snapshot_identity().evidence_identity().as_str(),
        snapshot_records("user:profile:signal-field:name", "alice"),
    );
    source.insert_committed_patch(patch);
    source
}

fn committed_patch(
    branch: &str,
    commit: &str,
    snapshot: &str,
    surface: &str,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            TruthCommitIdentity::from_bridge_harness_label(commit),
            TruthPatchIdentity::from_bridge_harness_label(format!("patch-{commit}")),
            TruthSnapshotIdentity::from_bridge_harness_label(snapshot),
            TruthBranchIdentity::from_bridge_harness_label(branch),
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "user",
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("profile")),
                CanonicalFieldPath::single(field_key(surface)),
            ),
        )],
    )
    .expect("query basis fixture committed patch envelope should construct")
}

fn snapshot_records(_key: &str, value: &str) -> Vec<SnapshotReadRecord> {
    let read = forge_runtime_bridge::facade::SnapshotReadRequest::for_coarse(
        "user",
        SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
    );
    vec![SnapshotReadRecord::for_request(
        &read,
        AspectValue::String(value.into()),
    )]
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid query basis bridge aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid query basis bridge field key")
}
