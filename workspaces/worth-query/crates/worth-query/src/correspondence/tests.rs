#[cfg(test)]
mod tests {
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
                    vec!["b".to_string(), "c".to_string()],
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
                vec!["record:a".to_string()],
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
                vec!["record:a".to_string(), "record:b".to_string()],
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
            vec!["record:structural".to_string()],
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
                vec!["record:a".to_string(), "record:b".to_string()],
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
}
