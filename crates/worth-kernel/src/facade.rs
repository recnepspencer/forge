//! Public API boundary for worth-kernel.

pub mod authoring;
pub mod certification;
pub mod diagnostics;
pub mod outcome;
pub mod prelude;

pub use authoring::construction::{
    primitive_construction_authoring, CanonicalPrimitiveConstructionArtifact, OrthotopeSpec,
    PrimitiveConstructionAuthoringSession, PrimitiveConstructionFamily,
    PrimitiveConstructionIntent, PrimitiveConstructionPhaseError,
    PrimitiveConstructionQueryEntryError, PrimitiveConstructionRequest, RegularPrismSpec,
    RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec, WireBodySpec,
};
pub use authoring::intents::{
    AnchorMatchSpatialIntent, ConstraintMoveSpatialIntent, ConstraintReorientSpatialIntent,
    CreateSpatialIntent, LiesOnSpatialIntent, MoveSpatialIntent, OffsetSpatialIntent,
    PointsTowardSpatialIntent, PrimitiveConstructionSpatialIntentError, ReorientSpatialIntent,
    RotateSpatialIntent, SpatialAuthoredActKind,
};
pub use authoring::policy::{
    SpatialArbitrationPosture, SpatialIntentPolicyProfile, SpatialIntentPolicyProfileOverride,
    SpatialPreviewRichness, SpatialThresholdPosture,
};
pub use outcome::{
    PreparedPrimitiveConstructionResult, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionResultError,
};
