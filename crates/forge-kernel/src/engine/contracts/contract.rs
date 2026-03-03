//! Feature contract types for the pipeline.
//!
//! DOMAIN: Compiler-enforced declarations that every feature must provide.
//! These drive policy pre-validation, post-invariant checking, and audit
//! emission — all without the feature author writing boilerplate.
//!
//! DEPENDENCIES: forge-core (PolicyKind, KernelError)

use forge_core::{KernelError, PolicyKind};

/// Compiler-enforced contract that every feature must declare.
///
/// The `Feature` trait requires `FeatureContract` as a supertrait,
/// so implementing `Feature` without `FeatureContract` is a compile error.
pub trait FeatureContract {
    /// Machine-readable feature kind identifier (e.g. "make_cube", "boolean").
    fn feature_kind(&self) -> &'static str;

    /// Policy kinds that must be configured before this feature can execute.
    fn required_policies(&self) -> &[PolicyKind];

    /// What kind of topological operations this feature performs.
    fn entity_origins(&self) -> &[EntityOriginKind];

    /// Which specific Euler operators this feature invokes.
    fn euler_ops(&self) -> &[EulerOpKind];

    /// What kinds of surfaces this feature generates or interacts with.
    fn surface_types(&self) -> &[SurfaceKind];

    /// Invariants that must hold after successful execution.
    fn post_invariants(&self) -> &[InvariantKind];

    /// How much audit metadata to emit.
    fn audit_level(&self) -> AuditLevel;

    /// Whether this feature produces persistent output topology that should be tracked.
    fn persistent_output(&self) -> bool {
        true
    }

    /// Config overrides this feature requires.
    /// Default: no overrides (use cascade defaults).
    fn config_overrides(&self) -> Option<crate::configuration::facade::ConfigOverride> {
        None
    }

    /// Numerical conditioning mode for this feature.
    ///
    /// Controls whether the pipeline computes an `OperationSpace` and
    /// transforms input geometry into local coordinates before execution.
    /// Default: `None` (identity transform, zero cost).
    fn conditioning_mode(&self) -> ConditioningMode {
        ConditioningMode::None
    }
}

/// Numerical conditioning mode for a feature.
///
/// Declared in the feature contract, enforced by the pipeline.
/// The pipeline computes the `OperationSpace`, emits a `TracedDecision`
/// recording condition number and activation state, and transforms
/// input geometry in-place before calling `execute_typed`.
///
/// Features never see the transform — they execute in conditioned space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditioningMode {
    /// No conditioning needed. Identity `OperationSpace`. (primitives, simple ops)
    None,
    /// Single-solid analysis. (fillet, extrude on existing solid)
    UnaryAnalysis,
    /// Two-or-more-solid analysis. (boolean, merge)
    BinaryAnalysis,
}

/// How much audit metadata the pipeline emits for a feature.
///
/// Does NOT derive `PartialOrd` — all dispatch uses explicit `match`
/// to prevent silent ordering bugs when adding new levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLevel {
    /// No audit emission.
    None,
    /// Summary-level audit (decision counts, duration).
    Summary,
    /// Full audit (individual decisions, topology diffs).
    Full,
}

/// Post-execution invariants validated by the pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantKind {
    /// Every edge has exactly 2 uses, Euler formula holds.
    ManifoldEdges,
    /// Face normals agree at shared edges within tangency tolerance.
    G1Continuity,
    /// No self-intersection in the solid.
    NoSelfIntersection,
    /// No faces below the sliver area threshold.
    NoSliverFaces,
}

/// Classifies what kind of topological operations a feature uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityOriginKind {
    /// Creates entities via MEV/MEF/MVF etc.
    TopoOperator,
    /// Splits existing entities (boolean split phase).
    SplitOperator,
    /// Merges entities (region merge, join_faces).
    MergeOperator,
    /// Copies entities from one solid to another.
    CopyOperator,
    /// Batch-inserts entities via direct arena insertion (no Euler ops).
    DirectInsertion,
}

/// Specific Euler operators invoked by a feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulerOpKind {
    MakeVertexFace,
    MakeEdgeFace,
    SplitEdge,
    SweepFace,
    /// Direct arena insertion — bypasses Euler operator framework.
    DirectInsertion,
}

/// Geometric surface types handled by a feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    Planar,
    Cylinder,
    Sphere,
    Nurbs,
}

/// Marker trait for typed feature input DTOs.
///
/// Carries semantic validation beyond just parsing — called by the pipeline
/// AFTER `parse_inputs` and BEFORE `execute_typed`.
pub trait FeatureInputs {
    /// Validate semantic constraints on the parsed inputs.
    fn validate(&self) -> Result<(), KernelError>;
}
