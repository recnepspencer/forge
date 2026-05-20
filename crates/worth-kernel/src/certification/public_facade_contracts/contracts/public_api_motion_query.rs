use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
    prepare_primitive_construction_rotate_witness_resolution_report, MoveSpatialIntent,
    PrimitiveConstructionIntent, PrimitiveConstructionMotionQueryFactProvenance,
    PrimitiveConstructionMotionQueryReadSurface,
    PrimitiveConstructionMotionWitnessResolutionFailureKind, RotateSpatialIntent, WireBodySpec,
};

#[test]
fn kernel_public_facade_exports_query_backed_motion_witness_parity_reports() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api-motion-query".to_string(),
    )
    .expect("workspace");
    let move_report = prepare_primitive_construction_query_motion_inspection_parity_report(
        &mut workspace,
        prepare_primitive_construction_move_witness_resolution_report(
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .to([10.0, 0.0, 3.0]),
        ),
    )
    .expect("move query report");
    let rotate_report =
        prepare_primitive_construction_query_motion_projection_consumption_receipt_report(
            &mut workspace,
            prepare_primitive_construction_rotate_witness_resolution_report(
                RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 4,
                }))
                .around([0.0, 0.0, 1.0])
                .by_radians(f64::NAN),
            ),
        )
        .expect("rotate query report");

    assert_eq!(
        move_report.read_surface(),
        PrimitiveConstructionMotionQueryReadSurface::MotionWitnessReportInspection
    );
    assert_eq!(
        move_report.fact_provenance(),
        PrimitiveConstructionMotionQueryFactProvenance::DirectMotionWitnessReport
    );
    assert!(move_report.parity_verified());
    assert_eq!(
        rotate_report.read_surface(),
        PrimitiveConstructionMotionQueryReadSurface::ProjectionConsumptionFromMotionWitnessReport
    );
    assert_eq!(
        rotate_report.fact_provenance(),
        PrimitiveConstructionMotionQueryFactProvenance::EquivalentProjectionConsumptionFacts
    );
    assert_eq!(
        rotate_report.failure_kind(),
        Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::NonFiniteRotationAngle)
    );
    assert!(rotate_report.parity_verified());
}
