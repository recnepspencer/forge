#[test]
fn runtime_admits_snapshot_bound_truth_view_policy() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );

    let resolution = runtime.resolve_truth_view_policy(&declaration);
    match resolution {
        BridgeTruthViewPolicyResolution::Admitted(policy) => {
            assert_eq!(
                policy.retention_admission(),
                crate::snapshot::TruthViewRetentionAdmission::SnapshotResident
            );
            assert_eq!(
                policy.source_capability(),
                crate::snapshot::TruthViewSourceCapability::DirectSnapshotRead
            );
        }
        BridgeTruthViewPolicyResolution::Rejected(rejection) => {
            panic!(
                "expected admitted policy, got rejection: {}",
                rejection.detail()
            )
        }
    }
}

#[test]
fn runtime_rejects_required_replay_when_runtime_policy_disallows_replay_artifacts() {
    let runtime = runtime(
        BridgeRuntimePolicy::operational()
            .with_route_record_limit(8)
            .with_replay_artifacts(false),
    );
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("main"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Required,
        BridgeDiagnosticsTier::Exhaustive,
        BridgeDeliveryIntent::PrepareOnly,
    );

    let resolution = runtime.resolve_truth_view_policy(&declaration);
    match resolution {
        BridgeTruthViewPolicyResolution::Rejected(rejection) => {
            assert_eq!(
                rejection.kind(),
                crate::snapshot::TruthViewPolicyRejectionKind::ReplayNotPermitted
            );
        }
        BridgeTruthViewPolicyResolution::Admitted(_) => {
            panic!("expected replay policy rejection")
        }
    }
}

#[test]
fn runtime_plans_truth_view_packet_from_admitted_policy() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );

    let planned = runtime
        .plan_truth_view_packet(declaration.clone(), SnapshotReadPacket::new(vec![]))
        .expect("snapshot-bound declaration should plan");

    assert_eq!(
        planned.declaration().declaration_identity(),
        declaration.declaration_identity()
    );
    assert_eq!(
        planned
            .authority_basis()
            .snapshot_identity()
            .map(|id: &TruthSnapshotIdentity| id.as_str()),
        Some("snapshot-a")
    );
}

#[test]
fn runtime_admits_commit_bound_truth_view_policy() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );

    let resolution = runtime.resolve_truth_view_policy(&declaration);
    match resolution {
        BridgeTruthViewPolicyResolution::Admitted(policy) => {
            assert_eq!(
                policy.retention_admission(),
                crate::snapshot::TruthViewRetentionAdmission::HistoricalLookupRequired
            );
        }
        BridgeTruthViewPolicyResolution::Rejected(rejection) => {
            panic!(
                "expected commit-bound selector admission, got rejection: {}",
                rejection.detail()
            )
        }
    }
}

#[test]
fn runtime_materializes_commit_bound_truth_view_observation() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let planned = runtime
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .expect("commit-bound declaration should plan");

    let observation = runtime
        .materialize_truth_view_observation(planned)
        .expect("commit-bound declaration should materialize");

    assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(
        observation
            .authority_basis()
            .commit_identity()
            .map(crate::input::envelope::TruthCommitIdentity::as_str),
        Some("commit-a")
    );
}

#[test]
fn runtime_materializes_branch_head_truth_view_observation() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("analysis")),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let planned = runtime
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .expect("branch-head declaration should plan");

    let observation = runtime
        .materialize_truth_view_observation(planned)
        .expect("branch-head declaration should materialize");

    assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(
        observation
            .authority_basis()
            .commit_identity()
            .map(crate::input::envelope::TruthCommitIdentity::as_str),
        Some("head-analysis")
    );
}

#[test]
fn runtime_materializes_snapshot_bound_truth_view_observation() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let planned = runtime
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .expect("snapshot-bound declaration should plan");

    let observation = runtime
        .materialize_truth_view_observation(planned)
        .expect("snapshot-bound declaration should materialize");
    let validated_reads = observation
        .read_planned_packet()
        .expect("materialized observation should execute its planned packet");

    assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(
        observation.snapshot_token().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(validated_reads.snapshot_identity().as_str(), "snapshot-a");
}

#[test]
fn runtime_canonicalizes_historical_evaluation_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let planned = runtime
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .expect("historical declaration should plan");
    let observation = runtime
        .materialize_truth_view_observation(planned)
        .expect("historical declaration should materialize");

    let record = runtime.canonicalize_historical_evaluation_record(&observation);

    assert_eq!(
        record.decision_log().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(
        record.decision_log().materialization_path(),
        BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot
    );
    assert_eq!(
        runtime
            .diagnostics()
            .last_historical_evaluation_record()
            .expect("historical record should be retained")
            .record_identity(),
        record.record_identity()
    );
}

#[test]
fn runtime_lowers_identical_historical_requests_to_identical_artifacts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let left_observation = runtime
        .materialize_truth_view_observation(
            runtime
                .plan_truth_view_packet(declaration.clone(), SnapshotReadPacket::new(vec![]))
                .expect("left historical declaration should plan"),
        )
        .expect("left historical declaration should materialize");
    let right_observation = runtime
        .materialize_truth_view_observation(
            runtime
                .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                .expect("right historical declaration should plan"),
        )
        .expect("right historical declaration should materialize");

    let left = runtime.lower_historical_evaluation_artifact(&left_observation);
    let right = runtime.lower_historical_evaluation_artifact(&right_observation);

    assert_eq!(left, right);
    assert_eq!(left.snapshot_identity().as_str(), "snapshot-a");
}

#[test]
fn runtime_admits_registered_source_declaration() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_source(
        "source:analysis-snapshot",
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ],
    );

    let contract = runtime
        .admit_source(declaration)
        .expect("registered source declaration should be admitted");

    assert_eq!(
        contract.declaration().declaration_identity().as_str(),
        "source:analysis-snapshot"
    );
}

#[test]
fn runtime_validates_registered_source_declaration() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_source(
        "source:analysis-history",
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayCompatibleRead,
        ],
    );

    let validated = runtime
        .validate_source_declaration(declaration.clone())
        .expect("registered source declaration should validate");
    let admitted = runtime
        .admit_source(declaration.clone())
        .expect("registered source declaration should admit");

    assert_eq!(validated.declaration(), &declaration);
    assert_eq!(
        validated.contract_identity(),
        admitted.contract_identity().as_str()
    );
    assert!(validated
        .canonical_basis()
        .contains("validated-source-declaration"));
}

#[test]
fn runtime_rejects_unregistered_source_declaration() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let error = runtime
        .admit_source(registered_source(
            "source:missing",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .expect_err("unregistered source declaration should fail");

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::SourceContractMismatch
    );
    let retained = runtime
        .diagnostics()
        .source_failure_for_declaration_identity("source:missing")
        .expect("unregistered source rejection should retain canonical source failure");
    assert_eq!(
        retained.failure_class(),
        crate::source::SourceFailureClass::SourceContractMismatch
    );
    assert_eq!(
        retained.delivery_error_kind(),
        crate::error::BridgeDeliveryErrorKind::SourceContractMismatch
    );
    assert_eq!(
        runtime
            .diagnostics()
            .explain_source_failure_record(&retained)
            .failure_identity(),
        retained.failure_identity().as_str()
    );
}

#[test]
fn runtime_materializes_registered_source_packet() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let observation = runtime
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered historical source should materialize");

    assert_eq!(observation.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(
        observation
            .authority_basis()
            .commit_identity()
            .map(crate::input::envelope::TruthCommitIdentity::as_str),
        Some("commit-a")
    );
}

#[test]
fn runtime_records_source_materialization_rejection_when_adapter_cannot_open_snapshot() {
    let runtime =
        runtime_with_source_adapter(BridgeRuntimePolicy::default(), RejectingSourceAdapter);
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let error = match runtime.materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
    {
        Ok(_) => panic!("rejecting source adapter should fail materialization"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::SnapshotAcquisitionFailure
    );
    let failure = runtime
        .diagnostics()
        .last_source_failure_record()
        .expect("source failure should be retained");
    assert_eq!(
        failure.failure_class(),
        crate::source::SourceFailureClass::SourceMaterializationRejected
    );
    assert_eq!(
        failure.delivery_error_kind(),
        crate::error::BridgeDeliveryErrorKind::SnapshotAcquisitionFailure
    );
}

#[test]
fn runtime_records_adapter_capability_drift_when_adapter_binds_wrong_snapshot() {
    let runtime = runtime_with_source_adapter(BridgeRuntimePolicy::default(), DriftSourceAdapter);
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let error = match runtime.materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
    {
        Ok(_) => panic!("drift source adapter should fail materialization"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::SnapshotIdentityMismatch
    );
    let failure = runtime
        .diagnostics()
        .last_source_failure_record()
        .expect("source failure should be retained");
    assert_eq!(
        failure.failure_class(),
        crate::source::SourceFailureClass::AdapterCapabilityDrift
    );
    assert_eq!(
        failure.delivery_error_kind(),
        crate::error::BridgeDeliveryErrorKind::SnapshotIdentityMismatch
    );
}

#[test]
fn runtime_rejects_source_packet_set_reordering_from_adapter() {
    let runtime =
        runtime_with_source_adapter(BridgeRuntimePolicy::default(), ReorderingSourceAdapter);
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let error = match runtime.materialize_source_packet_batch(
        &contract,
        vec![
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-2", "profile",
            )]),
        ],
    ) {
        Ok(_) => panic!("reordered source packet set should be rejected"),
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::SourceContractMismatch
    );
    let failure = runtime
        .diagnostics()
        .last_source_failure_record()
        .expect("source failure should be retained");
    assert_eq!(
        failure.failure_class(),
        crate::source::SourceFailureClass::AdapterCapabilityDrift
    );
    assert_eq!(
        failure.delivery_error_kind(),
        crate::error::BridgeDeliveryErrorKind::SourceContractMismatch
    );
}

#[test]
fn runtime_plans_registered_source_packet_set() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let planned = runtime
        .plan_source_packet_set(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered source packet set should plan");

    assert_eq!(planned.contract(), &contract);
    assert_eq!(planned.packets().len(), 1);
    assert_eq!(
        planned
            .validated_declaration()
            .declaration()
            .declaration_identity()
            .as_str(),
        "source:analysis-history"
    );
}

#[test]
fn runtime_materializes_registered_source_packet_set() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");
    let planned = runtime
        .plan_source_packet_set(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered source packet set should plan");

    let materialized = runtime
        .materialize_source(&planned)
        .expect("registered source packet set should materialize");

    assert_eq!(materialized.planned_packet_set().digest(), planned.digest());
    assert_eq!(materialized.observations().len(), 1);
    assert_eq!(
        materialized.first().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(materialized.planned_packet_set().packet_count(), 1);
    assert_eq!(materialized.planned_packet_set().packet_member_count(), 0);
    assert_eq!(materialized.materialization_count(), 1);
}

#[test]
fn runtime_canonicalizes_registered_source_materialization_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let observation = runtime
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered historical source should materialize");
    let record = runtime
        .canonicalize_source_materialization_record(&contract, &observation)
        .expect("registered source materialization should canonicalize");

    assert_eq!(
        record.source_declaration_identity(),
        "source:analysis-history"
    );
    assert_eq!(record.snapshot_identities().len(), 1);
    assert_eq!(record.snapshot_identities()[0].as_str(), "snapshot-a");
    assert_eq!(
        record.source_capability_digest(),
        contract.required_capabilities().digest()
    );
    assert_eq!(record.counters().source_packet_count(), 1);
    assert_eq!(record.counters().source_materialization_count(), 1);
    assert_eq!(
        record.planned_packet_set_digest(),
        runtime
            .plan_source_packet_set(&contract, SnapshotReadPacket::new(vec![]))
            .expect("source packet set should plan")
            .digest()
    );
}

#[test]
fn runtime_canonicalizes_registered_source_packet_set_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");
    let planned = runtime
        .plan_source_packet_set(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered source packet set should plan");
    let materialized = runtime
        .materialize_source(&planned)
        .expect("registered source packet set should materialize");

    let record = runtime
        .canonicalize_source_materialization_packet_set_record(&materialized)
        .expect("registered source packet set should canonicalize");

    assert_eq!(record.planned_packet_set_digest(), planned.digest());
    assert_eq!(
        record.materialized_packet_set_digest(),
        materialized.digest()
    );
    assert_eq!(
        record.counters().source_packet_count(),
        planned.packet_count()
    );
    assert_eq!(
        record.counters().source_packet_member_count(),
        planned.packet_member_count()
    );
    assert_eq!(
        record.counters().source_materialization_count(),
        materialized.materialization_count()
    );
}

#[test]
fn runtime_replays_multi_packet_source_materialization_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let planned_packet_set = runtime
        .plan_source_packet_batch(
            &contract,
            vec![
                SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-1", "profile",
                )]),
                SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-2", "profile",
                )]),
            ],
        )
        .expect("multi-packet source set should plan");
    let materialized = runtime
        .materialize_source_packet_batch(
            &contract,
            planned_packet_set
                .packets()
                .iter()
                .map(|packet| packet.read_packet().clone())
                .collect(),
        )
        .expect("multi-packet source set should materialize");
    let record = runtime
        .canonicalize_source_materialization_packet_set_record(&materialized)
        .expect("multi-packet source set should canonicalize");

    assert_eq!(record.counters().source_packet_count(), 2);
    assert_eq!(record.read_packets().len(), 2);
    assert_eq!(record.planned_packet_digests().len(), 2);
    assert_eq!(record.truth_view_digest(), materialized.digest());

    let replayed = runtime
        .replay_source_materialization_record(&record)
        .expect("multi-packet source materialization should replay");

    assert_eq!(replayed, record);
}

#[test]
fn runtime_lowers_identical_registered_source_requests_to_identical_records() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let left = runtime
        .canonicalize_source_materialization_record(
            &contract,
            &runtime
                .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
                .expect("left source packet should materialize"),
        )
        .expect("left source record should canonicalize");
    let right = runtime
        .canonicalize_source_materialization_record(
            &contract,
            &runtime
                .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
                .expect("right source packet should materialize"),
        )
        .expect("right source record should canonicalize");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn runtime_retains_source_materialization_record_in_diagnostics() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let observation = runtime
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered historical source should materialize");
    let record = runtime
        .canonicalize_source_materialization_record(&contract, &observation)
        .expect("registered source materialization should canonicalize");

    let retained = runtime
        .diagnostics()
        .last_source_materialization_record()
        .expect("source materialization record should be retained");

    assert_eq!(retained, record);
    assert_eq!(
        runtime
            .diagnostics()
            .source_materialization_record_for_identity(record.record_identity().as_str())
            .expect("record should be queryable by identity"),
        record
    );
}

#[test]
fn runtime_replays_registered_source_materialization_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("registered historical source should be admitted");

    let observation = runtime
        .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
        .expect("registered historical source should materialize");
    let record = runtime
        .canonicalize_source_materialization_record(&contract, &observation)
        .expect("registered source materialization should canonicalize");

    let replayed = runtime
        .replay_source_materialization_record(&record)
        .expect("registered source materialization should replay");

    assert_eq!(replayed, record);
}

#[test]
fn source_builder_order_does_not_change_materialized_source_truth() {
    let first = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("first builder order should succeed");

    let second = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .with_source_adapter(StaticSourceAdapter)
        .register_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::ReplayCompatibleRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::SnapshotRead,
            ],
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("second builder order should succeed");

    let first_contract = first
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("first runtime source should admit");
    let second_contract = second
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                crate::input::envelope::TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("second runtime source should admit");

    let first_record = first
        .canonicalize_source_materialization_record(
            &first_contract,
            &first
                .materialize_source_packet(&first_contract, SnapshotReadPacket::new(vec![]))
                .expect("first source packet should materialize"),
        )
        .expect("first source record should canonicalize");
    let second_record = second
        .canonicalize_source_materialization_record(
            &second_contract,
            &second
                .materialize_source_packet(&second_contract, SnapshotReadPacket::new(vec![]))
                .expect("second source packet should materialize"),
        )
        .expect("second source record should canonicalize");

    assert_eq!(
        first.source_registry().digest(),
        second.source_registry().digest()
    );
    assert_eq!(first_record, second_record);
}

#[test]
fn source_diagnostics_richness_preserves_source_truth() {
    let minimal = runtime(BridgeRuntimePolicy::operational());
    let exhaustive = runtime(BridgeRuntimePolicy::forensic());
    let declaration = registered_source(
        "source:analysis-history",
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            crate::input::envelope::TruthCommitIdentity::new("commit-a"),
        ),
        vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayCompatibleRead,
        ],
    );

    let minimal_contract = minimal
        .admit_source(declaration.clone())
        .expect("minimal diagnostics runtime should admit source");
    let exhaustive_contract = exhaustive
        .admit_source(declaration)
        .expect("exhaustive diagnostics runtime should admit source");

    let minimal_record = minimal
        .canonicalize_source_materialization_record(
            &minimal_contract,
            &minimal
                .materialize_source_packet(&minimal_contract, SnapshotReadPacket::new(vec![]))
                .expect("minimal diagnostics source should materialize"),
        )
        .expect("minimal diagnostics source record should canonicalize");
    let exhaustive_record = exhaustive
        .canonicalize_source_materialization_record(
            &exhaustive_contract,
            &exhaustive
                .materialize_source_packet(&exhaustive_contract, SnapshotReadPacket::new(vec![]))
                .expect("exhaustive diagnostics source should materialize"),
        )
        .expect("exhaustive diagnostics source record should canonicalize");

    assert_eq!(minimal_record, exhaustive_record);
    assert_eq!(
        minimal
            .diagnostics()
            .explain_source_materialization_record(&minimal_record)
            .snapshot_identities(),
        exhaustive
            .diagnostics()
            .explain_source_materialization_record(&exhaustive_record)
            .snapshot_identities()
    );
}

use super::*;
