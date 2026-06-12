mod closed_solid_recipes;
mod hostile_recipes;
mod sheet_recipes;
mod singular_vertex_recipes;
pub(crate) mod topology_record_constructors;
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
        TopologySeedKind::OpenSheet => build_open_sheet_seed(),
        TopologySeedKind::OpenWire => build_open_wire_seed(),
        TopologySeedKind::OpenShellNmtEdgeFan => {
            let pattern = super::nmt_topology_construction::NmtTopologyPattern::OpenRadialFan(
                super::nmt_topology_construction::OpenRadialFanSpec::new()
                    .incident_faces(recipe.requested_count().unwrap_or(3)),
            );
            super::nmt_topology_construction::build_nmt_topology_view(&pattern)
                .map(TopologySeedRecipeOutput::topology)
                .map_err(|_| {
                    seed_parameter_denial(
                        TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed,
                        "open shell NMT edge fan seeds must be constructed through the generic NMT topology boundary",
                    )
                })
        }
        TopologySeedKind::NmtOpenLayerStack => {
            super::nmt_topology_construction::build_nmt_topology_view(
                &super::nmt_topology_construction::NmtTopologyPattern::OpenLayerStack(
                    super::nmt_topology_construction::OpenLayerStackSpec::new()
                        .with_layer_identity()
                        .with_open_boundary_receipts()
                        .with_radial_adjacency_receipts(),
                ),
            )
            .map(TopologySeedRecipeOutput::topology)
            .map_err(|_| {
                seed_parameter_denial(
                    TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed,
                    "NMT open layer stack seeds must be constructed through the generic NMT topology boundary",
                )
            })
        }
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

fn build_open_sheet_seed() -> Result<TopologySeedRecipeOutput, TopologySeedRecipeDenial> {
    let pattern = super::nmt_topology_construction::NmtTopologyPattern::OpenSheetPatch(
        super::nmt_topology_construction::OpenSheetPatchSpec::new().strips(1),
    );
    super::nmt_topology_construction::build_nmt_topology_view(&pattern)
        .map(TopologySeedRecipeOutput::topology)
        .map_err(|_| {
            seed_parameter_denial(
                TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed,
                "open sheet seeds must be constructed through the generic NMT topology boundary",
            )
        })
}

fn build_open_wire_seed() -> Result<TopologySeedRecipeOutput, TopologySeedRecipeDenial> {
    let pattern = super::nmt_topology_construction::NmtTopologyPattern::OpenWireChain(
        super::nmt_topology_construction::OpenWireChainSpec::new().edges(4),
    );
    super::nmt_topology_construction::build_nmt_topology_view(&pattern)
        .map(TopologySeedRecipeOutput::topology)
        .map_err(|_| {
            seed_parameter_denial(
                TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed,
                "open wire seeds must be constructed through the generic NMT topology boundary",
            )
        })
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
