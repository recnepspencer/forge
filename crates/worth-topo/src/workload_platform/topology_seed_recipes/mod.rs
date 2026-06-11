mod closed_solid_recipes;
mod hostile_recipes;
mod sheet_recipes;
mod singular_vertex_recipes;
mod topology_record_constructors;
mod wire_recipes;

use super::topology_seed::{TopologySeedKind, TopologySeedNeighborhoodReceipt, TopologySeedRecipe};
use crate::brep::topology_graph::TopologyView;
use crate::workload_platform::topology_seed::TopologySeedCleanFailReasonCode;

pub(crate) struct TopologySeedRecipeOutput {
    pub(crate) topology: TopologyView,
    pub(crate) neighborhood: Option<TopologySeedNeighborhoodReceipt>,
}

impl TopologySeedRecipeOutput {
    fn topology(topology: TopologyView) -> Self {
        Self {
            topology,
            neighborhood: None,
        }
    }

    fn with_neighborhood(
        topology: TopologyView,
        neighborhood: TopologySeedNeighborhoodReceipt,
    ) -> Self {
        Self {
            topology,
            neighborhood: Some(neighborhood),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TopologySeedRecipeDenial {
    pub(crate) reason_code: TopologySeedCleanFailReasonCode,
    pub(crate) reason: &'static str,
}

pub(crate) fn build(
    recipe: &TopologySeedRecipe,
) -> Result<TopologySeedRecipeOutput, TopologySeedRecipeDenial> {
    match recipe.kind() {
        TopologySeedKind::Cube => Ok(TopologySeedRecipeOutput::topology(
            closed_solid_recipes::cube_topology_view(),
        )),
        TopologySeedKind::Tetrahedron => Ok(TopologySeedRecipeOutput::topology(
            closed_solid_recipes::tetrahedron_topology_view(),
        )),
        TopologySeedKind::SingleFaceLoop => {
            sheet_recipes::single_face_loop(recipe.requested_count().unwrap_or(4))
                .map(TopologySeedRecipeOutput::topology)
        }
        TopologySeedKind::MultiFaceShell => closed_solid_recipes::multi_face_shell_topology_view(
            recipe.requested_count().unwrap_or(6),
        )
        .map(TopologySeedRecipeOutput::topology),
        TopologySeedKind::OpenSheet => Ok(TopologySeedRecipeOutput::topology(
            sheet_recipes::open_sheet_topology_view(),
        )),
        TopologySeedKind::OpenWire => Ok(TopologySeedRecipeOutput::topology(
            wire_recipes::open_wire_topology_view(),
        )),
        TopologySeedKind::HighValenceVertex => {
            let (topology, neighborhood) =
                singular_vertex_recipes::high_valence_vertex_topology_view_with_valence(
                    recipe.requested_count().unwrap_or(5),
                );
            Ok(TopologySeedRecipeOutput::with_neighborhood(
                topology,
                neighborhood,
            ))
        }
        TopologySeedKind::SelfIntersectingLoop => Ok(TopologySeedRecipeOutput::topology(
            hostile_recipes::self_intersecting_loop_topology_view(),
        )),
        TopologySeedKind::NonManifoldWire => Ok(TopologySeedRecipeOutput::topology(
            hostile_recipes::non_manifold_wire_topology_view(),
        )),
        TopologySeedKind::ThinWallLocalBasis => Ok(TopologySeedRecipeOutput::topology(
            hostile_recipes::thin_wall_local_basis_topology_view(),
        )),
        TopologySeedKind::OrientationInconsistency => Ok(TopologySeedRecipeOutput::topology(
            hostile_recipes::orientation_inconsistency_topology_view(),
        )),
    }
}

pub(crate) fn seed_parameter_denial(
    reason_code: TopologySeedCleanFailReasonCode,
    reason: &'static str,
) -> TopologySeedRecipeDenial {
    TopologySeedRecipeDenial {
        reason_code,
        reason,
    }
}
