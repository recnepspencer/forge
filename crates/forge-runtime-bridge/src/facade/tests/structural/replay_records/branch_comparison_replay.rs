use super::*;

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
                    crate::input::envelope::TruthPatchIdentity::new("patch-a"),
                    TruthSnapshotIdentity::new("snapshot-a"),
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
            crate::input::envelope::BridgeCommittedPatchEnvelope,
            crate::adapter::RelationalBridgeSourceError,
        > {
            let snapshot = if branch_identity.as_str() == "right" {
                "snapshot-b"
            } else {
                "snapshot-a"
            };
            crate::input::envelope::BridgeCommittedPatchEnvelope::new(
                crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                    crate::input::envelope::TruthCommitIdentity::new(format!(
                        "head-{}",
                        branch_identity.as_str()
                    )),
                    crate::input::envelope::TruthPatchIdentity::new("patch-head"),
                    TruthSnapshotIdentity::new(snapshot),
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
