use worth_kernel::workload_composition::{
    OperatorOutcome, OperatorOutcomeKind, WorkloadCatalog, WorkloadOperator,
    WorkloadOperatorFamily, WorkloadStageRequirement,
};
use worth_spatial::facade::workload_operators::{
    CoplanarOverlapOperatorDenial, CoplanarOverlapWorkloadOperator,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::{
    certify_projected_storm_context, certify_projected_storm_extraction_bundle,
};
use crate::public_api_workload_vocabulary::evidence_ledger_receipts::counter_backed_receipts;

use super::operator_harness_support::{
    assert_operator_denial, assert_stage_link_denial, operator_extraction_bundle,
};

#[test]
fn operator_harness_consumes_projected_retained_transformed_workloads() {
    super::run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("operator-harness-consumes-real-workload")
            .build()
            .expect("operator harness catalog workload should build");
        let extraction_bundle = certify_projected_storm_extraction_bundle(
            "operator-harness-real-overlap-extractions",
            built.projected_workload(),
            built.transform_receipts(),
        );
        let context = certify_projected_storm_context(
            "operator-harness-real-overlap-extractions",
            built.projected_workload(),
            built.transform_receipts(),
        );
        let workload = built.into_workload();

        let run = WorkloadOperator::for_family(WorkloadOperatorFamily::CoplanarOverlap)
            .requiring(WorkloadStageRequirement::RetainedReplay)
            .declared_by_query("coplanar overlap consumes retained workload proof")
            .admit_for(&workload)
            .expect("real counter-backed workload should admit");

        assert_eq!(run.evidence_rows(), 8);
        assert_eq!(
            run.declaration().family(),
            WorkloadOperatorFamily::CoplanarOverlap
        );
        assert!(!run.declaration().query_declaration_digest().is_empty());
        assert!(!run.declaration().query_envelope_digest().is_empty());
        assert!(!run.declaration().query_handle_digest().is_empty());
        assert!(!run
            .declaration()
            .query_declaration_digest()
            .contains("operator-declaration"));
        assert!(run
            .evidence_binding()
            .required_stage_links()
            .links_to(WorkloadEvidenceStage::Projection));
        assert!(run
            .evidence_binding()
            .required_stage_links()
            .links_to(WorkloadEvidenceStage::Transform));
        assert!(run
            .evidence_binding()
            .required_stage_links()
            .links_to(WorkloadEvidenceStage::RetainedReplay));
        assert_eq!(
            run.evidence_binding().stage_index_identity(),
            workload.evidence_ledger().stage_index().index_identity()
        );
        assert!(!run.evidence_binding().binding_identity().is_empty());
        let consumed_link_set_identity = run
            .evidence_binding()
            .required_stage_links()
            .link_set_identity()
            .to_string();

        let operator_receipt = CoplanarOverlapWorkloadOperator::from_stage_links(
            run.evidence_binding().required_stage_links(),
        )
        .with_certification_context(&context)
        .with_extraction_bundle(&extraction_bundle)
        .execute()
        .expect("coplanar overlap operator should consume workload proof");
        assert!(operator_receipt.links_to_stage(WorkloadEvidenceStage::Projection));
        assert!(operator_receipt.links_to_stage(WorkloadEvidenceStage::Transform));
        assert!(operator_receipt.links_to_stage(WorkloadEvidenceStage::RetainedReplay));
        let outcome = OperatorOutcome::from_coplanar_overlap_receipt(run, operator_receipt)
            .expect("kernel outcome should consume spatial operator receipt");

        assert_eq!(outcome.kind(), OperatorOutcomeKind::Admitted);
        let receipts = outcome.receipts();
        assert_eq!(receipts.family(), WorkloadOperatorFamily::CoplanarOverlap);
        assert_eq!(
            receipts.consumed_stage_links().link_set_identity(),
            consumed_link_set_identity
        );
        assert!(receipts.links_to_stage(WorkloadEvidenceStage::Projection));
        assert!(receipts.links_to_stage(WorkloadEvidenceStage::Transform));
        assert!(receipts.links_to_stage(WorkloadEvidenceStage::RetainedReplay));
        assert_eq!(
            receipts.operator_evidence_row().stage(),
            WorkloadEvidenceStage::Operator
        );
        assert_eq!(
            receipts
                .operator_evidence_row()
                .counters()
                .operator_input_count(),
            40
        );
        assert!(
            receipts
                .operator_evidence_row()
                .counters()
                .operator_receipt_count()
                > 0
        );
    });
}

#[test]
fn coplanar_overlap_operator_branches_required_stage_denial_matrix() {
    super::run_stack_heavy_test(|| {
        let denial_bundle = operator_extraction_bundle("operator-denial-real-overlap-extractions");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
            ],
            WorkloadEvidenceLedgerError::MissingAuthorityStage(WorkloadEvidenceStage::Projection),
        );
        let missing_transform_receipts = counter_backed_receipts("operator-missing-transform");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &missing_transform_receipts.projection,
                ),
                WorkloadEvidenceRow::from_replay_receipt_set(&missing_transform_receipts.replay),
            ],
            WorkloadEvidenceLedgerError::MissingAuthorityStage(WorkloadEvidenceStage::Transform),
        );
        let missing_replay_receipts = counter_backed_receipts("operator-missing-replay");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &missing_replay_receipts.projection,
                ),
                WorkloadEvidenceRow::from_transform_receipt_set(&missing_replay_receipts.transform),
            ],
            WorkloadEvidenceLedgerError::MissingAuthorityStage(
                WorkloadEvidenceStage::RetainedReplay,
            ),
        );
        let primary_receipts = counter_backed_receipts("operator-primary-binding");
        let foreign_receipts = counter_backed_receipts("operator-foreign-binding");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(&primary_receipts.projection),
                WorkloadEvidenceRow::from_transform_receipt_set(&foreign_receipts.transform),
                WorkloadEvidenceRow::from_replay_receipt_set(&primary_receipts.replay),
            ],
            WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
                WorkloadEvidenceStage::Transform,
                WorkloadEvidenceStage::Projection,
            ),
        );
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(&primary_receipts.projection),
                WorkloadEvidenceRow::from_transform_receipt_set(&primary_receipts.transform),
                WorkloadEvidenceRow::from_replay_receipt_set(&foreign_receipts.replay),
            ],
            WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
                WorkloadEvidenceStage::RetainedReplay,
                WorkloadEvidenceStage::Transform,
            ),
        );

        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::Projection, "manual projection"),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
            ],
            WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Projection),
        );
        let manual_transform_receipts = counter_backed_receipts("operator-manual-transform");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &manual_transform_receipts.projection,
                ),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
                WorkloadEvidenceRow::from_replay_receipt_set(&manual_transform_receipts.replay),
            ],
            WorkloadEvidenceLedgerError::ManualAuthorityStage(WorkloadEvidenceStage::Transform),
        );
        let manual_replay_receipts = counter_backed_receipts("operator-manual-replay");
        assert_stage_link_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &manual_replay_receipts.projection,
                ),
                WorkloadEvidenceRow::from_transform_receipt_set(&manual_replay_receipts.transform),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
            ],
            WorkloadEvidenceLedgerError::ManualAuthorityStage(
                WorkloadEvidenceStage::RetainedReplay,
            ),
        );

        let counterless_projection_receipts =
            counter_backed_receipts("operator-counterless-projection");
        assert_operator_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt(
                    counterless_projection_receipts.projection.stage_receipt(),
                ),
                WorkloadEvidenceRow::from_transform_receipt_set(
                    &counterless_projection_receipts.transform,
                ),
                WorkloadEvidenceRow::from_replay_receipt_set(
                    &counterless_projection_receipts.replay,
                ),
            ],
            &denial_bundle,
            CoplanarOverlapOperatorDenial::SyntheticProjectedWorkload,
            "requires projected entities",
        );
        let counterless_transform_receipts =
            counter_backed_receipts("operator-counterless-transform");
        assert_operator_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &counterless_transform_receipts.projection,
                ),
                WorkloadEvidenceRow::from_transform_receipt(
                    counterless_transform_receipts.transform.stage_receipt(),
                ),
                WorkloadEvidenceRow::from_replay_receipt_set(
                    &counterless_transform_receipts.replay,
                ),
            ],
            &denial_bundle,
            CoplanarOverlapOperatorDenial::SyntheticTransformWorkload,
            "requires real transform step evidence",
        );
        let counterless_replay_receipts = counter_backed_receipts("operator-counterless-replay");
        assert_operator_denial(
            vec![
                WorkloadEvidenceRow::from_projection_receipt_set(
                    &counterless_replay_receipts.projection,
                ),
                WorkloadEvidenceRow::from_transform_receipt_set(
                    &counterless_replay_receipts.transform,
                ),
                WorkloadEvidenceRow::from_retained_replay_receipt(
                    counterless_replay_receipts.replay.stage_receipt(),
                ),
            ],
            &denial_bundle,
            CoplanarOverlapOperatorDenial::SyntheticRetainedReplayWorkload,
            "requires retained artifact and replay checkpoint evidence",
        );
        let missing_extractions = WorkloadCatalog::coplanar_overlap_storm()
            .declared("operator-missing-extractions-real-context")
            .build()
            .expect("missing extraction context source should build");
        let context = certify_projected_storm_context(
            "operator-missing-extractions-real-context",
            missing_extractions.projected_workload(),
            missing_extractions.transform_receipts(),
        );
        let stage_links = missing_extractions
            .workload()
            .evidence_ledger()
            .stage_index()
            .link_required_stages(&[
                WorkloadEvidenceStage::Projection,
                WorkloadEvidenceStage::Transform,
                WorkloadEvidenceStage::RetainedReplay,
            ])
            .expect("operator stage links should build");
        let denial = CoplanarOverlapWorkloadOperator::from_stage_links(&stage_links)
            .with_certification_context(&context)
            .execute()
            .expect_err("operator must deny missing overlap extraction receipts");
        assert_eq!(
            denial,
            CoplanarOverlapOperatorDenial::MissingOverlapExtractionReceipts
        );
        assert!(denial
            .human_reason()
            .contains("requires real overlap extraction receipts"));
        assert!(!denial.human_reason().contains('_'));
    });
}

#[test]
fn coplanar_overlap_operator_requires_context_for_real_extractions() {
    super::run_stack_heavy_test(|| {
        let built = WorkloadCatalog::coplanar_overlap_storm()
            .declared("operator-requires-context-source")
            .build()
            .expect("operator context source should build");
        let extraction_bundle = certify_projected_storm_extraction_bundle(
            "operator-requires-context-source",
            built.projected_workload(),
            built.transform_receipts(),
        );

        let stage_links = built
            .workload()
            .evidence_ledger()
            .stage_index()
            .link_required_stages(&[
                WorkloadEvidenceStage::Projection,
                WorkloadEvidenceStage::Transform,
                WorkloadEvidenceStage::RetainedReplay,
            ])
            .expect("operator stage links should build");
        let denial = CoplanarOverlapWorkloadOperator::from_stage_links(&stage_links)
            .with_extraction_bundle(&extraction_bundle)
            .execute()
            .expect_err("operator must reject extraction receipts without context");

        assert_eq!(
            denial,
            CoplanarOverlapOperatorDenial::MissingCertificationContext
        );
        assert!(denial
            .human_reason()
            .contains("workload certification context"));
        assert!(!denial.human_reason().contains('_'));
    });
}

#[test]
fn coplanar_overlap_operator_rejects_bundle_from_another_context() {
    super::run_stack_heavy_test(|| {
        let operator_source = WorkloadCatalog::coplanar_overlap_storm()
            .declared("operator-context-source")
            .build()
            .expect("operator source should build");
        let bundle_source = WorkloadCatalog::coplanar_overlap_storm()
            .declared("operator-bundle-source")
            .build()
            .expect("bundle source should build");
        let context = certify_projected_storm_context(
            "operator-context-source",
            operator_source.projected_workload(),
            operator_source.transform_receipts(),
        );
        let extraction_bundle = certify_projected_storm_extraction_bundle(
            "operator-bundle-source",
            bundle_source.projected_workload(),
            bundle_source.transform_receipts(),
        );

        let stage_links = operator_source
            .workload()
            .evidence_ledger()
            .stage_index()
            .link_required_stages(&[
                WorkloadEvidenceStage::Projection,
                WorkloadEvidenceStage::Transform,
                WorkloadEvidenceStage::RetainedReplay,
            ])
            .expect("operator stage links should build");
        let denial = CoplanarOverlapWorkloadOperator::from_stage_links(&stage_links)
            .with_certification_context(&context)
            .with_extraction_bundle(&extraction_bundle)
            .execute()
            .expect_err("operator must reject bundle compiled under another context");

        assert_eq!(
            denial,
            CoplanarOverlapOperatorDenial::MismatchedOverlapExtractionBundle
        );
        assert!(denial
            .human_reason()
            .contains("same workload certification context"));
        assert!(!denial.human_reason().contains('_'));
    });
}
