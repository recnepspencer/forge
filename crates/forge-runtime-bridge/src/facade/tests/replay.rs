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
fn runtime_replay_rejects_historical_authority_drift_as_authority_mismatch() {
    #[derive(Clone)]
    struct DriftSource;

    impl crate::adapter::CommittedPatchSource for DriftSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<
            crate::input::envelope::BridgeCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let snapshot = if request.commit_identity().as_str() == "commit-a" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            crate::input::envelope::BridgeCommittedPatchEnvelope::new(
                crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                    request.commit_identity().clone(),
                    crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                    TruthSnapshotIdentity::new(snapshot),
                    TruthBranchIdentity::new("analysis"),
                ),
                vec![
                    crate::input::envelope::BridgeCommittedPatchItem::with_target(
                        "entity-1",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                ],
            )
            .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
        }
    }

    impl crate::adapter::SnapshotReadSource for DriftSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<
            Box<dyn crate::snapshot::TruthSnapshotReader>,
            crate::adapter::RelationalBridgeSourceError,
        > {
            if identity.as_str() == "snapshot-b" {
                Ok(Box::new(StaticSnapshotReader))
            } else {
                Err(crate::adapter::RelationalBridgeSourceError::new(
                    "missing snapshot",
                ))
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for DriftSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<
            crate::input::envelope::BridgeCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            crate::input::envelope::BridgeCommittedPatchEnvelope::new(
                crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                    crate::input::envelope::TruthCommitIdentity::new("head-analysis"),
                    crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                    TruthSnapshotIdentity::new("snapshot-b"),
                    branch_identity.clone(),
                ),
                vec![
                    crate::input::envelope::BridgeCommittedPatchItem::with_target(
                        "entity-1",
                        crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                            forge_foundational::facade::AspectLocator::new(
                                forge_foundational::facade::LocatorAuthority::Authoritative,
                                forge_foundational::facade::AspectKey::new("profile")
                                    .expect("valid bridge patch aspect key"),
                            ),
                            forge_foundational::facade::CanonicalFieldPath::single(
                                forge_foundational::facade::FieldKey::new("name".to_owned())
                                    .expect("valid foundational field key"),
                            ),
                        ),
                    ),
                ],
            )
            .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
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
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(DriftSource)
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
                crate::facade::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ],
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("entity-1"),
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
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

use super::*;
