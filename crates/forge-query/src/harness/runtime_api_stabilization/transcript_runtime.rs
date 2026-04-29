use std::collections::BTreeMap;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
    TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
    TruthSnapshotReader,
};
use serde_json::Value;

use crate::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryAuthorityLane, ForgeQueryEffectPolicy,
    ForgeQueryIntentAuthorityAdapter, ForgeQueryIntentDeclaration, ForgeQueryIntentExecution,
    ForgeQueryLiveViewHandle, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeSupportProfile, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand, ForgeQueryWriteReceipt, QuerySchemaView,
    SubscriptionActivationInput,
};
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryEntity, ForgeQueryLivePatch};

pub(super) fn transcript_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(transcript_bridge())
        .schema_adapter(TranscriptSchemaAdapter)
        .source_adapter(TranscriptSourceAdapter::default())
        .write_authority(TranscriptWriteAuthority)
        .signal_sink(TranscriptSignalSink)
        .subscription_activation(TranscriptSubscriptionActivation)
        .preview_basis(TranscriptPreviewBasis)
        .inspector_evidence(TranscriptInspectorEvidence)
        .intent_authority(TranscriptIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("transcript runtime backend parts should build")
}

fn intent_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "transcript-subscription-activation",
        "transcript-preview-basis",
        "transcript-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Intent,
        [
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            ForgeQueryAuthorityLane::BranchLocalTruth,
            ForgeQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["transcript-intent-authority"],
    ))
}

struct TranscriptSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for TranscriptSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

#[derive(Default)]
struct TranscriptSourceAdapter {
    live_views: BTreeMap<String, String>,
}

impl ForgeQueryRuntimeSourceAdapter for TranscriptSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn snapshot_token(&self) -> String {
        "transcript-external-snapshot".to_string()
    }
}

struct TranscriptWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TranscriptWriteAuthority {
    #[allow(deprecated)]
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let (collection, aspect_paths) = match command {
            ForgeQueryWriteCommand::Insert { collection, .. } => (collection, Vec::new()),
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
            } => (
                collection,
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::UpdateAspect { aspect_path, .. } => {
                ("TranscriptEntity".to_string(), vec![aspect_path])
            }
            ForgeQueryWriteCommand::UpdateAspects { aspects, .. } => (
                "TranscriptEntity".to_string(),
                aspects
                    .iter()
                    .map(|aspect| aspect.aspect_path().to_string())
                    .collect(),
            ),
            ForgeQueryWriteCommand::Delete { .. } => ("TranscriptEntity".to_string(), Vec::new()),
        };
        Ok(ForgeQueryMutationReceipt {
            commit_identity: format!("transcript-commit:{collection}"),
            snapshot_token: format!("transcript-snapshot:{collection}"),
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: "transcript-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths,
            }],
        })
    }
}

struct TranscriptIntentAuthority;

impl ForgeQueryIntentAuthorityAdapter for TranscriptIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let collection = declaration
            .input()
            .get("collection")
            .and_then(Value::as_str)
            .unwrap_or("TranscriptEntity")
            .to_string();
        let mutation_receipt = ForgeQueryMutationReceipt {
            commit_identity: format!("transcript-intent-commit:{collection}"),
            snapshot_token: format!("transcript-intent-snapshot:{collection}"),
            deltas: vec![ForgeQueryMutationDelta {
                collection,
                entity_identity: "transcript-intent-entity-1".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: Vec::new(),
            }],
        };
        Ok(ForgeQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "transcript-strategy-descriptor-digest",
            declaration.input_digest(),
            hash_parts(&[
                "transcript-intent-produced-mutation".to_string(),
                mutation_receipt.commit_identity.clone(),
                mutation_receipt.snapshot_token.clone(),
            ]),
            [
                "transcript-relational-invariant:acyclic",
                "transcript-relational-invariant:authority-lane",
            ],
            mutation_receipt,
        ))
    }
}

struct TranscriptSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TranscriptSignalSink {
    fn route_write_receipt(
        &mut self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

struct TranscriptSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TranscriptSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "transcript-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "transcript-subscription-activation:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

struct TranscriptPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TranscriptPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["transcript-preview-basis"],
        ))
    }
}

struct TranscriptInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for TranscriptInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "transcript-write-receipt",
            receipt.authority_lane(),
            ["transcript-inspector-evidence"],
        ))
    }
}

#[derive(Clone, Debug)]
struct TranscriptBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for TranscriptBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("transcript-external-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "transcript-entity",
                "transcript-aspect",
                "value",
            )],
        ))
    }
}

impl SnapshotReadSource for TranscriptBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(TranscriptSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

struct TranscriptSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for TranscriptSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| SnapshotReadRecord::new(read.request_key(), Vec::new()))
                .collect(),
        ))
    }
}

struct TranscriptBridgeSink;

impl InvalidationSink for TranscriptBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn transcript_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(TranscriptBridgeSource)
        .with_signal_sink(TranscriptBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("transcript-external"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("transcript-external"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("transcript bridge should build")
}
