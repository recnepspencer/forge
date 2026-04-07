use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSourceError, SignalBridgeSinkError,
    SnapshotReadSource, TruthBranchHeadSource,
};
use crate::delivery::BridgeDeliveryReceipt;
use crate::facade::{
    BridgeAspectRegistration, BridgeLineageContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgeMappingRegistration, BridgeRuntimePolicy,
    RawCommittedPatchEnvelope, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotFixture {
    identity: TruthSnapshotIdentity,
    read_result_identity: TruthSnapshotIdentity,
    records: Vec<SnapshotReadRecord>,
}

impl SnapshotFixture {
    pub fn new(identity: TruthSnapshotIdentity, records: Vec<SnapshotReadRecord>) -> Self {
        Self {
            read_result_identity: identity.clone(),
            identity,
            records,
        }
    }

    pub fn with_read_result_identity(mut self, identity: TruthSnapshotIdentity) -> Self {
        self.read_result_identity = identity;
        self
    }

    pub fn identity(&self) -> &TruthSnapshotIdentity {
        &self.identity
    }

    pub fn read_result_identity(&self) -> &TruthSnapshotIdentity {
        &self.read_result_identity
    }

    pub fn records(&self) -> &[SnapshotReadRecord] {
        &self.records
    }
}

#[derive(Debug, Clone)]
pub struct BridgeHarnessFixture {
    policy: BridgeRuntimePolicy,
    mappings: Vec<BridgeMappingRegistration>,
    aspect_mappings: Vec<BridgeAspectRegistration>,
    committed_patches: Vec<RawCommittedPatchEnvelope>,
    snapshots: Vec<SnapshotFixture>,
    lineage_context: Option<BridgeLineageContext>,
    continuity_authorities: Vec<(String, BridgeHistoricalLineageAuthority)>,
}

impl BridgeHarnessFixture {
    pub fn new(mappings: Vec<BridgeMappingRegistration>) -> Self {
        Self {
            policy: BridgeRuntimePolicy::development(),
            mappings,
            aspect_mappings: Vec::new(),
            committed_patches: Vec::new(),
            snapshots: Vec::new(),
            lineage_context: None,
            continuity_authorities: Vec::new(),
        }
    }

    pub fn with_policy(mut self, policy: BridgeRuntimePolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_committed_patch(mut self, patch: RawCommittedPatchEnvelope) -> Self {
        self.committed_patches.push(patch);
        self
    }

    pub fn with_aspect_mapping(mut self, aspect_mapping: BridgeAspectRegistration) -> Self {
        self.aspect_mappings.push(aspect_mapping);
        self
    }

    pub fn with_snapshot(mut self, snapshot: SnapshotFixture) -> Self {
        self.snapshots.push(snapshot);
        self
    }

    pub fn with_lineage_context(mut self, lineage_context: BridgeLineageContext) -> Self {
        self.lineage_context = Some(lineage_context);
        self
    }

    pub fn with_continuity_authority(
        mut self,
        entity_identity: impl Into<String>,
        authority: BridgeHistoricalLineageAuthority,
    ) -> Self {
        self.continuity_authorities
            .push((entity_identity.into(), authority));
        self
    }

    pub fn policy(&self) -> BridgeRuntimePolicy {
        self.policy
    }

    pub fn mappings(&self) -> &[BridgeMappingRegistration] {
        &self.mappings
    }

    pub fn committed_patches(&self) -> &[RawCommittedPatchEnvelope] {
        &self.committed_patches
    }

    pub fn aspect_mappings(&self) -> &[BridgeAspectRegistration] {
        &self.aspect_mappings
    }

    pub fn snapshots(&self) -> &[SnapshotFixture] {
        &self.snapshots
    }

    pub fn lineage_context(&self) -> Option<&BridgeLineageContext> {
        self.lineage_context.as_ref()
    }

    pub fn continuity_authorities(&self) -> &[(String, BridgeHistoricalLineageAuthority)] {
        &self.continuity_authorities
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSignalDelivery {
    pub delivery: crate::routing::BridgeSignalInvalidationDelivery,
}

#[derive(Debug, Clone, Default)]
struct InMemoryRelationalState {
    committed_patches: BTreeMap<String, RawCommittedPatchEnvelope>,
    branch_heads: BTreeMap<String, String>,
    snapshots: BTreeMap<String, SnapshotFixture>,
    continuity_authorities: BTreeMap<String, BridgeHistoricalLineageAuthority>,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryRelationalBridgeSource {
    state: Arc<RwLock<InMemoryRelationalState>>,
}

impl InMemoryRelationalBridgeSource {
    pub fn insert_committed_patch(&self, patch: RawCommittedPatchEnvelope) {
        self.state
            .write()
            .expect("bridge source lock poisoned")
            .committed_patches
            .insert(patch.commit_identity().as_str().to_string(), patch.clone());
        self.state
            .write()
            .expect("bridge source lock poisoned")
            .branch_heads
            .insert(
                patch.branch_identity().as_str().to_string(),
                patch.commit_identity().as_str().to_string(),
            );
    }

    pub fn insert_snapshot(&self, snapshot: SnapshotFixture) {
        self.state
            .write()
            .expect("bridge source lock poisoned")
            .snapshots
            .insert(snapshot.identity().as_str().to_string(), snapshot);
    }

    pub fn insert_continuity_authority(
        &self,
        entity_identity: impl Into<String>,
        authority: BridgeHistoricalLineageAuthority,
    ) {
        self.state
            .write()
            .expect("bridge source lock poisoned")
            .continuity_authorities
            .insert(entity_identity.into(), authority);
    }
}

impl CommittedPatchSource for InMemoryRelationalBridgeSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("bridge source lock poisoned")
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

impl SnapshotReadSource for InMemoryRelationalBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        let snapshot = self
            .state
            .read()
            .expect("bridge source lock poisoned")
            .snapshots
            .get(identity.as_str())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no snapshot registered for `{}`",
                    identity.as_str()
                ))
            })?;
        Ok(Box::new(InMemorySnapshotReader { snapshot }))
    }
}

impl TruthBranchHeadSource for InMemoryRelationalBridgeSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &crate::facade::TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let state = self.state.read().expect("bridge source lock poisoned");
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
                    "branch head `{}` for `{}` had no registered committed patch envelope",
                    commit_identity,
                    branch_identity.as_str()
                ))
            })
    }
}

impl ContinuityLineageSource for InMemoryRelationalBridgeSource {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
        self.state
            .read()
            .expect("bridge source lock poisoned")
            .continuity_authorities
            .get(request.prior_slice().entity_identity())
            .cloned()
            .ok_or_else(|| {
                BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                    format!(
                        "no continuity lineage authority registered for `{}`",
                        request.prior_slice().entity_identity()
                    ),
                )
            })
    }
}

#[derive(Debug, Clone)]
struct InMemorySnapshotReader {
    snapshot: SnapshotFixture,
}

impl TruthSnapshotReader for InMemorySnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot.identity().clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError> {
        let record_lookup = self
            .snapshot
            .records()
            .iter()
            .map(|record| (record.request_key().to_string(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let records = request
            .reads()
            .iter()
            .filter_map(|read| record_lookup.get(read.request_key()).cloned())
            .collect::<Vec<_>>();
        Ok(SnapshotReadPacketResult::new(
            self.snapshot.read_result_identity().clone(),
            records,
        ))
    }
}

#[derive(Debug, Clone, Default)]
pub struct RecordingSignalBridgeSink {
    deliveries: Arc<RwLock<Vec<RecordedSignalDelivery>>>,
}

impl RecordingSignalBridgeSink {
    pub fn deliveries(&self) -> Vec<RecordedSignalDelivery> {
        self.deliveries
            .read()
            .expect("bridge sink lock poisoned")
            .clone()
    }

    pub fn last_delivery(&self) -> Option<RecordedSignalDelivery> {
        self.deliveries().into_iter().last()
    }
}

impl InvalidationSink for RecordingSignalBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: crate::routing::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        self.deliveries
            .write()
            .expect("bridge sink lock poisoned")
            .push(RecordedSignalDelivery {
                delivery: delivery.clone(),
            });

        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}
