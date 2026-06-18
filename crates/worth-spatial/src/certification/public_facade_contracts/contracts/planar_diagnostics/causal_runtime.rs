use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryCommitIdentity,
    ForgeQueryEffectPolicy, ForgeQueryEntity, ForgeQueryEntityIdentity, ForgeQueryEvidenceIdentity,
    ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQueryMutationDelta, ForgeQueryMutationKind,
    ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSnapshotIdentityAdapter, ForgeQueryRuntimeSourceAdapter,
    ForgeQueryRuntimeSubscriptionActivationAdapter, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt, LiveViewDeclarationAdmissionBoundaryReceipt,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationInput, WriteAuthorityExecutionReceipt,
};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RelationalBridgeRecordIdentityParts,
    RelationalBridgeSnapshotIdentityParts, RelationalBridgeSourceError,
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
        .snapshot_identity(DiagnosticSnapshotIdentity)
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
                TruthPatchIdentity::from_relational_patch_position(1),
                TruthSnapshotIdentity::from_relational_snapshot(
                    RelationalBridgeSnapshotIdentityParts::new(1, 1),
                ),
                TruthBranchIdentity::from_relational_branch_id("main"),
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
            BridgeMappingId::from_stable_name("planar-diagnostic"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                aspect_key("identity"),
                field_key("id"),
            ),
            SnapshotReadContract::scalar(aspect_key("identity"), ScalarAspectType::String),
            SignalInvalidationScope::from_stable_name("planar-diagnostic"),
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

struct DiagnosticSnapshotIdentity;

impl ForgeQueryRuntimeSnapshotIdentityAdapter for DiagnosticSnapshotIdentity {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        )
    }
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
}

struct DiagnosticWriteAuthority;

impl ForgeQueryRuntimeWriteAuthorityAdapter for DiagnosticWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut forge_relational::facade::runtime::RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
            ForgeQueryCommitIdentity::from_relational_commit_id(1),
            ForgeQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(1, 1),
            ),
            vec![ForgeQueryMutationDelta::new(
                command
                    .declared_collection()
                    .unwrap_or_else(|| "planar-diagnostic".to_string()),
                ForgeQueryEntityIdentity::from_relational_record(
                    RelationalBridgeRecordIdentityParts::entity(1, 1, 0),
                ),
                ForgeQueryMutationKind::Updated,
                command.declared_aspect_paths(),
            )],
        );
        Ok(self.build_write_authority_execution_receipt(&command, receipt))
    }
}

struct DiagnosticSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for DiagnosticSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

struct DiagnosticSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DiagnosticSubscriptionActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query::facade::runtime_subscription_support_evidence_identity(
            "planar-diagnostic-subscription-activation",
        )
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
            ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
                "planar-diagnostic-preview-basis",
            ]),
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
