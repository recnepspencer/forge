use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    BridgePreviewDiscardRecord, BridgePreviewExecutionRecord, BridgePreviewPromotionRecord,
    BridgePreviewReplayBundle, BridgePreviewResidueClass, BridgePreviewSession,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeSignalBranchIdentity,
    BridgeSignalInvalidationDelivery, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, CoarseRoutingMode, InvalidationSink, MappingSelector,
    PreviewActive, PreviewAdmitted, PreviewDeclared, PreviewDiscarded, PreviewPromoted,
    RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
    RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

#[derive(Clone)]
struct StaticSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("preview-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid bridge patch aspect key"),
                "name",
            )],
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
                .map(|read| SnapshotReadRecord::new(read.request_key(), b"preview".to_vec()))
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
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("preview-snapshot"),
            branch_identity.clone(),
            vec![BridgeCommittedPatchItem::new(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid bridge patch aspect key"),
                "name",
            )],
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
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
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
        format!("truth-view:{seed}"),
        format!("source-capability:{seed}"),
        format!("request-shape:{seed}"),
        format!("artifact-schema:{seed}"),
    )
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
        .promote_preview_session(
            active,
            &execution_record,
            &proof,
            format!("commit-boundary:{seed}"),
            format!("authoritative-artifact:{seed}"),
        )
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
        .replay_preview_bundle(promoted.session_identity().as_str())
        .expect("promoted preview replay bundle should succeed");
    (
        runtime,
        promoted,
        execution_record,
        promotion_record,
        replay_bundle,
    )
}
