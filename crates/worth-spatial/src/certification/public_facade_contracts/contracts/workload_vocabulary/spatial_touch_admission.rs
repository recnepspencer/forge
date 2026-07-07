use std::marker::PhantomData;

use crate::query_contract_helpers::aspect_touch;
use forge_query::facade::ForgeQueryGraphTouchDescriptor;
use topology::facade::{TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphBasis};
use worth_kernel::workload_composition::{
    PlanarBooleanCommonPlaneLocalFrameSelectedRequest,
    PlanarBooleanCommonPlaneOperandAProjectedRequest,
    PlanarBooleanCommonPlaneOperandBProjectedRequest, PlanarBooleanCommonPlanePlaneAgreedRequest,
    PlanarBooleanCommonPlanePostureAgreedRequest, PlanarBooleanCommonPlanePrecisionAgreedRequest,
    PlanarBooleanCommonPlaneReducedOperandPairRequest, PlanarBooleanCommonPlaneReductionRequest,
    PlanarBooleanCommonPlaneScopeAdmittedRequest,
    PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest, PlanarBooleanDeclaration,
    PlanarBooleanEntryBasis, PlanarBooleanExecutionLane, PlanarBooleanFamily,
    PlanarBooleanOperandPairIdentity, PlanarBooleanOperation, WorkloadCatalog,
    WorkloadCompositionError, WorthWorkload, WorthWorkloadParts,
};
use worth_spatial::facade::workload_vocabulary::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_manual_evidence_row_as_spatial_touch_authority,
    deny_query_descriptor_as_spatial_query_lowering_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    deny_topology_touched_graph_basis_as_spatial_touch_authority, BooleanEvidenceReceipt,
    BooleanEvidenceStageKind, SpatialEvidenceQueryLoweringDenialKind,
    SpatialEvidenceSubstitutionDenial, SpatialEvidenceTopologySubstitutionSurface,
    SpatialGeometryEvidenceTouchDenialKind, SpatialGeometryEvidenceTouchRequest,
    WorkloadEvidenceBacking, WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceSupport,
};

#[path = "../../../../../../worth-kernel/src/certification/public_facade_contracts/contracts/public_api_planar_boolean_entry/tests/support.rs"]
mod planar_boolean_entry_support;

#[test]
fn spatial_touch_facade_and_kernel_entrypoint_use_canonical_admission() {
    run_with_large_stack(|| {
        let (workload, receipt) = executable_spatial_touch_contract_subject();

        let facade_authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
            .with_complete_ledger(workload.evidence_ledger())
            .admit()
            .expect("facade request must admit real receipt plus complete ledger");
        let kernel_authority = workload
            .admit_spatial_geometry_evidence_touch(&receipt)
            .expect("kernel workload must admit through the same canonical contract");

        assert_eq!(facade_authority, kernel_authority);
        assert_eq!(
            facade_authority.boolean_stage(),
            BooleanEvidenceStageKind::SegmentPairEnumeration
        );
        assert_eq!(
            facade_authority.evidence_stage(),
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration
        );
        assert_eq!(
            facade_authority.evidence_identity(),
            receipt.segment_pair_enumeration_identity()
        );
        assert_eq!(
            facade_authority.support(),
            WorkloadEvidenceSupport::Admitted
        );
        assert_eq!(
            facade_authority.evidence_counters(),
            receipt.evidence_counters()
        );
        assert_eq!(
            facade_authority.stage_index_identity(),
            workload.evidence_ledger().stage_index().index_identity()
        );
        assert!(workload
            .evidence_ledger()
            .link_required_stages(&[WorkloadEvidenceStage::BooleanSegmentPairEnumeration])
            .expect("complete workload must link the admitted boolean stage")
            .links_to_identity(
                WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                receipt.segment_pair_enumeration_identity(),
            ));
        assert_eq!(
            facade_authority.digest().as_str(),
            kernel_authority.digest().as_str()
        );
    });
}

#[test]
fn spatial_touch_facade_and_kernel_denials_preserve_typed_blocking_contract() {
    run_with_large_stack(|| {
        let (workload, receipt) = executable_spatial_touch_denial_contract_subject();

        let facade_denial = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&receipt)
            .with_complete_ledger(workload.evidence_ledger())
            .admit()
            .expect_err("facade request must deny missing receipt-backed ledger row");
        let kernel_denial = workload
            .admit_spatial_geometry_evidence_touch(&receipt)
            .expect_err("kernel workload must preserve the same spatial denial");

        let WorkloadCompositionError::SpatialTouchAuthority(kernel_spatial_denial) = kernel_denial
        else {
            panic!("kernel denial must carry typed spatial touch denial");
        };

        assert_eq!(facade_denial.kind(), kernel_spatial_denial.kind());
        assert_eq!(
            facade_denial.kind(),
            SpatialGeometryEvidenceTouchDenialKind::LedgerIncompleteness
        );
        assert_eq!(facade_denial.locality(), kernel_spatial_denial.locality());
        assert_eq!(facade_denial.detail(), kernel_spatial_denial.detail());
    });
}

pub(crate) fn executable_spatial_touch_contract_subject() -> (
    WorthWorkload,
    worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt,
) {
    let pair = WorkloadCatalog::planar_boolean_event_carrier_clean_planar_body_pair()
        .with_retained_replay_artifacts()
        .declared("phase2 spatial touch executable contract")
        .build()
        .expect("catalog pair should build a real workload");
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            planar_boolean_entry_support::certified_boolean_readiness_workload_receipt(
                "phase2 spatial touch executable contract",
            ),
            "phase2 spatial touch executable contract basis",
        )
        .expect("boolean entry basis should certify"),
    )
    .declared_by_query("phase2 spatial touch executable contract declaration")
    .bind()
    .expect("boolean declaration should bind");
    let local_frame =
        PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
            PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
                PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
                    PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
                        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                                PlanarBooleanCommonPlaneReductionRequest::from_declaration_receipt_and_operand_pair(
                                    declaration,
                                    pair.clone(),
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
        .expect("local-frame selection should certify");
    let operand_a =
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A projection should certify");
    let operand_b =
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B projection should certify");
    let receipt =
        PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
            operand_a, operand_b,
        )
        .expect("reduced pair should certify")
        .segment_carrier_set()
        .expect("segment carrier set should certify")
        .canonical_segment_set()
        .expect("canonical segment set should certify")
        .segment_pair_enumeration_receipt()
        .expect("segment-pair receipt should certify");
    let left = pair.left().workload();
    let evidence_ledger = left
        .evidence_ledger()
        .with_boolean_evidence_receipt(&receipt)
        .expect("real receipt row should preserve complete-ledger proof");
    let workload = WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger,
    })
    .expect("workload should compose with receipt-backed boolean evidence");
    (workload, receipt)
}

pub(crate) fn executable_spatial_touch_denial_contract_subject() -> (
    WorthWorkload,
    worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt,
) {
    let pair = WorkloadCatalog::planar_boolean_event_carrier_clean_planar_body_pair()
        .with_retained_replay_artifacts()
        .declared("phase2 spatial touch denial contract")
        .build()
        .expect("catalog pair should build a real workload");
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            planar_boolean_entry_support::certified_boolean_readiness_workload_receipt(
                "phase2 spatial touch denial contract",
            ),
            "phase2 spatial touch denial contract basis",
        )
        .expect("boolean entry basis should certify"),
    )
    .declared_by_query("phase2 spatial touch denial contract declaration")
    .bind()
    .expect("boolean declaration should bind");
    let local_frame =
        PlanarBooleanCommonPlaneLocalFrameSelectedRequest::from_shared_plane_identified_request(
            PlanarBooleanCommonPlaneSharedPlaneIdentifiedRequest::from_precision_agreed_request(
                PlanarBooleanCommonPlanePrecisionAgreedRequest::from_posture_agreed_request(
                    PlanarBooleanCommonPlanePostureAgreedRequest::from_plane_agreed_request(
                        PlanarBooleanCommonPlanePlaneAgreedRequest::from_scope_admitted_request(
                            PlanarBooleanCommonPlaneScopeAdmittedRequest::from_reduction_request(
                                PlanarBooleanCommonPlaneReductionRequest::from_declaration_receipt_and_operand_pair(
                                    declaration,
                                    pair.clone(),
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
        .expect("local-frame selection should certify");
    let operand_a =
        PlanarBooleanCommonPlaneOperandAProjectedRequest::from_local_frame_selected_request(
            local_frame.clone(),
        )
        .expect("operand A projection should certify");
    let operand_b =
        PlanarBooleanCommonPlaneOperandBProjectedRequest::from_local_frame_selected_request(
            local_frame,
        )
        .expect("operand B projection should certify");
    let receipt =
        PlanarBooleanCommonPlaneReducedOperandPairRequest::from_operand_projection_requests(
            operand_a, operand_b,
        )
        .expect("reduced pair should certify")
        .segment_carrier_set()
        .expect("segment carrier set should certify")
        .canonical_segment_set()
        .expect("canonical segment set should certify")
        .segment_pair_enumeration_receipt()
        .expect("segment-pair receipt should certify");
    let left = pair.left().workload();
    let workload = WorthWorkload::compose(WorthWorkloadParts {
        topology: left.topology().clone(),
        geometry_binding: left.geometry_binding().clone(),
        surface_support: left.surface_support().clone(),
        projection: left.projection().clone(),
        transform: left.transform().clone(),
        retained_replay: left.retained_replay().clone(),
        batch_admission_execution: left.batch_admission_execution().cloned(),
        diagnostics: left.diagnostics().clone(),
        response: left.response().clone(),
        evidence_ledger: left.evidence_ledger().clone(),
    })
    .expect("workload should compose without optional boolean receipt row");
    (workload, receipt)
}

pub(crate) fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("spatial-touch-contract-public-boundary".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("contract thread should spawn")
        .join()
        .expect("contract thread should finish");
}

#[test]
fn spatial_touch_public_denials_name_non_authority_source_families() {
    let row = WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split");
    assert_eq!(
        deny_manual_evidence_row_as_spatial_touch_authority(&row),
        Err(SpatialEvidenceSubstitutionDenial::ManualEvidenceRow {
            stage: WorkloadEvidenceStage::BooleanSplit,
            backing: WorkloadEvidenceBacking::Manual,
        })
    );
    assert_eq!(
        deny_topology_touched_graph_basis_as_spatial_touch_authority(
            PhantomData::<TopologyTouchedGraphBasis>
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface: SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis,
        }
    );
    assert_eq!(
        deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority(
            PhantomData::<TopologyDeclaredTouchedGraphBasisProof>
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface:
                SpatialEvidenceTopologySubstitutionSurface::TopologyDeclaredTouchedGraphBasisProof,
        }
    );

    let query_descriptor = ForgeQueryGraphTouchDescriptor::read_family_shape(
        "spatial-evidence-touch",
        [forge_query::facade::runtime::ForgeQueryGraphTouchReadVerb::ObservesCollection],
        forge_query::facade::runtime::ForgeQueryGraphReadTouchShape::new([aspect_touch(
            "workload.evidence.boolean.segment_pair_enumeration",
        )]),
    )
    .expect("query descriptor fixture should build for denial assertion");

    let query_lowering_denials = [
        deny_raw_row_as_spatial_query_lowering_authority("WorkloadEvidenceRow"),
        deny_topology_touched_basis_as_spatial_query_lowering_authority(
            "TopologyTouchedGraphBasis",
        ),
        deny_query_descriptor_as_spatial_query_lowering_authority(&query_descriptor),
        deny_copied_receipt_fields_as_spatial_query_lowering_authority("copied receipt fields"),
    ];

    assert_eq!(
        query_lowering_denials[0].kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    assert_eq!(
        query_lowering_denials[1].kind(),
        SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution
    );
    assert_eq!(
        query_lowering_denials[2].kind(),
        SpatialEvidenceQueryLoweringDenialKind::QueryDescriptorSubstitution
    );
    assert_eq!(
        query_lowering_denials[3].kind(),
        SpatialEvidenceQueryLoweringDenialKind::CopiedReceiptSubstitution
    );
}
