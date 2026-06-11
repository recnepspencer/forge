use super::contract_subject::{acceptance_transform_sequence, projected_cube_workload};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarReorientation, PlanarRotationPosture,
};
use worth_spatial::facade::transform_workload::{TransformEvidenceKind, TransformWorkload};
use worth_spatial::facade::workload_vocabulary::SpatialWorkloadStage;

#[test]
fn transform_workload_changes_geometry_and_records_posture() {
    let projected = projected_cube_workload("transform changes geometry");
    let projected_identity = projected.receipts().stage_identity().receipt_identity();
    let projected_face_identity = projected.projected_faces()[0]
        .identity()
        .projected_fact_identity()
        .to_string();
    let expected_transformed_entities = projected.projected_faces().len()
        + projected.projected_edges().edges().len()
        + projected.projected_loops().len();

    let transformed = TransformWorkload::for_projected_workload(projected)
        .declared("translate rotate reorient cancel")
        .with_transform_sequence(acceptance_transform_sequence())
        .transform()
        .expect("real transform evidence should admit");

    assert_eq!(
        transformed.receipts().stage_identity().stage(),
        SpatialWorkloadStage::Transform
    );
    assert_eq!(
        transformed
            .receipts()
            .stage_receipt()
            .identity()
            .upstream_receipt(),
        projected_identity
    );
    assert_eq!(
        transformed.receipts().projected_workload_identity(),
        projected_identity
    );
    assert_eq!(transformed.receipts().counters().transform_steps(), 4);
    assert_eq!(
        transformed.receipts().counters().changed_coordinate_rows(),
        2
    );
    assert_eq!(
        transformed.receipts().counters().transformed_entities(),
        expected_transformed_entities
    );
    assert_eq!(transformed.receipts().counters().evidence_rows(), 4);
    assert_eq!(transformed.receipts().counters().cancellation_steps(), 16);
    assert_eq!(transformed.evidence().changed_coordinate_rows(), 2);
    let evidence_kinds = transformed
        .evidence()
        .rows()
        .iter()
        .map(|row| row.kind())
        .collect::<Vec<_>>();
    assert_eq!(
        evidence_kinds,
        vec![
            TransformEvidenceKind::CoordinateChange,
            TransformEvidenceKind::CoordinateChange,
            TransformEvidenceKind::PostureChange,
            TransformEvidenceKind::CancellationReplay,
        ]
    );

    let posture = transformed.receipts().transform_posture_receipt();
    assert_eq!(posture.projected_workload_identity(), projected_identity);
    assert_eq!(
        posture.rotation_posture(),
        PlanarRotationPosture::ExactCancellation
    );
    assert_eq!(
        posture.reorientation(),
        PlanarReorientation::PreservesHandedness
    );
    assert_eq!(
        posture.cancellation(),
        PlanarMotionCancellation::ExactBasisReplay
    );
    assert!(posture
        .posture_identity()
        .contains(&transformed.receipts().stage_identity().receipt_identity()));
    assert!(posture
        .posture_identity()
        .contains(PlanarMotionCancellation::ExactBasisReplay.as_str()));

    let transformed_face_identity = transformed.transformed_faces()[0].identity();
    assert_eq!(
        transformed_face_identity.projected_fact_identity(),
        projected_face_identity
    );
    assert_eq!(
        transformed_face_identity.transform_evidence_identity(),
        transformed.receipts().transform_evidence_identity()
    );
    assert!(transformed_face_identity
        .transformed_fact_identity()
        .contains(&projected_face_identity));
    assert!(transformed.can_enter_retained_replay_workload());
    assert!(!transformed.can_enter_operator_execution_without_projection_consumption());
}
