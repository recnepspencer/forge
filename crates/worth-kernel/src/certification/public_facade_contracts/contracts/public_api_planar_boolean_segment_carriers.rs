use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrierOperandSource,
    PlanarBooleanSegmentCarrierSet, PlanarBooleanSegmentCarrierSetDenialKind,
};

#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn segment_carriers_preserve_operand_loop_edge_and_projection_provenance() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier provenance",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let carriers = reduced_pair
            .segment_carrier_set()
            .expect("segment carriers should extract from the reduced pair proof");
        assert!(!carriers.left().is_empty());
        assert!(!carriers.right().is_empty());
        assert_eq!(carriers.left().len(), carriers.right().len());

        let left = &carriers.left()[0];
        assert_eq!(
            left.operand_side(),
            PlanarBooleanCommonPlaneOperandSide::Left
        );
        assert_eq!(left.loop_role(), PlanarBooleanLoopRole::OuterBoundary);
        assert!(!left.source_face_identity().is_empty());
        assert!(!left.source_loop_identity().is_empty());
        assert!(!left.source_edge_identity().is_empty());
        assert_eq!(
            left.local_frame_identity(),
            reduced_pair.local_frame_selection_identity()
        );
        assert_eq!(
            left.projection_stage_identity(),
            reduced_pair.left_projection_stage_identity()
        );
        assert_eq!(
            left.precision_basis_identity(),
            reduced_pair.precision_agreement_identity()
        );
        assert!(!left.start().projected_endpoint_fact_identity().is_empty());
        assert!(!left.end().projected_endpoint_fact_identity().is_empty());
        assert_ne!(
            left.start().projected_endpoint_fact_identity(),
            left.end().projected_endpoint_fact_identity()
        );
        assert_ne!(left.start().point(), left.end().point());
    });
}

#[test]
fn segment_carrier_extraction_rejects_foreign_projection_receipt_context() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier substitution",
            );
        let (_, foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier foreign substitution",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let foreign_reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                foreign_operand_a,
                foreign_operand_b,
            )
            .expect("foreign reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                foreign_reduced_pair.left_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("foreign projection receipt must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::OperandSourceContextMismatch
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_projected_workload_substitution() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier workload substitution",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.right_projected_workload(),
                reduced_pair.left_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("foreign projected workload must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::ProjectionStageIdentityMismatch
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_operand_side_substitution() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier side substitution",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("operand side substitution must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::ProjectionOperandSideMismatch
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_swapped_operand_slots() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier swapped slots",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                reduced_pair.left_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("swapped operand slots must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::OperandSlotSideMismatch
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_missing_precision_basis() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier missing precision",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                reduced_pair.left_projection_receipt(),
                "",
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("missing precision basis must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::MissingPrecisionBasisIdentity
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_mixed_reduced_pair_contexts() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier context one",
            );
        let (_, foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier context two",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let foreign_reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                foreign_operand_a,
                foreign_operand_b,
            )
            .expect("foreign reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                reduced_pair.left_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                foreign_reduced_pair.right_projected_workload(),
                foreign_reduced_pair.right_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
        )
        .expect_err("mixed reduced-pair carrier sources must deny before construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::OperandSourceContextMismatch
        );
    });
}

#[test]
fn segment_carrier_extraction_rejects_split_precision_contexts() {
    reduced_pair_support::run_with_large_stack(|| {
        let (_, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 segment carrier split precision",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let denial = PlanarBooleanSegmentCarrierSet::from_projected_operands(
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Left,
                reduced_pair.left_projected_workload(),
                reduced_pair.left_projection_receipt(),
                reduced_pair.precision_agreement_identity(),
            ),
            PlanarBooleanSegmentCarrierOperandSource::new(
                PlanarBooleanCommonPlaneOperandSide::Right,
                reduced_pair.right_projected_workload(),
                reduced_pair.right_projection_receipt(),
                "foreign precision basis",
            ),
        )
        .expect_err("split precision basis must deny before carrier construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanSegmentCarrierSetDenialKind::PrecisionBasisIdentityMismatch
        );
    });
}

#[test]
fn catalog_event_recipes_produce_real_carrier_backed_operand_pairs() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::event_carrier_projected_operand_requests_from_catalog(
                "phase7.2 catalog event carrier pair",
            );
        assert_eq!(
            pair.recipe().query_key(),
            "worth.catalog.boolean_event_carrier_clean_planar_body_pair"
        );

        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let carriers = reduced_pair
            .segment_carrier_set()
            .expect("catalog event recipe should produce carrier-backed operands");

        assert!(carriers.total_carrier_count() >= 8);
        assert!(carriers
            .left()
            .iter()
            .all(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Left));
        assert!(carriers
            .right()
            .iter()
            .all(|carrier| carrier.operand_side() == PlanarBooleanCommonPlaneOperandSide::Right));
    });
}
