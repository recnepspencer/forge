use super::error_mapping::map_placement_geometry;
use super::spatial_family_bridge::to_spatial_family;
use super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::digest::digest_owned_parts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_geom::facade::{build_direct_realization_report, Plane, PrimitiveRealizationReport};
use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
};
use worth_spatial::facade::birth::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::{apply_spatial_placement, AdmittedSpatialPlacement};

pub(super) struct PrimitiveConstructionBirthScaffoldPlan {
    family: PrimitiveConstructionFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
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
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        realization_report: PrimitiveRealizationReport,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            birth_contract,
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
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        label: &'static str,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            birth_contract,
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
        birth_contract,
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
        birth_contract,
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
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    intent_digest: &str,
    support_planes: Vec<Plane>,
    realization_report: PrimitiveRealizationReport,
    vertex_positions: Vec<[f64; 3]>,
    topology_counts: PrimitiveConstructionTopologyCounts,
) -> PrimitiveConstructionBirthScaffoldInput {
    let geometry_identity = scaffold_geometry_identity(&support_planes, &vertex_positions);
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
        format!(
            "scaffold-geometry:{}",
            geometry_identity.scaffold_geometry_digest().as_str()
        ),
        format!("realization:{}", realization_report.report_digest()),
    ]);
    PrimitiveConstructionBirthScaffoldInput::new_with_realization_and_contract(
        to_spatial_family(family),
        birth_contract,
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

fn scaffold_geometry_identity(
    support_planes: &[Plane],
    vertex_positions: &[[f64; 3]],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        support_planes.iter().map(plane_identity).collect(),
        vertex_positions
            .iter()
            .copied()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn plane_identity(plane: &Plane) -> PrimitiveSupportPlaneIdentity {
    let (a, b, c, d) = plane.exact_coefficients();
    PrimitiveSupportPlaneIdentity::new(a.to_string(), b.to_string(), c.to_string(), d.to_string())
}

#[cfg(test)]
mod tests {
    use super::scaffold_geometry_identity;
    use worth_geom::facade::Plane;

    #[test]
    fn scaffold_geometry_digest_changes_when_plane_or_vertex_identity_changes() {
        let base_plane =
            Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("base plane");
        let shifted_plane =
            Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).expect("shifted plane");
        let base_vertices = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let shifted_vertices = vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 1.0, 0.0]];

        let base = scaffold_geometry_identity(&[base_plane.clone()], &base_vertices)
            .scaffold_geometry_digest();
        let plane_shifted =
            scaffold_geometry_identity(&[shifted_plane], &base_vertices).scaffold_geometry_digest();
        let vertex_shifted =
            scaffold_geometry_identity(&[base_plane], &shifted_vertices).scaffold_geometry_digest();

        assert_ne!(base.as_str(), plane_shifted.as_str());
        assert_ne!(base.as_str(), vertex_shifted.as_str());
    }
}
