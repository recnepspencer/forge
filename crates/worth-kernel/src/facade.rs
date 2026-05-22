//! Public API boundary for worth-kernel.

pub mod authoring;
pub mod certification;
pub mod diagnostics;
pub mod outcome;
pub mod prelude;

pub use authoring::construction::{
    build_canonical_primitive_construction_artifact, lower_scaffold_to_topology,
    primitive_construction_authoring, AdmittedPrimitiveConstructionIntent,
    CanonicalPrimitiveConstructionArtifact, OrthotopeSpec, PrimitiveConstructionArtifactError,
    PrimitiveConstructionAuthoringSession, PrimitiveConstructionFamily,
    PrimitiveConstructionIntent, PrimitiveConstructionPhaseError, PrimitiveConstructionRequest,
    PrimitiveConstructionScaffold, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec,
    SimplexSolidSpec, WireBodySpec,
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
    prepare_primitive_construction_outcome, prepare_primitive_construction_result,
    PreparedPrimitiveConstructionResult, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionResultError,
};
