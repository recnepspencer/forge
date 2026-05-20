use crate::construction::admission::AdmittedPrimitiveConstructionGeometry;
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{
    PrimitiveConstructionFamily, PrimitiveConstructionGeometryError,
    PrimitiveConstructionPhaseError,
};
use crate::construction::scaffold::PrimitiveConstructionScaffold;
use crate::construction::scaffold_geometry::{
    orthotope_vertices, planar_support_plane, prism_vertices, pyramid_vertices,
    shell_with_hole_vertices, simplex_vertices, wire_body_vertices,
};
use crate::construction::topology_counts::PrimitiveConstructionTopologyCounts;
use worth_geom::facade::{
    build_direct_realization_report, realize_block_support, realize_prism_support,
    realize_pyramid_support, realize_tetrahedron_support,
};
use worth_spatial::facade::{apply_spatial_placement, SpatialPlacementError};

pub(crate) fn build_admitted_scaffold(
    family: PrimitiveConstructionFamily,
    request_digest: &str,
    intent_digest: &str,
    geometry: &AdmittedPrimitiveConstructionGeometry,
) -> Result<PrimitiveConstructionScaffold, PrimitiveConstructionPhaseError> {
    let (support_planes, realization_report, vertex_positions, topology_counts) =
        realize_admitted_geometry(geometry)?;
    let parts = [
        intent_digest.to_string(),
        format!("family:{}", family.as_str()),
        format!("planes:{}", support_planes.len()),
        format!("vertices:{}", vertex_positions.len()),
        format!("edges:{}", topology_counts.edge_count()),
        format!("loops:{}", topology_counts.loop_count()),
        format!("wires:{}", topology_counts.wire_count()),
        format!("faces:{}", topology_counts.face_count()),
        format!("shells:{}", topology_counts.shell_count()),
        format!("bodies:{}", topology_counts.body_count()),
        format!("realization:{}", realization_report.report_digest()),
    ];
    Ok(PrimitiveConstructionScaffold::new(
        family,
        request_digest.to_string(),
        intent_digest.to_string(),
        support_planes,
        realization_report,
        vertex_positions,
        topology_counts,
        digest_owned_parts(&parts),
    ))
}

fn realize_admitted_geometry(
    geometry: &AdmittedPrimitiveConstructionGeometry,
) -> Result<
    (
        Vec<worth_geom::facade::Plane>,
        worth_geom::facade::PrimitiveRealizationReport,
        Vec<[f64; 3]>,
        PrimitiveConstructionTopologyCounts,
    ),
    PrimitiveConstructionPhaseError,
> {
    match geometry {
        AdmittedPrimitiveConstructionGeometry::SimplexSolid { placement, scale } => {
            let realization =
                realize_tetrahedron_support([0.0, 0.0, 0.0], *scale).map_err(map_geometry)?;
            let embedded =
                apply_spatial_placement(placement, realization.planes(), &simplex_vertices(*scale))
                    .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes,
                realization.report().clone(),
                vertices,
                PrimitiveConstructionTopologyCounts::new(4, 6, 4, 0, 4, 1, 1),
            ))
        }
        AdmittedPrimitiveConstructionGeometry::Orthotope {
            placement,
            half_extents,
        } => {
            let realization =
                realize_block_support([0.0, 0.0, 0.0], *half_extents).map_err(map_geometry)?;
            let embedded = apply_spatial_placement(
                placement,
                realization.planes(),
                &orthotope_vertices(*half_extents),
            )
            .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes,
                realization.report().clone(),
                vertices,
                PrimitiveConstructionTopologyCounts::new(8, 12, 6, 0, 6, 1, 1),
            ))
        }
        AdmittedPrimitiveConstructionGeometry::RegularPrism {
            placement,
            sides,
            radius,
            height,
        } => {
            let realization = realize_prism_support([0.0, 0.0, 0.0], *sides, *radius, *height)
                .map_err(map_geometry)?;
            let embedded = apply_spatial_placement(
                placement,
                realization.planes(),
                &prism_vertices(*sides, *radius, *height),
            )
            .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes,
                realization.report().clone(),
                vertices,
                PrimitiveConstructionTopologyCounts::new(
                    (*sides as usize) * 2,
                    (*sides as usize) * 3,
                    (*sides as usize) + 2,
                    0,
                    (*sides as usize) + 2,
                    1,
                    1,
                ),
            ))
        }
        AdmittedPrimitiveConstructionGeometry::RegularPyramid {
            placement,
            sides,
            radius,
            height,
        } => {
            let realization = realize_pyramid_support([0.0, 0.0, 0.0], *sides, *radius, *height)
                .map_err(map_pyramid_geometry)?;
            let embedded = apply_spatial_placement(
                placement,
                realization.planes(),
                &pyramid_vertices(*sides, *radius, *height),
            )
            .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes,
                realization.report().clone(),
                vertices,
                PrimitiveConstructionTopologyCounts::new(
                    (*sides as usize) + 1,
                    (*sides as usize) * 2,
                    (*sides as usize) + 1,
                    0,
                    (*sides as usize) + 1,
                    1,
                    1,
                ),
            ))
        }
        AdmittedPrimitiveConstructionGeometry::WireBody {
            placement,
            edge_count,
        } => {
            let support_planes = vec![planar_support_plane().map_err(map_support_plane)?];
            let local_vertices = wire_body_vertices(*edge_count, 1.5);
            let embedded = apply_spatial_placement(placement, &support_planes, &local_vertices)
                .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes.clone(),
                build_direct_realization_report("wire_body", &vertices, &planes),
                vertices,
                PrimitiveConstructionTopologyCounts::new(
                    *edge_count as usize,
                    *edge_count as usize,
                    1,
                    1,
                    0,
                    0,
                    1,
                ),
            ))
        }
        AdmittedPrimitiveConstructionGeometry::ShellWithHole {
            placement,
            outer_loop_edge_count,
            hole_loop_edge_counts,
        } => {
            let edge_count = *outer_loop_edge_count as usize
                + hole_loop_edge_counts
                    .iter()
                    .map(|count| *count as usize)
                    .sum::<usize>();
            let support_planes = vec![planar_support_plane().map_err(map_support_plane)?];
            let local_vertices =
                shell_with_hole_vertices(*outer_loop_edge_count, hole_loop_edge_counts);
            let embedded = apply_spatial_placement(placement, &support_planes, &local_vertices)
                .map_err(map_placement_geometry)?;
            let (planes, vertices) = embedded.into_parts();
            Ok((
                planes.clone(),
                build_direct_realization_report("shell_with_hole", &vertices, &planes),
                vertices,
                PrimitiveConstructionTopologyCounts::new(
                    edge_count,
                    edge_count,
                    1 + hole_loop_edge_counts.len(),
                    0,
                    1,
                    1,
                    1,
                ),
            ))
        }
    }
}

fn map_geometry(error: impl ToString) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error.to_string(),
    ))
}

fn map_pyramid_geometry(
    error: worth_geom::facade::PrimitiveRealizationError,
) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(
        PrimitiveConstructionGeometryError::from_realization_error(error),
    )
}

fn map_support_plane(error: String) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error,
    ))
}

fn map_placement_geometry(error: SpatialPlacementError) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::Geometry(PrimitiveConstructionGeometryError::GeometryFailure(
        error.to_string(),
    ))
}
