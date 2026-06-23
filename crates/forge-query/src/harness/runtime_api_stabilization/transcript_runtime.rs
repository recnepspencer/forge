use std::collections::BTreeMap;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryAuthorityLane, ForgeQueryBasisAdmissionEvidenceRow,
    ForgeQueryEffectPolicy, ForgeQueryIntentAuthorityAdapter, ForgeQueryIntentDeclaration,
    ForgeQueryIntentExecution, ForgeQueryLiveArtifactTarget, ForgeQueryLiveViewHandle,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntime, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspaceError, ForgeQueryWriteReceipt,
    LiveViewDeclarationAdmissionBoundaryReceipt, QuerySchemaView,
    SignalInvalidationBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
    SubscriptionActivationInput,
};
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity};
use crate::memory_workspace::{ForgeQueryEntity, ForgeQueryLivePatch};
use crate::runtime::{
    runtime_subscription_support_evidence_identity, ForgeQueryMutationTargetCollectionIdentity,
};
use crate::ForgeQuerySessionLabel;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::{
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

pub(super) fn transcript_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
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

struct TranscriptSnapshotIdentity;

impl ForgeQueryRuntimeSnapshotIdentityAdapter for TranscriptSnapshotIdentity {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    ForgeQueryEvidenceTag::new("transcript_snapshot_authority"),
                    "runtime-api-stabilization",
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("transcript_snapshot_sequence"),
                    1,
                )
                .seal(),
        )
    }
}

impl ForgeQueryRuntimeSchemaAdapter for TranscriptSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

#[derive(Default)]
struct TranscriptSourceAdapter {
    live_views: BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQueryRuntimeSourceAdapter for TranscriptSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        let live_target = ForgeQueryLiveArtifactTarget::from_view_name(name.clone());
        self.live_views
            .insert(live_target, request.target_collection_identity());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities_for_target(
        &self,
        _target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget> {
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

impl ForgeQueryIntentAuthorityAdapter for TranscriptIntentAuthority {
    fn execute_intent(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError> {
        let collection = declaration
            .input_string_field("collection")
            .unwrap_or("TranscriptEntity")
            .to_string();
        let mutation_receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
            transcript_commit_identity("transcript-intent-commit", &collection),
            transcript_snapshot_identity("transcript-intent-snapshot", &collection),
            vec![ForgeQueryMutationDelta::from_touched_aspects(
                collection,
                crate::memory_workspace::admit_authored_entity_label("transcript-intent-entity-1"),
                ForgeQueryMutationKind::Updated,
                Vec::new(),
            )],
        );
        Ok(ForgeQueryIntentExecution::admitted(
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

fn transcript_commit_identity(namespace: &str, evidence: &str) -> ForgeQueryCommitIdentity {
    ForgeQueryCommitIdentity::from_relational_commit_id(stable_transcript_position(
        namespace, evidence,
    ))
}

fn transcript_snapshot_identity(namespace: &str, evidence: &str) -> ForgeQuerySnapshotIdentity {
    let snapshot_id = stable_transcript_position(namespace, evidence);
    let version_id = stable_transcript_position(format!("{namespace}:version"), evidence);
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
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

impl ForgeQueryRuntimeSignalSinkAdapter for TranscriptSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let routed = self.build_signal_invalidation_routing_receipt(receipt)?;
        self.build_signal_invalidation_boundary_receipt(receipt, routed)
    }
}

struct TranscriptSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TranscriptSubscriptionActivation {
    fn support_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("transcript-subscription-activation")
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

struct TranscriptPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TranscriptPreviewBasis {
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
            ForgeQueryBasisAdmissionEvidenceRow::rows_from_values(["transcript-preview-basis"]),
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::from_relational_patch_position(1),
                TruthSnapshotIdentity::from_relational_snapshot(
                    RelationalBridgeSnapshotIdentityParts::new(1, 1),
                ),
                TruthBranchIdentity::from_relational_branch_id("main"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "transcript-entity",
                forge_runtime_bridge::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("transcript-aspect")
                            .expect("valid native bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("value".to_owned())
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
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| {
                    SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::Null,
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
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[derive(Clone, Debug)]
struct TranscriptWritebackAuthority;

impl forge_runtime_bridge::facade::TruthWritebackAuthority for TranscriptWritebackAuthority {
    fn execute_writeback(
        &self,
        request: forge_runtime_bridge::facade::TruthWritebackRequest,
    ) -> Result<
        forge_runtime_bridge::facade::TruthWritebackReceipt,
        forge_runtime_bridge::facade::TruthWritebackAuthorityError,
    > {
        Ok(forge_runtime_bridge::facade::TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
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
                forge_foundational::facade::AspectKey::new("transcript-aspect")
                    .expect("valid transcript bridge mapping aspect key"),
                forge_foundational::facade::FieldKey::new("value".to_owned())
                    .expect("valid transcript bridge mapping field key"),
            ),
            forge_runtime_bridge::facade::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("transcript-aspect")
                    .expect("valid transcript bridge snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::from_stable_name("transcript-external"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("transcript bridge should build")
}
