use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_support_plane;
use super::super::geometry::{planar_support_plane, shell_with_hole_vertices};
use super::super::scalar_admission::admit_polygon_edge_count;
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_spatial::facade::{AdmittedSpatialPlacement, PrimitiveConstructionBirthScaffoldInput};

struct AdmittedShellWithHoleBirthParameters {
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: Vec<u32>,
}

pub(in super::super) fn build_shell_with_hole_birth_input(
    placement: &AdmittedSpatialPlacement,
    intent_digest: &str,
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
) -> Result<PrimitiveConstructionBirthScaffoldInput, PrimitiveConstructionPhaseError> {
    let admitted =
        admit_shell_with_hole_birth_parameters(outer_loop_edge_count, hole_loop_edge_counts)?;
    let support_planes = vec![planar_support_plane().map_err(map_support_plane)?];
    let edge_count = admitted.outer_loop_edge_count as usize
        + admitted
            .hole_loop_edge_counts
            .iter()
            .map(|count| *count as usize)
            .sum::<usize>();
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_direct_planar_support(
            PrimitiveConstructionFamily::ShellWithHole,
            "shell_with_hole",
            support_planes,
            shell_with_hole_vertices(
                admitted.outer_loop_edge_count,
                &admitted.hole_loop_edge_counts,
            ),
            PrimitiveConstructionTopologyCounts::new(
                edge_count,
                edge_count,
                1 + admitted.hole_loop_edge_counts.len(),
                0,
                1,
                1,
                1,
            ),
        ),
    )
}

fn admit_shell_with_hole_birth_parameters(
    outer_loop_edge_count: u32,
    hole_loop_edge_counts: &[u32],
) -> Result<AdmittedShellWithHoleBirthParameters, PrimitiveConstructionPhaseError> {
    let family = PrimitiveConstructionFamily::ShellWithHole;
    if hole_loop_edge_counts.is_empty() {
        return Err(PrimitiveConstructionPhaseError::InvalidRequest {
            family,
            reason: "shell-with-hole requires at least one inner hole loop",
        });
    }
    let outer_loop_edge_count = admit_polygon_edge_count(family, outer_loop_edge_count)?;
    let hole_loop_edge_counts = hole_loop_edge_counts
        .iter()
        .copied()
        .map(|count| admit_polygon_edge_count(family, count))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AdmittedShellWithHoleBirthParameters {
        outer_loop_edge_count,
        hole_loop_edge_counts,
    })
}
