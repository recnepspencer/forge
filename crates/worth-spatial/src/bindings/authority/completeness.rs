#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialBindingCompleteness {
    Complete,
    Incomplete(SpatialBindingIncompleteness),
}

impl SpatialBindingCompleteness {
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialBindingIncompleteness {
    MissingSupportPlane,
    MissingVertexGeometry,
    CurveWitnessRequiresAtLeastTwoVertices,
    PCurveWitnessRequiresPlanarSupport,
}
