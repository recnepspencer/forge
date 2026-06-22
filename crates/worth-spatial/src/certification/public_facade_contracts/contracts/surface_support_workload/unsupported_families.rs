use topology::facade::TopologySeed;
use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportStatus, SurfaceSupportWorkload,
    UnsupportedSurfaceSupportReasonCode,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::{SpatialWorkloadStage, WorkloadStageSupport};

#[test]
fn surface_support_workload_certifies_planes_and_denies_unadmitted_families() {
    for family in [
        SurfaceFamily::AnalyticNonPlanar,
        SurfaceFamily::Freeform,
        SurfaceFamily::GeneratedFeature,
        SurfaceFamily::Unknown,
    ] {
        let bound_geometry = bound_cube_geometry(family.human_label());
        let geometry_receipt_identity = bound_geometry
            .receipts()
            .stage_identity()
            .receipt_identity();
        let upstream_carriers = bound_geometry.receipts().counters().geometry_carriers();
        let unsupported = SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
            .declared(format!("classify {}", family.human_label()))
            .with_surface_family(family)
            .certify()
            .expect_err("non-plane families cannot certify in M6.5");

        assert_eq!(unsupported.family(), Some(family));
        assert_eq!(
            unsupported.reason_code(),
            UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted
        );
        assert_eq!(
            unsupported.human_reason(),
            format!(
                "{} is not admitted for M6.5 surface support.",
                family.human_label()
            )
        );
        assert_eq!(
            unsupported.posture().support(),
            WorkloadStageSupport::Unsupported
        );
        assert!(unsupported.upstream_geometry_binding_identity().is_some());
        assert!(unsupported.topology_query_surface().is_some());
        let receipt = unsupported
            .receipt()
            .expect("unsupported family denial should still produce a surface support receipt");
        assert_eq!(
            receipt.stage_identity().stage(),
            SpatialWorkloadStage::SurfaceSupport
        );
        assert_eq!(
            receipt.stage_identity().upstream_receipt(),
            geometry_receipt_identity
        );
        assert_eq!(
            receipt.envelope().posture().reason(),
            unsupported.human_reason()
        );
        assert_eq!(receipt.family(), Some(family));
        assert_eq!(
            receipt.reason_code(),
            UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted
        );
        assert_eq!(receipt.matrix_rows(), unsupported.matrix_rows());
        assert_eq!(receipt.counters().classified_families(), 5);
        assert_eq!(receipt.counters().certified_planes(), 1);
        assert_eq!(receipt.counters().unsupported_families(), 4);
        assert_eq!(
            receipt.counters().upstream_geometry_carriers(),
            upstream_carriers
        );
        assert_eq!(unsupported.matrix_rows().len(), 5);
        assert_all_surface_families_are_classified(unsupported.matrix_rows());
        assert!(unsupported.matrix_rows().iter().any(|row| {
            row.family() == family && row.status() == SurfaceSupportStatus::Unsupported
        }));
        assert!(!unsupported.can_enter_local_frame_workload());
        assert!(!unsupported.can_enter_projection_workload());
        assert!(!unsupported.can_enter_operator_execution());
    }
}

#[test]
fn surface_support_workload_blocks_future_family_stubs() {
    let unsupported =
        SurfaceSupportWorkload::for_bound_geometry(bound_cube_geometry("generated feature denial"))
            .declared("try generated feature support")
            .with_surface_family(SurfaceFamily::GeneratedFeature)
            .certify()
            .expect_err("generated features must be typed unsupported");

    assert_eq!(unsupported.family(), Some(SurfaceFamily::GeneratedFeature));
    assert_eq!(
        unsupported.reason_code(),
        UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted
    );
    assert_eq!(
        unsupported.human_reason(),
        "generated feature surface is not admitted for M6.5 surface support."
    );
    assert!(unsupported.receipt().is_some());
    assert!(!unsupported.can_enter_local_frame_workload());
    assert!(!unsupported.can_enter_projection_workload());
    assert!(!unsupported.can_enter_operator_execution());
}

fn assert_all_surface_families_are_classified(
    matrix_rows: &[worth_spatial::facade::surface_support::SurfaceSupportMatrixRow],
) {
    for family in SurfaceFamily::ALL {
        let expected_status = if family == SurfaceFamily::Plane {
            SurfaceSupportStatus::Certified
        } else {
            SurfaceSupportStatus::Unsupported
        };
        assert!(matrix_rows
            .iter()
            .any(|row| row.family() == family && row.status() == expected_status));
    }
}

fn bound_cube_geometry(declaration: &str) -> BoundGeometryWorkload {
    let topology = TopologySeed::cube()
        .with_declaration(declaration)
        .build()
        .expect("cube topology seed should be admitted");

    GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit")
}
