use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlanePlaneAgreedRequest, PlanarBooleanCommonPlanePostureAgreedRequest,
    PlanarBooleanCommonPlanePrecisionAgreedRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorkloadStageRequirement, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
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

        let admitted = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &shared_plane,
            )],
        );
        admitted
            .require_boolean_shared_plane_identity(&shared_plane)
            .expect("real shared-plane identity evidence should pass");
        assert_eq!(
            admitted
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanSharedPlaneIdentity)
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
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &foreign_shared_plane,
            )],
        );
        assert_eq!(
            mismatched
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("foreign shared-plane identity evidence must fail"),
            WorkloadCompositionError::MismatchedEvidenceStage(
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
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &CounterlessSharedPlaneEvidence::new(&shared_plane),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("counterless shared-plane row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanSharedPlaneIdentity
            )
        );

        let unsupported = rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
                &SupportMismatchedSharedPlaneEvidence::new(&shared_plane),
            )],
        );
        assert_eq!(
            unsupported
                .require_boolean_shared_plane_identity(&shared_plane)
                .expect_err("support-mismatched shared-plane row must fail"),
            WorkloadCompositionError::UnsupportedStage(
                WorkloadStageRequirement::BooleanSharedPlaneIdentity
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
                WorkloadEvidenceRow::from_boolean_evidence_receipt(&shared_plane),
                WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split"),
            ],
        );

        assert_eq!(
            workload
                .evidence_ledger()
                .row_for_stage(WorkloadEvidenceStage::BooleanSharedPlaneIdentity)
                .expect("shared-plane row should exist")
                .counters()
                .boolean_shared_plane_identity_count(),
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

struct CounterlessSharedPlaneEvidence {
    digest: String,
}

impl CounterlessSharedPlaneEvidence {
    fn new(shared_plane: &PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest) -> Self {
        Self {
            digest: shared_plane
                .shared_plane_identified_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessSharedPlaneEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SharedPlaneIdentity
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

impl BooleanEvidenceRowAuthority for CounterlessSharedPlaneEvidence {}

struct SupportMismatchedSharedPlaneEvidence {
    digest: String,
}

impl SupportMismatchedSharedPlaneEvidence {
    fn new(shared_plane: &PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest) -> Self {
        Self {
            digest: shared_plane
                .shared_plane_identified_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedSharedPlaneEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SharedPlaneIdentity
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_shared_plane_identity()
    }
}

impl BooleanEvidenceRowAuthority for SupportMismatchedSharedPlaneEvidence {}

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
    let mut rows = left.evidence_ledger().rows().to_vec();
    rows.extend(boolean_rows);
    let ledger = WorkloadEvidenceLedger::from_rows(rows)
        .expect("shared-plane evidence rows should stay inspectable")
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
