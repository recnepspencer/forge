use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, certification_only_unsupported_stage_row,
    complete_ledger_stage_snapshot, complete_ledger_with_additional_rows,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn worth_workload_requires_real_shared_plane_identity_evidence() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane evidence")
            .build()
            .expect("clean planar body pair should build");
        let shared_plane = shared_plane_from_pair(pair.clone());
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("bare workload must reject missing shared-plane identity evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );

        let certification_only = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                shared_plane.shared_plane_identified_request_identity(),
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
            )],
        );
        assert_eq!(
            certification_only
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err(
                    "certification-only shared-plane rows must not satisfy production evidence"
                ),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );
        assert_eq!(
            complete_ledger_stage_snapshot(
                certification_only.evidence_ledger(),
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
            )
            .expect("shared-plane row should exist")
            .counters()
            .boolean_shared_plane_identity_count(),
            1
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_shared_plane_identity_evidence_row() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane evidence left")
            .build()
            .expect("first clean planar body pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane evidence right")
            .build()
            .expect("second clean planar body pair should build");
        let shared_plane = shared_plane_from_pair(pair.clone());
        let foreign_shared_plane = shared_plane_from_pair(other_pair);

        let mismatched = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                foreign_shared_plane.shared_plane_identified_request_identity(),
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("certification-only foreign shared-plane evidence must fail before identity matching"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );
    });
}

#[test]
fn worth_workload_rejects_manual_counterless_and_support_mismatched_shared_plane_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 hostile shared plane evidence")
            .build()
            .expect("clean planar body pair should build");
        let shared_plane = shared_plane_from_pair(pair.clone());

        let manual = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                shared_plane.shared_plane_identified_request_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("manual shared-plane row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );

        let counterless = rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                shared_plane.shared_plane_identified_request_identity(),
                WorkloadEvidenceStageCounters::default(),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("certification-only counterless shared-plane row must fail before counter matching"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );

        let unsupported = rebuild_left_workload(
            &pair,
            vec![certification_only_unsupported_stage_row(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                shared_plane.shared_plane_identified_request_identity(),
                WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("certification-only support-mismatched shared-plane row must fail before support matching"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );
    });
}

#[test]
fn shared_plane_stage_counters_count_only_real_receipt_backed_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 shared plane counters")
            .build()
            .expect("clean planar body pair should build");
        let shared_plane = shared_plane_from_pair(pair.clone());
        let workload = rebuild_left_workload(
            &pair,
            vec![
                certification_only_admitted_stage_row(
                    WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
                    shared_plane.shared_plane_identified_request_identity(),
                    WorkloadEvidenceStageCounters::boolean_shared_plane_identity(),
                ),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            ],
        );

        assert_eq!(
            complete_ledger_stage_snapshot(
                workload.evidence_ledger(),
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity,
            )
            .expect("shared-plane row should exist")
            .counters()
            .boolean_shared_plane_identity_count(),
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

fn shared_plane_from_pair(
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest {
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
                "phase7.1 shared plane evidence basis",
            ),
            "phase7.1 shared plane evidence basis",
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query("phase7.1 shared plane evidence declaration")
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
    let precision =
        PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(posture)
            .expect("precision agreement should certify");
    let identity_receipt = PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(
        precision
            .posture_agreed_request()
            .plane_agreed_request()
            .agreement_receipt(),
    );
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_parts(precision, identity_receipt)
        .expect("shared-plane identity should certify")
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
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger: ledger,
    })
    .expect("left workload should re-compose with shared-plane evidence rows")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-shared-plane-evidence".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("shared-plane evidence contract thread should spawn")
        .join()
        .expect("shared-plane evidence contract thread should finish");
}
