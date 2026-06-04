#[cfg(test)]
mod tests {
    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeDeliveryReceipt,
        BridgeHistoricalLineageAuthority, BridgeHistoricalResolvedLineageIdentity,
        BridgeHistoricalResolvedRecordIdentity, BridgeMappingId, BridgeMappingRegistration,
        BridgeRuntimePolicy, BridgeSignalInvalidationDelivery, BridgeSnapshotReadError,
        BridgeSourceAdapter, BridgeSourceCapability, BridgeSourceCapabilitySet,
        BridgeTruthViewSelector, CoarseRoutingMode, CommittedPatchSource, InvalidationSink,
        ReducedStructuralMatchSet, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
        RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
        SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
        SourceDeclaration, SourceDeclarationIdentity, StructuralCandidateIdentity,
        StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
        StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
        StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
        StructuralIdentityDeclarationIdentity, StructuralMatchCandidate,
        StructuralMatchCandidateKind, StructuralMatchOutcomeClass, StructuralSchemaIdentity,
        StructuralTruthViewBasis, TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity,
        TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
    };

    use super::super::bridge_lowering::{
        lower_lineage_authority, lower_reduced_structural_match_set,
    };
    use super::super::contracts::{
        CorrespondenceComplexityContract, CorrespondencePerformanceStatusMarker,
        StructuralCandidateBudget, UniqueStructuralCorrespondenceWitness,
    };
    use super::super::cost::{
        CorrespondenceCostPosture, StructuralCandidateDiscoveryPlan,
        StructuralCandidateOrderingContract,
    };
    use super::super::counters::CorrespondenceCounterSnapshot;
    use super::super::outcome::{
        AdvisoryStructuralAmbiguous, AdvisoryStructuralUnique, CorrespondenceDenied,
        CorrespondenceOutcome, LineageContinuity,
    };
    use super::super::report::CorrespondenceVocabularyReport;
    use super::super::request::CorrespondenceEvaluationRequest;
    use super::super::resolution::resolve_correspondence_evidence;

    #[derive(Clone)]
    struct StaticSource;

    impl CommittedPatchSource for StaticSource {
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
            request: &SnapshotReadPacket,
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
            CorrespondenceCostPosture::LineageDirect.as_str(),
            "lineage_direct"
        );
        assert_eq!(
            CorrespondenceCostPosture::StructuralCandidateBounded.as_str(),
            "structural_candidate_bounded"
        );
        assert_eq!(
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded.as_str(),
            "fingerprint_bucket_bounded"
        );
        assert_eq!(
            StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder.as_str(),
            "stable_fingerprint_then_lineage_hint_order"
        );
        assert_eq!(
            CorrespondencePerformanceStatusMarker::Verified.as_str(),
            "verified"
        );
    }

    #[test]
    fn correspondence_outcome_family_names_are_distinct() {
        let lineage = CorrespondenceOutcome::lineage_continuity(LineageContinuity::new("a", "b"));
        let unique = CorrespondenceOutcome::advisory_structural_unique(
            AdvisoryStructuralUnique::new("b", UniqueStructuralCorrespondenceWitness::new()),
        );
        let ambiguous =
            CorrespondenceOutcome::advisory_structural_ambiguous(AdvisoryStructuralAmbiguous::new(
                super::super::candidate_set::CorrespondenceCandidateSet::new(
                    vec!["b".into(), "c".into()],
                    StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                    StructuralCandidateBudget::new(2),
                    StructuralCandidateOrderingContract::StableFingerprintOrder,
                ),
            ));

        assert_eq!(lineage.family_name(), "lineage_continuity");
        assert_eq!(unique.family_name(), "advisory_structural_unique");
        assert_eq!(ambiguous.family_name(), "advisory_structural_ambiguous");
        assert_ne!(lineage.family_name(), unique.family_name());
        assert_ne!(unique.family_name(), ambiguous.family_name());
    }

    #[test]
    fn complexity_contract_names_are_deterministic() {
        assert_eq!(
            CorrespondenceComplexityContract::lineage_direct().contract_name(),
            "correspondence_lineage_direct"
        );
        assert_eq!(
            CorrespondenceComplexityContract::structural_candidate_bounded().contract_name(),
            "correspondence_structural_candidate_bounded"
        );
        assert_eq!(
            CorrespondenceComplexityContract::structural_ambiguity_bounded().contract_name(),
            "correspondence_structural_ambiguity_bounded"
        );
    }

    #[test]
    fn vocabulary_report_preserves_selected_family_and_posture() {
        let outcome = CorrespondenceOutcome::denied(CorrespondenceDenied::new(
            CorrespondenceCostPosture::CorrespondenceDeniedByBreadth,
            "denied",
        ));
        let report = CorrespondenceVocabularyReport::from_outcome(
            &outcome,
            CorrespondenceCostPosture::CorrespondenceDeniedByBreadth,
            CorrespondenceComplexityContract::structural_candidate_bounded(),
            CorrespondenceCounterSnapshot::vocabulary_baseline(),
        );

        assert_eq!(report.outcome_family_name(), "correspondence_denied");
        assert_eq!(
            report.cost_posture().as_str(),
            "correspondence_denied_by_breadth"
        );
        assert_eq!(
            report.complexity_contract().contract_name(),
            "correspondence_structural_candidate_bounded"
        );
    }

    #[test]
    fn lineage_only_single_successor_lowers_to_lineage_continuity() {
        let resolved =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::lineage_only(
                "subject:a",
                "record:a",
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                1,
            ))
            .expect("lineage-only request should resolve");

        assert_eq!(resolved.outcome().family_name(), "lineage_continuity");
        assert_eq!(resolved.cost_posture().as_str(), "lineage_direct");
        assert_eq!(
            resolved.complexity_contract().contract_name(),
            "correspondence_lineage_direct"
        );
        assert_eq!(
            resolved
                .counters()
                .correspondence_executor_rediscovery_count(),
            0
        );
    }

    #[test]
    fn structural_only_one_candidate_lowers_to_unique() {
        let resolved =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
                vec!["record:a".into()],
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                1,
                StructuralCandidateOrderingContract::StableFingerprintOrder,
            ))
            .expect("structural-only request should resolve");

        assert_eq!(
            resolved.outcome().family_name(),
            "advisory_structural_unique"
        );
        assert_eq!(
            resolved.cost_posture().as_str(),
            "structural_candidate_bounded"
        );
        assert_eq!(
            resolved.counters().predicted_structural_candidate_count(),
            1
        );
        assert_eq!(resolved.counters().structural_unique_witness_count(), 1);
    }

    #[test]
    fn structural_only_multiple_candidates_lowers_to_ambiguity() {
        let resolved =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
                vec!["record:a".into(), "record:b".into()],
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                2,
                StructuralCandidateOrderingContract::StableFingerprintOrder,
            ))
            .expect("ambiguous structural request should resolve");

        assert_eq!(
            resolved.outcome().family_name(),
            "advisory_structural_ambiguous"
        );
        assert_eq!(
            resolved.cost_posture().as_str(),
            "structural_ambiguity_bounded"
        );
        assert_eq!(resolved.counters().structural_ambiguity_count(), 1);
        assert_eq!(
            resolved
                .counters()
                .correspondence_executor_rediscovery_count(),
            0
        );
    }

    #[test]
    fn lineage_and_structural_mismatch_lowers_to_disagreement() {
        let resolved = resolve_correspondence_evidence(CorrespondenceEvaluationRequest::mixed(
            "subject:a",
            "record:lineage",
            vec!["record:structural".into()],
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            1,
            StructuralCandidateOrderingContract::StableFingerprintThenLineageHintOrder,
        ))
        .expect("mixed mismatch request should resolve");

        assert_eq!(
            resolved.outcome().family_name(),
            "lineage_structural_disagreement"
        );
        assert_eq!(
            resolved.counters().lineage_structural_disagreement_count(),
            1
        );
        assert_eq!(
            resolved.complexity_contract().contract_name(),
            "correspondence_lineage_structural_disagreement"
        );
    }

    #[test]
    fn structural_breadth_overflow_lowers_to_denied() {
        let resolved =
            resolve_correspondence_evidence(CorrespondenceEvaluationRequest::structural_only(
                vec!["record:a".into(), "record:b".into()],
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                1,
                StructuralCandidateOrderingContract::StableFingerprintOrder,
            ))
            .expect("breadth overflow should become typed denial");

        assert_eq!(resolved.outcome().family_name(), "correspondence_denied");
        assert_eq!(
            resolved.cost_posture().as_str(),
            "correspondence_denied_by_breadth"
        );
        assert_eq!(
            resolved.counters().structural_candidate_rejection_count(),
            1
        );
    }

    #[test]
    fn unsupported_structural_family_lowers_to_typed_denial() {
        let resolved = resolve_correspondence_evidence(
            CorrespondenceEvaluationRequest::unsupported_structural_family(
                "unsupported_test_family",
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                1,
            ),
        )
        .expect("unsupported structural family should resolve into denial");

        assert_eq!(resolved.outcome().family_name(), "correspondence_denied");
        assert_eq!(
            resolved.counters().structural_candidate_rejection_count(),
            1
        );
    }

    #[test]
    fn unsupported_lineage_topology_lowers_to_typed_denial() {
        let resolved = resolve_correspondence_evidence(
            CorrespondenceEvaluationRequest::unsupported_lineage_topology(
                "merge_like_successor",
                StructuralCandidateDiscoveryPlan::IndexBackedBounded,
                1,
            ),
        )
        .expect("unsupported lineage topology should resolve into denial");

        assert_eq!(resolved.outcome().family_name(), "correspondence_denied");
        assert_eq!(
            resolved.cost_posture().as_str(),
            "correspondence_denied_by_topology"
        );
    }

    #[test]
    fn mixed_lineage_conflict_lowers_to_disagreement_without_best_effort_promotion() {
        let resolved = resolve_correspondence_evidence(
            CorrespondenceEvaluationRequest::mixed_lineage_conflict(
                "subject:a",
                "record:lineage",
                "record:structural",
                StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
                1,
            ),
        )
        .expect("lineage conflict should resolve");

        assert_eq!(
            resolved.outcome().family_name(),
            "lineage_structural_disagreement"
        );
        assert_eq!(
            resolved
                .counters()
                .structural_authority_promotion_denial_count(),
            1
        );
    }

    #[test]
    fn bridge_lineage_single_successor_becomes_authoritative_continuity() {
        let authority = BridgeHistoricalLineageAuthority::try_new(
            forge_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
                TruthBranchIdentity::new("main"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![BridgeHistoricalResolvedLineageIdentity::new(
                "lineage:subject-a",
            )],
            vec![BridgeHistoricalResolvedRecordIdentity::new("record:a")],
            vec![1],
        )
        .expect("lineage authority should build");

        let lowered = lower_lineage_authority(&authority);
        let request = CorrespondenceEvaluationRequest::from_inputs(
            Some(lowered),
            None,
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            StructuralCandidateBudget::bounded(1),
        )
        .expect("bridge lowered request should be valid");
        let resolved = resolve_correspondence_evidence(request).expect("resolved");

        assert_eq!(
            authority.topology(),
            forge_runtime_bridge::facade::BridgeHistoricalLineageTopology::SingleSuccessor
        );
        assert_eq!(resolved.outcome().family_name(), "lineage_continuity");
    }

    #[test]
    fn bridge_lineage_non_single_successor_does_not_promote_to_continuity() {
        let authority = BridgeHistoricalLineageAuthority::try_new(
            forge_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
                TruthBranchIdentity::new("main"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                BridgeHistoricalResolvedLineageIdentity::new("lineage:a"),
                BridgeHistoricalResolvedLineageIdentity::new("lineage:b"),
            ],
            vec![
                BridgeHistoricalResolvedRecordIdentity::new("record:a"),
                BridgeHistoricalResolvedRecordIdentity::new("record:b"),
            ],
            vec![1, 2],
        )
        .expect("split lineage authority should build");

        let lowered = lower_lineage_authority(&authority);
        let request = CorrespondenceEvaluationRequest::from_inputs(
            Some(lowered),
            None,
            StructuralCandidateDiscoveryPlan::IndexBackedBounded,
            StructuralCandidateBudget::bounded(2),
        )
        .expect("bridge lowered request should be valid");
        let resolved = resolve_correspondence_evidence(request).expect("resolved");

        assert_eq!(
            authority.topology(),
            forge_runtime_bridge::facade::BridgeHistoricalLineageTopology::SplitSuccessors
        );
        assert_eq!(resolved.outcome().family_name(), "correspondence_denied");
        assert_eq!(
            resolved.cost_posture().as_str(),
            "correspondence_denied_by_topology"
        );
    }

    #[test]
    fn bridge_reduced_structural_candidates_lower_without_exposing_bridge_records() {
        let reduced = advisory_reduced_set(vec![
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:b"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            ),
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:a"),
                StructuralMatchCandidateKind::AdvisoryReuseCandidate,
            ),
        ]);

        let lowered = lower_reduced_structural_match_set(
            &reduced,
            &StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            &StructuralCandidateBudget::bounded(2),
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        )
        .expect("reduced structural set should lower");
        let request = CorrespondenceEvaluationRequest::from_inputs(
            None,
            Some(lowered),
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            StructuralCandidateBudget::bounded(2),
        )
        .expect("bridge lowered request should be valid");
        let resolved = resolve_correspondence_evidence(request).expect("resolved");

        assert_eq!(
            reduced.outcome_class(),
            StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch
        );
        assert_eq!(
            resolved.outcome().family_name(),
            "advisory_structural_ambiguous"
        );
    }

    #[test]
    fn bridge_lowering_preserves_candidate_order() {
        let reduced = advisory_reduced_set(vec![
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:z"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            ),
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:a"),
                StructuralMatchCandidateKind::AdvisoryReuseCandidate,
            ),
        ]);

        let lowered = lower_reduced_structural_match_set(
            &reduced,
            &StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            &StructuralCandidateBudget::bounded(2),
            StructuralCandidateOrderingContract::StableFingerprintOrder,
        )
        .expect("reduced structural set should lower");
        let request = CorrespondenceEvaluationRequest::from_inputs(
            None,
            Some(lowered),
            StructuralCandidateDiscoveryPlan::FingerprintBucketBounded,
            StructuralCandidateBudget::bounded(2),
        )
        .expect("bridge lowered request should be valid");
        let resolved = resolve_correspondence_evidence(request).expect("resolved");
        let ambiguous = resolved
            .outcome()
            .as_advisory_structural_ambiguous()
            .expect("should stay ambiguous");

        assert_eq!(
            ambiguous.candidate_set().candidates(),
            &["candidate:a".to_string(), "candidate:z".to_string()]
        );
    }

    fn advisory_reduced_set(
        candidates: Vec<StructuralMatchCandidate>,
    ) -> ReducedStructuralMatchSet {
        let runtime = runtime();
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
            .plan_structural_match_packet_set(&contract, candidates)
            .expect("structural candidates should plan");
        runtime
            .reduce_structural_match_set(&planned)
            .expect("planned structural packet set should reduce")
    }

    fn runtime() -> RuntimeBridge {
        RuntimeBridgeBuilder::new()
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
            .register_structural(registered_structural(
                "structural:analysis-snapshot",
                StructuralFingerprintFamily::TopologyFingerprint,
                StructuralTruthViewBasis::explicit_snapshot(
                    BridgeTruthViewSelector::branch_snapshot(
                        TruthBranchIdentity::new("analysis"),
                        TruthSnapshotIdentity::new("snapshot-a"),
                    ),
                ),
            ))
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("mapping"),
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
            ))
            .build()
            .expect("runtime should build for structural lowering tests")
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

    fn registered_structural(
        id: &str,
        family: StructuralFingerprintFamily,
        truth_view_basis: StructuralTruthViewBasis,
    ) -> StructuralIdentityDeclaration {
        StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::new(id),
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::new("schema:geometry"),
                family,
                "geometry-v1",
                StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
                StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
                StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
            ),
            truth_view_basis,
        )
    }
}
