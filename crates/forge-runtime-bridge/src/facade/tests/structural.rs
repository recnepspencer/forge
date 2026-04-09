use super::*;
use crate::facade::{
    StructuralCandidateIdentity, StructuralComparisonMode,
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralMatchCandidate, StructuralMatchCandidateKind,
    StructuralMatchOutcomeClass, StructuralSchemaIdentity, StructuralTruthViewBasis,
};

#[test]
fn runtime_admits_registered_structural_declaration() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    assert_eq!(
        contract
            .validated_declaration()
            .declaration()
            .comparison_mode(),
        StructuralComparisonMode::AdvisoryRemap
    );
}

#[test]
fn runtime_plans_and_reduces_structural_candidates_for_advisory_remap() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:geometry-a"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            )],
        )
        .expect("structural candidates should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("planned structural packet set should reduce");
    let artifact = runtime
        .publish_structural_remap_artifact(&reduced)
        .expect("advisory remap outcome should publish");

    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::ExactAdvisoryMatch
    );
    assert!(artifact
        .canonical_basis()
        .contains("published-structural-remap-artifact"));
}

#[test]
fn runtime_rejects_branch_diff_candidate_for_advisory_remap_contract() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let error = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:branch-diff"),
                StructuralMatchCandidateKind::BranchDiff,
            )],
        )
        .expect_err("branch diff candidate should be rejected for advisory remap");

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::StructuralPlanRejected
    );
}

#[test]
fn runtime_reduces_branch_comparison_candidates_to_branch_artifact() {
    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("diff:one"),
                StructuralMatchCandidateKind::BranchDiff,
            )],
        )
        .expect("branch diff candidate should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("branch comparison should reduce");
    let artifact = runtime
        .publish_branch_comparison_artifact(&reduced)
        .expect("branch comparison outcome should publish");

    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::BranchComparisonArtifact
    );
    assert!(artifact
        .canonical_basis()
        .contains("published-branch-comparison-artifact"));
}

#[test]
fn runtime_rejects_remap_publication_for_ambiguous_reduced_match_set() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![
                StructuralMatchCandidate::new(
                    StructuralCandidateIdentity::new("candidate:a"),
                    StructuralMatchCandidateKind::ExactAdvisoryMatch,
                ),
                StructuralMatchCandidate::new(
                    StructuralCandidateIdentity::new("candidate:b"),
                    StructuralMatchCandidateKind::AdvisoryReuseCandidate,
                ),
            ],
        )
        .expect("ambiguous structural candidates should still plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("ambiguous planned packet set should reduce");

    let error = runtime
        .publish_structural_remap_artifact(&reduced)
        .expect_err("ambiguous reduced set should not publish a remap artifact");
    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::StructuralPlanRejected
    );
}

#[test]
fn runtime_reduces_lineage_structural_divergence_to_typed_rejection() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:lineage-divergence"),
                StructuralMatchCandidateKind::LineageStructuralDivergence,
            )],
        )
        .expect("lineage divergence candidate should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("lineage divergence should reduce");

    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence
    );
    assert!(
        runtime.publish_structural_remap_artifact(&reduced).is_err(),
        "lineage divergence must not publish an advisory remap artifact"
    );
    assert_eq!(reduced.branch_diff_count(), 0);
}

#[test]
fn runtime_canonicalizes_and_replays_structural_remap_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-1", "profile"),
            ])],
        )
        .expect("structural candidates should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("planned structural packet set should reduce");
    let artifact = runtime
        .publish_structural_remap_artifact(&reduced)
        .expect("advisory remap outcome should publish");
    let record =
        runtime.canonicalize_structural_remap_record(&contract, &planned, &reduced, &artifact);

    let replay = runtime
        .replay_canonical_structural_remap_record(&record)
        .expect("structural remap replay should succeed");
    let explanation = runtime
        .diagnostics()
        .explain_last_structural_remap_record()
        .expect("structural remap record should be retained");

    assert_eq!(replay.digest(), artifact.digest());
    assert_eq!(
        runtime
            .diagnostics()
            .last_structural_remap_record()
            .expect("structural remap record should be present")
            .record_identity(),
        record.record_identity()
    );
    assert_eq!(explanation.candidate_count(), 1);
    assert_eq!(
        explanation.outcome_class(),
        StructuralMatchOutcomeClass::ExactAdvisoryMatch
    );
    assert_eq!(explanation.counters().structural_declaration_count(), 1);
    assert_eq!(explanation.counters().structural_contract_count(), 1);
    assert_eq!(explanation.counters().structural_match_packet_count(), 1);
    assert_eq!(explanation.counters().structural_candidate_count(), 1);
    assert_eq!(explanation.counters().structural_exact_match_count(), 1);
    assert_eq!(explanation.counters().structural_fingerprint_count(), 2);
    assert_eq!(explanation.counters().structural_replay_request_count(), 0);
    assert_eq!(explanation.counters().structural_replay_mismatch_count(), 0);
    assert_eq!(
        explanation
            .counters()
            .branch_comparison_drift_rejection_count(),
        0
    );
}

#[test]
fn runtime_canonicalizes_and_replays_structural_branch_comparison_record() {
    #[derive(Clone)]
    struct BranchDiffSource;

    #[derive(Clone)]
    struct SnapshotBReader;

    impl crate::snapshot::TruthSnapshotReader for SnapshotBReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-b")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-b"),
                request
                    .reads()
                    .iter()
                    .map(|read| {
                        crate::snapshot::SnapshotReadRecord::new(
                            read.request_key(),
                            b"fixture-value-b".to_vec(),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl crate::adapter::CommittedPatchSource for BranchDiffSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
                vec![],
            ))
        }
    }

    impl crate::adapter::SnapshotReadSource for BranchDiffSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<
            Box<dyn crate::snapshot::TruthSnapshotReader>,
            crate::adapter::RelationalBridgeSourceError,
        > {
            match identity.as_str() {
                "snapshot-a" => Ok(Box::new(StaticSnapshotReader)),
                "snapshot-b" => Ok(Box::new(SnapshotBReader)),
                other => Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{other}`"
                ))),
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for BranchDiffSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let snapshot = if branch_identity.as_str() == "right" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                TruthSnapshotIdentity::new(snapshot),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-b"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(BranchDiffSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(BranchDiffSource)
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_branch_comparison_from_read_packet(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
        )
        .expect("branch diff candidate should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("branch comparison should reduce");
    let artifact = runtime
        .publish_branch_comparison_artifact(&reduced)
        .expect("branch comparison outcome should publish");
    let record = runtime
        .canonicalize_structural_branch_comparison_record(&contract, &planned, &reduced, &artifact);

    let replay = runtime
        .replay_canonical_structural_branch_comparison_record(&record)
        .expect("structural branch comparison replay should succeed");
    let explanation = runtime
        .diagnostics()
        .explain_last_structural_branch_comparison_record()
        .expect("structural branch comparison record should be retained");

    assert_eq!(replay.digest(), artifact.digest());
    assert_eq!(explanation.branch_diff_count(), 1);
    assert_eq!(explanation.candidate_count(), 1);
    assert_eq!(explanation.counters().branch_comparison_count(), 1);
    assert_eq!(explanation.counters().branch_comparison_diff_count(), 1);
    assert_eq!(explanation.counters().structural_candidate_count(), 1);
    assert_eq!(explanation.counters().structural_fingerprint_count(), 3);
    assert_eq!(explanation.counters().structural_widened_scan_count(), 0);
    assert_eq!(
        explanation
            .counters()
            .branch_comparison_drift_rejection_count(),
        0
    );
    assert_eq!(explanation.counters().structural_replay_request_count(), 0);
    assert_eq!(explanation.counters().structural_replay_mismatch_count(), 0);
}

#[test]
fn runtime_replay_rejects_incompatible_structural_remap_record_version() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-1", "profile"),
            ])],
        )
        .expect("structural candidates should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("planned structural packet set should reduce");
    let artifact = runtime
        .publish_structural_remap_artifact(&reduced)
        .expect("advisory remap outcome should publish");
    let record = runtime
        .canonicalize_structural_remap_record(&contract, &planned, &reduced, &artifact)
        .with_schema_version_for_test("forge-runtime-bridge.structural-remap-record.v0");

    let error = runtime
        .replay_canonical_structural_remap_record(&record)
        .expect_err("structural remap replay should reject unsupported schema versions");

    assert_eq!(
        error.kind(),
        crate::error::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
}

#[test]
fn runtime_replay_rejects_incompatible_structural_branch_record_version() {
    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_branch_comparison_from_read_packet(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
        )
        .expect("branch diff candidate should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("branch comparison should reduce");
    let artifact = runtime
        .publish_branch_comparison_artifact(&reduced)
        .expect("branch comparison outcome should publish");
    let record = runtime
        .canonicalize_structural_branch_comparison_record(&contract, &planned, &reduced, &artifact)
        .with_schema_version_for_test(
            "forge-runtime-bridge.structural-branch-comparison-record.v0",
        );

    let error = runtime
        .replay_canonical_structural_branch_comparison_record(&record)
        .expect_err(
            "structural branch comparison replay should reject unsupported schema versions",
        );

    assert_eq!(
        error.kind(),
        crate::error::BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure
    );
}

#[test]
fn runtime_rejects_structural_declaration_with_different_semantics_version() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let registered = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );

    runtime
        .admit_structural_comparison(registered)
        .expect("registered structural declaration should be admitted");

    let mismatched = StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::new("structural:analysis-snapshot"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "geometry-topology-v2",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );

    let error = runtime
        .admit_structural_comparison(mismatched)
        .expect_err("mismatched semantics version should fail explicitly");

    assert_eq!(
        error.kind(),
        crate::error::BridgeDeliveryErrorKind::StructuralContractMismatch
    );
}

#[test]
fn runtime_replay_rejects_truncated_structural_remap_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:geometry-a"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            )],
        )
        .expect("structural candidates should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("planned structural packet set should reduce");
    let artifact = runtime
        .publish_structural_remap_artifact(&reduced)
        .expect("advisory remap outcome should publish");
    let record =
        runtime.canonicalize_structural_remap_record(&contract, &planned, &reduced, &artifact);

    let error = runtime
        .replay_canonical_structural_remap_record(&record)
        .expect_err("replay should reject a remap record without retained fingerprint basis");

    assert_eq!(
        error.kind(),
        crate::error::BridgeReplayErrorKind::StructuralReplayBasisTruncated
    );
}

#[test]
fn runtime_replay_rejects_truncated_structural_branch_basis() {
    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_match_packet_set(
            &contract,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("diff:one"),
                StructuralMatchCandidateKind::BranchDiff,
            )],
        )
        .expect("branch diff candidate should plan");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("branch comparison should reduce");
    let artifact = runtime
        .publish_branch_comparison_artifact(&reduced)
        .expect("branch comparison outcome should publish");
    let record = runtime
        .canonicalize_structural_branch_comparison_record(&contract, &planned, &reduced, &artifact);

    let error = runtime
        .replay_canonical_structural_branch_comparison_record(&record)
        .expect_err("replay should reject a branch record without retained fingerprint basis");

    assert_eq!(
        error.kind(),
        crate::error::BridgeReplayErrorKind::StructuralReplayBasisTruncated
    );
}

#[test]
fn runtime_derives_structural_candidates_from_read_packets() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-1", "profile"),
            ])],
        )
        .expect("structural candidates should derive from read packets");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("derived structural packet set should reduce");

    assert_eq!(planned.candidate_count(), 1);
    assert_eq!(
        planned.candidates()[0].candidate_kind(),
        StructuralMatchCandidateKind::ExactAdvisoryMatch
    );
    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::ExactAdvisoryMatch
    );
}

#[test]
fn runtime_derives_identity_authority_conflict_from_same_snapshot_same_structure() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-2", "profile"),
            ])],
        )
        .expect("structural candidates should derive from read packets");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("derived structural packet set should reduce");

    assert_eq!(planned.candidate_count(), 1);
    assert_eq!(
        planned.candidates()[0].candidate_kind(),
        StructuralMatchCandidateKind::IdentityAuthorityConflict
    );
    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::RejectedIdentityAuthorityConflict
    );
}

#[test]
fn runtime_materializes_structural_fingerprint_from_truth_view_read() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let fingerprint = runtime
        .materialize_structural_fingerprint(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
        )
        .expect("structural fingerprint should materialize");

    assert_eq!(
        fingerprint.family(),
        StructuralFingerprintFamily::TopologyFingerprint
    );
    assert_eq!(fingerprint.snapshot_identity(), "snapshot-a");
    assert!(fingerprint
        .digest()
        .starts_with("structural-fingerprint:sha256:"));
}

#[test]
fn runtime_derives_branch_comparison_candidates_from_branch_pair_reads() {
    #[derive(Clone)]
    struct BranchDiffSource;

    #[derive(Clone)]
    struct SnapshotBReader;

    impl crate::snapshot::TruthSnapshotReader for SnapshotBReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-b")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-b"),
                request
                    .reads()
                    .iter()
                    .map(|read| {
                        crate::snapshot::SnapshotReadRecord::new(
                            read.request_key(),
                            b"fixture-value-b".to_vec(),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl crate::adapter::CommittedPatchSource for BranchDiffSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
                vec![],
            ))
        }
    }

    impl crate::adapter::SnapshotReadSource for BranchDiffSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<
            Box<dyn crate::snapshot::TruthSnapshotReader>,
            crate::adapter::RelationalBridgeSourceError,
        > {
            match identity.as_str() {
                "snapshot-a" => Ok(Box::new(StaticSnapshotReader)),
                "snapshot-b" => Ok(Box::new(SnapshotBReader)),
                other => Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{other}`"
                ))),
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for BranchDiffSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let snapshot = if branch_identity.as_str() == "right" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                TruthSnapshotIdentity::new(snapshot),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-b"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(BranchDiffSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(BranchDiffSource)
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_branch_comparison_from_read_packet(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1", "profile",
            )]),
        )
        .expect("branch comparison should derive candidates from paired reads");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("derived branch comparison packet set should reduce");

    assert_eq!(planned.candidate_count(), 1);
    assert_eq!(
        planned.candidates()[0].candidate_kind(),
        StructuralMatchCandidateKind::BranchDiff
    );
    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::BranchComparisonArtifact
    );
}

#[test]
fn runtime_branch_comparison_ignores_read_result_order_when_structure_is_equal() {
    #[derive(Clone)]
    struct ReorderedSource;

    #[derive(Clone)]
    struct ForwardReader;

    #[derive(Clone)]
    struct ReverseReader;

    impl crate::snapshot::TruthSnapshotReader for ForwardReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-a")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                request
                    .reads()
                    .iter()
                    .enumerate()
                    .map(|(index, read)| {
                        crate::snapshot::SnapshotReadRecord::new(
                            read.request_key(),
                            format!("payload-{index}").into_bytes(),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl crate::snapshot::TruthSnapshotReader for ReverseReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-b")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-b"),
                request
                    .reads()
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(index, read)| {
                        crate::snapshot::SnapshotReadRecord::new(
                            read.request_key(),
                            format!("payload-{index}").into_bytes(),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl crate::adapter::CommittedPatchSource for ReorderedSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
                vec![],
            ))
        }
    }

    impl crate::adapter::SnapshotReadSource for ReorderedSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<
            Box<dyn crate::snapshot::TruthSnapshotReader>,
            crate::adapter::RelationalBridgeSourceError,
        > {
            match identity.as_str() {
                "snapshot-a" => Ok(Box::new(ForwardReader)),
                "snapshot-b" => Ok(Box::new(ReverseReader)),
                other => Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{other}`"
                ))),
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for ReorderedSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<
            crate::input::envelope::RawCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let snapshot = if branch_identity.as_str() == "right" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                TruthSnapshotIdentity::new(snapshot),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("left"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("right"),
                TruthSnapshotIdentity::new("snapshot-b"),
            ),
        ),
    );

    let runtime = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(ReorderedSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(ReorderedSource)
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
        .register_structural(declaration.clone())
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
        .expect("runtime should build with structural declaration");

    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("branch comparison declaration should be admitted");
    let planned = runtime
        .plan_structural_branch_comparison_from_read_packet(
            &contract,
            SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-1", "profile"),
                crate::snapshot::SnapshotReadRequest::for_coarse("entity-2", "profile"),
            ]),
        )
        .expect("branch comparison should plan from reordered equal reads");
    let reduced = runtime
        .reduce_structural_match_set(&planned)
        .expect("reordered equal reads should still reduce");

    assert_eq!(planned.candidate_count(), 0);
    assert_eq!(reduced.branch_diff_count(), 0);
    assert_eq!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::BranchComparisonArtifact
    );
}
