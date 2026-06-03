use super::error_mapping::map_placement_geometry;
use super::spatial_family_bridge::to_spatial_family;
use super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_geom::facade::{build_direct_realization_report, Plane, PrimitiveRealizationReport};
use worth_spatial::facade::bindings::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::{apply_spatial_placement, AdmittedSpatialPlacement};

pub(super) struct PrimitiveConstructionBirthScaffoldPlan {
    family: PrimitiveConstructionFamily,
    support_planes: Vec<Plane>,
    local_vertices: Vec<[f64; 3]>,
    realization: PrimitiveConstructionBirthScaffoldRealizationPlan,
    topology_counts: PrimitiveConstructionTopologyCounts,
}

pub(super) enum PrimitiveConstructionBirthScaffoldRealizationPlan {
    SupportReport(PrimitiveRealizationReport),
    DirectPlanar { label: &'static str },
}

impl PrimitiveConstructionBirthScaffoldPlan {
    pub(super) fn from_realized_support(
        family: PrimitiveConstructionFamily,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        realization_report: PrimitiveRealizationReport,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            support_planes,
            local_vertices,
            realization: PrimitiveConstructionBirthScaffoldRealizationPlan::SupportReport(
                realization_report,
            ),
            topology_counts,
        }
    }

    pub(super) fn from_direct_planar_support(
        family: PrimitiveConstructionFamily,
        label: &'static str,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            support_planes,
            local_vertices,
            realization: PrimitiveConstructionBirthScaffoldRealizationPlan::DirectPlanar { label },
            topology_counts,
        }
    }
}

pub(super) fn lower_family_birth_scaffold_plan(
    intent_digest: &str,
    placement: &AdmittedSpatialPlacement,
    scaffold_plan: PrimitiveConstructionBirthScaffoldPlan,
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let PrimitiveConstructionBirthScaffoldPlan {
        family,
        support_planes,
        local_vertices,
        realization,
        topology_counts,
    } = scaffold_plan;
    let (planes, vertices) = apply_spatial_placement(placement, &support_planes, &local_vertices)
        .map_err(map_placement_geometry)?
        .into_parts();
    let realization_report =
        materialize_birth_scaffold_realization_report(realization, &vertices, &planes);
    Ok(build_birth_scaffold_input(
        family,
        intent_digest,
        planes,
        realization_report,
        vertices,
        topology_counts,
    ))
}

fn materialize_birth_scaffold_realization_report(
    realization: PrimitiveConstructionBirthScaffoldRealizationPlan,
    vertex_positions: &[[f64; 3]],
    support_planes: &[Plane],
) -> PrimitiveRealizationReport {
    match realization {
        PrimitiveConstructionBirthScaffoldRealizationPlan::SupportReport(report) => report,
        PrimitiveConstructionBirthScaffoldRealizationPlan::DirectPlanar { label } => {
            build_direct_realization_report(label, vertex_positions, support_planes)
        }
    }
}

fn build_birth_scaffold_input(
    family: PrimitiveConstructionFamily,
    intent_digest: &str,
    support_planes: Vec<Plane>,
    realization_report: PrimitiveRealizationReport,
    vertex_positions: Vec<[f64; 3]>,
    topology_counts: PrimitiveConstructionTopologyCounts,
) -> PrimitiveConstructionBirthScaffoldInput {
    let scaffold_digest = digest_owned_parts(&[
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
    ]);
    PrimitiveConstructionBirthScaffoldInput::new_with_realization(
        to_spatial_family(family),
        family.topology_birth_class(),
        scaffold_digest,
        support_planes,
        realization_report,
        vertex_positions,
        topology_counts.vertex_count(),
        topology_counts.edge_count(),
        topology_counts.loop_count(),
        topology_counts.wire_count(),
        topology_counts.face_count(),
        topology_counts.shell_count(),
        topology_counts.body_count(),
    )
}
