#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KernelCompiledProductQueryBoundaryLane {
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
}

impl KernelCompiledProductQueryBoundaryLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectionConsumption => "projection-consumption",
            Self::LowerRuntimeBoundaryEnvelope => "lower-runtime-boundary-envelope",
        }
    }
}
