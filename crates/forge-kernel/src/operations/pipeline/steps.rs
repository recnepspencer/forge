//! Reusable step structs for the operation pipeline (Tier 3).
//!
//! DOMAIN: Cross-feature step declarations. Each step wraps an existing
//! function from forge-geom, forge-topo, or forge-kernel and declares
//! its policy requirements and precision sensitivity via `StepContract`.
//!
//! These steps are not implementations — they are contract declarations.
//! The actual work is done by the functions they wrap, called from within
//! `OperationPipeline::run_step` closures by each feature's `execute_typed`.
//!
//! CONSUMERS: operations/boolean, operations/fillet, operations/chamfer, etc.

/// Resolve persistent entity references (face IDs, edge IDs) from
/// a selection specification.
///
/// Wraps: forge-topo entity lookup / selection resolution
pub struct ResolvePersistentSelection;

crate::declare_step!(ResolvePersistentSelection,
    name: "resolve_persistent_selection",
    policies: [],
    precision_sensitive: false,
);

/// Classify the geometric relationship between two surfaces
/// (coincident, disjoint, tangent, general intersection).
///
/// Wraps: crate::geom_facade::classify_surface_pair
pub struct ClassifySurfacePair;

crate::declare_step!(ClassifySurfacePair,
    name: "classify_surface_pair",
    policies: [
        forge_core::PolicyKind::CoincidentGeometry,
        forge_core::PolicyKind::NearTangency,
    ],
    precision_sensitive: true,
);

/// Classify edge convexity for fillet/chamfer operations
/// (convex, concave, smooth).
///
/// Wraps: future forge-topo edge convexity analysis
pub struct ClassifyEdgeConvexity;

crate::declare_step!(ClassifyEdgeConvexity,
    name: "classify_edge_convexity",
    policies: [forge_core::PolicyKind::NearTangency],
    precision_sensitive: true,
);

/// Certify that a boundary loop is geometrically valid
/// (closed, non-self-intersecting, consistent winding).
///
/// Wraps: crate::geom_facade::cert::certify_boundary
pub struct CertifyBoundary;

crate::declare_step!(CertifyBoundary,
    name: "certify_boundary",
    policies: [forge_core::PolicyKind::CoincidentGeometry],
    precision_sensitive: true,
);

/// Construct a surface from geometric constraints
/// (planar, NURBS, analytic, swept, lofted).
///
/// Wraps: future surface construction routines
pub struct ConstructSurface;

crate::declare_step!(ConstructSurface,
    name: "construct_surface",
    policies: [],
    precision_sensitive: true,
);

/// Apply Euler operators to create/modify topology transactionally.
/// All changes happen on a `MutableDraft` and are committed atomically (D6).
///
/// Wraps: forge_topo::operations Euler operators via MutableDraft
pub struct ApplyEulerOps;

crate::declare_step!(ApplyEulerOps,
    name: "apply_euler_ops",
    policies: [],
    precision_sensitive: false,
);

/// Validate that all edges in the result topology are manifold
/// (exactly 2 uses per edge, Euler formula holds).
///
/// Wraps: forge_topo::validate::validate_topology
pub struct ValidateManifold;

crate::declare_step!(ValidateManifold,
    name: "validate_manifold",
    policies: [],
    precision_sensitive: false,
);

/// Detect sliver faces below the area threshold.
///
/// Wraps: forge_spatial::integrity::sliver::analyze_slivers
pub struct DetectSlivers;

crate::declare_step!(DetectSlivers,
    name: "detect_slivers",
    policies: [forge_core::PolicyKind::SliverFace],
    precision_sensitive: true,
);
