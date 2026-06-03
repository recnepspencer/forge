use forge_foundational::facade::{
    AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity,
    BridgeCommittedPatchItem, BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, BridgePreviewDiscardRecord, BridgePreviewExecutionRecord,
    BridgePreviewPromotionRecord, BridgePreviewReplayBundle, BridgePreviewResidueClass,
    BridgePreviewRetainedArtifactSchema, BridgePreviewSession, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeSignalBranchIdentity,
    BridgeSignalInvalidationDelivery, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
    BridgeTruthViewSelector, CoarseRoutingMode, InvalidationSink, MappingSelector, PreviewActive,
    PreviewAdmitted, PreviewDeclared, PreviewDiscarded, PreviewPromoted,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthPatchTargetSelector, TruthSnapshotIdentity, TruthSnapshotReader,
};

#[derive(Clone)]
struct StaticSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_patch_envelope(
            request.commit_identity().clone(),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("preview-snapshot"),
            TruthBranchIdentity::new("main"),
        ))
    }
}

#[derive(Clone)]
struct StaticSnapshotReader;

impl TruthSnapshotReader for StaticSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("preview-snapshot")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("preview-snapshot"),
            request
                .reads()
                .iter()
                .map(|read| {
                    SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::String("preview".into()),
                    )
                })
                .collect(),
        ))
    }
}

impl forge_runtime_bridge::facade::SnapshotReadSource for StaticSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "preview-snapshot" {
            Ok(Box::new(StaticSnapshotReader))
        } else {
            Err(RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl TruthBranchHeadSource for StaticSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_patch_envelope(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("preview-snapshot"),
            branch_identity.clone(),
        ))
    }
}

#[derive(Clone)]
struct StaticSink;

impl InvalidationSink for StaticSink {
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

fn runtime() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(StaticSource)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("preview-mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                AspectKeySelector::exact(aspect_key("profile")),
                TruthPatchTargetSelector::entity_field(field_key("name")),
            ),
            SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("preview fixture runtime should build")
}

fn preview_declaration(seed: &str) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new(format!("preview-declaration:{seed}")),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new(format!("preview-binding:{seed}")),
            TruthBranchIdentity::new("main"),
            BridgeSignalBranchIdentity::new(format!("signal:{seed}")),
        ),
        preview_session_basis(seed),
    )
}

fn native_patch_envelope(
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "entity-1",
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("profile")),
                CanonicalFieldPath::single(field_key("name")),
            ),
        )],
    )
    .expect("preview native patch envelope should construct")
}

fn preview_session_basis(seed: &str) -> BridgePreviewSessionBasis {
    BridgePreviewSessionBasis::new(
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new(format!("preview-snapshot:{seed}")),
        ),
        BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
        BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
    )
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid preview fixture aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid preview fixture field key")
}

pub fn declared_preview_session(
    seed: &str,
) -> (RuntimeBridge, BridgePreviewSession<PreviewDeclared>) {
    let runtime = runtime();
    let declared = runtime
        .declare_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{seed}")),
            preview_declaration(seed),
        )
        .expect("declared preview session should succeed");
    (runtime, declared)
}

pub fn admitted_preview_session(
    seed: &str,
) -> (RuntimeBridge, BridgePreviewSession<PreviewAdmitted>) {
    let runtime = runtime();
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{seed}")),
            preview_declaration(seed),
        )
        .expect("admitted preview session should succeed");
    (runtime, admitted)
}

pub fn active_preview_artifacts(
    seed: &str,
) -> (
    RuntimeBridge,
    BridgePreviewSession<PreviewActive>,
    BridgePreviewExecutionRecord,
) {
    let runtime = runtime();
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{seed}")),
            preview_declaration(seed),
        )
        .expect("admitted preview session should succeed");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 3, 1, 1);
    (runtime, active, execution_record)
}

pub fn discarded_preview_artifacts(
    seed: &str,
) -> (
    RuntimeBridge,
    BridgePreviewSession<PreviewDiscarded>,
    BridgePreviewDiscardRecord,
) {
    let runtime = runtime();
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{seed}")),
            preview_declaration(seed),
        )
        .expect("admitted preview session should succeed");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);
    let (discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::TemporaryRoutingResidue,
            ],
        )
        .expect("discarded preview session should succeed");
    (runtime, discarded, discard_record)
}

pub fn promoted_preview_artifacts(
    seed: &str,
) -> (
    RuntimeBridge,
    BridgePreviewSession<PreviewPromoted>,
    BridgePreviewExecutionRecord,
    BridgePreviewPromotionRecord,
) {
    let runtime = runtime();
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{seed}")),
            preview_declaration(seed),
        )
        .expect("admitted preview session should succeed");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);
    let proof = active.promotion_admissibility_proof();
    let (promoted, promotion_record) = runtime
        .promote_preview_session(active, &execution_record, &proof)
        .expect("promoted preview session should succeed");
    (runtime, promoted, execution_record, promotion_record)
}

pub fn promoted_preview_replay_bundle(
    seed: &str,
) -> (
    RuntimeBridge,
    BridgePreviewSession<PreviewPromoted>,
    BridgePreviewExecutionRecord,
    BridgePreviewPromotionRecord,
    BridgePreviewReplayBundle,
) {
    let (runtime, promoted, execution_record, promotion_record) = promoted_preview_artifacts(seed);
    let replay_bundle = runtime
        .replay_preview_bundle(promoted.session_identity())
        .expect("promoted preview replay bundle should succeed");
    (
        runtime,
        promoted,
        execution_record,
        promotion_record,
        replay_bundle,
    )
}
