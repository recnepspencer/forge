use worth_kernel::workload_composition::{
    TransformRecipe, WorkloadCatalog, WorkloadTopologyBreadth,
};
use worth_spatial::facade::projected_overlap_faces::{
    CertifiedProjectedOverlapFaceSet, CoplanarOverlapExtractionBundle, ProjectedOverlapFaceSet,
};
use worth_spatial::facade::workload_certification_context::{
    WorkloadCertificationContext, WorkloadCertificationContextDenialKind, WorkloadMotionAdversary,
    WorkloadMotionBinding, WorkloadPrecisionPolicy,
};

use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::context_contracts;

#[test]
fn workload_certification_context_derives_authority_from_projected_workload_and_transform() {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 })
        .declared("context derives storm authority from workload receipts")
        .build()
        .expect("context source workload should build");

    let context = WorkloadCertificationContext::from_projected_workload(built.projected_workload())
        .with_transform_receipts(built.transform_receipts())
        .with_precision_policy(WorkloadPrecisionPolicy::LocalFeatureScale)
        .compile(context_contracts("context-derives-authority"))
        .expect("context should certify from real projected workload");

    assert_eq!(
        context.projection_stage_identity(),
        built
            .projected_workload()
            .receipts()
            .stage_identity()
            .receipt_identity()
    );
    assert_eq!(
        context.movement_rotation_posture_identity(),
        built
            .transform_receipts()
            .transform_posture_receipt()
            .posture_identity()
    );
    assert_eq!(
        context.precision_receipt().basis().local_frame_identity(),
        context.frame_identity()
    );
    assert_eq!(
        context
            .precision_receipt()
            .basis()
            .topology_basis_identity(),
        context.topology_neighborhood_identity()
    );
    assert_eq!(
        context
            .local_frame_receipt()
            .basis()
            .transform_chain_digest(),
        built
            .transform_receipts()
            .stage_identity()
            .receipt_identity()
    );
    assert!(!context.analysis_surface().surface_identity().is_empty());
    assert!(!context.context_identity().is_empty());
}

#[test]
fn workload_certification_context_rejects_transform_receipts_from_another_projection() {
    let first = WorkloadCatalog::coplanar_overlap_storm()
        .declared("context mismatch first workload")
        .build()
        .expect("first context source should build");
    let second = WorkloadCatalog::coplanar_overlap_storm()
        .declared("context mismatch second workload")
        .build()
        .expect("second context source should build");

    let result = WorkloadCertificationContext::from_projected_workload(first.projected_workload())
        .with_transform_receipts(second.transform_receipts())
        .compile(context_contracts("context-rejects-mismatched-transform"));
    let denial = match result {
        Ok(_) => panic!("context must reject transform receipts from another projection"),
        Err(denial) => denial,
    };

    assert!(denial.reason().contains("same projected workload"));
    assert!(!denial.reason().contains('_'));
    assert_eq!(
        denial.kind(),
        WorkloadCertificationContextDenialKind::MismatchedTransformReceipts
    );
}

#[test]
fn workload_certification_context_rebinding_changes_motion_without_reauthoring_basis() {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_transform(TransformRecipe::MovementRotationStack)
        .declared("context adversarial motion rebind")
        .build()
        .expect("motion rebind source should build");
    let context = WorkloadCertificationContext::from_projected_workload(built.projected_workload())
        .with_transform_receipts(built.transform_receipts())
        .compile(context_contracts("context-motion-rebind"))
        .expect("base context should certify");
    let adversarial = context
        .with_motion_binding(WorkloadMotionBinding::adversarial_for_context(
            &context,
            WorkloadMotionAdversary::TinyRotationExitsCoplanarClass,
        ))
        .expect("adversarial context should stay bound to source projection");

    assert_eq!(
        adversarial.projection_stage_identity(),
        context.projection_stage_identity()
    );
    assert_ne!(
        adversarial.movement_rotation_posture_identity(),
        context.movement_rotation_posture_identity()
    );
    assert_ne!(adversarial.context_identity(), context.context_identity());

    let projected_faces = ProjectedOverlapFaceSet::from_context(&context)
        .expect("context should expose projected overlap faces");
    let certified_faces =
        CertifiedProjectedOverlapFaceSet::from_projected_faces(projected_faces, &context)
            .expect("context-certified faces should compile");
    let bundle = CoplanarOverlapExtractionBundle::from_context_candidate_pairs(
        certified_faces.candidate_pairs(),
        &context,
    )
    .expect("context-certified candidate pairs should extract");
    assert_eq!(bundle.context_identity(), context.context_identity());
    assert_eq!(
        bundle.candidate_pair_count(),
        certified_faces.candidate_pair_count()
    );
}
