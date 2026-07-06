use topology::facade::{
    NmtTopologyConstructionReceipt, TopologySeed, TopologySeedReceipt, TopologySeedRecipe,
};

use super::error::WorkloadCatalogError;
use super::recipe_kind::{WorkloadCatalogRecipeKind, WorkloadTopologyBreadth};
use crate::workload_composition::{trace_note, trace_scope};

pub(super) fn build_topology_seed(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
    topology_breadth: WorkloadTopologyBreadth,
    topology_construction: Option<&NmtTopologyConstructionReceipt>,
) -> Result<TopologySeedReceipt, WorkloadCatalogError> {
    trace_scope("build_topology_seed", || {
        if let Some(construction) = topology_construction {
            if !recipe.consumes_nmt_topology_construction() {
                return Err(WorkloadCatalogError::UnsupportedRecipe {
                    recipe,
                    reason: "NMT topology construction receipts can only enter catalog recipes that explicitly consume the generic NMT topology construction boundary".to_string(),
                });
            }
            trace_note(format!(
                "reuse nmt topology construction seed for {declaration}"
            ));
            return Ok(construction.topology_seed_receipt().clone());
        }
        if recipe.consumes_nmt_topology_construction() {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "NMT topology construction workloads require a production NmtTopologyConstructionReceipt before spatial binding".to_string(),
            });
        }

        let seed = trace_scope("topology_seed_select_recipe", || {
            topology_seed_for_breadth(recipe, topology_breadth)
        })?;
        trace_scope("topology_seed_build_receipt", || {
            seed.with_declaration(format!("topology seed for {declaration}"))
                .build()
                .map_err(WorkloadCatalogError::from)
        })
    })
}

fn topology_seed_for_breadth(
    recipe: WorkloadCatalogRecipeKind,
    topology_breadth: WorkloadTopologyBreadth,
) -> Result<TopologySeedRecipe, WorkloadCatalogError> {
    match topology_breadth {
        WorkloadTopologyBreadth::Default => Ok(default_topology_seed(recipe)),
        WorkloadTopologyBreadth::SingleFaceLoopEdges { edge_count }
            if recipe == WorkloadCatalogRecipeKind::SingleFaceLoop =>
        {
            Ok(TopologySeed::single_face_loop(edge_count))
        }
        WorkloadTopologyBreadth::MultiFaceShell { face_count }
            if recipe == WorkloadCatalogRecipeKind::CoplanarOverlapStorm =>
        {
            Ok(TopologySeed::multi_face_shell(face_count))
        }
        WorkloadTopologyBreadth::HighValenceVertex { valence }
            if recipe == WorkloadCatalogRecipeKind::HighValenceVertex =>
        {
            Ok(TopologySeed::high_valence_vertex_with_valence(valence))
        }
        WorkloadTopologyBreadth::MultiFaceShell { .. } => {
            Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "explicit multi-face shell breadth is only admitted for the coplanar overlap storm recipe".to_string(),
            })
        }
        WorkloadTopologyBreadth::SingleFaceLoopEdges { .. } => {
            Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "explicit single-face loop edge breadth is only admitted for the single face loop recipe".to_string(),
            })
        }
        WorkloadTopologyBreadth::HighValenceVertex { .. } => {
            Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "explicit high-valence vertex breadth is only admitted for the high-valence vertex recipe".to_string(),
            })
        }
    }
}

fn default_topology_seed(recipe: WorkloadCatalogRecipeKind) -> TopologySeedRecipe {
    match recipe {
        WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair
        | WorkloadCatalogRecipeKind::BooleanEventCarrierCleanPlanarBodyPair
        | WorkloadCatalogRecipeKind::BooleanEventExtractionMetabossPair
        | WorkloadCatalogRecipeKind::BooleanMismatchedPosturePair
        | WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair
        | WorkloadCatalogRecipeKind::BooleanThinFeaturePair
        | WorkloadCatalogRecipeKind::BooleanHighValenceContactPair
        | WorkloadCatalogRecipeKind::BooleanBoundaryOnlyCoincidentPair
        | WorkloadCatalogRecipeKind::BooleanMixedBoundaryAreaPair
        | WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair
        | WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair => unreachable!(
            "boolean operand-pair recipes must build through the dedicated pair orchestrator"
        ),
        WorkloadCatalogRecipeKind::Cube
        | WorkloadCatalogRecipeKind::MixedSurfaceKillBox
        | WorkloadCatalogRecipeKind::TransformCycle
        | WorkloadCatalogRecipeKind::RetainedCancellationChain => TopologySeed::cube(),
        WorkloadCatalogRecipeKind::CoplanarOverlapStorm => TopologySeed::multi_face_shell(64),
        WorkloadCatalogRecipeKind::Tetrahedron => TopologySeed::tetrahedron(),
        WorkloadCatalogRecipeKind::SingleFaceLoop => TopologySeed::single_face_loop(4),
        WorkloadCatalogRecipeKind::ThinFeatureWall => TopologySeed::single_face_loop(64),
        WorkloadCatalogRecipeKind::HighValenceVertex => TopologySeed::high_valence_vertex(),
        WorkloadCatalogRecipeKind::OpenWire
        | WorkloadCatalogRecipeKind::OpenSheet
        | WorkloadCatalogRecipeKind::OpenShellNmtEdgeFan
        | WorkloadCatalogRecipeKind::OpenLayerStack
        | WorkloadCatalogRecipeKind::GrazingBasketStack
        | WorkloadCatalogRecipeKind::NmtTopologyConstruction => unreachable!(
            "NMT topology construction catalog recipes are handled before default seed selection"
        ),
        WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop => {
            TopologySeed::self_intersecting_loop()
        }
    }
}
