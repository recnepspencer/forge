use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest, PlanarBooleanCommonPlanePlaneAgreedRequest,
    PlanarBooleanCommonPlanePostureAgreedRequest, PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReductionRequest, PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog, WorthWorkload,
    WorthWorkloadParts,
};
use worth_spatial::certification::workload_evidence::complete_ledger_with_additional_rows;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;
use worth_kernel::workload_composition::trace_scope;

pub(crate) fn projected_operand_requests(
    readiness_scope: &'static str,
) -> (
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let (_, operand_a, operand_b) = projected_operand_requests_from_catalog(readiness_scope);
    (operand_a, operand_b)
}

pub(crate) fn projected_operand_requests_from_catalog(
    readiness_scope: &'static str,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let pair = build_admitted_operand_pair(readiness_scope);
    let declaration = bind_boolean_declaration(readiness_scope, &pair);
    let local_frame = select_common_plane_local_frame(declaration, pair.clone());

    let operand_a =
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A should certify");
    let operand_b =
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B should certify");

    (pair, operand_a, operand_b)
}

pub(crate) fn projected_operand_requests_from_pair(
    readiness_scope: &'static str,
    pair: BuiltBooleanOperandPairRecipe,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let declaration = bind_boolean_declaration(readiness_scope, &pair);
    let local_frame = select_common_plane_local_frame(declaration, pair.clone());
    let operand_a =
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A should certify");
    let operand_b =
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B should certify");

    (pair, operand_a, operand_b)
}

pub(crate) fn event_carrier_projected_operand_requests_from_catalog(
    readiness_scope: &'static str,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let pair = trace_scope("event_carrier_pair_build", || {
        WorkloadCatalog::planar_boolean_event_carrier_clean_planar_body_pair()
            .with_retained_replay_artifacts()
            .declared(readiness_scope)
            .build()
            .expect("event carrier pair should build")
    });
    let declaration = trace_scope("event_carrier_boolean_declaration", || {
        bind_boolean_declaration(readiness_scope, &pair)
    });
    let local_frame = trace_scope("event_carrier_local_frame_selection", || {
        select_common_plane_local_frame(declaration, pair.clone())
    });

    let operand_a = trace_scope("event_carrier_operand_a_projection", || {
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A should certify")
    });
    let operand_b = trace_scope("event_carrier_operand_b_projection", || {
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B should certify")
    });

    (pair, operand_a, operand_b)
}

pub(crate) fn metaboss_projected_operand_requests_from_catalog(
    readiness_scope: &'static str,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let pair = trace_scope("metaboss_pair_build", || {
        WorkloadCatalog::planar_boolean_event_extraction_metaboss_pair()
            .declared(readiness_scope)
            .build()
            .expect("metaboss event extraction pair should build")
    });
    let declaration = trace_scope("metaboss_boolean_declaration", || {
        bind_boolean_declaration(readiness_scope, &pair)
    });
    let local_frame = trace_scope("metaboss_local_frame_selection", || {
        select_common_plane_local_frame(declaration, pair.clone())
    });

    let operand_a = trace_scope("metaboss_operand_a_projection", || {
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A should certify")
    });
    let operand_b = trace_scope("metaboss_operand_b_projection", || {
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B should certify")
    });

    (pair, operand_a, operand_b)
}

fn build_admitted_operand_pair(readiness_scope: &'static str) -> BuiltBooleanOperandPairRecipe {
    WorkloadCatalog::planar_boolean_clean_planar_body_pair()
        .declared(readiness_scope)
        .build()
        .expect("pair should build")
}

fn bind_boolean_declaration(
    readiness_scope: &'static str,
    pair: &BuiltBooleanOperandPairRecipe,
) -> worth_kernel::workload_composition::PlanarBooleanDeclarationReceipt {
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand-pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            entry_support::certified_boolean_readiness_workload_receipt_from_ledger(
                readiness_scope,
                pair.left().workload().evidence_ledger().clone(),
            ),
            format!("{readiness_scope} basis"),
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query(format!("{readiness_scope} declaration"))
    .bind()
    .expect("boolean declaration should bind");
    entry_support::assert_planar_boolean_query_digest(declaration.basis_query_declaration_digest());
    entry_support::assert_planar_boolean_query_digest(declaration.query_declaration_digest());
    declaration
}

fn select_common_plane_local_frame(
    declaration: worth_kernel::workload_composition::PlanarBooleanDeclarationReceipt,
    pair: BuiltBooleanOperandPairRecipe,
) -> PlanarBooleanCommonPlaneLocalFrameSelectedRequest {
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
        PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
            PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
                PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
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
                .expect("posture agreement should certify"),
            )
            .expect("precision agreement should certify"),
        )
        .expect("shared-plane identity should certify"),
    )
    .expect("local-frame selection should certify")
}

pub(crate) fn rebuild_left_workload(
    pair: &BuiltBooleanOperandPairRecipe,
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
    .expect("left workload should re-compose with reduced-pair evidence rows")
}

pub(crate) fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-reduced-pair-request".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("reduced-pair request contract thread should spawn")
        .join()
        .expect("reduced-pair request contract thread should finish");
}

fn run_with_large_stack_anchor() {
    run_with_large_stack(|| {});
}

const _: () = {
    let _ = projected_operand_requests;
    let _ = projected_operand_requests_from_catalog;
    let _ = projected_operand_requests_from_pair;
    let _ = event_carrier_projected_operand_requests_from_catalog;
    let _ = metaboss_projected_operand_requests_from_catalog;
    let _ = rebuild_left_workload;
};
const _: fn() = run_with_large_stack_anchor;
