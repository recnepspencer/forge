use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEffectPolicy, ForgeQueryEntity, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQuerySessionLabel, ForgeQueryWorkspaceError,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationInput, WriteAuthorityExecutionReceipt,
};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult,
    SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity, TruthPatchIdentity,
    TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};

pub(super) fn diagnostic_causal_query_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(diagnostic_runtime_bridge())
        .schema_adapter(DiagnosticSchemaAdapter)
        .source_adapter(DiagnosticSourceAdapter)
        .write_authority(DiagnosticWriteAuthority)
        .signal_sink(DiagnosticSignalSink)
        .subscription_activation(DiagnosticSubscriptionActivation)
        .preview_basis(DiagnosticPreviewBasis)
        .inspector_evidence(DiagnosticInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("runtime-backed Query fixture")
}

#[derive(Clone, Debug)]
struct DiagnosticBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for DiagnosticBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch:{}", request.commit_identity().as_str())),
                TruthSnapshotIdentity::new("planar-diagnostic-snapshot"),
                TruthBranchIdentity::new("main"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "planar-diagnostic",
                BridgeCommittedPatchTarget::entity_field_path(
                    AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("identity")),
                    CanonicalFieldPath::single(field_key("id")),
                ),
            )],
        )
        .expect("diagnostic bridge patch envelope"))
    }
}

impl SnapshotReadSource for DiagnosticBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(DiagnosticSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

struct DiagnosticSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for DiagnosticSnapshotReader {
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
                .map(|read| SnapshotReadRecord::for_request(read, AspectValue::Null))
                .collect(),
        ))
    }
}

struct DiagnosticBridgeSink;

impl InvalidationSink for DiagnosticBridgeSink {
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

fn diagnostic_runtime_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(DiagnosticBridgeSource)
        .with_signal_sink(DiagnosticBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("planar-diagnostic"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                aspect_key("identity"),
                field_key("id"),
            ),
            SnapshotReadContract::scalar(aspect_key("identity"), ScalarAspectType::String),
            SignalInvalidationScope::new("planar-diagnostic"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("diagnostic runtime bridge")
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("diagnostic bridge aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("diagnostic bridge field key")
}

struct DiagnosticSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DiagnosticSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &forge_query::facade::QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

struct DiagnosticSourceAdapter;

impl ForgeQueryRuntimeSourceAdapter for DiagnosticSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: forge_query::facade::QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        "planar-diagnostic-snapshot".to_string()
    }
}

struct DiagnosticWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for DiagnosticWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut forge_relational::facade::runtime::RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let receipt = ForgeQueryMutationReceipt {
            commit_identity: "planar-diagnostic-commit".to_string(),
            snapshot_token: "planar-diagnostic-snapshot".to_string(),
            deltas: vec![ForgeQueryMutationDelta {
                collection: command
                    .declared_collection()
                    .unwrap_or_else(|| "planar-diagnostic".to_string()),
                entity_identity: "planar-diagnostic".to_string(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            }],
            bridge_authority: None,
        };
        Ok(self.build_write_authority_execution_receipt(&command, receipt))
    }
}

struct DiagnosticSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for DiagnosticSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt);
        Ok(self.build_signal_invalidation_boundary_receipt(receipt, routed))
    }
}

struct DiagnosticSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DiagnosticSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "planar-diagnostic-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

struct DiagnosticPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for DiagnosticPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            ["planar-diagnostic-preview-basis"],
        ))
    }
}

struct DiagnosticInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for DiagnosticInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "planar-diagnostic-write-receipt",
            receipt.authority_lane(),
            ["planar-diagnostic-inspector-evidence"],
        ))
    }
}
