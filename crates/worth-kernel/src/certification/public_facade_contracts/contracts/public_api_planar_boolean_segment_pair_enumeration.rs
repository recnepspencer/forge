use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneReducedOperandPairRequest, WorkloadCompositionError,
    WorkloadStageRequirement,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, certification_only_unsupported_stage_row,
    complete_ledger_stage_snapshot,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanCanonicalSegmentSet,
    PlanarBooleanSegmentPairEnumerationReceipt,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;
#[test]
fn query_candidate_index_product_owns_rows_counters_culls_and_fallback_posture() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt =
            segment_pair_enumeration_from_catalog("phase7.2 segment pair indexed breadth");
        let counters = receipt.counters();

        assert_eq!(
            counters.expected_pair_breadth(),
            counters.left_segment_count() * counters.right_segment_count()
        );
        assert_eq!(counters.expected_pair_breadth(), 16);
        assert!(
            !counters.expected_pair_breadth_overflowed(),
            "admitted catalog pair enumeration must expose representable breadth"
        );
        assert_eq!(counters.emitted_pair_breadth(), receipt.work_items().len());
        assert_eq!(
            counters.emitted_pair_breadth(),
            receipt.candidate_rows().len()
        );
        assert_eq!(counters.emitted_pair_breadth(), 12);
        assert_eq!(counters.skipped_pair_count(), 4);
        assert_eq!(
            counters.query_index_candidate_count(),
            counters.emitted_pair_breadth()
        );
        assert_eq!(
            counters.query_index_culled_pair_count(),
            counters.skipped_pair_count()
        );
        assert!(
            counters.skipped_pair_count() > 0,
            "candidate planning must prove spatial-index culling instead of full cross-product work"
        );
        assert!(!receipt.query_index_identity().is_empty());
        assert!(!receipt.query_index_declaration_digest().is_empty());
        assert!(!receipt.query_index_plan_digest().is_empty());
        assert!(!receipt.query_index_envelope_digest().is_empty());
        assert_eq!(
            receipt.candidate_index_strategy(),
            PlanarBooleanCandidateIndexStrategy::AabbSweep
        );
        assert_eq!(
            receipt.fallback_posture(),
            PlanarBooleanCandidateIndexFallbackPosture::NotUsed
        );
        assert_eq!(
            receipt.candidate_index_lifecycle_outcome(),
            PlanarBooleanCandidateIndexLifecycleOutcome::Bound
        );
        assert_eq!(
            receipt.candidate_index_product_identity(),
            receipt.query_index_identity()
        );
        assert_eq!(
            counters.envelope_expanded_pair_count(),
            receipt.candidate_rows().len()
        );
        assert!(counters.broad_phase_comparison_count() > 0);
        assert!(
            counters.broad_phase_comparison_count() < counters.expected_pair_breadth(),
            "indexed candidate discovery must expose actual broad-phase work below full pair breadth"
        );
        assert_eq!(counters.degenerate_skip_count(), 0);
        assert!(!counters.fallback_used());
        assert!(receipt.candidate_rows().iter().all(|row| {
            !row.candidate_identity().is_empty()
                && row.local_frame_identity() == row.left().local_frame_identity()
                && row.local_frame_identity() == row.right().local_frame_identity()
                && row.precision_basis_identity() == row.left().precision_basis_identity()
                && row.precision_basis_identity() == row.right().precision_basis_identity()
                && !row.left_source_edge_identity().is_empty()
                && !row.right_source_edge_identity().is_empty()
        }));
        assert!(receipt.work_items().iter().all(|work_item| {
            work_item.left().operand_side() == PlanarBooleanCommonPlaneOperandSide::Left
                && work_item.right().operand_side() == PlanarBooleanCommonPlaneOperandSide::Right
        }));
    });
}

#[test]
fn candidate_index_product_denies_or_marks_nonproduction_full_breadth_fallback() {
    reduced_pair_support::run_with_large_stack(|| {
        let receipt = segment_pair_enumeration_from_catalog("phase7.2 candidate index no fallback");
        let counters = receipt.counters();

        assert_eq!(
            receipt.fallback_posture(),
            PlanarBooleanCandidateIndexFallbackPosture::NotUsed
        );
        assert_eq!(
            receipt.candidate_index_lifecycle_outcome(),
            PlanarBooleanCandidateIndexLifecycleOutcome::Bound
        );
        assert!(!counters.fallback_used());
        assert!(
            counters.query_index_culled_pair_count() > 0,
            "production candidate indexing must not hide a full-breadth fallback behind admitted Query evidence"
        );
        assert!(
            counters.broad_phase_comparison_count() < counters.expected_pair_breadth(),
            "production candidate indexing must expose broad-phase work below full breadth"
        );
    });
}

#[test]
fn segment_pair_enumeration_is_replay_stable_from_catalog_reduced_pair() {
    reduced_pair_support::run_with_large_stack(|| {
        let first = segment_pair_enumeration_from_catalog("phase7.2 segment pair replay stable");
        let second = segment_pair_enumeration_from_catalog("phase7.2 segment pair replay stable");

        assert_eq!(pair_identities(&first), pair_identities(&second));
        assert_eq!(
            first.segment_pair_enumeration_identity(),
            second.segment_pair_enumeration_identity()
        );
        assert_eq!(
            first.canonical_segment_set_identity(),
            second.canonical_segment_set_identity()
        );
    });
}

#[test]
fn worth_workload_requires_real_segment_pair_enumeration_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, canonical_segments) =
            canonical_segments_from_catalog("phase7.2 segment pair workload evidence");
        let receipt = canonical_segments
            .segment_pair_enumeration_receipt()
            .expect("pair enumeration should certify");
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("bare workload must reject missing pair-enumeration evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            )
        );

        let admitted = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(&receipt)],
        );
        admitted
            .require_boolean_segment_pair_enumeration(&receipt)
            .expect("real pair-enumeration receipt evidence should pass");
        let evidence_counters = complete_ledger_stage_snapshot(
            admitted.evidence_ledger(),
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        )
        .expect("pair-enumeration evidence row should exist")
        .counters();
        assert_eq!(
            evidence_counters.boolean_segment_pair_enumeration_count(),
            1
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_expected_breadth(),
            receipt.counters().expected_pair_breadth()
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_emitted_breadth(),
            receipt.counters().emitted_pair_breadth()
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_envelope_expanded_count(),
            receipt.counters().envelope_expanded_pair_count()
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_broad_phase_comparison_count(),
            receipt.counters().broad_phase_comparison_count()
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_degenerate_skip_count(),
            0
        );
        assert_eq!(
            evidence_counters.boolean_segment_pair_fallback_used_count(),
            0
        );
    });
}

#[test]
fn segment_pair_enumeration_rejects_missing_or_synthetic_pair_rows() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, canonical_segments) =
            canonical_segments_from_catalog("phase7.2 hostile segment pair evidence");
        let receipt = canonical_segments
            .segment_pair_enumeration_receipt()
            .expect("pair enumeration should certify");

        let manual = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                receipt.segment_pair_enumeration_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("manual pair-enumeration row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            )
        );

        let counterless = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                receipt.segment_pair_enumeration_identity(),
                WorkloadEvidenceStageCounters::default(),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("counterless pair-enumeration row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            )
        );

        let unsupported = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_unsupported_stage_row(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                receipt.segment_pair_enumeration_identity(),
                WorkloadEvidenceStageCounters::boolean_segment_pair_enumeration(receipt.counters()),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("support-mismatched pair-enumeration row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanSegmentPairEnumeration
            )
        );

        let wrong_counter_family = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                receipt.segment_pair_enumeration_identity(),
                WorkloadEvidenceStageCounters::boolean_event_extraction_request(),
            )],
        );
        assert_eq!(
            wrong_counter_family
                .require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("wrong counter family must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            )
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_segment_pair_enumeration_evidence_row() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, canonical_segments) =
            canonical_segments_from_catalog("phase7.2 segment pair evidence subject");
        let (_, foreign_canonical_segments) =
            canonical_segments_from_catalog("phase7.2 segment pair evidence foreign");
        let receipt = canonical_segments
            .segment_pair_enumeration_receipt()
            .expect("pair enumeration should certify");
        let foreign_receipt = foreign_canonical_segments
            .segment_pair_enumeration_receipt()
            .expect("foreign pair enumeration should certify");

        let mismatched = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &foreign_receipt,
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_segment_pair_enumeration(&receipt)
                .expect_err("foreign pair-enumeration evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration
            )
        );
    });
}

fn segment_pair_enumeration_from_catalog(
    readiness_scope: &'static str,
) -> PlanarBooleanSegmentPairEnumerationReceipt {
    canonical_segments_from_catalog(readiness_scope)
        .1
        .segment_pair_enumeration_receipt()
        .expect("pair enumeration should derive from canonical segment proof")
}

fn canonical_segments_from_catalog(
    readiness_scope: &'static str,
) -> (
    worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    PlanarBooleanCanonicalSegmentSet,
) {
    let (pair, operand_a, operand_b) =
        reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
            readiness_scope,
        );
    let canonical_segments =
        PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
            operand_a, operand_b,
        )
        .expect("reduced pair should certify")
        .segment_carrier_set()
        .expect("carrier set should certify")
        .canonical_segment_set()
        .expect("canonical segment set should certify");
    (pair, canonical_segments)
}

fn pair_identities(receipt: &PlanarBooleanSegmentPairEnumerationReceipt) -> Vec<String> {
    receipt
        .work_items()
        .iter()
        .map(|work_item| work_item.segment_pair_identity().to_string())
        .collect()
}
