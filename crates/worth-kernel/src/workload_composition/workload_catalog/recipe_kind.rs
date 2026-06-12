#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadCatalogRecipeKind {
    Cube,
    Tetrahedron,
    SingleFaceLoop,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadTopologyBreadth {
    Default,
    MultiFaceShell { face_count: usize },
    HighValenceVertex { valence: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RetainedReplayRecipe {
    StageReceiptOnly,
    RetainedCancellationChain,
}
