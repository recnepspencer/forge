use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReference, BridgeCausalEvidenceReferenceIdentity,
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, BridgeRouteIdentity, BridgeSignalInvalidationDelivery,
    BridgeSnapshotReadError, BridgeSourceAdapter, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeTruthViewSelector, CoarseRoutingMode, InvalidationSink,
    MappingSelector, RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
    SourceDeclaration, SourceDeclarationIdentity, StructuralFingerprintEquivalenceContract,
    StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity,
    StructuralTruthViewBasis, TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthPatchTargetSelector, TruthSnapshotIdentity,
    TruthSnapshotReader, TruthWritebackAuthority, TruthWritebackAuthorityError,
    TruthWritebackReceipt, TruthWritebackRequest,
};

use super::super::super::super::*;

struct MaterializationSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for MaterializationSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::from_bridge_harness_label(format!(
                    "patch-{}",
                    request.commit_identity().evidence_identity()
                )),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-causal-materialization"),
                TruthBranchIdentity::from_bridge_harness_label("analysis"),
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

impl SnapshotReadSource for MaterializationSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(MaterializationSnapshotReader::new(
            identity.clone(),
        )))
    }
}

impl TruthBranchHeadSource for MaterializationSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                TruthCommitIdentity::from_bridge_harness_label(format!(
                    "head-{}",
                    branch_identity.evidence_identity()
                )),
                TruthPatchIdentity::from_bridge_harness_label(format!(
                    "patch-{}",
                    branch_identity.evidence_identity()
                )),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-causal-materialization"),
                branch_identity.clone(),
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
        .expect("native bridge branch head envelope fixture must construct"))
    }
}

impl BridgeSourceAdapter for MaterializationSource {
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
        Ok(Box::new(MaterializationSnapshotReader::new(
            identity.clone(),
        )))
    }
}

struct MaterializationSnapshotReader {
    snapshot_identity: TruthSnapshotIdentity,
}

impl MaterializationSnapshotReader {
    fn new(snapshot_identity: TruthSnapshotIdentity) -> Self {
        Self { snapshot_identity }
    }
}

impl TruthSnapshotReader for MaterializationSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot_identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        Ok(SnapshotReadPacketResult::new(
            self.snapshot_identity.clone(),
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

struct MaterializationSink;

impl InvalidationSink for MaterializationSink {
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

struct MaterializationWritebackAuthority;

impl TruthWritebackAuthority for MaterializationWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}

pub(in crate::runtime::tests::causal_inspection) fn bridge_runtime() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(MaterializationSource)
        .with_source_adapter(MaterializationSource)
        .with_truth_branch_head_source(MaterializationSource)
        .with_signal_sink(MaterializationSink)
        .with_writeback_authority(MaterializationWritebackAuthority)
        .register_source(registered_source(
            "source:causal-materialization-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::from_bridge_harness_label("analysis"),
                TruthCommitIdentity::from_bridge_harness_label("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ],
        ))
        .register_structural(registered_structural(
            "structural:causal-materialization-snapshot",
            StructuralFingerprintFamily::TopologyFingerprint,
            StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::from_bridge_harness_label("analysis"),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-causal-materialization"),
            )),
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("mapping:causal-materialization"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                AspectKeySelector::exact(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid native mapping aspect key"),
                ),
                TruthPatchTargetSelector::entity_field(
                    forge_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid native mapping field key"),
                ),
            ),
            SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native snapshot aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::from_stable_name("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("bridge runtime should build")
}

pub(in crate::runtime::tests::causal_inspection) fn registered_source(
    id: &str,
    selector: BridgeTruthViewSelector,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::from_stable_name(id),
        selector,
        BridgeSourceCapabilitySet::new(capabilities),
    )
}

pub(in crate::runtime::tests::causal_inspection) fn registered_structural(
    id: &str,
    family: StructuralFingerprintFamily,
    truth_view_basis: StructuralTruthViewBasis,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::from_stable_name(id),
        StructuralSchemaIdentity::from_stable_name("schema:causal-materialization"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::from_stable_name("schema:causal-materialization"),
            family,
            "causal-materialization-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        truth_view_basis,
    )
}

pub(in crate::runtime::tests::causal_inspection) fn bridge_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    let family = identity.family();
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge causal reference should be valid")
}

pub(in crate::runtime::tests::causal_inspection) fn external_reference(
    owner: BridgeCausalEvidenceOwner,
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(owner, identity.family(), identity)
        .expect("external causal reference should be valid")
}

pub(in crate::runtime::tests::causal_inspection) fn query_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
    )
    .expect("query causal reference should be valid")
}

pub(in crate::runtime::tests::causal_inspection) fn changed_reference_set(
    route_identity: &BridgeRouteIdentity,
) -> CausalEvidenceReferenceSet {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Changed,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                        "query-inspection:phase5",
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeRoute,
                    route_identity.evidence_identity(),
                ),
            ],
        ),
        CausalInspectionReason::ChangedResult,
    )
    .expect("changed receipt should anchor");
    let CausalEvidenceReferenceResolution::Resolved { reference_set, .. } =
        resolve_causal_evidence_references(
            anchor,
            &[
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
            ],
        )
    else {
        panic!("changed references should resolve");
    };
    reference_set
}

pub(in crate::runtime::tests::causal_inspection) fn replay_reference_set_with_signal_cursor(
    route_identity: &BridgeRouteIdentity,
    signal_replay_cursor_identity: &str,
) -> CausalEvidenceReferenceSet {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Replayed,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                        "query-inspection:replay-materialization",
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeRoute,
                    route_identity.evidence_identity(),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::SignalReplayCursor,
                    crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                        signal_replay_cursor_identity,
                    ),
                ),
            ],
        ),
        CausalInspectionReason::HistoricalReplayResult,
    )
    .expect("replay receipt should anchor");
    let CausalEvidenceReferenceResolution::Resolved { reference_set, .. } =
        resolve_causal_evidence_references(
            anchor,
            &[
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
                CausalEvidenceFamily::SignalReplayCursor,
            ],
        )
    else {
        panic!("replay references should resolve");
    };
    reference_set
}

pub(in crate::runtime::tests::causal_inspection) fn request_for(
    reference_set: CausalEvidenceReferenceSet,
    richness: CausalInspectionRichness,
) -> CausalInspectionRequest {
    let receipt = reference_set.anchor().observation_receipt();
    let target = causal_inspection_target(
        receipt.observation_target().clone(),
        receipt.result_shape_context().clone(),
    )
    .expect("target should match receipt");
    request_causal_inspection(
        reference_set,
        target,
        CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
        richness,
        &[CausalEvidenceFamily::BridgeRoute],
    )
    .expect("causal inspection request should be admitted to admission boundary")
}

pub(in crate::runtime::tests::causal_inspection) fn request_for_families(
    reference_set: CausalEvidenceReferenceSet,
    richness: CausalInspectionRichness,
    requested_evidence_families: &[CausalEvidenceFamily],
) -> CausalInspectionRequest {
    let receipt = reference_set.anchor().observation_receipt();
    let target = causal_inspection_target(
        receipt.observation_target().clone(),
        receipt.result_shape_context().clone(),
    )
    .expect("target should match receipt");
    request_causal_inspection(
        reference_set,
        target,
        CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
        richness,
        requested_evidence_families,
    )
    .expect("causal inspection request should be admitted to admission boundary")
}
