#![allow(dead_code)]
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
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
mod entry_support;

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

pub(crate) fn event_carrier_projected_operand_requests_from_catalog(
    readiness_scope: &'static str,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let pair = WorkloadCatalog::planar_boolean_event_carrier_clean_planar_body_pair()
        .declared(readiness_scope)
        .build()
        .expect("event carrier pair should build");
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

pub(crate) fn metaboss_projected_operand_requests_from_catalog(
    readiness_scope: &'static str,
) -> (
    BuiltBooleanOperandPairRecipe,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest,
) {
    let pair = WorkloadCatalog::planar_boolean_event_extraction_metaboss_pair()
        .declared(readiness_scope)
        .build()
        .expect("metaboss event extraction pair should build");
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
            entry_support::certified_boolean_readiness_workload_receipt(readiness_scope),
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
    let mut rows = left.evidence_ledger().rows().to_vec();
    rows.extend(boolean_rows);
    let ledger = WorkloadEvidenceLedger::from_rows(rows)
        .expect("reduced-pair evidence rows should stay inspectable")
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
