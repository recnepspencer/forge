use super::boolean_operand_pair::WorkloadCatalogBooleanOperandPairRecipe;
use super::catalog::{WorkloadCatalog, WorkloadCatalogRecipe};
use super::grazing_basket_spec::GrazingBasketStackSpec;
use super::open_class_triad::OpenClassTriadCatalogRecipe;
use super::recipe_kind::WorkloadCatalogRecipeKind;
use super::topology_construction_plan::WorkloadCatalogTopologyConstructionPlan;
use topology::facade::{
    NmtTopologyConstructionReceipt, OpenLayerStackSpec, OpenRadialFanSpec, OpenSheetPatchSpec,
    OpenWireChainSpec,
};

impl WorkloadCatalog {
    pub fn cube() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::Cube)
    }

    pub fn tetrahedron() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::Tetrahedron)
    }

    pub fn single_face_loop() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::SingleFaceLoop)
    }

    pub fn planar_boolean_clean_planar_body_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanCleanPlanarBodyPair)
    }

    pub fn planar_boolean_event_carrier_clean_planar_body_pair(
    ) -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanEventCarrierCleanPlanarBodyPair)
    }

    pub fn planar_boolean_event_extraction_metaboss_pair() -> WorkloadCatalogBooleanOperandPairRecipe
    {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanEventExtractionMetabossPair)
    }

    pub fn planar_boolean_mismatched_posture_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanMismatchedPosturePair)
    }

    pub fn planar_boolean_coplanar_overlap_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanCoplanarOverlapPair)
    }

    pub fn planar_boolean_thin_feature_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanThinFeaturePair)
    }

    pub fn planar_boolean_high_valence_contact_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanHighValenceContactPair)
    }

    pub fn planar_boolean_dirty_clean_fail_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanDirtyCleanFailPair)
    }

    pub fn planar_boolean_open_unbounded_denial_pair() -> WorkloadCatalogBooleanOperandPairRecipe {
        boolean_pair(WorkloadCatalogRecipeKind::BooleanOpenUnboundedDenialPair)
    }

    pub fn coplanar_overlap_storm() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::CoplanarOverlapStorm)
    }

    pub fn thin_feature_wall() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::ThinFeatureWall)
    }

    pub fn dirty_self_intersecting_loop() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop)
    }

    pub fn high_valence_vertex() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::HighValenceVertex)
    }

    pub fn mixed_surface_kill_box() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::MixedSurfaceKillBox)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenSheet(
                OpenSheetPatchSpec::new(),
            ))
    }

    pub fn open_wire() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenWire)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenWire(
                OpenWireChainSpec::new(),
            ))
    }

    pub fn open_sheet() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenSheet)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenSheet(
                OpenSheetPatchSpec::new(),
            ))
    }

    pub fn open_shell_nmt_edge_fan(incident_faces: usize) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenShellNmtEdgeFan)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenRadialFan(
                    OpenRadialFanSpec::new().incident_faces(incident_faces),
                ),
            )
    }

    pub fn open_layer_stack(spec: OpenLayerStackSpec) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenLayerStack)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenLayerStack(spec),
            )
    }

    pub fn grazing_open_shell_basket_stack(spec: GrazingBasketStackSpec) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::GrazingBasketStack)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenLayerStack(
                    spec.into_open_layer_stack_spec(),
                ),
            )
    }

    pub fn open_class_triad(incident_faces: usize) -> OpenClassTriadCatalogRecipe {
        OpenClassTriadCatalogRecipe::new(incident_faces)
    }

    pub fn transform_cycle() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::TransformCycle)
    }

    pub fn retained_cancellation_chain() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::RetainedCancellationChain)
    }

    pub fn from_topology_construction(
        construction: NmtTopologyConstructionReceipt,
    ) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::NmtTopologyConstruction)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::Receipt(
                construction,
            ))
    }
}

fn boolean_pair(kind: WorkloadCatalogRecipeKind) -> WorkloadCatalogBooleanOperandPairRecipe {
    WorkloadCatalogBooleanOperandPairRecipe::new(kind)
}
