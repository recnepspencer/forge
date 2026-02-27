//! Step contract declarations for the parametric Boolean pipeline.
//!
//! DOMAIN: Each Boolean phase is declared as a `StepContract` step with
//! its policy requirements and precision sensitivity. The pipeline executor
//! validates these automatically before running each phase.
//!
//! DEPENDENCIES: forge-core (PolicyKind), pipeline/step_contract (StepContract)
//!
//! CONSUMERS: `parametric/mod.rs` (pipeline execution)

/// Validate that both input solids are well-formed for a Boolean operation.
///
/// Checks twin reciprocity, minimum face counts, and input consistency.
/// No policies required — this is pure structural validation.
pub struct ValidateInputs;

crate::declare_step!(ValidateInputs,
    name: "boolean_validate_inputs",
    policies: [],
    precision_sensitive: false,
);

/// Split faces of both solids at their mutual intersection curves.
///
/// Uses exact predicates (D3) to determine where faces of solid A
/// cross faces of solid B, then inserts new edges and vertices at
/// those intersections.
pub struct SplitFaces;

crate::declare_step!(SplitFaces,
    name: "boolean_split_faces",
    policies: [forge_core::PolicyKind::CoincidentGeometry],
    precision_sensitive: true,
);

/// Classify each face as inside, outside, or on-boundary relative
/// to the other solid.
///
/// Uses ray-casting point-in-solid classification with exact
/// predicates. Coplanar faces are classified via normal alignment.
pub struct ClassifyFaces;

crate::declare_step!(ClassifyFaces,
    name: "boolean_classify_faces",
    policies: [
        forge_core::PolicyKind::CoincidentGeometry,
        forge_core::PolicyKind::NearTangency,
    ],
    precision_sensitive: true,
);

/// Select which faces to keep based on the Boolean operation type.
///
/// Union: keep outside faces from both solids.
/// Subtraction: keep outside faces from target, inside faces from tool (flipped).
/// Intersection: keep inside faces from both solids.
pub struct SelectFaces;

crate::declare_step!(SelectFaces,
    name: "boolean_select_faces",
    policies: [],
    precision_sensitive: false,
);

/// Assemble selected faces into result topology via halfedge stitching.
///
/// Creates a new `KernelDraft` transaction, copies selected faces,
/// and stitches shared edges. Atomic via D6.
pub struct AssembleResult;

crate::declare_step!(AssembleResult,
    name: "boolean_assemble_result",
    policies: [],
    precision_sensitive: false,
);

/// Postprocess the assembled result: merge coplanar faces, remove slivers,
/// close gaps, and validate the final topology.
pub struct Postprocess;

crate::declare_step!(Postprocess,
    name: "boolean_postprocess",
    policies: [forge_core::PolicyKind::SliverFace],
    precision_sensitive: true,
);
