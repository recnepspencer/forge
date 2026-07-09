#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalBasisDomain {
    Value,
    AspectContract,
    AspectMask,
    AuthoritativeState,
    AuthoritativePatch,
    Identity,
    Locator,
    Profile,
    Performance,
    BoundaryArtifact,
    Transition,
    Diagnostic,
    CompatibilityLowering,
    Future(&'static str),
}
