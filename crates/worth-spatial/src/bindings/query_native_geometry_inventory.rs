#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GeometryPublicSurface {
    GeometryTargetIdentity,
    SpatialAnchorSelection,
    PrimitiveBinding,
    PrimitiveAnchorBinding,
    PrimitiveRebinding,
    TopologyNeighborhoodReplacement,
    ToleranceAndPrecisionCertification,
    HistoricalGeometryInspection,
    BranchLocalGeometryInspection,
    GeometryReplayParity,
    GeometryRecoveryAction,
    GeometryProjectionConsumption,
}

impl GeometryPublicSurface {
    pub const fn all() -> [Self; 12] {
        [
            Self::GeometryTargetIdentity,
            Self::SpatialAnchorSelection,
            Self::PrimitiveBinding,
            Self::PrimitiveAnchorBinding,
            Self::PrimitiveRebinding,
            Self::TopologyNeighborhoodReplacement,
            Self::ToleranceAndPrecisionCertification,
            Self::HistoricalGeometryInspection,
            Self::BranchLocalGeometryInspection,
            Self::GeometryReplayParity,
            Self::GeometryRecoveryAction,
            Self::GeometryProjectionConsumption,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeometryTargetIdentity => "GeometryTargetIdentity",
            Self::SpatialAnchorSelection => "SpatialAnchorSelection",
            Self::PrimitiveBinding => "PrimitiveBinding",
            Self::PrimitiveAnchorBinding => "PrimitiveAnchorBinding",
            Self::PrimitiveRebinding => "PrimitiveRebinding",
            Self::TopologyNeighborhoodReplacement => "TopologyNeighborhoodReplacement",
            Self::ToleranceAndPrecisionCertification => "ToleranceAndPrecisionCertification",
            Self::HistoricalGeometryInspection => "HistoricalGeometryInspection",
            Self::BranchLocalGeometryInspection => "BranchLocalGeometryInspection",
            Self::GeometryReplayParity => "GeometryReplayParity",
            Self::GeometryRecoveryAction => "GeometryRecoveryAction",
            Self::GeometryProjectionConsumption => "GeometryProjectionConsumption",
        }
    }
}
