use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration, BridgeRuntimePolicy,
    BridgeSignalInvalidationDelivery, BridgeSnapshotReadError, BridgeSourceAdapter,
    BridgeSourceCapability, BridgeSourceCapabilitySet, BridgeTruthViewSelector, CoarseRoutingMode,
    InvalidationSink, MappingSelector, RawCommittedPatchEnvelope, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader,
};
use std::sync::Arc;

type ProjectionBridgeRow = (String, String, String);

pub(super) fn projection_bridge_runtime() -> RuntimeBridge {
    let rows = Arc::new(vec![
        (
            "entity-1".to_string(),
            "task-1".to_string(),
            "todo".to_string(),
        ),
        (
            "entity-2".to_string(),
            "task-2".to_string(),
            "doing".to_string(),
        ),
    ]);
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(ProjectionBridgeSource { rows: rows.clone() })
        .with_source_adapter(ProjectionBridgeSourceAdapter { rows: rows.clone() })
        .with_truth_branch_head_source(ProjectionBridgeSource { rows })
        .with_signal_sink(ProjectionBridgeSink)
        .register_source(SourceDeclaration::new(
            SourceDeclarationIdentity::new("source:lower-runtime-certification"),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("main"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ]),
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("projection-bridge-mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("status"),
                MappingSelector::exact("lane"),
            ),
            SignalInvalidationScope::new("signal:projection-bridge"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("projection bridge runtime should build")
}

#[derive(Clone)]
struct ProjectionBridgeSource {
    rows: Arc<Vec<ProjectionBridgeRow>>,
}

impl forge_runtime_bridge::facade::CommittedPatchSource for ProjectionBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("main"),
            vec![],
        ))
    }
}

#[derive(Clone)]
struct ProjectionBridgeSnapshotReader {
    rows: Arc<Vec<ProjectionBridgeRow>>,
}

impl TruthSnapshotReader for ProjectionBridgeSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-a")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        Ok(SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            request
                .reads()
                .iter()
                .map(|read| {
                    let payload = self
                        .rows
                        .iter()
                        .find_map(|(entity_identity, identity_value, grouping_value)| {
                            (read.entity_identity() == entity_identity.as_str()).then(|| match read
                                .aspect_label()
                            {
                                "identity.id" => identity_value.as_bytes().to_vec(),
                                "status" => grouping_value.as_bytes().to_vec(),
                                _ => b"unknown".to_vec(),
                            })
                        })
                        .unwrap_or_else(|| b"unknown".to_vec());
                    SnapshotReadRecord::new(read.request_key(), payload)
                })
                .collect(),
        ))
    }
}

impl SnapshotReadSource for ProjectionBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(ProjectionBridgeSnapshotReader {
                rows: self.rows.clone(),
            }))
        } else {
            Err(RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl TruthBranchHeadSource for ProjectionBridgeSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("snapshot-a"),
            branch_identity.clone(),
            vec![],
        ))
    }
}

#[derive(Clone)]
struct ProjectionBridgeSourceAdapter {
    rows: Arc<Vec<ProjectionBridgeRow>>,
}

impl BridgeSourceAdapter for ProjectionBridgeSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        ProjectionBridgeSource {
            rows: self.rows.clone(),
        }
        .open_snapshot(identity)
    }
}

struct ProjectionBridgeSink;

impl InvalidationSink for ProjectionBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}
