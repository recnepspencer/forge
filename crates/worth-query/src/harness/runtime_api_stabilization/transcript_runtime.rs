use std::collections::BTreeMap;

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::facade::foundation::{
    DeclarativeLiveQueryRequest, WorthQueryLiveViewHandle, WorthQueryMutationDelta,
    WorthQueryMutationKind, WorthQueryMutationReceipt, WorthQueryWorkspaceError,
};
use crate::facade::runtime::{
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationInput, WorthQueryAuthorityLane, WorthQueryBasisAdmissionEvidenceRow,
    WorthQueryEffectPolicy, WorthQueryIntentAuthorityAdapter, WorthQueryIntentDeclaration,
    WorthQueryIntentExecution, WorthQueryLiveArtifactTarget, WorthQueryPreviewBasisAdmission,
    WorthQueryRuntime, WorthQueryRuntimeEvidenceAuthority, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimeFamilySupport, WorthQueryRuntimeInspectionEvidence,
    WorthQueryRuntimeInspectorEvidenceAdapter, WorthQueryRuntimePreviewBasisAdapter,
    WorthQueryRuntimeSchemaAdapter, WorthQueryRuntimeSignalSinkAdapter,
    WorthQueryRuntimeSnapshotIdentityAdapter, WorthQueryRuntimeSourceAdapter,
    WorthQueryRuntimeSubscriptionActivationAdapter, WorthQueryRuntimeSupportProfile,
    WorthQueryWriteReceipt,
};
use crate::identity::hash_parts;
use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQuerySnapshotIdentity};
use crate::memory_workspace::{WorthQueryEntity, WorthQueryLivePatch};
use crate::runtime::{
    runtime_subscription_support_evidence_identity, WorthQueryMutationTargetCollectionIdentity,
};
use crate::WorthQuerySessionLabel;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, InvalidationSink, MappingSelector,
    RelationalBridgeSnapshotIdentityParts, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadSource, TruthBranchIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

mod transcript_authority;

use transcript_authority::TranscriptWriteAuthority;

pub(super) fn transcript_runtime() -> WorthQueryRuntime {
    WorthQueryRuntime::builder()
        .runtime_bridge(transcript_bridge())
        .schema_adapter(TranscriptSchemaAdapter)
        .source_adapter(TranscriptSourceAdapter::default())
        .snapshot_identity(TranscriptSnapshotIdentity)
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

fn intent_support_profile() -> WorthQueryRuntimeSupportProfile {
    WorthQueryRuntimeSupportProfile::bridge_backed(
        "transcript-subscription-activation",
        "transcript-preview-basis",
        "transcript-inspector-evidence",
    )
    .with_family_support(WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Intent,
        [
            WorthQueryAuthorityLane::AuthoritativeTruth,
            WorthQueryAuthorityLane::BranchLocalTruth,
            WorthQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["transcript-intent-authority"],
    ))
}

struct TranscriptSchemaAdapter;

struct TranscriptSnapshotIdentity;

impl WorthQueryRuntimeSnapshotIdentityAdapter for TranscriptSnapshotIdentity {
    fn current_snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        WorthQuerySnapshotIdentity::preview(
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    WorthQueryEvidenceTag::new("transcript_snapshot_authority"),
                    "runtime-api-stabilization",
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("transcript_snapshot_sequence"),
                    1,
                )
                .seal(),
        )
    }
}

impl WorthQueryRuntimeSchemaAdapter for TranscriptSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, WorthQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

#[derive(Default)]
struct TranscriptSourceAdapter {
    live_views: BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryMutationTargetCollectionIdentity>,
}

impl WorthQueryRuntimeSourceAdapter for TranscriptSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveViewHandle, WorthQueryWorkspaceError> {
        let live_target = WorthQueryLiveArtifactTarget::from_view_name(name.clone());
        self.live_views
            .insert(live_target, request.target_collection_identity());
        Ok(WorthQueryLiveViewHandle::new(name))
    }

    fn live_entities_for_target(
        &self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &WorthQueryLiveArtifactTarget,
    ) -> Vec<WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Vec<WorthQueryLiveArtifactTarget> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| {
                        delta
                            .target_collection_identity()
                            .same_target_collection_as(collection)
                    })
                    .map(|(target, _)| target.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }
}

struct TranscriptIntentAuthority;

impl WorthQueryIntentAuthorityAdapter for TranscriptIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentExecution, WorthQueryWorkspaceError> {
        let collection = declaration
            .input_string_field("collection")
            .unwrap_or("TranscriptEntity")
            .to_string();
        let mutation_receipt = WorthQueryMutationReceipt::from_authoritative_parts(
            transcript_commit_identity("transcript-intent-commit", &collection),
            transcript_snapshot_identity("transcript-intent-snapshot", &collection),
            vec![WorthQueryMutationDelta::from_touched_aspects(
                collection,
                crate::memory_workspace::admit_authored_entity_label("transcript-intent-entity-1"),
                WorthQueryMutationKind::Updated,
                Vec::new(),
            )],
        );
        Ok(WorthQueryIntentExecution::admitted(
            declaration.strategy_name(),
            declaration.strategy_version(),
            "transcript-strategy-descriptor-digest",
            declaration.input_digest(),
            hash_parts(&[
                "transcript-intent-produced-mutation".to_string(),
                mutation_receipt
                    .commit_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
                mutation_receipt
                    .snapshot_identity
                    .evidence_identity()
                    .as_str()
                    .to_string(),
            ]),
            [
                "transcript-relational-invariant:acyclic",
                "transcript-relational-invariant:authority-lane",
            ],
            mutation_receipt,
        ))
    }
}

fn transcript_commit_identity(namespace: &str, evidence: &str) -> WorthQueryCommitIdentity {
    WorthQueryCommitIdentity::from_relational_commit_id(stable_transcript_position(
        namespace, evidence,
    ))
}

fn transcript_snapshot_identity(namespace: &str, evidence: &str) -> WorthQuerySnapshotIdentity {
    let snapshot_id = stable_transcript_position(namespace, evidence);
    let version_id = stable_transcript_position(format!("{namespace}:version"), evidence);
    WorthQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(snapshot_id, version_id),
    )
}

fn stable_transcript_position(namespace: impl AsRef<str>, evidence: impl AsRef<str>) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.as_ref().bytes().chain(evidence.as_ref().bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}

struct TranscriptSignalSink;

impl WorthQueryRuntimeSignalSinkAdapter for TranscriptSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &WorthQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, WorthQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

struct TranscriptSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for TranscriptSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("transcript-subscription-activation")
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

struct TranscriptPreviewBasis;

impl WorthQueryRuntimePreviewBasisAdapter for TranscriptPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        Ok(WorthQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            WorthQueryBasisAdmissionEvidenceRow::rows_from_values(["transcript-preview-basis"]),
        ))
    }
}

struct TranscriptInspectorEvidence;

impl WorthQueryRuntimeInspectorEvidenceAdapter for TranscriptInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        Ok(WorthQueryRuntimeInspectionEvidence::new(
            authority,
            "transcript-write-receipt",
            receipt.authority_lane(),
            ["transcript-inspector-evidence"],
        ))
    }
}

#[derive(Clone, Debug)]
struct TranscriptBridgeSource;

impl worth_runtime_bridge::facade::CommittedPatchSource for TranscriptBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            worth_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::from_relational_patch_position(1),
                TruthSnapshotIdentity::from_relational_snapshot(
                    RelationalBridgeSnapshotIdentityParts::new(1, 1),
                ),
                TruthBranchIdentity::from_relational_branch_id("main"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "transcript-entity",
                worth_runtime_bridge::facade::BridgeCommittedPatchTarget::entity_field_path(
                    worth_foundational::facade::AspectLocator::new(
                        worth_foundational::facade::LocatorAuthority::Authoritative,
                        worth_foundational::facade::AspectKey::new("transcript-aspect")
                            .expect("valid native bridge patch aspect key"),
                    ),
                    worth_foundational::facade::CanonicalFieldPath::single(
                        worth_foundational::facade::FieldKey::new("value".to_owned())
                            .expect("valid native bridge patch field key"),
                    ),
                ),
            )],
        )
        .expect("native bridge patch envelope fixture must construct"))
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
    ) -> Result<SnapshotReadPacketResult, worth_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| {
                    SnapshotReadRecord::for_request(
                        read,
                        worth_foundational::facade::AspectValue::Null,
                    )
                })
                .collect(),
        ))
    }
}

struct TranscriptBridgeSink;

impl InvalidationSink for TranscriptBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[derive(Clone, Debug)]
struct TranscriptWritebackAuthority;

impl worth_runtime_bridge::facade::TruthWritebackAuthority for TranscriptWritebackAuthority {
    fn execute_writeback(
        &self,
        request: worth_runtime_bridge::facade::TruthWritebackRequest,
    ) -> Result<
        worth_runtime_bridge::facade::TruthWritebackReceipt,
        worth_runtime_bridge::facade::TruthWritebackAuthorityError,
    > {
        Ok(worth_runtime_bridge::facade::TruthWritebackReceipt::new(
            worth_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}

fn transcript_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(TranscriptBridgeSource)
        .with_signal_sink(TranscriptBridgeSink)
        .with_writeback_authority(TranscriptWritebackAuthority)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("transcript-external"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                worth_foundational::facade::AspectKey::new("transcript-aspect")
                    .expect("valid transcript bridge mapping aspect key"),
                worth_foundational::facade::FieldKey::new("value".to_owned())
                    .expect("valid transcript bridge mapping field key"),
            ),
            worth_runtime_bridge::facade::SnapshotReadContract::scalar(
                worth_foundational::facade::AspectKey::new("transcript-aspect")
                    .expect("valid transcript bridge snapshot aspect key"),
                worth_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::from_stable_name("transcript-external"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("transcript bridge should build")
}
