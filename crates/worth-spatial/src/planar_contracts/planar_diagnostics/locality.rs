#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticTriggerLocality {
    PredicateAuthority,
    TopologyContract,
    BindingOrRebinding,
    PolicyBoundary,
    ProjectionBasis,
    RetainedTransformStep,
    MotionOrRotationPosture,
    UnsupportedPlanarClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticTruthEffect {
    DoesNotChangePlanarTruth,
}
