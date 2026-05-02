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

pub mod grouped_truth {
    pub use crate::grouped_truth::{
        materialize_relational_authoritative_row_set, project_relational_grouped_truth,
        GroupedProjectionContract, RelationalAuthoritativeRowArtifact,
        RelationalAuthoritativeRowSetArtifact, RelationalFieldBindingKey, RelationalFieldValue,
        RelationalGroupedMemberRow, RelationalGroupedProjectionArtifact,
        RelationalGroupedProjectionDigest, RelationalGroupedTruthError, RelationalGroupingValue,
        RelationalRowIdentity, RelationalRowSetDigest,
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
    use std::sync::Arc;

    #[cfg(test)]
    use std::collections::BTreeMap;
    #[cfg(test)]
    use std::sync::RwLock;

    use crate::capabilities::{CommitEnvelopeSource, SnapshotSource};
    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchItem, BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest,
        BridgeLineageSourceError, BridgeLineageSourceErrorKind, CommittedPatchSource,
        ContinuityLineageSource, RawCommittedPatchEnvelope, RelationalBridgeSourceError,
        RelationalCommittedPatchRequest, SnapshotReadPacket, SnapshotReadPacketResult,
        SnapshotReadRecord, SnapshotReadSource, TruthBranchHeadSource, TruthBranchIdentity,
        TruthCommitIdentity, TruthPatchIdentity, TruthSnapshotIdentity, TruthSnapshotReader,
    };

    use crate::history::data::CommitId;
    use crate::identity::data::{EntityId, PartitionId, RelationId, VersionId};
    use crate::lineage::data::HistoricalResolutionBoundednessBasis;
    use crate::logic::runtime::RelationalRuntime;
    use crate::publication::patch::data::{PatchRecord, RelationalPatchRecord};
    use crate::snapshots::data::{SnapshotHandle, SnapshotId};
    use crate::symbols::data::InternedString;
    use crate::transactions::data::RecordRef;

    #[derive(Debug, Clone)]
    pub struct RuntimeBridgeRelationalSource {
        runtime: Arc<RelationalRuntime>,
    }

    impl RuntimeBridgeRelationalSource {
        pub fn new(runtime: Arc<RelationalRuntime>) -> Self {
            Self { runtime }
        }
    }

    #[cfg(test)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct PublicationBridgeSnapshot {
        identity: TruthSnapshotIdentity,
        read_result_identity: TruthSnapshotIdentity,
        records: Vec<SnapshotReadRecord>,
    }

    #[cfg(test)]
    impl PublicationBridgeSnapshot {
        pub fn new(identity: TruthSnapshotIdentity, records: Vec<SnapshotReadRecord>) -> Self {
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

    #[cfg(test)]
    #[derive(Debug, Clone, Default)]
    pub struct PublicationBridgeCatalog {
        state: Arc<RwLock<PublicationBridgeCatalogState>>,
    }

    #[cfg(test)]
    #[derive(Debug, Default)]
    struct PublicationBridgeCatalogState {
        committed_patches: BTreeMap<String, RawCommittedPatchEnvelope>,
        snapshots: BTreeMap<String, PublicationBridgeSnapshot>,
    }

    #[cfg(test)]
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

    #[cfg(test)]
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

    impl CommittedPatchSource for RuntimeBridgeRelationalSource {
        fn load_committed_patch(
            &self,
            request: RelationalCommittedPatchRequest,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            let commit_id = parse_bridge_commit_identity(request.commit_identity())?;
            let envelope = self.runtime.commit_envelope(commit_id).ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "relational runtime has no authoritative commit envelope for bridge commit `{}`",
                    request.commit_identity()
                ))
            })?;

            Ok(commit_envelope_to_bridge_envelope(envelope))
        }
    }

    #[cfg(test)]
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

    #[cfg(test)]
    impl TruthBranchHeadSource for PublicationBridgeCatalog {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            self.state
                .read()
                .expect("publication bridge catalog lock poisoned")
                .committed_patches
                .values()
                .filter(|envelope| envelope.branch_identity() == branch_identity)
                .cloned()
                .last()
                .ok_or_else(|| {
                    RelationalBridgeSourceError::new(format!(
                        "no publication bridge branch head registered for `{}`",
                        branch_identity.as_str()
                    ))
                })
        }
    }

    impl SnapshotReadSource for RuntimeBridgeRelationalSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
            let version_id = resolve_bridge_snapshot_version(&self.runtime, identity)?;

            Ok(Box::new(RuntimePublicationSnapshotReader::new(
                Arc::clone(&self.runtime),
                identity.clone(),
                version_id,
            )))
        }
    }

    impl TruthBranchHeadSource for RuntimeBridgeRelationalSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            let branch_id = crate::history::data::BranchId(branch_identity.as_str().to_string());
            let history = self.runtime.history();
            let head = history.branch_head(&branch_id).ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "relational runtime has no branch head for `{}`",
                    branch_identity.as_str()
                ))
            })?;
            let envelope = self.runtime.commit_envelope(head.commit_id).ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "relational runtime has no authoritative commit envelope for branch head `{}` on `{}`",
                    head.commit_id.0,
                    branch_identity.as_str()
                ))
            })?;

            Ok(commit_envelope_to_bridge_envelope(envelope))
        }
    }

    impl ContinuityLineageSource for RuntimeBridgeRelationalSource {
        fn historical_lineage(
            &self,
            request: BridgeHistoricalLineageRequest,
        ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
            let branch_id = crate::history::data::BranchId(
                request
                    .authority_basis()
                    .branch_identity()
                    .as_str()
                    .to_string(),
            );
            let record = parse_bridge_record_identity(request.prior_slice().entity_identity())
                .map_err(|error| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        error.to_string(),
                    )
                })?;
            let RecordRef::Entity(entity_id) = record else {
                return Err(BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
                    "bridge continuity lineage adapter currently supports entity record identities only",
                ));
            };
            let resolution = self
                .runtime
                .lineage_access()
                .resolve_record_history(crate::lineage::data::RecordHistoryRequest {
                    branch_id,
                    entity_id,
                    boundedness_basis:
                        HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                })
                .ok_or_else(|| {
                    BridgeLineageSourceError::new(
                        BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                        format!(
                        "bridge continuity lineage adapter could not resolve record history for `{}`",
                        request.prior_slice().entity_identity()
                    ),
                    )
                })?;
            let mut canonical_resolved_lineage_keys = resolution
                .resolved
                .iter()
                .map(|lineage| Arc::<str>::from(format!("lineage:{}", lineage.0)))
                .collect::<Vec<_>>();
            canonical_resolved_lineage_keys.sort_unstable();
            canonical_resolved_lineage_keys.dedup();

            let snapshot_version_id = resolve_bridge_snapshot_version(
                &self.runtime,
                request.authority_basis().snapshot_identity(),
            )
            .map_err(|error| {
                BridgeLineageSourceError::new(
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                    error.to_string(),
                )
            })?;
            let projection = self
                .runtime
                .read_truth()
                .project_version(snapshot_version_id);
            let mut canonical_resolved_record_keys = resolution
                .resolved
                .iter()
                .filter_map(|lineage_id| self.runtime.lineage.nodes.get(lineage_id))
                .filter_map(|node| {
                    projection.entity_record(node.entity_id()).map(|_| {
                        Arc::<str>::from(record_ref_identity(&RecordRef::Entity(node.entity_id())))
                    })
                })
                .collect::<Vec<_>>();
            canonical_resolved_record_keys.sort_unstable();
            canonical_resolved_record_keys.dedup();

            let mut traversed_event_ids = resolution.traversed_event_ids.clone();
            traversed_event_ids.sort_unstable();
            traversed_event_ids.dedup();

            BridgeHistoricalLineageAuthority::try_new(
                request.authority_basis().clone(),
                canonical_resolved_lineage_keys,
                canonical_resolved_record_keys,
                traversed_event_ids,
            )
        }
    }

    #[derive(Debug, Clone)]
    struct RuntimePublicationSnapshotReader {
        runtime: Arc<RelationalRuntime>,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    }

    impl RuntimePublicationSnapshotReader {
        fn new(
            runtime: Arc<RelationalRuntime>,
            snapshot_identity: TruthSnapshotIdentity,
            version_id: VersionId,
        ) -> Self {
            Self {
                runtime,
                snapshot_identity,
                version_id,
            }
        }
    }

    impl TruthSnapshotReader for RuntimePublicationSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            self.snapshot_identity.clone()
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
        {
            let projection = self.runtime.read_truth().project_version(self.version_id);
            let mut records = Vec::with_capacity(request.reads().len());
            for read in request.reads() {
                let record_ref =
                    parse_bridge_record_identity(read.entity_identity()).map_err(|error| {
                        forge_runtime_bridge::facade::BridgeSnapshotReadError::new(
                            error.to_string(),
                        )
                    })?;
                let payload = match record_ref {
                    RecordRef::Entity(entity_id) => {
                        let record = projection.entity_record(entity_id).ok_or_else(|| {
                            forge_runtime_bridge::facade::BridgeSnapshotReadError::new(format!(
                                "relational bridge snapshot reader could not find entity `{}` in authoritative snapshot `{}`",
                                read.entity_identity(),
                                self.snapshot_identity.as_str()
                            ))
                        })?;
                        payload_bytes_for_entity_aspect(&record, read.aspect_label()).ok_or_else(|| {
                            forge_runtime_bridge::facade::BridgeSnapshotReadError::new(format!(
                                "relational bridge snapshot reader could not resolve aspect `{}` on entity `{}` in authoritative snapshot `{}`",
                                read.aspect_label(),
                                read.entity_identity(),
                                self.snapshot_identity.as_str()
                            ))
                        })?
                    }
                    RecordRef::Relation(relation_id) => {
                        let record = projection.relation_record(relation_id).ok_or_else(|| {
                            forge_runtime_bridge::facade::BridgeSnapshotReadError::new(format!(
                                "relational bridge snapshot reader could not find relation `{}` in authoritative snapshot `{}`",
                                read.entity_identity(),
                                self.snapshot_identity.as_str()
                            ))
                        })?;
                        payload_bytes_for_relation_aspect(&record, read.aspect_label()).ok_or_else(|| {
                            forge_runtime_bridge::facade::BridgeSnapshotReadError::new(format!(
                                "relational bridge snapshot reader could not resolve aspect `{}` on relation `{}` in authoritative snapshot `{}`",
                                read.aspect_label(),
                                read.entity_identity(),
                                self.snapshot_identity.as_str()
                            ))
                        })?
                    }
                };
                records.push(SnapshotReadRecord::new(read.request_key(), payload));
            }

            Ok(SnapshotReadPacketResult::new(
                self.snapshot_identity.clone(),
                records,
            ))
        }
    }

    const RELATIONAL_BRIDGE_SNAPSHOT_PREFIX: &str = "relational-snapshot";
    const RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT: &str = "version";

    pub fn bridge_snapshot_identity_for_handle(handle: &SnapshotHandle) -> TruthSnapshotIdentity {
        bridge_snapshot_identity_for_binding(handle.snapshot_id, handle.version_id)
    }

    pub fn bridge_snapshot_identity_for_commit(
        commit_id: CommitId,
        version_id: VersionId,
    ) -> TruthSnapshotIdentity {
        bridge_snapshot_identity_for_binding(SnapshotId(commit_id.0), version_id)
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

    pub fn publication_bundle_to_bridge_envelope(
        bundle: &crate::publication::bundle::PublicationBundle<
            crate::logic::runtime::RelationalReplayRecord,
        >,
    ) -> RawCommittedPatchEnvelope {
        publication_patch_to_bridge_envelope(
            bundle.commit.commit_id,
            bundle.commit.branch_id.0.clone(),
            bridge_snapshot_identity_for_handle(&bundle.snapshot)
                .as_str()
                .to_string(),
            &bundle.patch,
        )
    }

    pub fn commit_envelope_to_bridge_envelope(
        envelope: &crate::replay::data::CanonicalCommitEnvelope,
    ) -> RawCommittedPatchEnvelope {
        publication_patch_to_bridge_envelope(
            envelope.commit.commit_id,
            envelope.commit.branch_id.0.clone(),
            bridge_snapshot_identity_for_commit(
                envelope.commit.commit_id,
                envelope.commit.version_id,
            )
            .as_str()
            .to_string(),
            &envelope.patch,
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

    fn parse_bridge_record_identity(
        identity: &str,
    ) -> Result<RecordRef, RelationalBridgeSourceError> {
        let mut parts = identity.split(':');
        let kind = parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge record kind"))?;
        let partition_id = PartitionId(
            parts
                .next()
                .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge partition id"))?
                .parse::<u32>()
                .map_err(|_| RelationalBridgeSourceError::new("invalid bridge partition id"))?,
        );
        let local_slot = parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge local slot"))?
            .parse::<u64>()
            .map_err(|_| RelationalBridgeSourceError::new("invalid bridge local slot"))?;
        let generation = parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge generation"))?
            .parse::<u32>()
            .map_err(|_| RelationalBridgeSourceError::new("invalid bridge generation"))?;
        if parts.next().is_some() {
            return Err(RelationalBridgeSourceError::new(
                "bridge record identity had too many fields",
            ));
        }

        Ok(match kind {
            "entity" => RecordRef::Entity(EntityId::new(partition_id, local_slot, generation)),
            "relation" => {
                RecordRef::Relation(RelationId::new(partition_id, local_slot, generation))
            }
            _ => {
                return Err(RelationalBridgeSourceError::new(format!(
                    "unsupported bridge record kind `{kind}`"
                )))
            }
        })
    }

    fn bridge_snapshot_identity_for_binding(
        snapshot_id: SnapshotId,
        version_id: VersionId,
    ) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new(format!(
            "{RELATIONAL_BRIDGE_SNAPSHOT_PREFIX}:{}:{RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT}:{}",
            snapshot_id.0, version_id.0
        ))
    }

    fn parse_bridge_snapshot_identity(
        identity: &TruthSnapshotIdentity,
    ) -> Result<(SnapshotId, VersionId), RelationalBridgeSourceError> {
        let mut parts = identity.as_str().split(':');
        let prefix = parts
            .next()
            .ok_or_else(|| RelationalBridgeSourceError::new("missing bridge snapshot prefix"))?;
        if prefix != RELATIONAL_BRIDGE_SNAPSHOT_PREFIX {
            return Err(RelationalBridgeSourceError::new(format!(
                "unsupported relational bridge snapshot identity `{}`",
                identity.as_str()
            )));
        }

        let snapshot_id = SnapshotId(
            parts
                .next()
                .ok_or_else(|| RelationalBridgeSourceError::new("missing relational snapshot id"))?
                .parse::<u64>()
                .map_err(|_| RelationalBridgeSourceError::new("invalid relational snapshot id"))?,
        );
        let version_segment = parts.next().ok_or_else(|| {
            RelationalBridgeSourceError::new("missing relational version segment")
        })?;
        if version_segment != RELATIONAL_BRIDGE_SNAPSHOT_VERSION_SEGMENT {
            return Err(RelationalBridgeSourceError::new(format!(
                "unsupported relational bridge snapshot version segment `{version_segment}`"
            )));
        }
        let version_id = VersionId(
            parts
                .next()
                .ok_or_else(|| RelationalBridgeSourceError::new("missing relational version id"))?
                .parse::<u64>()
                .map_err(|_| RelationalBridgeSourceError::new("invalid relational version id"))?,
        );
        if parts.next().is_some() {
            return Err(RelationalBridgeSourceError::new(
                "relational bridge snapshot identity had too many fields",
            ));
        }

        Ok((snapshot_id, version_id))
    }

    fn parse_bridge_commit_identity(
        identity: &str,
    ) -> Result<CommitId, RelationalBridgeSourceError> {
        let raw = identity.strip_prefix("commit-").ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "unsupported relational bridge commit identity `{identity}`"
            ))
        })?;
        let commit_id = raw.parse::<u64>().map_err(|_| {
            RelationalBridgeSourceError::new(format!(
                "invalid relational bridge commit identity `{identity}`"
            ))
        })?;
        Ok(CommitId(commit_id))
    }

    fn resolve_bridge_snapshot_version(
        runtime: &RelationalRuntime,
        identity: &TruthSnapshotIdentity,
    ) -> Result<VersionId, RelationalBridgeSourceError> {
        let (snapshot_id, expected_version_id) = parse_bridge_snapshot_identity(identity)?;
        let commit_id = CommitId(snapshot_id.0);
        let observed_version_id = runtime
            .commit_envelope(commit_id)
            .map(|envelope| envelope.commit.version_id)
            .or_else(|| {
                runtime
                    .active_snapshot_binding(snapshot_id)
                    .map(|(version_id, _)| version_id)
            })
            .or_else(|| runtime.published_snapshot_version(snapshot_id))
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "relational bridge snapshot identity `{}` does not resolve to an authoritative commit envelope or active/published snapshot binding",
                    identity.as_str()
                ))
            })?;
        if observed_version_id != expected_version_id {
            return Err(RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot identity `{}` expected version `{}` but authoritative binding resolved to version `{}`",
                identity.as_str(),
                expected_version_id.0,
                observed_version_id.0
            )));
        }

        Ok(observed_version_id)
    }

    fn payload_bytes_for_entity_aspect(
        record: &crate::storage::data::EntityReadRecord,
        aspect_label: &str,
    ) -> Option<Vec<u8>> {
        if aspect_label == "lifecycle" {
            return serde_json::to_vec(&record.lifecycle).ok();
        }
        payload_bytes_for_payload(record.payload.as_json()?, aspect_label)
    }

    fn payload_bytes_for_relation_aspect(
        record: &crate::storage::data::RelationReadRecord,
        aspect_label: &str,
    ) -> Option<Vec<u8>> {
        match aspect_label {
            "source" => Some(record_ref_identity(&RecordRef::Entity(record.source)).into_bytes()),
            "target" => Some(record_ref_identity(&RecordRef::Entity(record.target)).into_bytes()),
            "lifecycle" => serde_json::to_vec(&record.lifecycle).ok(),
            _ => payload_bytes_for_payload(record.payload.as_ref()?.as_json()?, aspect_label),
        }
    }

    fn payload_bytes_for_payload(value: &serde_json::Value, aspect_label: &str) -> Option<Vec<u8>> {
        let scoped = json_value_for_aspect(value, aspect_label)
            .or_else(|| json_value_for_terminal_field(value, aspect_label))
            .or_else(|| structural_snapshot_value(value, aspect_label))?;
        Some(match scoped {
            serde_json::Value::String(text) => text.as_bytes().to_vec(),
            other => serde_json::to_vec(other).ok()?,
        })
    }

    fn json_value_for_aspect<'a>(
        value: &'a serde_json::Value,
        aspect_label: &str,
    ) -> Option<&'a serde_json::Value> {
        let mut current = value;
        for segment in aspect_label.split('.') {
            current = current.get(segment)?;
        }
        Some(current)
    }

    fn json_value_for_terminal_field<'a>(
        value: &'a serde_json::Value,
        aspect_label: &str,
    ) -> Option<&'a serde_json::Value> {
        let terminal = aspect_label.rsplit('.').next()?;
        value.get(terminal)
    }

    fn structural_snapshot_value<'a>(
        value: &'a serde_json::Value,
        aspect_label: &str,
    ) -> Option<&'a serde_json::Value> {
        match aspect_label {
            "created" | "updated" | "deleted" | "retained_for_audit" => Some(value),
            _ => None,
        }
    }

    #[cfg(test)]
    #[derive(Debug, Clone)]
    struct PublicationSnapshotReader {
        snapshot: PublicationBridgeSnapshot,
    }

    #[cfg(test)]
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
        use crate::facade::transactions::{
            EntityMutationIntent, MutationIntent, ReplaceEntityIntent, TransactionOptions,
            WorkerIntentBatch,
        };
        use crate::publication::patch::data::{PatchCompatibilityClass, PatchDetail};
        use crate::tests::support::{
            changed_entities, create_entity_outcome, runtime_with_test_schema,
        };
        use forge_runtime_bridge::facade::{
            BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeDeliveryReceipt,
            BridgeMappingId, BridgeMappingRegistration, BridgeRouteRequest, CoarseRoutingMode,
            InvalidationSink, MappingSelector, RelationalCommittedPatchRequest,
            RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
            SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
        };

        struct TestSink;

        impl InvalidationSink for TestSink {
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

        fn exact_registration(
            mapping_id: &str,
            entity_identity: &str,
            aspect_label: &str,
            surface_label: &str,
        ) -> BridgeMappingRegistration {
            BridgeMappingRegistration::new(
                BridgeMappingId::new(mapping_id),
                TruthPatchScope::new(
                    MappingSelector::exact(entity_identity),
                    MappingSelector::exact(aspect_label),
                    MappingSelector::exact(surface_label),
                ),
                SignalInvalidationScope::new("signal.user.profile"),
                CoarseRoutingMode::Direct,
            )
        }

        fn exact_aspect_registration(
            registration_id: &str,
            entity_identity: &str,
            aspect_label: &str,
            surface_label: &str,
        ) -> BridgeAspectRegistration {
            BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::new(registration_id),
                TruthPatchScope::new(
                    MappingSelector::exact(entity_identity),
                    MappingSelector::exact(aspect_label),
                    MappingSelector::exact(surface_label),
                ),
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            )
        }

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
                vec![SnapshotReadRecord::new(
                    "entity:0:4:1:profile.name",
                    b"alice".to_vec(),
                )],
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

        #[test]
        fn runtime_bridge_lineage_source_resolves_real_relational_history() {
            let mut runtime = runtime_with_test_schema();
            let created = create_entity_outcome(&mut runtime, "source");
            let entity = changed_entities(&created)[0];

            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            txn.push_batch(
                WorkerIntentBatch::new("replace").push(MutationIntent::Entity(
                    EntityMutationIntent::Replace(ReplaceEntityIntent {
                        entity_id: entity,
                        replacement: crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: crate::facade::identity::KindId(1),
                            client_key: InternedString::Raw("replacement".to_string()),
                            payload: crate::payloads::data::RecordPayload::StructuredJson(
                                serde_json::json!({"name":"replacement"}),
                            ),
                        },
                    }),
                )),
            );
            txn.commit().expect("replace should commit");
            let latest_bundle = runtime
                .publication()
                .latest_bundle()
                .expect("runtime publication bundle")
                .clone();
            let expected_snapshot_identity = bridge_snapshot_identity_for_commit(
                latest_bundle.commit.commit_id,
                latest_bundle.commit.version_id,
            );
            let expected_successor_record_keys = runtime
                .read_truth()
                .project_version(latest_bundle.commit.version_id)
                .all_entity_records()
                .into_iter()
                .filter_map(|record| {
                    record.lineage_id.map(|_| {
                        Arc::<str>::from(record_ref_identity(&RecordRef::Entity(record.entity_id)))
                    })
                })
                .collect::<Vec<_>>();

            let runtime = Arc::new(runtime);
            let source = RuntimeBridgeRelationalSource::new(Arc::clone(&runtime));
            let request = BridgeHistoricalLineageRequest::new(
                forge_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    expected_snapshot_identity.clone(),
                ),
                forge_runtime_bridge::facade::PriorSubscriptionSlice::from_parts(
                    forge_runtime_bridge::facade::BridgeSubscriptionSliceIdentity::new("slice:a"),
                    format!(
                        "entity:{}:{}:{}",
                        entity.partition_id.0, entity.local_slot.0, entity.generation.0
                    ),
                    "profile.name",
                    "name",
                    forge_runtime_bridge::facade::SubscriptionSliceKind::SignalField,
                    forge_runtime_bridge::facade::FineGrainedMatchStatus::Matched,
                ),
            );
            let authority = source
                .historical_lineage(request)
                .expect("runtime lineage source should resolve");

            assert_eq!(authority.branch_identity().as_str(), "main");
            assert_eq!(authority.snapshot_identity(), &expected_snapshot_identity);
            assert_eq!(authority.canonical_resolved_lineage_keys().len(), 1);
            assert_eq!(
                authority.canonical_resolved_record_keys(),
                expected_successor_record_keys.as_slice()
            );
            assert_eq!(authority.traversed_event_ids().len(), 1);
        }

        #[test]
        fn runtime_bridge_relational_source_exposes_latest_publication_bundle_authoritatively() {
            let mut runtime = runtime_with_test_schema();
            create_entity_outcome(&mut runtime, "alice");

            let bundle = runtime
                .publication()
                .latest_bundle()
                .expect("runtime publication bundle")
                .clone();
            let expected_snapshot_identity = bridge_snapshot_identity_for_commit(
                bundle.commit.commit_id,
                bundle.commit.version_id,
            );
            let expected_commit_identity = RelationalCommittedPatchRequest::new(format!(
                "commit-{}",
                bundle.commit.commit_id.0
            ));

            let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
            let envelope = source
                .load_committed_patch(expected_commit_identity)
                .expect("runtime bridge committed patch");
            let reader = source
                .open_snapshot(&expected_snapshot_identity)
                .expect("runtime bridge snapshot reader");

            assert_eq!(envelope.snapshot_identity(), &expected_snapshot_identity);
            assert_eq!(reader.snapshot_identity(), expected_snapshot_identity);
            assert_eq!(envelope.patch_items().len(), 1);
        }

        #[test]
        fn runtime_bridge_relational_source_drives_public_bridge_delivery_with_canonical_snapshot_authority(
        ) {
            let mut runtime = runtime_with_test_schema();
            create_entity_outcome(&mut runtime, "alice");

            let bundle = runtime
                .publication()
                .latest_bundle()
                .expect("runtime publication bundle")
                .clone();
            let commit_identity = format!("commit-{}", bundle.commit.commit_id.0);
            let expected_snapshot_identity = bridge_snapshot_identity_for_commit(
                bundle.commit.commit_id,
                bundle.commit.version_id,
            );

            let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
            let envelope = source
                .load_committed_patch(RelationalCommittedPatchRequest::new(
                    commit_identity.clone(),
                ))
                .expect("runtime bridge committed patch");
            let first_patch_item = envelope
                .patch_items()
                .first()
                .expect("runtime bridge envelope should contain at least one patch item");
            let mut builder = RuntimeBridgeBuilder::new()
                .with_relational_source(source.clone())
                .with_signal_sink(TestSink)
                .with_continuity_lineage_source(source.clone())
                .register_mapping(exact_registration(
                    "runtime-publication-item-0",
                    first_patch_item.entity_identity(),
                    first_patch_item.aspect_label(),
                    first_patch_item.surface_label(),
                ))
                .register_aspect_mapping(exact_aspect_registration(
                    "runtime-publication-item-field-0",
                    first_patch_item.entity_identity(),
                    first_patch_item.aspect_label(),
                    first_patch_item.surface_label(),
                ));
            for (index, patch_item) in envelope.patch_items().iter().enumerate().skip(1) {
                builder = builder
                    .register_mapping(exact_registration(
                        &format!("runtime-publication-item-{index}"),
                        patch_item.entity_identity(),
                        patch_item.aspect_label(),
                        patch_item.surface_label(),
                    ))
                    .register_aspect_mapping(exact_aspect_registration(
                        &format!("runtime-publication-item-field-{index}"),
                        patch_item.entity_identity(),
                        patch_item.aspect_label(),
                        patch_item.surface_label(),
                    ));
            }
            let bridge = builder
                .build()
                .expect("runtime bridge should build from runtime-backed relational source");

            let route = bridge
                .plan_committed_patch(BridgeRouteRequest::for_commit(commit_identity))
                .expect("runtime-backed relational bridge route");
            let result = bridge
                .deliver_invalidation(route)
                .expect("runtime-backed relational bridge delivery");

            assert_eq!(
                result.result_summary().snapshot_identity(),
                &expected_snapshot_identity
            );
            assert_eq!(
                result.receipt().snapshot_identity(),
                &expected_snapshot_identity
            );
        }

        #[test]
        fn runtime_bridge_replays_historical_commit_after_newer_publication_arrives() {
            let mut runtime = runtime_with_test_schema();
            create_entity_outcome(&mut runtime, "alice");
            let historical_commit_id = runtime
                .publication()
                .latest_bundle()
                .expect("first runtime publication bundle")
                .commit
                .commit_id;

            let mut txn = runtime.begin_transaction(TransactionOptions::default());
            txn.push_batch(
                WorkerIntentBatch::new("update").push(MutationIntent::Create(
                    crate::transactions::data::CreateIntent::Entity(
                        crate::transactions::data::EntitySpec {
                            partition_id: PartitionId::main(),
                            kind_id: crate::facade::identity::KindId(1),
                            client_key: InternedString::Raw("bob".to_string()),
                            payload: crate::payloads::data::RecordPayload::StructuredJson(
                                serde_json::json!({"name":"bob"}),
                            ),
                        },
                    ),
                )),
            );
            txn.commit().expect("second commit should publish");

            let source = RuntimeBridgeRelationalSource::new(Arc::new(runtime));
            let historical_commit_identity =
                RelationalCommittedPatchRequest::new(format!("commit-{}", historical_commit_id.0));
            let envelope = source
                .load_committed_patch(historical_commit_identity.clone())
                .expect("historical bridge committed patch");
            let expected_snapshot_identity = envelope.snapshot_identity().clone();
            let first_patch_item = envelope
                .patch_items()
                .first()
                .expect("historical bridge envelope should contain at least one patch item");
            let bridge = RuntimeBridgeBuilder::new()
                .with_relational_source(source.clone())
                .with_signal_sink(TestSink)
                .with_continuity_lineage_source(source.clone())
                .register_mapping(exact_registration(
                    "historical-publication-item-0",
                    first_patch_item.entity_identity(),
                    first_patch_item.aspect_label(),
                    first_patch_item.surface_label(),
                ))
                .register_aspect_mapping(exact_aspect_registration(
                    "historical-publication-item-field-0",
                    first_patch_item.entity_identity(),
                    first_patch_item.aspect_label(),
                    first_patch_item.surface_label(),
                ))
                .build()
                .expect("runtime bridge should build from runtime-backed relational source");

            let planned = bridge
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    historical_commit_identity.commit_identity().to_string(),
                ))
                .expect("historical route should still plan after newer publication");
            let result = bridge
                .deliver_invalidation(planned)
                .expect("historical route should still deliver after newer publication");
            let canonical = bridge
                .diagnostics()
                .last_canonical_route_record()
                .expect("historical route record");
            let replay = bridge
                .replay_canonical_record(&canonical)
                .expect("historical replay should remain reconstructable");

            assert_eq!(
                result.result_summary().snapshot_identity(),
                &expected_snapshot_identity
            );
            assert_eq!(
                result.receipt().snapshot_identity(),
                &expected_snapshot_identity
            );
            assert_eq!(replay.source_snapshot(), &expected_snapshot_identity);
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
        LoweredRecordDenialBundle, LoweredRecordDenialKind, LoweredRecordExecutionBundle,
        LoweredRecordExecutionIntentKind, MergeAncestrySummary, MergeArtifactDigestBasis,
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
        RelationalMergeInspectionAdmission, RelationalMergeInspectionArtifact,
        RelationalMergeInspectionInput, RelationalMergeInspectionRow, ResolvedMergeBase,
        SchemaDeclaredCorrespondenceValidationSummary, TopologyExecutionClass,
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
    pub use crate::validation::data::{
        CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
        CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
        CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
        CustomInvariantRuleId, CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCostClass, InvariantGroup,
        InvariantGroupSet, PlannedRelationEndpointUpdate, StructuralRelationRecord,
        StructuralRelationView,
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
        CommitTraceEvent, ConflictClass, CreateIntent, CreatedEntityRef, CrossContextEndpointClass,
        DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
        EntitySpec, LineageSafeBulkMutationBatch, MergeCommitMutationPlan, MergeExecutionOutcome,
        MergeExecutionStructuralSummary, MergeExecutionSummary, MergedCommitPlan, MutationIntent,
        NamingStableBulkMutationBatch, PatchVsTruthDeltaReport, PlannedBulkMutationBatch,
        PlannedLineageTransition, ProvenanceCompleteBulkMutationBatch, RecordRef,
        RelationMutationIntent, RelationScope, RelationSpec, ReplaceEntityIntent, RollbackEffect,
        RollbackOutcome, RollbackSummary, SavepointId, TransactionCommitError, TransactionId,
        TransactionOptions, UndoRecord, UpdateEntityIntent, UpdateRelationEndpointsIntent,
        WorkerIntentBatch,
    };
    pub use crate::transactions::logic::RelationalTransaction;
}
