use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneReducedOperandPairAssemblyError,
    PlanarBooleanCommonPlaneReducedOperandPairRequest, PlanarBooleanEventExtractionRequest,
    WorkloadCompositionError, WorkloadStageRequirement,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneReducedOperandPairReceipt;
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceRow, WorkloadEvidenceStage};

#[path = "public_api_planar_boolean_event_extraction_request_support.rs"]
mod event_request_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

#[test]
fn event_extraction_request_preserves_reduced_pair_identity_across_replay() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.2 event request parity");
        let ordinary_reduced =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a.clone(),
                operand_b.clone(),
            )
            .expect("ordinary reduced pair should certify");
        let replayed_reduced =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("replayed reduced pair should certify");

        let ordinary =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(ordinary_reduced);
        let replayed =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(replayed_reduced);

        assert_eq!(
            ordinary.event_extraction_request_identity(),
            replayed.event_extraction_request_identity()
        );
        assert_eq!(
            ordinary.reduced_operand_pair_request_identity(),
            replayed.reduced_operand_pair_request_identity()
        );
        assert_eq!(
            ordinary.reduced_operand_pair_identity(),
            replayed.reduced_operand_pair_identity()
        );
    });
}

#[test]
fn event_extraction_request_exposes_common_plane_projection_and_source_workload_identities() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.2 event request identity");
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(reduced_pair.clone());

        assert_eq!(
            event_request.reduced_operand_pair_request_identity(),
            reduced_pair.reduced_operand_pair_request_identity()
        );
        assert_eq!(
            event_request.shared_plane_identity(),
            reduced_pair.shared_plane_identity()
        );
        assert_eq!(
            event_request.shared_plane_receipt_identity(),
            reduced_pair.shared_plane_receipt_identity()
        );
        assert_eq!(
            event_request.plane_agreement_identity(),
            reduced_pair.plane_agreement_identity()
        );
        assert_eq!(
            event_request.precision_agreement_identity(),
            reduced_pair.precision_agreement_identity()
        );
        assert_eq!(
            event_request.local_frame_selection_identity(),
            reduced_pair.local_frame_selection_identity()
        );
        assert_eq!(
            event_request.left_projection_identity(),
            reduced_pair.left_projection_identity()
        );
        assert_eq!(
            event_request.right_projection_identity(),
            reduced_pair.right_projection_identity()
        );
        assert_eq!(
            event_request.left_projection_stage_identity(),
            reduced_pair.left_projection_stage_identity()
        );
        assert_eq!(
            event_request.right_projection_stage_identity(),
            reduced_pair.right_projection_stage_identity()
        );
        assert_eq!(
            event_request.source_left_operand_workload_identity(),
            reduced_pair.source_left_operand_workload_identity()
        );
        assert_eq!(
            event_request.source_right_operand_workload_identity(),
            reduced_pair.source_right_operand_workload_identity()
        );
        assert_ne!(
            event_request.event_extraction_request_identity(),
            event_request.reduced_operand_pair_request_identity(),
            "7.2 must mint a new phase identity without erasing the 7.1 reduced-pair identity"
        );
    });
}

#[test]
fn event_extraction_request_cannot_start_from_mismatched_common_plane_and_projection_ids() {
    reduced_pair_support::run_with_large_stack(|| {
        let (operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.2 event request source");
        let (foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::projected_operand_requests("phase7.2 event request foreign");
        let foreign_receipt =
            PlanarBooleanCommonPlaneReducedOperandPairReceipt::from_operand_projection_receipts(
                foreign_operand_a.projection_receipt(),
                foreign_operand_b.projection_receipt(),
            )
            .expect("foreign receipt should certify only for its own projection chain");

        let error = PlanarBooleanCommonPlaneReducedOperandPairRequest::from_parts(
            operand_a,
            operand_b,
            foreign_receipt,
        )
        .expect_err("event extraction must never receive a mismatched reduced-pair request");

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
fn worth_workload_requires_real_event_extraction_request_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.2 event request evidence",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(reduced_pair);
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_event_extraction_request(&event_request)
                .expect_err("bare workload must reject missing event-extraction request evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanEventExtractionRequest
            )
        );

        let admitted = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &event_request,
            )],
        );
        admitted
            .require_boolean_event_extraction_request(&event_request)
            .expect("real event-extraction request evidence should pass");
        assert_eq!(
            admitted
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanEventExtractionRequest)
                .expect("event-extraction request row should exist")
                .counters()
                .boolean_event_extraction_request_count(),
            1
        );
    });
}

#[test]
fn worth_workload_rejects_manual_counterless_and_support_mismatched_event_request_rows() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.2 hostile event request evidence",
            );
        let reduced_pair =
            PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
                operand_a, operand_b,
            )
            .expect("reduced pair should certify");
        let event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(reduced_pair);

        let manual = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanEventExtractionRequest,
                event_request.event_extraction_request_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_event_extraction_request(&event_request)
                .expect_err("manual event-extraction request row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanEventExtractionRequest
            )
        );

        let counterless = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &event_request_support::CounterlessEventExtractionRequestEvidence::new(
                    &event_request,
                ),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_event_extraction_request(&event_request)
                .expect_err("counterless event-extraction request row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanEventExtractionRequest
            )
        );

        let unsupported = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &event_request_support::SupportMismatchedEventExtractionRequestEvidence::new(
                    &event_request,
                ),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_event_extraction_request(&event_request)
                .expect_err("support-mismatched event-extraction request row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanEventExtractionRequest
            )
        );

        let wrong_counter_family = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &event_request_support::WrongCounterFamilyEventExtractionRequestEvidence::new(
                    &event_request,
                ),
            )],
        );
        assert_eq!(
            wrong_counter_family
                .require_boolean_event_extraction_request(&event_request)
                .expect_err("event request rows backed by reduced-pair counters must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanEventExtractionRequest
            )
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_event_extraction_request_evidence_row() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, operand_a, operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.2 event request evidence left",
            );
        let (_, foreign_operand_a, foreign_operand_b) =
            reduced_pair_support::projected_operand_requests_from_catalog(
                "phase7.2 event request evidence right",
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
        let event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(reduced_pair);
        let foreign_event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(foreign_reduced_pair);

        let mismatched = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &foreign_event_request,
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_event_extraction_request(&event_request)
                .expect_err("foreign event-extraction request evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanEventExtractionRequest
            )
        );
    });
}
