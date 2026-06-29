use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, certification_only_unsupported_stage_row,
    complete_ledger_stage_snapshot, complete_ledger_with_additional_rows,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn worth_workload_requires_real_precision_agreement_evidence() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision evidence")
            .build()
            .expect("clean planar body pair should build");
        let precision = precision_from_pair(pair.clone());
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_precision_agreement(&precision)
                .expect_err("bare workload must reject missing precision evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanPrecisionAgreement
            )
        );

        let admitted = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
                precision.precision_agreement_identity(),
                WorkloadEvidenceStageCounters::boolean_precision_agreement(),
            )],
        );
        admitted
            .require_boolean_precision_agreement(&precision)
            .expect("real precision evidence should pass");
        assert_eq!(
            complete_ledger_stage_snapshot(
                admitted.evidence_ledger(),
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
            )
            .expect("precision row should exist")
            .counters()
            .boolean_precision_agreement_count(),
            1
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_precision_agreement_evidence_row() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision evidence left")
            .build()
            .expect("first clean planar body pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision evidence right")
            .build()
            .expect("second clean planar body pair should build");
        let precision = precision_from_pair(pair.clone());
        let foreign_precision = precision_from_pair(other_pair);

        let mismatched = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
                foreign_precision.precision_agreement_identity(),
                WorkloadEvidenceStageCounters::boolean_precision_agreement(),
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_precision_agreement(&precision)
                .expect_err("foreign precision evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanPrecisionAgreement
            )
        );
    });
}

#[test]
fn worth_workload_rejects_manual_counterless_and_support_mismatched_precision_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 hostile precision evidence")
            .build()
            .expect("clean planar body pair should build");
        let precision = precision_from_pair(pair.clone());

        let manual = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
                precision.precision_agreement_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_precision_agreement(&precision)
                .expect_err("manual precision row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanPrecisionAgreement
            )
        );

        let counterless = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
                precision.precision_agreement_identity(),
                WorkloadEvidenceStageCounters::default(),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_precision_agreement(&precision)
                .expect_err("counterless precision row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanPrecisionAgreement
            )
        );

        let unsupported = rebuild_left_workload(
            &pair,
            vec![certification_only_unsupported_stage_row(
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
                precision.precision_agreement_identity(),
                WorkloadEvidenceStageCounters::boolean_precision_agreement(),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_precision_agreement(&precision)
                .expect_err("support-mismatched precision row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanPrecisionAgreement
            )
        );
    });
}

#[test]
fn precision_stage_counters_count_only_real_receipt_backed_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 precision counters")
            .build()
            .expect("clean planar body pair should build");
        let precision = precision_from_pair(pair.clone());
        let workload = rebuild_left_workload(
            &pair,
            vec![
                certification_only_admitted_stage_row(
                    WorkloadEvidenceStage::BooleanPrecisionAgreement,
                    precision.precision_agreement_identity(),
                    WorkloadEvidenceStageCounters::boolean_precision_agreement(),
                ),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            ],
        );

        assert_eq!(
            complete_ledger_stage_snapshot(
                workload.evidence_ledger(),
                WorkloadEvidenceStage::BooleanPrecisionAgreement,
            )
            .expect("precision row should exist")
            .counters()
            .boolean_precision_agreement_count(),
            1
        );
        assert_eq!(
            complete_ledger_stage_snapshot(
                workload.evidence_ledger(),
                WorkloadEvidenceStage::BooleanSplit,
            )
            .expect("manual split row should exist")
            .counters()
            .total_receipt_backed_counters(),
            0
        );
    });
}

fn precision_from_pair(
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlanePrecisionAgreedRequest {
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand-pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            entry_support::certified_boolean_readiness_workload_receipt(
                "phase7.1 precision evidence basis",
            ),
            "phase7.1 precision evidence basis",
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query("phase7.1 precision evidence declaration")
    .bind()
    .expect("boolean declaration should bind");
    entry_support::assert_planar_boolean_query_digest(declaration.basis_query_declaration_digest());
    entry_support::assert_planar_boolean_query_digest(declaration.query_declaration_digest());

    let posture = PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                PlanarBooleanCommonPlaneReductionRequest::from_declaration_receipt_and_operand_pair(
                    declaration,
                    pair,
                )
                .expect("reduction request should build"),
            )
            .expect("scope admission should certify"),
        )
        .expect("plane agreement should certify"),
    )
    .expect("posture agreement should certify");

    PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(posture)
        .expect("precision agreement should certify")
}

fn rebuild_left_workload(
    pair: &worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> WorthWorkload {
    let left = pair.left().workload();
    let ledger = complete_ledger_with_additional_rows(left.evidence_ledger(), boolean_rows)
        .expect("classical stages should remain complete");

    WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger: ledger,
    })
    .expect("left workload should re-compose with precision evidence rows")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-precision-evidence".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("precision evidence contract thread should spawn")
        .join()
        .expect("precision evidence contract thread should finish");
}
