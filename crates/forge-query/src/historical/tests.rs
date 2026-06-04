#[cfg(test)]
mod tests {
    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeDeliveryIntent,
        BridgeDeliveryReceipt, BridgeDiagnosticsTier, BridgeHistoricalMaterializationPath,
        BridgeReplayMode, BridgeRuntimePolicy, BridgeSignalInvalidationDelivery,
        BridgeSnapshotReadError, BridgeSourceAdapter, BridgeSourceCapability,
        BridgeSourceCapabilitySet, BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector,
        CoarseRoutingMode, HistoricalEvaluationDeclaration, InvalidationSink,
        RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
        RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
        SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, SourceDeclaration,
        SourceDeclarationIdentity, TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity,
        TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
    };

    use super::super::bridge_lowering::{
        lower_materialization_from_artifact, lower_materialization_from_decision_log,
        lower_policy_resolution,
    };
    use super::super::contracts::{
        HistoricalPathComplexityContract, HistoricalPathReuseDescriptor,
    };
    use super::super::cost::{HistoricalPathCostPosture, PerformancePredictionDriftOutcome};
    use super::super::planner::{
        admit_historical_evaluation_path, materialization_metadata_from_resolved,
        resolve_historical_materialization_path,
    };
    use super::super::report::HistoricalPathVocabularyReport;
    use super::super::request::{HistoricalEvaluationRequest, HistoricalMaterializationDescriptor};
    use super::super::{
        AdmittedHistoricalPathClass, HistoricalPathCompatibilityOutcome,
        RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
    };

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
                forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                    TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
                    TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
                    TruthSnapshotIdentity::new("snapshot-a"),
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
        ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
            Ok(BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    #[test]
    fn closed_enum_names_are_stable() {
        assert_eq!(
            RequestedHistoricalPathClass::RequestedRetainedSnapshotPath.as_str(),
            "requested_retained_snapshot_path"
        );
        assert_eq!(
            AdmittedHistoricalPathClass::AdmittedDeltaReplayPath.as_str(),
            "admitted_delta_replay_path"
        );
        assert_eq!(
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath.as_str(),
            "resolved_full_reconstruction_path"
        );
        assert_eq!(
            HistoricalPathCostPosture::HistoricalReplayBounded.as_str(),
            "historical_replay_bounded"
        );
        assert_eq!(
            PerformancePredictionDriftOutcome::HistoricalReplaySpanDrift.as_str(),
            "historical_replay_span_drift"
        );
    }

    #[test]
    fn requested_admitted_and_resolved_path_classes_are_distinct() {
        let requested = RequestedHistoricalPathClass::RequestedRetainedSnapshotPath;
        let admitted = AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath;
        let resolved = ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath;

        assert_ne!(requested.as_str(), admitted.as_str());
        assert_ne!(admitted.as_str(), resolved.as_str());
        assert_ne!(requested.as_str(), resolved.as_str());
    }

    #[test]
    fn complexity_contract_names_are_deterministic() {
        assert_eq!(
            HistoricalPathComplexityContract::retained_path().contract_name(),
            "historical_retained_path"
        );
        assert_eq!(
            HistoricalPathComplexityContract::replay_path().contract_name(),
            "historical_replay_path"
        );
        assert_eq!(
            HistoricalPathComplexityContract::reconstruction_path().contract_name(),
            "historical_reconstruction_path"
        );
    }

    #[test]
    fn vocabulary_report_preserves_requested_family_and_posture() {
        let request = HistoricalEvaluationRequest::full_reconstruction(
            "basis:reconstruction",
            5,
            9,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let report = HistoricalPathVocabularyReport::from_request(
            &request,
            HistoricalPathComplexityContract::reconstruction_path(),
            super::super::counters::HistoricalCounterSnapshot::vocabulary_baseline(),
        );

        assert_eq!(
            report.requested_path_class_name(),
            "requested_full_reconstruction_path"
        );
        assert_eq!(
            report.cost_posture().as_str(),
            "historical_reconstruction_expensive"
        );
    }

    #[test]
    fn retained_snapshot_request_admits_and_resolves_retained_path() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Disabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let request = HistoricalEvaluationRequest::retained_snapshot(
            declaration.declaration_identity().as_str(),
            1,
            1,
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let capability = lower_policy_resolution(
            &declaration,
            &runtime.resolve_truth_view_policy(&declaration),
            None,
            request.requested_path_class(),
        )
        .expect("snapshot policy should lower");

        let admission = admit_historical_evaluation_path(request, capability)
            .expect("retained path should admit");

        assert_eq!(
            admission.compatibility_outcome(),
            &HistoricalPathCompatibilityOutcome::Admitted
        );
        assert_eq!(
            admission.admitted_path().admitted_path_class(),
            &AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath
        );
        assert_eq!(
            admission.cost_posture().as_str(),
            "historical_retained_fast_path"
        );
        assert_eq!(
            admission.complexity_contract().contract_name(),
            "historical_retained_path"
        );

        let evaluation = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_branch_snapshot(
                    TruthBranchIdentity::new("analysis"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                )
                .with_replay_mode(BridgeReplayMode::Disabled),
            )
            .expect("snapshot evaluation should succeed");
        let lowered = lower_materialization_from_artifact(
            &runtime.lower_historical_evaluation_artifact(evaluation.observation()),
            &RequestedHistoricalPathClass::RequestedRetainedSnapshotPath,
        )
        .expect("materialized artifact should lower");

        let resolved = resolve_historical_materialization_path(admission, lowered)
            .expect("retained path should resolve");
        let metadata = materialization_metadata_from_resolved(resolved.clone());

        assert_eq!(
            resolved.resolved_path_class(),
            &ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath
        );
        assert_eq!(
            metadata.requested_path_class(),
            &RequestedHistoricalPathClass::RequestedRetainedSnapshotPath
        );
        assert_eq!(
            metadata.admitted_path_class(),
            &AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath
        );
        assert_eq!(
            metadata.resolved_path_class(),
            &ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath
        );
        assert_eq!(
            resolved
                .counters()
                .history_work_avoided_by_retained_path_count(),
            0
        );
    }

    #[test]
    fn replay_request_admits_and_resolves_replay_path() {
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
        .expect("historical commit policy should lower");

        let admission = admit_historical_evaluation_path(request, capability)
            .expect("replay path should admit");
        assert_eq!(
            admission.admitted_path().admitted_path_class(),
            &AdmittedHistoricalPathClass::AdmittedDeltaReplayPath
        );
        assert_eq!(
            admission.cost_posture().as_str(),
            "historical_replay_bounded"
        );

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
            .expect("decision log should lower");
        let resolved = resolve_historical_materialization_path(admission, lowered)
            .expect("replay path should resolve");

        assert_eq!(
            resolved.resolved_path_class(),
            &ResolvedHistoricalPathClass::ResolvedDeltaReplayPath
        );
        assert_eq!(
            resolved.complexity_contract().contract_name(),
            "historical_replay_path"
        );
    }

    #[test]
    fn reconstruction_request_admits_full_reconstruction_path() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let request = HistoricalEvaluationRequest::full_reconstruction(
            declaration.declaration_identity().as_str(),
            3,
            10,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let capability = lower_policy_resolution(
            &declaration,
            &runtime.resolve_truth_view_policy(&declaration),
            None,
            request.requested_path_class(),
        )
        .expect("branch-head policy should lower");

        let admission = admit_historical_evaluation_path(request, capability)
            .expect("full reconstruction path should admit");

        assert_eq!(
            admission.admitted_path().admitted_path_class(),
            &AdmittedHistoricalPathClass::AdmittedFullReconstructionPath
        );
        assert_eq!(
            admission.complexity_contract().contract_name(),
            "historical_reconstruction_path"
        );
    }

    #[test]
    fn replay_request_is_denied_when_replay_mode_is_not_admitted() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeReplayMode::Disabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let request = HistoricalEvaluationRequest::delta_replay(
            declaration.declaration_identity().as_str(),
            2,
            2,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let capability = lower_policy_resolution(
            &declaration,
            &runtime.resolve_truth_view_policy(&declaration),
            None,
            request.requested_path_class(),
        )
        .expect("snapshot policy should lower");

        let error = admit_historical_evaluation_path(request, capability)
            .expect_err("replay should be denied when replay mode is disabled");

        assert_eq!(
            error.failure_class(),
            super::super::error::HistoricalEvaluationFailureClass::ReplayNotPermitted
        );
    }

    #[test]
    fn admitted_path_class_must_match_requested_lane_proof() {
        let request = HistoricalEvaluationRequest::delta_replay(
            "basis:mismatch",
            2,
            2,
            HistoricalPathReuseDescriptor::no_reuse(),
        );
        let capability = super::super::request::HistoricalCapabilityDescriptor::new(
            "basis:mismatch",
            Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath),
            true,
            false,
            true,
            false,
            HistoricalPathReuseDescriptor::no_reuse(),
        );

        let error = admit_historical_evaluation_path(request, capability)
            .expect_err("mismatched admitted proof should fail");

        assert_eq!(
            error.failure_class(),
            super::super::error::HistoricalEvaluationFailureClass::UnsupportedHistoricalPathRequest
        );
    }

    #[test]
    fn hidden_path_substitution_is_denied() {
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
        .expect("historical commit policy should lower");
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("replay path should admit");

        let wrong_path = HistoricalMaterializationDescriptor::new(
            declaration.declaration_identity().as_str(),
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath,
        );

        let error = resolve_historical_materialization_path(admission, wrong_path)
            .expect_err("hidden substitution should be denied");

        assert_eq!(
            error.failure_class(),
            super::super::error::HistoricalEvaluationFailureClass::HiddenPathSubstitutionDenied
        );
    }

    #[test]
    fn resolved_historical_counters_preserve_admission_lane_and_metadata() {
        let request = HistoricalEvaluationRequest::delta_replay(
            "basis:counts",
            4,
            8,
            HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
        );
        let capability = super::super::request::HistoricalCapabilityDescriptor::new(
            "basis:counts",
            Some(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath),
            true,
            false,
            false,
            true,
            HistoricalPathReuseDescriptor::with_replay_tail_reuse(),
        );
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("admission should succeed");
        let resolved = resolve_historical_materialization_path(
            admission,
            HistoricalMaterializationDescriptor::new(
                "basis:counts",
                ResolvedHistoricalPathClass::ResolvedDeltaReplayPath,
            ),
        )
        .expect("resolution should succeed");

        assert_eq!(resolved.counters().historical_admitted_path_count(), 1);
        assert_eq!(resolved.counters().historical_resolved_path_count(), 1);
        assert_eq!(
            resolved.counters().historical_result_path_metadata_count(),
            1
        );
        assert_eq!(
            resolved
                .counters()
                .historical_delta_replay_admission_count(),
            1
        );
        assert_eq!(
            resolved
                .counters()
                .history_work_avoided_by_retained_path_count(),
            0
        );
    }

    #[test]
    fn retained_reuse_counter_only_increments_when_capability_proves_reuse() {
        let request = HistoricalEvaluationRequest::retained_snapshot(
            "basis:retained-reuse",
            3,
            5,
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let capability = super::super::request::HistoricalCapabilityDescriptor::retained_snapshot(
            "basis:retained-reuse",
            HistoricalPathReuseDescriptor::retained_reuse(),
        );
        let admission = admit_historical_evaluation_path(request, capability)
            .expect("admission should succeed");
        let resolved = resolve_historical_materialization_path(
            admission,
            HistoricalMaterializationDescriptor::retained_snapshot("basis:retained-reuse"),
        )
        .expect("resolution should succeed");

        assert_eq!(
            resolved
                .counters()
                .history_work_avoided_by_retained_path_count(),
            8
        );
    }

    #[test]
    fn bridge_lowering_preserves_decision_log_path_semantics() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let evaluation = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_historical_commit(
                    TruthBranchIdentity::new("analysis"),
                    TruthCommitIdentity::new("commit-a"),
                )
                .with_replay_mode(BridgeReplayMode::Required),
            )
            .expect("historical evaluation should succeed");
        assert_eq!(
            evaluation.record().decision_log().materialization_path(),
            BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot
        );
        let lowered = lower_materialization_from_decision_log(evaluation.record().decision_log())
            .expect("decision log should lower");

        assert_eq!(
            lowered.resolved_path_class(),
            &ResolvedHistoricalPathClass::ResolvedDeltaReplayPath
        );
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
                        forge_runtime_bridge::facade::AspectKeySelector::exact(
                            forge_foundational::facade::AspectKey::new("profile")
                                .expect("valid native mapping aspect key"),
                        ),
                        forge_runtime_bridge::facade::TruthPatchTargetSelector::entity_field(
                            forge_foundational::facade::FieldKey::new("name".to_owned())
                                .expect("valid native mapping field key"),
                        ),
                    ),
                    forge_runtime_bridge::facade::SnapshotReadContract::scalar(
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid native snapshot aspect key"),
                        forge_foundational::facade::ScalarAspectType::String,
                    ),
                    SignalInvalidationScope::new("signal:profile"),
                    CoarseRoutingMode::Direct,
                ),
            )
            .build()
            .expect("runtime should build for historical lowering tests")
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
}
