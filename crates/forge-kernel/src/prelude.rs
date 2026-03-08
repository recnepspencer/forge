pub use crate::context::facade::ModelingContext;
pub use crate::engine::facade::FeatureTree;
pub use crate::engine::facade::SolidEnvelope;
pub use crate::engine::facade::SpecEnvelope;
pub use crate::engine::facade::compute_spec_pipeline_fingerprint;
pub use crate::engine::facade::{validate_spec_envelope_invariant, validate_spec_invariant};
pub use crate::geometry::facade::GeometryStore;
pub use crate::proof::{
    ValidationCheckpoint, ValidationConfig, ValidationResult, run_spec_checkpoint,
    run_spec_envelope_checkpoint, validate_spec_envelope_structure, validate_spec_structure,
};
pub use crate::operations::facade::{
    execute_boolean, execute_boolean_direct, BooleanInput, BooleanOp, BooleanResult,
};
pub use crate::operations::primitives::{
    build_halfedge_mesh, make_block, make_convex_solid, make_cube, make_dodecahedron, make_prism,
    make_pyramid, make_tetrahedron, make_wedge,
};
