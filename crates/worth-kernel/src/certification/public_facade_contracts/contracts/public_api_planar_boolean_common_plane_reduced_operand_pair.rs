use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneReducedOperandPairAssemblyError,
    PlanarBooleanCommonPlaneReducedOperandPairRequest, WorkloadCompositionError,
    WorkloadStageRequirement,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, certification_only_unsupported_stage_row,
    complete_ledger_stage_snapshot,
};
use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    PlanarBooleanCommonPlaneReducedOperandPairReceipt,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn reduced_operand_pair_request_converges_across_construction_paths() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair parity");
        let ordinary =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a.clone(),
                operand_b.clone(),
            )
            .expect("ordinary reduced pair should certify");
        let advanced_receipt =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                operand_a.projection_receipt(),
                operand_b.projection_receipt(),
            )
            .expect("advanced reduced-pair receipt should certify from the same child proof");
        let advanced = PlanarBooleanCommonPlaneReducedOperandPairRequest::from_parts(
            operand_a,
            operand_b,
            advanced_receipt,
        )
        .expect("advanced reduced pair should preserve identity");

        assert_eq!(
            ordinary.reduced_operand_pair_identity(),
            advanced.reduced_operand_pair_identity()
        );
        assert_eq!(ordinary.ordering_contract(), advanced.ordering_contract());
        assert_eq!(
            ordinary.source_left_operand_workload_identity(),
            advanced.source_left_operand_workload_identity()
        );
        assert_eq!(
            ordinary.source_right_operand_workload_identity(),
            advanced.source_right_operand_workload_identity()
        );
    });
}

#[test]
fn reduced_operand_pair_request_preserves_explicit_ordering_contract() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair one");
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        assert_eq!(
            reduced_pair.ordering_contract().semantic_left_side(),
            worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Left
        );
        assert_eq!(
            reduced_pair.ordering_contract().semantic_right_side(),
            worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide::Right
        );
        assert_ne!(
            reduced_pair.left_projection_stage_identity(),
            reduced_pair.right_projection_stage_identity(),
            "phase 10 must preserve distinct operand-local projection stage identities rather than flattening them"
        );
    });
}

#[test]
fn reduced_operand_pair_request_rejects_mismatched_reduced_pair_receipt() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair one");
        let (foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair two");
        let foreign_receipt =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                foreign_operand_a,
                foreign_operand_b,
            )
            .expect("foreign reduced pair should certify")
            .reduced_pair_receipt()
            .clone();

        let error = PlanarBooleanCommonPlaneReducedOperandPairRequest::from_parts(
            operand_a,
            operand_b,
            foreign_receipt,
        )
        .expect_err("foreign reduced-pair receipt must fail");

        assert!(matches!(
            error,
            PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LeftOperandProjectionIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::RightOperandProjectionIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SharedPlaneReceiptIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SharedPlaneIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::PlaneAgreementIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LocalFrameSelectionIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::LeftProjectionStageIdentityMismatch { .. }
                | PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::RightProjectionStageIdentityMismatch { .. }
        ));
    });
}

#[test]
fn reduced_operand_pair_request_preserves_spatial_mixed_chain_denial() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, _) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair one");
        let (_, foreign_operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.1 reduced pair two");

        let error =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a,
                foreign_operand_b,
            )
            .expect_err("mixed reduction chains must deny before reduced-pair proof is minted");

        match error {
            PlanarBooleanCommonPlaneReducedOperandPairAssemblyError::SpatialReducedOperandPairDenied {
                kind,
                human_reason,
            } => {
                assert!(matches!(
                    kind,
                    PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneReceiptIdentityMismatch
                        | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::SharedPlaneIdentityMismatch
                        | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::PlaneAgreementIdentityMismatch
                        | PlanarBooleanCommonPlaneReducedOperandPairDenialKind::LocalFrameSelectionIdentityMismatch
                ));
                assert!(human_reason.contains("requires"));
            }
            other => panic!("expected preserved spatial denial, got {other:?}"),
        }
    });
}

#[test]
fn reduced_operand_pair_evidence_row_replays_to_one_identity_across_construction_paths() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.1 reduced pair evidence replay",
            );
        let ordinary =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a.clone(),
                operand_b.clone(),
            )
            .expect("ordinary reduced pair should certify");
        let advanced_receipt =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                operand_a.projection_receipt(),
                operand_b.projection_receipt(),
            )
            .expect("advanced reduced-pair receipt should certify from the same child proof");
        let advanced = PlanarBooleanCommonPlaneReducedOperandPairRequest::from_parts(
            operand_a,
            operand_b,
            advanced_receipt,
        )
        .expect("advanced reduced pair should preserve identity");

        let ordinary_workload = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                ordinary.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            )],
        );
        let ordinary_row = complete_ledger_stage_snapshot(
            ordinary_workload.evidence_ledger(),
            WorkloadEvidenceStage::BooleanReducedOperandPair,
        )
        .expect("ordinary reduced-pair evidence row");
        let advanced_workload = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                advanced.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            )],
        );
        let advanced_row = complete_ledger_stage_snapshot(
            advanced_workload.evidence_ledger(),
            WorkloadEvidenceStage::BooleanReducedOperandPair,
        )
        .expect("advanced reduced-pair evidence row");

        assert_eq!(
            ordinary_row.evidence_identity(),
            advanced_row.evidence_identity()
        );
        assert_eq!(ordinary_row.counters(), advanced_row.counters());
        assert_eq!(ordinary_row.support(), advanced_row.support());
    });
}

#[test]
fn worth_workload_requires_real_reduced_operand_pair_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.1 reduced pair evidence",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("bare workload must reject missing reduced-pair evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanReducedOperandPair
            )
        );

        let admitted = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                reduced_pair.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            )],
        );
        admitted
            .require_boolean_reduced_operand_pair(&reduced_pair)
            .expect("real reduced-pair evidence should pass");
        assert_eq!(
            complete_ledger_stage_snapshot(
                admitted.evidence_ledger(),
                WorkloadEvidenceStage::BooleanReducedOperandPair,
            )
            .expect("reduced-pair row should exist")
            .counters()
            .boolean_reduced_operand_pair_count(),
            1
        );
    });
}

#[test]
fn worth_workload_rejects_manual_counterless_and_support_mismatched_reduced_pair_rows() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.1 hostile reduced pair evidence",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");

        let manual = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                reduced_pair.reduced_operand_pair_request_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("manual reduced-pair row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanReducedOperandPair
            )
        );

        let counterless = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                reduced_pair.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::default(),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("counterless reduced-pair row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanReducedOperandPair
            )
        );

        let unsupported = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_unsupported_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                reduced_pair.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("support-mismatched reduced-pair row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanReducedOperandPair
            )
        );

        let wrong_counter_family = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                reduced_pair.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption(),
            )],
        );
        assert_eq!(
            wrong_counter_family
                .require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("reduced-pair rows backed only by operand-A counters must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanReducedOperandPair
            )
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_reduced_pair_evidence_row() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.1 reduced pair evidence left",
            );
        let (_, foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.1 reduced pair evidence right",
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

        let mismatched = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanReducedOperandPair,
                foreign_reduced_pair.reduced_operand_pair_request_identity(),
                WorkloadEvidenceStageCounters::boolean_reduced_operand_pair(),
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_reduced_operand_pair(&reduced_pair)
                .expect_err("foreign reduced-pair evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanReducedOperandPair
            )
        );
    });
}
