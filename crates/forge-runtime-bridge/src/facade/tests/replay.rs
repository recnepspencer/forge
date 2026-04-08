    #[test]
    fn runtime_replays_canonical_historical_evaluation_record() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let observation = runtime
            .materialize_truth_view_observation(
                runtime
                    .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                    .expect("branch-head declaration should plan"),
            )
            .expect("branch-head declaration should materialize");
        let record = runtime.canonicalize_historical_evaluation_record(&observation);

        let replay = runtime
            .replay_canonical_historical_evaluation_record(&record)
            .expect("historical record replay should succeed");

        assert_eq!(replay.record_identity(), record.record_identity());
        assert_eq!(replay.snapshot_identity().as_str(), "snapshot-a");
    }

    #[test]
    fn runtime_replay_rejects_historical_authority_drift() {
        #[derive(Clone)]
        struct DriftSource;

        impl crate::adapter::CommittedPatchSource for DriftSource {
            fn load_committed_patch(
                &self,
                request: crate::adapter::RelationalCommittedPatchRequest,
            ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
                let snapshot = if request.commit_identity() == "commit-a" {
                    "snapshot-b"
                } else {
                    "snapshot-a"
                };
                Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                    crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                    crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                    TruthSnapshotIdentity::new(snapshot),
                    TruthBranchIdentity::new("analysis"),
                    vec![],
                ))
            }
        }

        impl crate::adapter::SnapshotReadSource for DriftSource {
            fn open_snapshot(
                &self,
                identity: &TruthSnapshotIdentity,
            ) -> Result<Box<dyn crate::snapshot::TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
                if identity.as_str() == "snapshot-b" {
                    Ok(Box::new(StaticSnapshotReader))
                } else {
                    Err(crate::adapter::RelationalBridgeSourceError::new("missing snapshot"))
                }
            }
        }

        impl crate::adapter::TruthBranchHeadSource for DriftSource {
            fn load_branch_head_patch(
                &self,
                branch_identity: &TruthBranchIdentity,
            ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
                Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                    crate::input::envelope::TruthCommitIdentity::new("head-analysis"),
                    crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                    TruthSnapshotIdentity::new("snapshot-b"),
                    branch_identity.clone(),
                    vec![],
                ))
            }
        }

        let original = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let record = original.canonicalize_historical_evaluation_record(
            &original
                .materialize_truth_view_observation(
                    original
                        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                        .expect("original historical declaration should plan"),
                )
                .expect("original historical declaration should materialize"),
        );
        let drifted = RuntimeBridgeBuilder::new()
            .with_policy(BridgeRuntimePolicy::default())
            .with_relational_source(DriftSource)
            .with_truth_branch_head_source(DriftSource)
            .with_signal_sink(StaticSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("mapping"),
                TruthPatchScope::new(
                    MappingSelector::exact("profile"),
                    MappingSelector::any(),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new("signal:profile"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("drifted runtime should build");

        let error = drifted
            .replay_canonical_historical_evaluation_record(&record)
            .expect_err("historical replay should reject authority drift");

        assert_eq!(
            error.kind(),
            crate::error::BridgeReplayErrorKind::HistoricalEvaluationAuthorityMismatch
        );
        assert_eq!(
            drifted
                .diagnostics()
                .last_historical_evaluation_failure()
                .expect("historical replay mismatch should be recorded")
                .failure_class(),
            crate::facade::BridgeHistoricalEvaluationFailureClass::HistoricalReplayMismatch
        );
    }

    #[test]
    fn runtime_replay_rejects_incompatible_historical_record_version() {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let declaration = HistoricalEvaluationDeclaration::new(
            BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
            BridgeReplayMode::Enabled,
            BridgeDiagnosticsTier::Standard,
            BridgeDeliveryIntent::PrepareSignalEvaluation,
        );
        let record = runtime
            .canonicalize_historical_evaluation_record(
                &runtime
                    .materialize_truth_view_observation(
                        runtime
                            .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                            .expect("branch-head declaration should plan"),
                    )
                    .expect("branch-head declaration should materialize"),
            )
            .with_schema_version_for_test("forge-runtime-bridge.historical-evaluation-record.v0");

        let error = runtime
            .replay_canonical_historical_evaluation_record(&record)
            .expect_err("historical replay should reject unsupported schema versions");

        assert_eq!(
            error.kind(),
            crate::error::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
        );
    }
use super::*;
