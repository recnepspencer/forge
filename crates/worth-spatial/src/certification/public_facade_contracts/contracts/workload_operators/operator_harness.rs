use worth_kernel::workload_composition::{
    OperatorOutcome, OperatorOutcomeKind, WorkloadCatalog, WorkloadOperator,
    WorkloadOperatorFamily, WorkloadStageRequirement,
};
use worth_spatial::facade::projected_overlap_faces::CoplanarOverlapExtractionBundle;
use worth_spatial::facade::workload_operators::{
    CoplanarOverlapOperatorDenial, CoplanarOverlapWorkloadOperator,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceRow, WorkloadEvidenceStage};

use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::{
    certify_projected_storm_context, certify_projected_storm_extraction_bundle,
};
use crate::public_api_workload_vocabulary::evidence_ledger_receipts::counter_backed_receipts;

#[test]
fn operator_harness_consumes_projected_retained_transformed_workloads() {
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
        .consumed_evidence()
        .iter()
        .any(|row| row.stage() == WorkloadEvidenceStage::Projection));
    assert!(run
        .consumed_evidence()
        .iter()
        .any(|row| row.stage() == WorkloadEvidenceStage::Transform));
    assert!(run
        .consumed_evidence()
        .iter()
        .any(|row| row.stage() == WorkloadEvidenceStage::RetainedReplay));

    let operator_receipt =
        CoplanarOverlapWorkloadOperator::from_consumed_evidence(run.consumed_evidence())
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
}

#[test]
fn coplanar_overlap_operator_branches_required_stage_denial_matrix() {
    let denial_bundle = operator_extraction_bundle("operator-denial-real-overlap-extractions");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::MissingProjectedWorkload,
        "requires projected planar workload evidence",
    );
    let missing_transform_receipts = counter_backed_receipts("operator-missing-transform");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(
                &missing_transform_receipts.projection,
            ),
            WorkloadEvidenceRow::from_replay_receipt_set(&missing_transform_receipts.replay),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::MissingTransformWorkload,
        "requires transform workload evidence",
    );
    let missing_replay_receipts = counter_backed_receipts("operator-missing-replay");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(&missing_replay_receipts.projection),
            WorkloadEvidenceRow::from_transform_receipt_set(&missing_replay_receipts.transform),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::MissingRetainedReplayWorkload,
        "requires retained replay workload evidence",
    );

    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Projection, "manual projection"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::ManualProjectedWorkload,
        "rejects hand-filled projection evidence",
    );
    let manual_transform_receipts = counter_backed_receipts("operator-manual-transform");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(&manual_transform_receipts.projection),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
            WorkloadEvidenceRow::from_replay_receipt_set(&manual_transform_receipts.replay),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::ManualTransformWorkload,
        "rejects hand-filled transform evidence",
    );
    let manual_replay_receipts = counter_backed_receipts("operator-manual-replay");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(&manual_replay_receipts.projection),
            WorkloadEvidenceRow::from_transform_receipt_set(&manual_replay_receipts.transform),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::ManualRetainedReplayWorkload,
        "rejects hand-filled retained replay evidence",
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
            WorkloadEvidenceRow::from_replay_receipt_set(&counterless_projection_receipts.replay),
        ],
        &denial_bundle,
        CoplanarOverlapOperatorDenial::SyntheticProjectedWorkload,
        "requires projected entities",
    );
    let counterless_transform_receipts = counter_backed_receipts("operator-counterless-transform");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(
                &counterless_transform_receipts.projection,
            ),
            WorkloadEvidenceRow::from_transform_receipt(
                counterless_transform_receipts.transform.stage_receipt(),
            ),
            WorkloadEvidenceRow::from_replay_receipt_set(&counterless_transform_receipts.replay),
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
            WorkloadEvidenceRow::from_transform_receipt_set(&counterless_replay_receipts.transform),
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
    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(
        missing_extractions.workload().evidence_ledger().rows(),
    )
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
}

#[test]
fn coplanar_overlap_operator_requires_context_for_real_extractions() {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .declared("operator-requires-context-source")
        .build()
        .expect("operator context source should build");
    let extraction_bundle = certify_projected_storm_extraction_bundle(
        "operator-requires-context-source",
        built.projected_workload(),
        built.transform_receipts(),
    );

    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(
        built.workload().evidence_ledger().rows(),
    )
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
}

#[test]
fn coplanar_overlap_operator_rejects_bundle_from_another_context() {
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

    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(
        operator_source.workload().evidence_ledger().rows(),
    )
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
}

fn assert_operator_denial(
    consumed_evidence: Vec<WorkloadEvidenceRow>,
    extraction_bundle: &CoplanarOverlapExtractionBundle,
    expected: CoplanarOverlapOperatorDenial,
    human_reason_fragment: &str,
) {
    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(&consumed_evidence)
        .with_extraction_bundle(extraction_bundle)
        .execute()
        .expect_err("operator must deny invalid stage evidence");

    assert_eq!(denial, expected);
    assert!(denial.human_reason().contains(human_reason_fragment));
    assert!(!denial.human_reason().contains('_'));
    assert!(!denial.human_reason().contains(".operator."));
}

fn operator_extraction_bundle(world: &'static str) -> CoplanarOverlapExtractionBundle {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .declared(format!("operator harness projected overlap bundle {world}"))
        .build()
        .expect("operator harness projected overlap workload should build");
    certify_projected_storm_extraction_bundle(
        world,
        built.projected_workload(),
        built.transform_receipts(),
    )
}
