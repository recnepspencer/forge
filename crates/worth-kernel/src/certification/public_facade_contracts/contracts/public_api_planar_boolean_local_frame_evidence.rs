use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest, PlanarBooleanCommonPlanePlaneAgreedRequest,
    PlanarBooleanCommonPlanePostureAgreedRequest, PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReductionRequest, PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

#[test]
fn worth_workload_requires_real_local_frame_selection_evidence() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local-frame evidence")
            .build()
            .expect("clean planar body pair should build");
        let local_frame = local_frame_from_pair(pair.clone());
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_local_frame_selection(&local_frame)
                .expect_err("bare workload must reject missing local-frame selection evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanLocalFrameSelection
            )
        );

        let admitted = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &local_frame,
            )],
        );
        admitted
            .require_boolean_local_frame_selection(&local_frame)
            .expect("real local-frame selection evidence should pass");
        assert_eq!(
            admitted
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanLocalFrameSelection)
                .expect("local-frame row should exist")
                .counters()
                .boolean_local_frame_selection_count(),
            1
        );
    });
}

#[test]
fn worth_workload_rejects_foreign_local_frame_selection_evidence_row() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local-frame evidence left")
            .build()
            .expect("first clean planar body pair should build");
        let other_pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local-frame evidence right")
            .build()
            .expect("second clean planar body pair should build");
        let local_frame = local_frame_from_pair(pair.clone());
        let foreign_local_frame = local_frame_from_pair(other_pair);

        let mismatched = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &foreign_local_frame,
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_local_frame_selection(&local_frame)
                .expect_err("foreign local-frame selection evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
                WorkloadEvidenceStage::BooleanLocalFrameSelection
            )
        );
    });
}

#[test]
fn worth_workload_rejects_manual_counterless_and_support_mismatched_local_frame_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 hostile local-frame evidence")
            .build()
            .expect("clean planar body pair should build");
        let local_frame = local_frame_from_pair(pair.clone());

        let manual = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanLocalFrameSelection,
                local_frame.local_frame_selection_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_local_frame_selection(&local_frame)
                .expect_err("manual local-frame row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanLocalFrameSelection
            )
        );

        let counterless = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &CounterlessLocalFrameEvidence::new(&local_frame),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_local_frame_selection(&local_frame)
                .expect_err("counterless local-frame row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanLocalFrameSelection
            )
        );

        let unsupported = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &SupportMismatchedLocalFrameEvidence::new(&local_frame),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_local_frame_selection(&local_frame)
                .expect_err("support-mismatched local-frame row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanLocalFrameSelection
            )
        );
    });
}

#[test]
fn local_frame_stage_counters_count_only_real_receipt_backed_rows() {
    run_with_large_stack(|| {
        let pair = WorkloadCatalog::planar_boolean_clean_planar_body_pair()
            .declared("phase7.1 local-frame counters")
            .build()
            .expect("clean planar body pair should build");
        let local_frame = local_frame_from_pair(pair.clone());
        let workload = rebuild_left_workload(
            &pair,
            vec![
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&local_frame),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            ],
        );

        assert_eq!(
            workload
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanLocalFrameSelection)
                .expect("local-frame row should exist")
                .counters()
                .boolean_local_frame_selection_count(),
            1
        );
        assert_eq!(
            workload
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanSplit)
                .expect("manual split row should exist")
                .counters()
                .total_receipt_backed_counters(),
            0
        );
    });
}

struct CounterlessLocalFrameEvidence {
    digest: String,
}

impl CounterlessLocalFrameEvidence {
    fn new(local_frame: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest) -> Self {
        Self {
            digest: local_frame.local_frame_selection_identity().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessLocalFrameEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LocalFrameSelection
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::default()
    }
}

struct SupportMismatchedLocalFrameEvidence {
    digest: String,
}

impl SupportMismatchedLocalFrameEvidence {
    fn new(local_frame: &PlanarBooleanCommonPlaneLocalFrameSelectedRequest) -> Self {
        Self {
            digest: local_frame.local_frame_selection_identity().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedLocalFrameEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::LocalFrameSelection
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_local_frame_selection()
    }
}

fn local_frame_from_pair(
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
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
                "phase7.1 local-frame evidence basis",
            ),
            "phase7.1 local-frame evidence basis",
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query("phase7.1 local-frame evidence declaration")
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
    let shared_plane =
        PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
            precision,
        )
        .expect("shared-plane identity should certify");
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
        shared_plane,
    )
    .expect("local-frame selection should certify")
}

fn rebuild_left_workload(
    pair: &worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
    boolean_rows: Vec<WorkloadEvidenceRow>,
) -> WorthWorkload {
    let left = pair.left().workload();
    let mut rows = left.evidence_ledger().rows().to_vec();
    rows.extend(boolean_rows);
    let ledger = WorkloadEvidenceLedger::from_rows(rows)
        .expect("local-frame evidence rows should stay inspectable")
        .certify_complete()
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
    .expect("left workload should re-compose with local-frame evidence rows")
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-local-frame-evidence".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("local-frame evidence contract thread should spawn")
        .join()
        .expect("local-frame evidence contract thread should finish");
}
