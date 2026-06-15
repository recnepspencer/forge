#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanReadinessRequiredStage {
    Topology,
    GeometryBinding,
    SurfaceSupport,
    Projection,
    Transform,
    RetainedReplay,
    ProjectionFactParity,
    Diagnostics,
    UserResponse,
    ContractBundle,
}

impl PlanarBooleanReadinessRequiredStage {
    pub const ALL: [Self; 10] = [
        Self::Topology,
        Self::GeometryBinding,
        Self::SurfaceSupport,
        Self::Projection,
        Self::Transform,
        Self::RetainedReplay,
        Self::ProjectionFactParity,
        Self::Diagnostics,
        Self::UserResponse,
        Self::ContractBundle,
    ];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Topology => "topology evidence",
            Self::GeometryBinding => "geometry binding evidence",
            Self::SurfaceSupport => "surface support evidence",
            Self::Projection => "projection evidence",
            Self::Transform => "transform evidence",
            Self::RetainedReplay => "retained replay evidence",
            Self::ProjectionFactParity => "projection fact parity receipt",
            Self::Diagnostics => "diagnostic evidence",
            Self::UserResponse => "user response evidence",
            Self::ContractBundle => "boolean-readiness contract bundle",
        }
    }
}
