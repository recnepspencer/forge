//! Public API boundary for `forge-relational`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub mod config {
    pub use crate::config::data::{
        AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CheckpointPolicy,
        CommitStrategiesConfig, CompiledLanePolicy, ConfigProvenance, ConfigProvenanceEntry,
        ConfigValueSource, CrossContextPolicy, DiagnosticsBoundary, DurabilityPolicy,
        DurableLogPolicy, DurableLogRetentionMode, MvccConfig, PatchSurfacePolicy,
        PublicationConfig, RelationalConfigOverride, RelationalRuntimeProfile, RetentionBackend,
        RetentionPolicy, RuntimeExecutionLane, RuntimeProfileBoundaryPolicy, SnapshotReleasePolicy,
        StorageLayoutConfig, VisibilityCachePolicy,
    };
}

pub mod commit_strategies {
    pub use crate::commit_strategies::data::{
        CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact,
        CanonicalStrategyInputDigest, CanonicalStrategyOutputArtifact,
        CanonicalStrategyOutputDigest, CommitStrategyDescriptor, CommitStrategyDescriptorDigest,
        CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyFamilyName,
        CommitStrategyId, CommitStrategyRegistration, CommitStrategyRegistrationError,
        CommitStrategySemanticName, CommitStrategyVersion, LoweredStrategyCommitPlan,
        PersistentArtifactName, RawStrategyCommitRequest, StrategyCallerProvenance,
        StrategyCommitArtifactBundle, StrategyCommitRequestError, StrategyExecutionDraft,
        StrategyExecutionResult, StrategyExecutionSummary, StrategyExecutorFailure,
        StrategyExecutorFailureClass, StrategyInputSchemaName, StrategyInputSchemaVersion,
        StrategyIntentName, StrategyIntentScopeDigest, StrategyLoweringError,
        StrategyLoweringProvenance, StrategyLoweringSummary, StrategyMergeConflictClass,
        StrategyMergeDescriptor, StrategyMergeSemantics, StrategyMutationProgram,
        StrategyMutationProgramDigest, StrategyObservationContext, StrategyOutputSchemaName,
        StrategyPacketContract, StrategyReadContract, StrategyReadCostClass,
        StrategyReadLocalityClass, StrategyReadScopeClass, StrategyReplayDescriptor,
        StrategyRequestCanonicalization, StrategyRequestOrigin, StrategyTraversalBasis,
        StrategyVisibilityReadView, ValidatedStrategyCommitPlan,
    };
    pub use crate::commit_strategies::facade::{
        CommitStrategiesAuthorityFacade, CommitStrategiesFacade,
    };
    pub use crate::commit_strategies::strategies::{
        AspectFieldReconciliationInput, AspectFieldReconciliationOutput,
        AspectFieldReconciliationStrategy, EntityReplacementReconciliationAction,
        EntityReplacementReconciliationInput, EntityReplacementReconciliationOutput,
        EntityReplacementReconciliationStrategy, IntentReconciliationAction,
        IntentReconciliationInput, IntentReconciliationOutput, IntentReconciliationStrategy,
        ReplicaConvergenceAction, ReplicaConvergenceInput, ReplicaConvergenceOutput,
        ReplicaConvergenceStrategy,
    };
    pub use crate::commit_strategies::{FrozenCommitStrategyRegistry, StrategyExecutionError};
}

pub mod bridge {
    use std::collections::BTreeMap;
    use std::sync::{Arc, RwLock};

    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchItem, CommittedPatchSource, RawCommittedPatchEnvelope,
        RelationalBridgeSourceError, RelationalCommittedPatchRequest, SnapshotReadPacket,
        SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
        TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthSnapshotIdentity,
        TruthSnapshotReader,
    };

    use crate::history::data::CommitId;
    use crate::publication::patch::data::{PatchRecord, RelationalPatchRecord};
    use crate::symbols::data::InternedString;
    use crate::transactions::data::RecordRef;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PublicationBridgeSnapshot {
        identity: TruthSnapshotIdentity,
        read_result_identity: TruthSnapshotIdentity,
        records: Vec<SnapshotReadRecord>,
    }

    impl PublicationBridgeSnapshot {
        pub fn new(
            identity: TruthSnapshotIdentity,
            records: Vec<SnapshotReadRecord>,
        ) -> Self {
            Self {
                read_result_identity: identity.clone(),
                identity,
                records,
            }
        }

        pub fn with_read_result_identity(mut self, identity: TruthSnapshotIdentity) -> Self {
            self.read_result_identity = identity;
            self
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct PublicationBridgeCatalog {
        state: Arc<RwLock<PublicationBridgeCatalogState>>,
    }

    #[derive(Debug, Default)]
    struct PublicationBridgeCatalogState {
        committed_patches: BTreeMap<String, RawCommittedPatchEnvelope>,
        snapshots: BTreeMap<String, PublicationBridgeSnapshot>,
    }

    impl PublicationBridgeCatalog {
        pub fn register_patch(
            &self,
            commit_id: CommitId,
            branch_identity: impl Into<String>,
            snapshot_identity: impl Into<String>,
            patch: &RelationalPatchRecord,
        ) {
            let envelope = publication_patch_to_bridge_envelope(
                commit_id,
                branch_identity,
                snapshot_identity,
                patch,
            );
            self.state
                .write()
                .expect("publication bridge catalog lock poisoned")
                .committed_patches
                .insert(envelope.commit_identity().as_str().to_string(), envelope);
        }

        pub fn register_snapshot(&self, snapshot: PublicationBridgeSnapshot) {
            self.state
                .write()
                .expect("publication bridge catalog lock poisoned")
                .snapshots
                .insert(snapshot.identity.as_str().to_string(), snapshot);
        }
    }

    impl CommittedPatchSource for PublicationBridgeCatalog {
        fn load_committed_patch(
            &self,
            request: RelationalCommittedPatchRequest,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            self.state
                .read()
                .expect("publication bridge catalog lock poisoned")
                .committed_patches
                .get(request.commit_identity())
                .cloned()
                .ok_or_else(|| {
                    RelationalBridgeSourceError::new(format!(
                        "no publication bridge patch registered for commit `{}`",
                        request.commit_identity()
                    ))
                })
        }

    }

    impl SnapshotReadSource for PublicationBridgeCatalog {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
            let snapshot = self
                .state
                .read()
                .expect("publication bridge catalog lock poisoned")
                .snapshots
                .get(identity.as_str())
                .cloned()
                .ok_or_else(|| {
                    RelationalBridgeSourceError::new(format!(
                        "no publication bridge snapshot registered for `{}`",
                        identity.as_str()
                    ))
                })?;
            Ok(Box::new(PublicationSnapshotReader { snapshot }))
        }
    }

    pub fn publication_patch_to_bridge_envelope(
        commit_id: CommitId,
        branch_identity: impl Into<String>,
        snapshot_identity: impl Into<String>,
        patch: &RelationalPatchRecord,
    ) -> RawCommittedPatchEnvelope {
        let snapshot_identity = TruthSnapshotIdentity::new(snapshot_identity.into());
        RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("commit-{}", commit_id.0)),
            TruthPatchIdentity::new(format!("patch-{}", patch.position.0)),
            snapshot_identity,
            TruthBranchIdentity::new(branch_identity.into()),
            bridge_patch_items(&patch.canonicalized().records),
        )
    }

    fn bridge_patch_items(records: &[PatchRecord]) -> Vec<BridgeCommittedPatchItem> {
        let mut items = Vec::new();
        for record in records {
            let entity_identity = record_ref_identity(&record.target);
            if record.aspects.is_empty() {
                items.push(BridgeCommittedPatchItem::new(
                    entity_identity.clone(),
                    structural_change_label(record),
                    "structural",
                ));
                continue;
            }

            for aspect in record.aspects.iter() {
                items.push(BridgeCommittedPatchItem::new(
                    entity_identity.clone(),
                    aspect_key_label(aspect.0.clone()),
                    structural_change_label(record),
                ));
            }
        }
        items
    }

    fn record_ref_identity(record: &RecordRef) -> String {
        match record {
            RecordRef::Entity(entity) => format!(
                "entity:{}:{}:{}",
                entity.partition_id.0, entity.local_slot.0, entity.generation.0
            ),
            RecordRef::Relation(relation) => format!(
                "relation:{}:{}:{}",
                relation.partition_id.0, relation.local_slot.0, relation.generation.0
            ),
        }
    }

    fn aspect_key_label(aspect: InternedString) -> String {
        match aspect {
            InternedString::Raw(value) => value,
            InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
        }
    }

    fn structural_change_label(record: &PatchRecord) -> &'static str {
        match record.structural_change {
            crate::publication::patch::data::RecordStructuralChange::Created => "created",
            crate::publication::patch::data::RecordStructuralChange::Updated => "updated",
            crate::publication::patch::data::RecordStructuralChange::Deleted => "deleted",
            crate::publication::patch::data::RecordStructuralChange::RetainedForAudit => {
                "retained_for_audit"
            }
        }
    }

    #[derive(Debug, Clone)]
    struct PublicationSnapshotReader {
        snapshot: PublicationBridgeSnapshot,
    }

    impl TruthSnapshotReader for PublicationSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            self.snapshot.identity.clone()
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
        {
            let records_by_key = self
                .snapshot
                .records
                .iter()
                .map(|record| (record.request_key().to_string(), record.clone()))
                .collect::<BTreeMap<_, _>>();
            let records = request
                .reads()
                .iter()
                .filter_map(|read| records_by_key.get(read.request_key()).cloned())
                .collect::<Vec<_>>();
            Ok(SnapshotReadPacketResult::new(
                self.snapshot.read_result_identity.clone(),
                records,
            ))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::facade::history::CommitId;
        use crate::facade::identity::{EntityId, PartitionId};
        use crate::facade::publication::{
            AspectKey, CanonicalAspectSet, PatchOrdering, PatchPublicationMode, PatchRecord,
            PatchRecordKind, PatchStreamPosition, RecordStructuralChange, RelationalPatchRecord,
        };
        use crate::publication::patch::data::{PatchCompatibilityClass, PatchDetail};
        use forge_runtime_bridge::facade::RelationalCommittedPatchRequest;

        #[test]
        fn publication_bridge_catalog_exposes_committed_patch_and_snapshot() {
            let catalog = PublicationBridgeCatalog::default();
            catalog.register_patch(
                CommitId(7),
                "main",
                "snapshot-a",
                &RelationalPatchRecord {
                    ordering: PatchOrdering::CanonicalCommitOrder,
                    publication_mode: PatchPublicationMode::CommitNative,
                    position: PatchStreamPosition(11),
                    compatibility: PatchCompatibilityClass::StructuredCompatible,
                    records: vec![PatchRecord {
                        kind: PatchRecordKind::Updated,
                        target: crate::transactions::data::RecordRef::Entity(EntityId::new(
                            PartitionId::main(),
                            4,
                            1,
                        )),
                        structural_change: RecordStructuralChange::Updated,
                        aspects: CanonicalAspectSet::new([AspectKey("profile.name".into())]),
                        contains_degraded_precision: false,
                        detail: PatchDetail::StructuredJson(serde_json::json!({"after": "alice"})),
                    }],
                },
            );
            catalog.register_snapshot(PublicationBridgeSnapshot::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![SnapshotReadRecord::new("entity:0:4:1:profile.name", b"alice".to_vec())],
            ));

            let envelope = catalog
                .load_committed_patch(RelationalCommittedPatchRequest::new("commit-7"))
                .expect("registered publication patch");
            let reader = catalog
                .open_snapshot(&TruthSnapshotIdentity::new("snapshot-a"))
                .expect("registered publication snapshot");

            assert_eq!(envelope.patch_identity().as_str(), "patch-11");
            assert_eq!(envelope.patch_items()[0].aspect_label(), "profile.name");
            assert_eq!(reader.snapshot_identity().as_str(), "snapshot-a");
        }
    }
}

pub mod diagnostics {
    pub use crate::diagnostics::data::{
        DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsDeliveryClass,
        DiagnosticsScope, RelationalArtifactPolicy, RelationalDiagnosticArtifact,
        RelationalDiagnosticsEntry, RelationalDiagnosticsProfile,
    };
    pub use crate::diagnostics::facade::RelationalDiagnosticsFacade;
}

pub mod durability {
    pub use crate::durability::data::{
        CheckpointCoverage, CompactionOutcome, CompactionPlan, CompactionPolicy, DurabilityError,
        DurabilityMode, DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest,
        DurableIntegrityStatus, DurableSegmentId, DurableSegmentManifest, DurableStore,
        DurableStoreLayout, PartitionCheckpointImage, RecoveryAuthorityParity,
        RecoveryCompatibilityCheck, RecoveryCompatibilityMismatch, RecoveryCoverage,
        RecoveryCursor, RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan,
        RecoveryVerificationMode, RecoveryVerificationOutcome, RecoveryVerificationPlan,
        RelationIntegrityContractFamily, SegmentRetentionClass,
    };
}

pub mod errors {
    pub use crate::errors::data::{
        ErrorContext, ErrorOperation, RelationalError, RelationalSubsystem, SuggestedFix,
    };
}

pub mod history {
    pub use crate::history::data::{
        AspectFilter, AspectFilterMode, AspectHistoryCommitSpan, AspectHistoryDigest,
        AspectHistoryEntry, AspectHistoryLineageEventSpan, AspectHistoryOrigin,
        AspectHistoryQueryResult, AspectHistoryResolutionTrace, AspectResolutionContext,
        BranchCreateError, BranchCreateErrorClass, BranchHead, BranchId, CommitId, CommitReference,
        HistoryAspectQueryTarget, HistoryDriftClass, HistoryRetentionClass,
        HistoryShapeClassification, LineageAspectHistory, LineageAspectHistoryQueryResult,
        LineageAspectResolutionDigest, MergeConflictRecord, MergeInspection, OrderedParentList,
        RequestedAspectSet, VersionGraphPolicy, VersionGraphSnapshot, VersionNode,
    };
    pub use crate::history::logic::{HistoryAccess, HistoryAuthority};
}

pub mod identity {
    pub use crate::identity::data::{
        EntityId, EntityStorageId, Generation, KindId, LineageId, LocalSlot, PartitionId,
        RelationId, RelationStorageId, StructuralFingerprint, VersionBound, VersionId,
    };
}

pub mod inspection {
    pub use crate::inspection::data::{
        CommitInspection, ConnectivityComponentSummary, ConnectivityInspectionBudget,
        ConnectivityInspectionRequest, ConnectivityInspectionSummary, GraphInspectionBudget,
        GraphInspectionRequest, GraphInspectionSummary, HistoricalAspectObservation,
        HistoricalAvailabilityObservation, HistoricalInspectionMode, HistoricalOpenResult,
        HistoricalRecordInspection, HistoricalRecordObservation, HistoricalRecordValue,
        HistoricalSnapshotView, InspectionAccessPath, InspectionAvailability,
        InspectionDegradation, InspectionOrigin, InspectionRecordClass,
        InspectionResolutionContext, InspectionScope, KindInspectionRequest, KindInspectionSummary,
        NeighborInspectionResult, PinStateObservation, RecentCommitInspectionRequest,
        RecentCommitInspectionWindow, ReclaimEligibility, RecordRetentionInspection,
        RetentionExecutionInspection, RetentionInspectionRequest, RetentionInspectionSummary,
        RetentionStateObservation, SavepointInspectionSurface, SnapshotPinInspection,
        StructuralIdentityComparison, StructuralIdentityComparisonVerdict,
        StructuralIdentityEvidence, StructuralIdentityQueryRequest, TransactionInspectionSurface,
        TransactionIntentCounts,
    };
    pub use crate::inspection::logic::InspectionAccess;
}

pub mod indexes {
    pub use crate::indexes::data::{
        DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
        DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
        DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
    };
}

pub mod lineage {
    pub use crate::lineage::data::{
        CorrespondenceCandidate, CorrespondenceCandidateId,
        CorrespondencePromotionExecutionFailureClass, CorrespondencePromotionOutcome,
        CorrespondencePromotionRejectionClass, CorrespondenceResolution,
        HistoricalLineageResolution, HistoricalLineageResolutionDigestBasis,
        HistoricalLineageResolutionMetrics, HistoricalResolutionBoundednessBasis,
        HistoricalResolutionDigestMode, HistoricalResolutionRequest, HistoricalResolutionTrace,
        LineageArtifactCounters, LineageCheckpointArtifact, LineageCheckpointCounters,
        LineageCheckpointDigestBasis, LineageDecisionKind, LineageDecisionLogDigestBasis,
        LineageDigestBasis, LineageDivergenceMetrics, LineageDivergenceRequest,
        LineageDivergenceSummary, LineageDivergenceTraversalBasis, LineageEventBatchDigestBasis,
        LineageEventKind, LineageEventRecord, LineageGraphDigestBasis, LineageGraphDigestMode,
        LineageGraphMetrics, LineageGraphRequest, LineageGraphSnapshot, LineageGraphTraversalBasis,
        LineageInvariant, LineageNode, LineageResolutionStatus, RecordHistoryRequest,
    };
}

pub mod merge {
    pub use crate::merge::data::{
        AspectMergePolicyDeclaration, AspectMergePolicyKind, BranchCausalDot, BranchDeltaSummary,
        CausalAnnotationSummary, CausalFrontier, CommitCausalMetadata, CommitCausalRelation,
        ConflictClassificationSummary, CustomIdentityBasisIdentity, CustomMergePolicyIdentity,
        DeletionExecutionClass, DeletionMergeClass, EndpointContinuityClass,
        ExecutedMergeAspectClass, ExecutedMergeAspectDiagnosticRow, ExecutedMergeRecordClass,
        ExecutedMergeRecordDiagnosticRow, IdentityBasisDeclaration, IdentityBasisKind,
        IdentityBasisScope, IdentityDiscoverySummary, IdentityMatchCandidate, IdentityMatchClass,
        IdentityResolutionReason, LoweredAspectAction, LoweredAspectOutcome, LoweredMergeAction,
        LoweredMergeBlockedReason, LoweredMergePlanRecord, LoweredMergePlanSummary,
        LoweredMergeRejectedReason, LoweredRecordDecision, LoweredRecordDecisionKind,
        LoweredRecordDenialKind, MergeAncestrySummary, MergeArtifactDigestBasis,
        MergeBaseSelectionRule, MergeCausalEvidenceModel, MergeConflictClass,
        MergeConflictClassification, MergeExecutableClass, MergeExecutionAuthorityContract,
        MergeExecutionAuthorizationRule, MergeExecutionCompilationError,
        MergeExecutionDecisionSurface, MergeExecutionDeniedRecord, MergeExecutionDiagnosticsPlan,
        MergeExecutionError, MergeExecutionFreshnessPolicy, MergeExecutionMutationPlanError,
        MergeExecutionPreparationError, MergeExecutionReadiness, MergeExecutionReadinessReport,
        MergeExecutionRequest, MergeIntent, MergeManualResolutionClass, MergePlanningArtifactCore,
        MergePlanningDecisionKind, MergePlanningDecisionLog, MergePlanningDecisionLogDigestBasis,
        MergePlanningDecisionRecord, MergePlanningError, MergePlanningRequest,
        MergePlanningSummary, MergePolicyDecisionBoundary, MergePolicyOwnershipClass,
        MergePolicyOwnershipSurface, MergePolicyProofBoundary, MergePolicyRejectClass,
        MergePolicyResolution, MergeRecordCausalAnnotation, MergeRecordCausalDisposition,
        MergeRecordIdentity, MergeResolutionClass, MergeResolvedAspectValueStrategy,
        MergeSchemaKindClass, MergeSchemaKindSemanticSnapshot, MergeSchemaSnapshotDigestBasis,
        MergeVisibilityEvidence, MergeVisibilityEvidenceKind, MergeVisibilityState,
        PreparedMergeExecution, RelationConflictPropagation, RelationContinuityClass,
        ResolvedMergeBase, SchemaDeclaredCorrespondenceValidationSummary, TopologyExecutionClass,
        TopologyRegionConflictReason, TopologyRewireAdmissionPolicy,
    };
    pub use crate::merge::logic::MergeAccess;
    pub use crate::transactions::data::MergeExecutionOutcome;
}

pub mod runtime {
    pub use crate::logic::builder::RelationalRuntimeBuilder;
    pub use crate::logic::commit::CommitAuthorityContract;
    pub use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
    #[cfg(test)]
    pub use crate::logic::runtime::HarnessAuditMode;
    pub use crate::logic::runtime::{
        ChunkVisibilitySummary, ChunkedStorageSummary, CompiledArtifactCompatibility,
        CompiledArtifactError, CompiledExecutionArtifact, ComplexityContract, ComplexityStatus,
        EntityReadRecord, EntityRecordProjection, InvariantAccess, InvariantCatalog,
        InvariantCheckResult, InvariantClass, InvariantDecisionKind, InvariantDecisionRecord,
        InvariantExecutionPoint, InvariantFailureEffect, InvariantRegistration, InvariantRule,
        PartitionStorageStats, RelationReadRecord, RelationRecordProjection, RelationalReadView,
        RelationalReplayRecord, RelationalRuntime, RelationalRuntimeConfig, ReplaySchemaVersion,
        RuntimeComplexityCounters, SimulationAccess, SimulationAuthority, SnapshotGuard,
        StorageStats, TopologyFreezeMode, VisibilityProjectionView, VisibilityReadContext,
        VisibilityRetentionAuthority,
    };
    pub use crate::presentation::api::RelationalRuntimeApi;
    pub use crate::presentation::contracts::{
        ImmutableReadContract, RelationalBoundaryContract, SerializedAuthorityContract,
    };
    pub use crate::visibility::authority::VisibilityAuthority as SnapshotAuthority;
}

pub mod payloads {
    pub use crate::payloads::data::{
        PayloadClass, PayloadCompatibility, PayloadEncoding, PayloadPolicy, RecordPayload,
    };
}

#[cfg(test)]
pub mod harness {
    pub use crate::presentation::harness::{
        default_harness_expectations, FixtureEntity, FixtureRelation, RelationalFixture,
        RelationalHarnessAdapter, RelationalHarnessError, RelationalHarnessExpectations,
        RelationalHarnessPlan,
    };
}

pub mod publication {
    pub use crate::publication::bundle::{PublicationBundle, PublicationStage, PublicationStatus};
    pub use crate::publication::cdc::data::{
        SubscriberCheckpoint, SubscriberRecoveryDecision, SubscriberRecoveryDisposition,
        SubscriberRecoverySource, SubscriberResumeRequest, SubscriberStreamBatch,
        SubscriberStreamFailure, SubscriberStreamFailureClass,
    };
    pub use crate::publication::data::PublicationError;
    pub use crate::publication::patch::data::{
        AspectKey, CanonicalAspectSet, PatchFragmentBudget, PatchOrdering, PatchPublicationMode,
        PatchRecord, PatchRecordKind, PatchStreamBatch, PatchStreamPosition, PatchStreamReadError,
        PatchStreamReadErrorClass, PatchStreamRequest, RecordStructuralChange,
        RelationalPatchRecord,
    };
}

pub mod query {
    pub use crate::query::data::{
        CanonicalQueryResult, DeterministicQueryFragmentKey, DeterministicQueryPlanKey,
        FallbackParityMode, FallbackParityVerifiedQueryOutcome, IndexQueryRejectionClass,
        PartitionHint, PlannedQueryPacket, QueryAccessPath, QueryComplexitySummary,
        QueryExecutionOutcome, QueryExecutionShape, QueryFallbackContract, QueryFragmentCounters,
        QueryLocalityClass, QueryOrderingContract, QueryParallelLegality,
        QueryParallelProfitability, QueryPlanContextId, QueryPlanEvidenceBasis, QueryScope,
        QuerySerialReason, QueryWorkerFragment, ReductionDiscipline, SnapshotPinnedQueryPlan,
    };
}

pub mod replay {
    pub use crate::replay::data::{
        CanonicalCommitAuthorityKind, CanonicalCommitEnvelope,
        CertifiedLineageSurfaceComparisonBasis, CertifiedLineageSurfaceDigest,
        LineageCertifiedSurfaceKind, RelationalReplayOutcome, RelationalReplayRequest,
        ReplayAuthorityBasisKind, ReplayError, ReplayExecutionMode, ReplayFailureClass,
        ReplayLineageAuthorityBasis, ReplayLineageDigestMode, ReplayMismatch, ReplayMismatchClass,
        ReplayObservableSurface, ReplaySnapshotSurface, ReplayVerificationLayer,
        ReplayVerificationMode, ReplayVerificationPlan,
    };
}

pub mod schema {
    pub use crate::publication::patch::data::AspectKey;
    pub use crate::schema::data::{
        AcyclicityContractDeclaration, AllowedCycleClass, AspectBinding, AspectComparator,
        AspectDeclarationTrace, AspectDeclarationTraceRow, AspectLoweringTrace,
        AspectLoweringTraceRow, AspectPlanRevision, AspectPrecision,
        CardinalityContractDeclaration, CompatibilityObservation,
        ConnectivityMinimumContractDeclaration, ConnectivityMinimumEnforcement, ContractId,
        DeclaredAspect, DescriptorCanonicalizationCompatibilityPolicy,
        DescriptorCanonicalizationVersion, DescriptorSemanticsCompatibilityPolicy,
        DescriptorSemanticsVersion, DirectedTraversalKind, EndpointDeletionIntegrityDeclaration,
        EndpointDeletionIntegrityMode, EndpointKindContractDeclaration, EntityKindRegistration,
        FreeFormSchemaDiffIntent, HistoricalInterpretationSensitivity, KindAspectDeclarations,
        KindResolution, LoweredAcyclicityContract, LoweredAspectBinding, LoweredAspectComparator,
        LoweredAspectExtractor, LoweredAspectPlan, LoweredCardinalityMaximumContract,
        LoweredCardinalityMinimumContract, LoweredConnectivityMinimumContract,
        LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
        LoweredExecutableAspectBindingKind, LoweredPartitionIsolationContract,
        LoweredPayloadSchemaContract, LoweredRelationIntegrityPlan, LoweredSchemaTransitionPlan,
        LoweredSymmetryContract, LoweredUniquenessContract, MinimumCardinalityEnforcement,
        PairMinimumSemantics, PartitionIsolationContractDeclaration, PartitionIsolationMode,
        PayloadContractRecordKind, PayloadFieldConstraint, PayloadFieldConstraintDeclaration,
        PayloadSchemaDeclaration, PayloadSchemaValueType, ProposedSchemaTransition,
        RelationIntegrityDeclarations, RelationIntegrityPlanCatalog, RelationIntegrityPlanRevision,
        RelationKindRegistration, RelationPayloadClass, RelationalSchemaRegistry,
        SchemaBoundaryFingerprint, SchemaBridgeDescriptor, SchemaBridgeabilityClassification,
        SchemaContinuationClassification, SchemaContinuationDescriptor, SchemaDiffAtom,
        SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaLineageArtifact,
        SchemaLineageOrderingSemantics, SchemaPublicationImpact,
        SchemaReconciliationClassification, SchemaReconciliationDescriptor,
        SchemaReconciliationOrderingMode, SchemaReconciliationPolicy, SchemaRegistryError,
        SchemaRegistryErrorClass, SchemaStratum, SchemaSubscriberImpact, SchemaTransitionArtifact,
        SchemaTransitionBarrier, SchemaTransitionSummary, SchemaVersionId,
        SubscriberBoundaryVisibility, SymmetryContractDeclaration, SymmetryMode,
        UniquenessContractDeclaration, UniquenessScope, ValidatedSchemaTransition,
    };
}

pub mod snapshots {
    pub use crate::snapshots::data::{
        SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
    };
}

pub mod storage {
    pub use crate::storage::data::RecordLifecycleState;
}

pub mod symbols {
    pub use crate::symbols::data::{
        InternedString, StringInterner, Symbol, SymbolPolicy, SymbolTableSnapshot,
    };
}

pub mod transactions {
    pub use crate::transactions::data::{
        AspectEmissionTrace, AspectEvaluationTrace, AspectEvaluationTraceRow,
        AspectLifecycleTransitionClass, AspectTagAccuracyReport, AspectTraceEvidence,
        AuthoritativeApplyPlan, AuthorityMode, BulkEntityCreateIntent, BulkMutationLineagePlan,
        BulkMutationLocalityFootprint, BulkMutationNamingPlan, BulkMutationProvenancePlan,
        BulkMutationScope, BulkRelationCreateIntent, CommitAspectSummary, CommitAuthority,
        CommitChangeSummary, CommitConflict, CommitHistorySummary, CommitLog, CommitOutcome,
        CommitPatchBudgetSummary, CommitPhase, CommitPhaseTiming, CommitPublicationSummary,
        CommitResult, CommitSchemaSummary, CommitStructuralSummary, CommitSummary, CommitTopology,
        CommitTraceEvent, ConflictClass, CreateIntent, CrossContextEndpointClass,
        DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntitySpec,
        LineageSafeBulkMutationBatch, MergeCommitMutationPlan, MergeExecutionOutcome,
        MergeExecutionStructuralSummary, MergeExecutionSummary, MergedCommitPlan, MutationIntent,
        NamingStableBulkMutationBatch, PatchVsTruthDeltaReport, PlannedBulkMutationBatch,
        PlannedLineageTransition, ProvenanceCompleteBulkMutationBatch, RecordRef,
        RelationMutationIntent, RelationScope, RelationSpec, ReplaceEntityIntent, RollbackEffect,
        RollbackOutcome, RollbackSummary, SavepointId, TransactionCommitError, TransactionId,
        TransactionOptions, UndoRecord, UpdateEntityIntent, WorkerIntentBatch,
    };
    pub use crate::transactions::logic::RelationalTransaction;
}
