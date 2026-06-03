use forge_foundational::facade::{
    AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity,
    BridgeCommittedPatchItem, BridgeCommittedPatchTarget, BridgeDeliveryIntent,
    BridgeDiagnosticsTier, BridgeReplayMode, BridgeRuntimePolicy, BridgeSignalInvalidationDelivery,
    BridgeSnapshotReadError, BridgeSourceAdapter, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector,
    CoarseRoutingMode, HistoricalEvaluationDeclaration, InvalidationSink,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, SourceDeclaration,
    SourceDeclarationIdentity, TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthPatchTargetSelector, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

use crate::historical::{
    admit_historical_evaluation_path, lower_materialization_from_decision_log,
    lower_policy_resolution, resolve_historical_materialization_path, HistoricalEvaluationRequest,
    HistoricalPathReuseDescriptor,
};
use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_historical_bridge_lowering_row() -> RepresentativeArtifacts {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Required,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let request = HistoricalEvaluationRequest::delta_replay(
        declaration.declaration_identity().as_str(),
        4,
        8,
        HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
    );
    let capability = lower_policy_resolution(
        &declaration,
        &runtime.resolve_truth_view_policy(&declaration),
        None,
        request.requested_path_class(),
    )
    .expect("historical policy should lower");
    let admission = admit_historical_evaluation_path(request, capability)
        .expect("historical path should admit");
    let evaluation = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            )
            .with_replay_mode(BridgeReplayMode::Required),
        )
        .expect("historical evaluation should succeed");
    let lowered = lower_materialization_from_decision_log(evaluation.record().decision_log())
        .expect("historical decision log should lower");
    let resolved = resolve_historical_materialization_path(admission, lowered)
        .expect("historical path should resolve");

    let retained_evidence = hash_parts(&[
        declaration.declaration_identity().as_str().to_string(),
        resolved.resolved_path_class().as_str().to_string(),
        resolved.complexity_contract().contract_name().to_string(),
    ]);
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Historical policy lowering",
        hash_parts(&[
            "historical_bridge_lowering_subject_v1".to_string(),
            format!(
                "declaration:{}",
                declaration.declaration_identity().as_str()
            ),
            format!("resolved:{}", resolved.resolved_path_class().as_str()),
        ]),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        hash_parts(&[
            resolved.admitted_path_class().as_str().to_string(),
            resolved.resolved_path_class().as_str().to_string(),
        ]),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        resolved.complexity_contract().contract_name(),
    );
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        retained_evidence.clone(),
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        &route_plan,
        &boundary_receipt,
        &retained_evidence,
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

#[derive(Clone)]
struct StaticSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                forge_runtime_bridge::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid native bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid native bridge patch field key"),
                    ),
                ),
            )],
        )
        .expect("native bridge patch envelope fixture must construct"))
    }
}

impl SnapshotReadSource for StaticSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
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
        Ok(BridgeCommittedPatchEnvelope::new(
            BridgeCommittedPatchEnvelopeIdentity::new(
                TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
                TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
                TruthSnapshotIdentity::new("snapshot-a"),
                branch_identity.clone(),
            ),
            vec![profile_name_patch_item("entity-1")],
        )
        .expect("historical branch-head fixture must build a native patch envelope"))
    }
}

#[derive(Clone)]
struct StaticSnapshotReader;

impl TruthSnapshotReader for StaticSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-a")
    }

    fn read_packet(
        &self,
        request: &forge_runtime_bridge::facade::SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        Ok(SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            request
                .reads()
                .iter()
                .map(|read| {
                    SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::String("fixture".into()),
                    )
                })
                .collect(),
        ))
    }
}

#[derive(Clone)]
struct StaticSourceAdapter;

impl BridgeSourceAdapter for StaticSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayContinuityRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(StaticSnapshotReader))
        } else {
            Err(RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

struct StaticSink;

impl InvalidationSink for StaticSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<forge_runtime_bridge::facade::BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(forge_runtime_bridge::facade::BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(policy)
        .with_relational_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ],
        ))
        .register_mapping(
            forge_runtime_bridge::facade::BridgeMappingRegistration::new(
                forge_runtime_bridge::facade::BridgeMappingId::new("mapping"),
                TruthPatchScope::new(
                    forge_runtime_bridge::facade::MappingSelector::exact("entity-1"),
                    AspectKeySelector::exact(aspect_key("profile")),
                    TruthPatchTargetSelector::entity_field(field_key("name")),
                ),
                SnapshotReadContract::scalar(aspect_key("profile"), ScalarAspectType::String),
                SignalInvalidationScope::new("signal:profile"),
                CoarseRoutingMode::Direct,
            ),
        )
        .build()
        .expect("historical lower-runtime runtime should build")
}

fn profile_name_patch_item(entity_identity: &str) -> BridgeCommittedPatchItem {
    BridgeCommittedPatchItem::with_target(
        entity_identity,
        BridgeCommittedPatchTarget::entity_field_path(
            AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("profile")),
            CanonicalFieldPath::single(field_key("name")),
        ),
    )
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid historical fixture aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid historical fixture field key")
}

fn registered_source(
    id: &str,
    selector: BridgeTruthViewSelector,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(id),
        selector,
        BridgeSourceCapabilitySet::new(capabilities),
    )
}
