/// Stable machine-readable registration diagnostic code.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityDiagnosticCode {
    DuplicateCapabilityId,
    UnsupportedPostureReference,
    MissingComponentPropSchema,
    MissingComponentStateOwnership,
    IllegalComponentChildPolicy,
    UnsupportedSurfacePlacementClass,
    InvalidSurfaceStateClass,
    ProductDomainSurfaceKind,
    MissingMosaicRegionSizingBehavior,
    MissingMosaicRegionScrollOwnership,
    MissingMosaicRegionFocusScope,
    MissingMosaicRegionChildRule,
    MissingMosaicRegionAllowedSurfaceClass,
    MissingMosaicRegionPersistence,
    MissingMosaicRegionClipping,
    MissingMosaicRegionHitTest,
    ProductDomainMosaicRegionRole,
    UnsupportedMosaicRegionSurfaceClass,
    MissingDependency,
    FamilyMismatch,
}
