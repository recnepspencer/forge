use super::super::birth_scaffold::{
    lower_family_birth_scaffold_plan, PrimitiveConstructionBirthScaffoldPlan,
};
use super::super::error_mapping::map_support_plane;
use super::super::geometry::{planar_support_plane, shell_with_hole_vertices};
use super::super::scalar_admission::admit_polygon_edge_count;
use super::super::topology_counts::PrimitiveConstructionTopologyCounts;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use worth_primitives::{
    derive_shell_with_hole_layout, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveWitnessDescriptor, ShellWithHoleWitnessLayoutPolicy,
};
use worth_spatial::facade::birth::PrimitiveConstructionBirthScaffoldInput;
use worth_spatial::facade::placement::AdmittedSpatialPlacement;

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
    derive_shell_with_hole_layout(
        admitted.outer_loop_edge_count,
        &admitted.hole_loop_edge_counts,
        ShellWithHoleWitnessLayoutPolicy::default(),
    )
    .map_err(map_shell_with_hole_layout)?;
    let support_planes = vec![planar_support_plane().map_err(map_support_plane)?];
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: admitted.outer_loop_edge_count,
            hole_loop_edge_counts: admitted.hole_loop_edge_counts.clone(),
        },
    );
    lower_family_birth_scaffold_plan(
        intent_digest,
        placement,
        PrimitiveConstructionBirthScaffoldPlan::from_direct_planar_support(
            PrimitiveConstructionFamily::ShellWithHole,
            birth_contract,
            "shell_with_hole",
            support_planes,
            shell_with_hole_vertices(
                admitted.outer_loop_edge_count,
                &admitted.hole_loop_edge_counts,
            )
            .map_err(map_shell_with_hole_layout)?,
            PrimitiveConstructionTopologyCounts::from_contract(birth_contract.topology_contract()),
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

fn map_shell_with_hole_layout(
    error: worth_primitives::ShellWithHoleWitnessLayoutError,
) -> PrimitiveConstructionPhaseError {
    PrimitiveConstructionPhaseError::InvalidRequest {
        family: PrimitiveConstructionFamily::ShellWithHole,
        reason: match error {
            worth_primitives::ShellWithHoleWitnessLayoutError::OuterLoopTooSmall => {
                "shell-with-hole outer loop must admit at least three edges"
            }
            worth_primitives::ShellWithHoleWitnessLayoutError::HoleLoopTooSmall => {
                "shell-with-hole hole loops must admit at least three edges"
            }
            worth_primitives::ShellWithHoleWitnessLayoutError::MissingHoleLoop => {
                "shell-with-hole requires at least one inner hole loop"
            }
        },
    }
}
