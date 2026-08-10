#[cfg(test)]
mod tests {
    use worth_runtime_bridge::facade::{
        BridgeHistoricalLineageAuthority, BridgeHistoricalResolvedLineageIdentity,
        BridgeHistoricalResolvedRecordIdentity, BridgeMappingId, BridgeMappingRegistration,
        BridgeRuntimePolicy, BridgeSourceCapability, BridgeSourceCapabilitySet,
        BridgeTruthViewSelector, CoarseRoutingMode, ReducedStructuralMatchSet, RuntimeBridge,
        RuntimeBridgeBuilder, SignalInvalidationScope, SourceDeclaration,
        SourceDeclarationIdentity, StructuralCandidateIdentity,
        StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
        StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
        StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
        StructuralIdentityDeclarationIdentity, StructuralMatchCandidate,
        StructuralMatchCandidateKind, StructuralMatchOutcomeClass, StructuralSchemaIdentity,
        StructuralTruthViewBasis, TruthBranchIdentity, TruthCommitIdentity, TruthSnapshotIdentity,
    };

    use super::super::bridge_lowering::{
        lower_lineage_authority, lower_reduced_structural_match_set,
    };
    use super::super::bridge_lowering_fixtures::{StaticSink, StaticSource, StaticSourceAdapter};
    use super::super::contracts::StructuralCandidateBudget;
    use super::super::cost::{
        StructuralCandidateDiscoveryPlan, StructuralCandidateOrderingContract,
    };
    use super::super::request::CorrespondenceEvaluationRequest;
    use super::super::resolution::resolve_correspondence_evidence;

    #[test]
    fn bridge_lineage_single_successor_becomes_authoritative_continuity() {
        let authority = BridgeHistoricalLineageAuthority::try_new(
            worth_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
                TruthBranchIdentity::from_bridge_harness_label("main"),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
            ),
            vec![
                BridgeHistoricalResolvedLineageIdentity::from_bridge_harness_label(
                    "lineage:subject-a",
                ),
            ],
            vec![BridgeHistoricalResolvedRecordIdentity::from_bridge_harness_label("record:a")],
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
            worth_runtime_bridge::facade::BridgeHistoricalLineageTopology::SingleSuccessor
        );
        assert_eq!(resolved.outcome().family_name(), "lineage_continuity");
    }

    #[test]
    fn bridge_lineage_non_single_successor_does_not_promote_to_continuity() {
        let authority = BridgeHistoricalLineageAuthority::try_new(
            worth_runtime_bridge::facade::BridgeContinuityAuthorityBasis::new(
                TruthBranchIdentity::from_bridge_harness_label("main"),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
            ),
            vec![
                BridgeHistoricalResolvedLineageIdentity::from_bridge_harness_label("lineage:a"),
                BridgeHistoricalResolvedLineageIdentity::from_bridge_harness_label("lineage:b"),
            ],
            vec![
                BridgeHistoricalResolvedRecordIdentity::from_bridge_harness_label("record:a"),
                BridgeHistoricalResolvedRecordIdentity::from_bridge_harness_label("record:b"),
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
            worth_runtime_bridge::facade::BridgeHistoricalLineageTopology::SplitSuccessors
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
                StructuralCandidateIdentity::from_stable_name("candidate:b"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            ),
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::from_stable_name("candidate:a"),
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
                StructuralCandidateIdentity::from_stable_name("candidate:z"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            ),
            StructuralMatchCandidate::new(
                StructuralCandidateIdentity::from_stable_name("candidate:a"),
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
                TruthBranchIdentity::from_bridge_harness_label("analysis"),
                TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
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
                    TruthBranchIdentity::from_bridge_harness_label("analysis"),
                    TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
                ),
                vec![
                    BridgeSourceCapability::SnapshotRead,
                    BridgeSourceCapability::BranchRead,
                ],
            ))
            .register_source(registered_source(
                "source:analysis-history",
                BridgeTruthViewSelector::historical_commit(
                    TruthBranchIdentity::from_bridge_harness_label("analysis"),
                    TruthCommitIdentity::from_bridge_harness_label("commit-a"),
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
                        TruthBranchIdentity::from_bridge_harness_label("analysis"),
                        TruthSnapshotIdentity::from_bridge_harness_label("snapshot-a"),
                    ),
                ),
            ))
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::from_stable_name("mapping"),
                worth_runtime_bridge::facade::TruthPatchScope::new(
                    worth_runtime_bridge::facade::MappingSelector::exact("entity-1"),
                    worth_runtime_bridge::facade::AspectKeySelector::exact(
                        worth_foundational::facade::AspectKey::new("profile")
                            .expect("valid native mapping aspect key"),
                    ),
                    worth_runtime_bridge::facade::TruthPatchTargetSelector::entity_field(
                        worth_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid native mapping field key"),
                    ),
                ),
                worth_runtime_bridge::facade::SnapshotReadContract::scalar(
                    worth_foundational::facade::AspectKey::new("profile")
                        .expect("valid native snapshot aspect key"),
                    worth_foundational::facade::ScalarAspectType::String,
                ),
                SignalInvalidationScope::from_stable_name("signal:profile"),
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
            SourceDeclarationIdentity::from_stable_name(id),
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
            StructuralIdentityDeclarationIdentity::from_stable_name(id),
            StructuralSchemaIdentity::from_stable_name("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::from_stable_name("schema:geometry"),
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
