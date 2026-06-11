use topology::facade::TopologyWorkloadReceipt;
use worth_kernel::workload_composition::{
    OperatorOutcome, OperatorOutcomeKind, WorkloadOperator, WorkloadOperatorFamily,
    WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_operators::{
    CoplanarOverlapOperatorDenial, CoplanarOverlapWorkloadOperator,
};
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, ResponseWorkload, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage,
};

use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::certify_storm_overlap_extractions;
use crate::public_api_workload_vocabulary::evidence_ledger_receipts::{
    counter_backed_receipts, CounterBackedReceipts,
};

#[test]
fn operator_harness_consumes_projected_retained_transformed_workloads() {
    let workload = counter_backed_operator_workload("operator-harness-consumes-real-workload");

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
            .with_overlap_extractions(&certify_storm_overlap_extractions(
                "operator-harness-real-overlap-extractions",
            ))
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
        44
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
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
        ],
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
        CoplanarOverlapOperatorDenial::MissingTransformWorkload,
        "requires transform workload evidence",
    );
    let missing_replay_receipts = counter_backed_receipts("operator-missing-replay");
    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::from_projection_receipt_set(&missing_replay_receipts.projection),
            WorkloadEvidenceRow::from_transform_receipt_set(&missing_replay_receipts.transform),
        ],
        CoplanarOverlapOperatorDenial::MissingRetainedReplayWorkload,
        "requires retained replay workload evidence",
    );

    assert_operator_denial(
        vec![
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Projection, "manual projection"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::Transform, "manual transform"),
            WorkloadEvidenceRow::new(WorkloadEvidenceStage::RetainedReplay, "manual replay"),
        ],
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
        CoplanarOverlapOperatorDenial::SyntheticRetainedReplayWorkload,
        "requires retained artifact and replay checkpoint evidence",
    );
    let missing_extractions = counter_backed_receipts("operator-missing-extractions");
    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(&[
        WorkloadEvidenceRow::from_projection_receipt_set(&missing_extractions.projection),
        WorkloadEvidenceRow::from_transform_receipt_set(&missing_extractions.transform),
        WorkloadEvidenceRow::from_replay_receipt_set(&missing_extractions.replay),
    ])
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

fn assert_operator_denial(
    consumed_evidence: Vec<WorkloadEvidenceRow>,
    expected: CoplanarOverlapOperatorDenial,
    human_reason_fragment: &str,
) {
    let denial = CoplanarOverlapWorkloadOperator::from_consumed_evidence(&consumed_evidence)
        .with_overlap_extractions(&certify_storm_overlap_extractions(
            "operator-denial-real-overlap-extractions",
        ))
        .execute()
        .expect_err("operator must deny invalid stage evidence");

    assert_eq!(denial, expected);
    assert!(denial.human_reason().contains(human_reason_fragment));
    assert!(!denial.human_reason().contains('_'));
    assert!(!denial.human_reason().contains(".operator."));
}

fn counter_backed_operator_workload(world: &'static str) -> WorthWorkload {
    let receipts = counter_backed_receipts(world);
    let topology = receipts
        .topology
        .query_receipts()
        .declaration_receipt()
        .clone();
    let diagnostics = DiagnosticWorkload::for_retained_replay(receipts.replay.stage_receipt())
        .declared(format!("operator diagnostics for {world}"))
        .admit()
        .expect("simple diagnostic stage remains admitted");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("operator response for {world}"))
        .admit()
        .expect("simple response stage remains admitted");
    let ledger = WorkloadEvidenceLedger::from_rows(operator_rows(
        &topology,
        &receipts,
        &diagnostics,
        &response,
    ))
    .expect("operator workload ledger should be inspectable")
    .certify_complete()
    .expect("operator workload ledger should be complete");

    WorthWorkload::compose(WorthWorkloadParts {
        topology,
        geometry_binding: receipts.geometry.stage_receipt().clone(),
        surface_support: receipts.support.stage_receipt().clone(),
        projection: receipts.projection.stage_receipt().clone(),
        transform: receipts.transform.stage_receipt().clone(),
        retained_replay: receipts.replay.stage_receipt().clone(),
        diagnostics,
        response,
        evidence_ledger: ledger,
    })
    .expect("operator workload should compose")
}

fn operator_rows(
    topology: &TopologyWorkloadReceipt,
    receipts: &CounterBackedReceipts,
    diagnostics: &worth_spatial::facade::workload_vocabulary::DiagnosticWorkloadReceipt,
    response: &worth_spatial::facade::workload_vocabulary::ResponseWorkloadReceipt,
) -> Vec<WorkloadEvidenceRow> {
    vec![
        WorkloadEvidenceRow::from_topology_workload_and_seed_receipts(topology, &receipts.topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt_set(&receipts.geometry),
        WorkloadEvidenceRow::from_surface_support_receipt_set(&receipts.support),
        WorkloadEvidenceRow::from_projection_receipt_set(&receipts.projection),
        WorkloadEvidenceRow::from_transform_receipt_set(&receipts.transform),
        WorkloadEvidenceRow::from_replay_receipt_set(&receipts.replay),
        WorkloadEvidenceRow::from_diagnostic_receipt(diagnostics),
        WorkloadEvidenceRow::from_response_receipt(response),
    ]
}
