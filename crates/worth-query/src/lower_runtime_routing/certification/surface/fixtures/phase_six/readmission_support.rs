use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use worth_foundational::facade::{
    AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
use worth_runtime_bridge::facade::{
    AdmittedBridgeSubscription, AspectKeySelector, BridgeAspectRegistration,
    BridgeAspectRegistrationId, BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity,
    BridgeCommittedPatchItem, BridgeCommittedPatchTarget, BridgeContinuityAuthorityBasis,
    BridgeDeliveredContinuityResult, BridgeDeliveryReceipt, BridgeHistoricalLineageAuthority,
    BridgeHistoricalLineageRequest, BridgeHistoricalResolvedLineageIdentity,
    BridgeHistoricalResolvedRecordIdentity, BridgeLineageContext, BridgeLineageSourceError,
    BridgeMappingId, BridgeMappingRegistration, BridgeProducerMetadata,
    BridgeSignalInvalidationDelivery, CoarseRoutingMode, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, MappingSelector, NormalizedSubscriptionSliceIntent,
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
    RelationalBridgeSourceError, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SliceWideningPolicy, SnapshotReadContract, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, SubscriptionSliceKind,
    TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthPatchScope, TruthPatchTargetSelector, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

pub(crate) const PHASE_SIX_MAIN_BRANCH: &str = "main";
const COMMIT_A: &str = "commit-a";
const SNAPSHOT_A: &str = "snapshot-a";

#[derive(Debug, Clone, Default)]
struct TestRelationalState {
    committed_patches: BTreeMap<TruthCommitIdentity, BridgeCommittedPatchEnvelope>,
    branch_heads: BTreeMap<TruthBranchIdentity, TruthCommitIdentity>,
    snapshots: BTreeMap<TruthSnapshotIdentity, Vec<SnapshotReadRecord>>,
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
            .expect("fixture bridge source lock poisoned");
        state.branch_heads.insert(
            patch.branch_identity().clone(),
            patch.commit_identity().clone(),
        );
        state
            .committed_patches
            .insert(patch.commit_identity().clone(), patch);
    }

    fn insert_snapshot(&self, snapshot_identity: &str, records: Vec<SnapshotReadRecord>) {
        let snapshot_key = fixture_snapshot_identity(snapshot_identity);
        self.state
            .write()
            .expect("fixture bridge source lock poisoned")
            .snapshots
            .insert(snapshot_key, records);
    }
}

impl CommittedPatchSource for TestRelationalSource {
    fn load_committed_patch(
        &self,
        request: worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("fixture bridge source lock poisoned")
            .committed_patches
            .get(request.commit_identity())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no committed patch registered for `{:?}`",
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
            .get(identity)
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no snapshot registered for `{:?}`",
                    identity
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
        let state = self
            .state
            .read()
            .expect("fixture bridge source lock poisoned");
        let commit_identity = state.branch_heads.get(branch_identity).ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "no branch head registered for `{:?}`",
                branch_identity
            ))
        })?;
        state
            .committed_patches
            .get(commit_identity)
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "branch head `{:?}` for `{:?}` had no patch envelope",
                    commit_identity, branch_identity
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
    ) -> Result<SnapshotReadPacketResult, worth_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        let fixture_value = self
            .records
            .first()
            .and_then(SnapshotReadRecord::scalar_aspect_value)
            .cloned()
            .unwrap_or_else(|| {
                crate::runtime::WorthQueryAdmittedAspectValue::native_string_value("unknown")
            });
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
            fixture_snapshot_identity(SNAPSHOT_A),
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
            vec![BridgeHistoricalResolvedLineageIdentity::from_relational_lineage_id(1)],
            vec![
                BridgeHistoricalResolvedRecordIdentity::from_relational_record(
                    RelationalBridgeRecordIdentityParts::entity(0, 4, 2),
                ),
            ],
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
            worth_runtime_bridge::facade::BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                aspect_key("profile"),
                field_key("name"),
                SubscriptionSliceKind::SignalField,
            )
            .expect("detail slice intent should validate")],
            worth_runtime_bridge::facade::BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("detail declaration should admit");
    runtime
        .admit_subscription(
            &declaration,
            worth_runtime_bridge::facade::BridgeSubscriptionBasisRequest::branch_head(
                fixture_branch_identity(PHASE_SIX_MAIN_BRANCH),
            ),
        )
        .expect("branch-head subscription basis should admit")
}

pub(crate) fn delivered_continuity(runtime: &RuntimeBridge) -> BridgeDeliveredContinuityResult {
    let route = runtime
        .plan_committed_patch_with_mapping_context(
            worth_runtime_bridge::facade::BridgeRouteRequest::for_commit(fixture_commit_identity(
                COMMIT_A,
            )),
            worth_runtime_bridge::facade::BridgeMappingContext::default().with_lineage_context(
                BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
                    fixture_branch_identity(PHASE_SIX_MAIN_BRANCH),
                    fixture_snapshot_identity(SNAPSHOT_A),
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
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            fixture_commit_identity(commit),
            fixture_patch_identity(commit),
            fixture_snapshot_identity(snapshot),
            fixture_branch_identity(branch),
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "user",
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("profile")),
                CanonicalFieldPath::single(field_key(surface)),
            ),
        )],
    )
    .expect("readmission fixture committed patch envelope should construct")
}

fn snapshot_records(_key: &str, value: &str) -> Vec<SnapshotReadRecord> {
    let read = worth_runtime_bridge::facade::SnapshotReadRequest::for_coarse(
        "user",
        SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
    );
    vec![SnapshotReadRecord::for_request(
        &read,
        crate::runtime::WorthQueryAdmittedAspectValue::native_string_value(value),
    )]
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid phase-six bridge aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid phase-six bridge field key")
}

fn fixture_branch_identity(branch: &str) -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id(branch)
}

fn fixture_commit_identity(commit: &str) -> TruthCommitIdentity {
    TruthCommitIdentity::from_relational_commit_id(match commit {
        COMMIT_A => 6,
        _ => 7,
    })
}

fn fixture_patch_identity(commit: &str) -> TruthPatchIdentity {
    TruthPatchIdentity::from_relational_patch_position(match commit {
        COMMIT_A => 6,
        _ => 7,
    })
}

fn fixture_snapshot_identity(snapshot: &str) -> TruthSnapshotIdentity {
    let snapshot_id = match snapshot {
        SNAPSHOT_A => 6,
        "external-snapshot" => 7,
        _ => 8,
    };
    TruthSnapshotIdentity::from_relational_snapshot(RelationalBridgeSnapshotIdentityParts::new(
        snapshot_id,
        1,
    ))
}
