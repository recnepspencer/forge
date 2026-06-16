use super::*;

#[test]
fn runtime_derives_structural_candidates_from_read_packets() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1",
                crate::snapshot::SnapshotReadContract::scalar(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                    forge_foundational::facade::ScalarAspectType::String,
                ),
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-1",
                    crate::snapshot::SnapshotReadContract::scalar(
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid snapshot aspect key"),
                        forge_foundational::facade::ScalarAspectType::String,
                    ),
                ),
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
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1",
                crate::snapshot::SnapshotReadContract::scalar(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                    forge_foundational::facade::ScalarAspectType::String,
                ),
            )]),
            vec![SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-2",
                    crate::snapshot::SnapshotReadContract::scalar(
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid snapshot aspect key"),
                        forge_foundational::facade::ScalarAspectType::String,
                    ),
                ),
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
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let fingerprint = runtime
        .materialize_structural_fingerprint(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1",
                crate::snapshot::SnapshotReadContract::scalar(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                    forge_foundational::facade::ScalarAspectType::String,
                ),
            )]),
        )
        .expect("structural fingerprint should materialize");

    assert_eq!(
        fingerprint.family(),
        StructuralFingerprintFamily::TopologyFingerprint
    );
    assert_eq!(
        fingerprint.snapshot_identity(),
        &crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a")
    );
    assert_eq!(
        fingerprint.snapshot_identity_text(),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a").as_str()
    );
    assert!(fingerprint
        .digest()
        .starts_with("structural-fingerprint:sha256:"));
    assert_eq!(fingerprint.record_value_evidence().records().len(), 1);
    assert_eq!(fingerprint.equivalence_member_evidence().members().len(), 1);
    assert!(fingerprint.record_value_evidence().records()[0]
        .canonical_basis()
        .starts_with("structural-record-value-evidence|"));
    assert!(fingerprint.equivalence_member_evidence().members()[0]
        .canonical_basis()
        .starts_with("structural-equivalence-member|"));
    assert!(fingerprint
        .canonical_basis()
        .contains("record-values=structural-record-value-evidence-set|"));
    assert!(fingerprint
        .canonical_basis()
        .contains("equivalence-members=structural-equivalence-member-set|"));
}

#[test]
fn runtime_derives_branch_comparison_candidates_from_branch_pair_reads() {
    #[derive(Clone)]
    struct BranchDiffSource;

    #[derive(Clone)]
    struct SnapshotBReader;

    impl crate::snapshot::TruthSnapshotReader for SnapshotBReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
                request
                    .reads()
                    .iter()
                    .map(|read| {
                        crate::snapshot::SnapshotReadRecord::for_request(
                            read,
                            forge_foundational::facade::AspectValue::String(
                                "fixture-value-b".into(),
                            ),
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
            crate::input::envelope::BridgeCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            crate::input::envelope::BridgeCommittedPatchEnvelope::new(
                crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                    request.commit_identity().clone(),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                    crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
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

    impl crate::adapter::SnapshotReadSource for BranchDiffSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<
            Box<dyn crate::snapshot::TruthSnapshotReader>,
            crate::adapter::RelationalBridgeSourceError,
        > {
            if crate::truth_identity_fixtures::truth_snapshot_fixture_matches(
                identity,
                "snapshot-a",
            ) {
                Ok(Box::new(StaticSnapshotReader))
            } else if crate::truth_identity_fixtures::truth_snapshot_fixture_matches(
                identity,
                "snapshot-b",
            ) {
                Ok(Box::new(SnapshotBReader))
            } else {
                Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{}`",
                    identity.as_str()
                )))
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for BranchDiffSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<
            crate::input::envelope::BridgeCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let branch_label = branch_identity.relational_branch_id().unwrap_or("unknown");
            let snapshot = if branch_label == "right" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            crate::input::envelope::BridgeCommittedPatchEnvelope::new(
                crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                    crate::truth_identity_fixtures::truth_commit_fixture(format!(
                        "head-{}",
                        branch_label
                    )),
                    crate::truth_identity_fixtures::truth_patch_fixture("patch-head"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture(snapshot),
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

    let declaration = StructuralIdentityDeclaration::branch_comparison(
        StructuralIdentityDeclarationIdentity::admit_bridge_owned("structural:branch-compare"),
        StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
            StructuralFingerprintFamily::BranchComparisonFingerprint,
            "geometry-branch-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_branch_pair(
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("left"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            BridgeTruthViewSelector::branch_snapshot(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-b"),
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
                crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_structural(declaration.clone())
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::admit_bridge_owned("mapping"),
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
            SignalInvalidationScope::admit_bridge_owned("signal:profile"),
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
                "entity-1",
                crate::snapshot::SnapshotReadContract::scalar(
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                    forge_foundational::facade::ScalarAspectType::String,
                ),
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
