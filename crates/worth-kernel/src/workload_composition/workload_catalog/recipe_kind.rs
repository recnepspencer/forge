#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadCatalogRecipeKind {
    Cube,
    Tetrahedron,
    SingleFaceLoop,
    CoplanarOverlapStorm,
    ThinFeatureWall,
    DirtySelfIntersectingLoop,
    HighValenceVertex,
    OpenSheet,
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
            Self::OpenSheet => "worth.catalog.open_sheet",
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
            Self::OpenSheet => "open sheet workload recipe",
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
            Self::OpenSheet => "catalog open sheet workload",
            Self::TransformCycle => "catalog transform cycle workload",
            Self::RetainedCancellationChain => "catalog retained cancellation chain workload",
        }
    }

    pub fn is_admitted_now(self) -> bool {
        !matches!(self, Self::DirtySelfIntersectingLoop)
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
pub(crate) enum RetainedReplayRecipe {
    StageReceiptOnly,
    RetainedCancellationChain,
}
