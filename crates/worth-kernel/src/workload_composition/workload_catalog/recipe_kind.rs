#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadCatalogRecipeKind {
    Cube,
    Tetrahedron,
    SingleFaceLoop,
    BooleanCleanPlanarBodyPair,
    BooleanEventCarrierCleanPlanarBodyPair,
    BooleanEventExtractionMetabossPair,
    BooleanMismatchedPosturePair,
    BooleanCoplanarOverlapPair,
    BooleanThinFeaturePair,
    BooleanHighValenceContactPair,
    BooleanDirtyCleanFailPair,
    BooleanOpenUnboundedDenialPair,
    CoplanarOverlapStorm,
    ThinFeatureWall,
    DirtySelfIntersectingLoop,
    HighValenceVertex,
    MixedSurfaceKillBox,
    OpenWire,
    OpenSheet,
    OpenShellNmtEdgeFan,
    OpenLayerStack,
    GrazingBasketStack,
    NmtTopologyConstruction,
    TransformCycle,
    RetainedCancellationChain,
}

impl WorkloadCatalogRecipeKind {
    pub fn query_key(self) -> &'static str {
        match self {
            Self::Cube => "worth.catalog.cube",
            Self::Tetrahedron => "worth.catalog.tetrahedron",
            Self::SingleFaceLoop => "worth.catalog.single_face_loop",
            Self::BooleanCleanPlanarBodyPair => "worth.catalog.boolean_clean_planar_body_pair",
            Self::BooleanEventCarrierCleanPlanarBodyPair => {
                "worth.catalog.boolean_event_carrier_clean_planar_body_pair"
            }
            Self::BooleanEventExtractionMetabossPair => {
                "worth.catalog.boolean_event_extraction_metaboss_pair"
            }
            Self::BooleanMismatchedPosturePair => "worth.catalog.boolean_mismatched_posture_pair",
            Self::BooleanCoplanarOverlapPair => "worth.catalog.boolean_coplanar_overlap_pair",
            Self::BooleanThinFeaturePair => "worth.catalog.boolean_thin_feature_pair",
            Self::BooleanHighValenceContactPair => {
                "worth.catalog.boolean_high_valence_contact_pair"
            }
            Self::BooleanDirtyCleanFailPair => "worth.catalog.boolean_dirty_clean_fail_pair",
            Self::BooleanOpenUnboundedDenialPair => {
                "worth.catalog.boolean_open_unbounded_denial_pair"
            }
            Self::CoplanarOverlapStorm => "worth.catalog.coplanar_overlap_storm",
            Self::ThinFeatureWall => "worth.catalog.thin_feature_wall",
            Self::DirtySelfIntersectingLoop => "worth.catalog.dirty_self_intersecting_loop",
            Self::HighValenceVertex => "worth.catalog.high_valence_vertex",
            Self::MixedSurfaceKillBox => "worth.catalog.mixed_surface_kill_box",
            Self::OpenWire => "worth.catalog.open_wire",
            Self::OpenSheet => "worth.catalog.open_sheet",
            Self::OpenShellNmtEdgeFan => "worth.catalog.open_shell_nmt_edge_fan",
            Self::OpenLayerStack => "worth.catalog.open_layer_stack",
            Self::GrazingBasketStack => "worth.catalog.grazing_open_shell_basket_stack",
            Self::NmtTopologyConstruction => "worth.catalog.nmt_topology_construction",
            Self::TransformCycle => "worth.catalog.transform_cycle",
            Self::RetainedCancellationChain => "worth.catalog.retained_cancellation_chain",
        }
    }

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Cube => "cube workload recipe",
            Self::Tetrahedron => "tetrahedron workload recipe",
            Self::SingleFaceLoop => "single face loop workload recipe",
            Self::BooleanCleanPlanarBodyPair => "boolean clean planar body pair recipe",
            Self::BooleanEventCarrierCleanPlanarBodyPair => {
                "boolean event-carrier clean planar body pair recipe"
            }
            Self::BooleanEventExtractionMetabossPair => {
                "boolean event-extraction metaboss operand pair recipe"
            }
            Self::BooleanMismatchedPosturePair => "boolean mismatched posture pair recipe",
            Self::BooleanCoplanarOverlapPair => "boolean coplanar overlap pair recipe",
            Self::BooleanThinFeaturePair => "boolean thin feature pair recipe",
            Self::BooleanHighValenceContactPair => "boolean high-valence contact pair recipe",
            Self::BooleanDirtyCleanFailPair => "boolean dirty clean-fail pair recipe",
            Self::BooleanOpenUnboundedDenialPair => "boolean open or unbounded denial pair recipe",
            Self::CoplanarOverlapStorm => "coplanar overlap storm workload recipe",
            Self::ThinFeatureWall => "thin feature wall workload recipe",
            Self::DirtySelfIntersectingLoop => "dirty self-intersecting loop workload recipe",
            Self::HighValenceVertex => "high valence vertex workload recipe",
            Self::MixedSurfaceKillBox => "mixed surface kill box workload recipe",
            Self::OpenWire => "open wire workload recipe",
            Self::OpenSheet => "open sheet workload recipe",
            Self::OpenShellNmtEdgeFan => "open shell NMT edge fan workload recipe",
            Self::OpenLayerStack => "open layer stack workload recipe",
            Self::GrazingBasketStack => "grazing open shell basket stack workload recipe",
            Self::NmtTopologyConstruction => "NMT topology construction workload recipe",
            Self::TransformCycle => "transform cycle workload recipe",
            Self::RetainedCancellationChain => "retained cancellation chain workload recipe",
        }
    }

    pub fn default_declaration(self) -> &'static str {
        match self {
            Self::Cube => "catalog cube workload",
            Self::Tetrahedron => "catalog tetrahedron workload",
            Self::SingleFaceLoop => "catalog single face loop workload",
            Self::BooleanCleanPlanarBodyPair => "catalog boolean clean planar body pair",
            Self::BooleanEventCarrierCleanPlanarBodyPair => {
                "catalog boolean event-carrier clean planar body pair"
            }
            Self::BooleanEventExtractionMetabossPair => {
                "catalog boolean event-extraction metaboss operand pair"
            }
            Self::BooleanMismatchedPosturePair => "catalog boolean mismatched posture pair",
            Self::BooleanCoplanarOverlapPair => "catalog boolean coplanar overlap pair",
            Self::BooleanThinFeaturePair => "catalog boolean thin feature pair",
            Self::BooleanHighValenceContactPair => "catalog boolean high-valence contact pair",
            Self::BooleanDirtyCleanFailPair => "catalog boolean dirty clean-fail pair",
            Self::BooleanOpenUnboundedDenialPair => "catalog boolean open or unbounded denial pair",
            Self::CoplanarOverlapStorm => "catalog coplanar overlap storm workload",
            Self::ThinFeatureWall => "catalog thin feature wall workload",
            Self::DirtySelfIntersectingLoop => "catalog dirty self-intersecting loop workload",
            Self::HighValenceVertex => "catalog high valence vertex workload",
            Self::MixedSurfaceKillBox => "catalog mixed surface kill box workload",
            Self::OpenWire => "catalog open wire workload",
            Self::OpenSheet => "catalog open sheet workload",
            Self::OpenShellNmtEdgeFan => "catalog open shell NMT edge fan workload",
            Self::OpenLayerStack => "catalog open layer stack workload",
            Self::GrazingBasketStack => "catalog grazing open shell basket stack workload",
            Self::NmtTopologyConstruction => "catalog NMT topology construction workload",
            Self::TransformCycle => "catalog transform cycle workload",
            Self::RetainedCancellationChain => "catalog retained cancellation chain workload",
        }
    }

    pub fn is_admitted_now(self) -> bool {
        !matches!(self, Self::DirtySelfIntersectingLoop)
    }

    pub(crate) fn consumes_nmt_topology_construction(self) -> bool {
        matches!(
            self,
            Self::OpenWire
                | Self::MixedSurfaceKillBox
                | Self::OpenSheet
                | Self::OpenShellNmtEdgeFan
                | Self::OpenLayerStack
                | Self::GrazingBasketStack
                | Self::NmtTopologyConstruction
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadCatalogSupportPosture {
    Admitted,
    Unsupported,
}

impl WorkloadCatalogSupportPosture {
    pub fn query_key(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransformRecipe {
    MovementRotationStack,
    HostileCancellation,
    ReorientedMovementRotationStack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadTopologyBreadth {
    Default,
    SingleFaceLoopEdges { edge_count: usize },
    MultiFaceShell { face_count: usize },
    HighValenceVertex { valence: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RetainedReplayRecipe {
    StageReceiptOnly,
    RetainedCancellationChain,
}
