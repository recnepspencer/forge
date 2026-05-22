mod compile_fail_report;
mod digest_protocol;
mod digest_protocol_report;
mod proof_grade;
mod substrate_closeout_report;
mod truth_projection_matrix;
mod verified_artifact_surface_report;

pub(crate) use compile_fail_report::PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES;
pub use compile_fail_report::{
    prepare_primitive_construction_proof_boundary_compile_fail_report,
    PrimitiveConstructionProofBoundaryCompileFailFixture,
    PrimitiveConstructionProofBoundaryCompileFailReport,
};
pub(crate) use digest_protocol::{
    digest_owned_parts, digest_owned_parts_with_scope, ConstructionDigestScope,
};
pub use digest_protocol_report::{
    prepare_primitive_construction_digest_protocol_report,
    PrimitiveConstructionDigestProtocolReport,
};
pub use proof_grade::{PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject};
pub use substrate_closeout_report::{
    prepare_primitive_construction_proof_substrate_closeout_report,
    PrimitiveConstructionProofSubstrateCloseoutReport,
    PrimitiveConstructionProofSubstrateCloseoutReportError,
    PrimitiveConstructionProofSubstrateCloseoutVerificationFailure,
    PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch,
};
pub use truth_projection_matrix::{
    prepare_primitive_construction_truth_projection_matrix,
    PrimitiveConstructionTruthProjectionMatrix, PrimitiveConstructionTruthProjectionRow,
};
pub use verified_artifact_surface_report::{
    prepare_primitive_construction_verified_artifact_surface_report,
    PrimitiveConstructionVerifiedArtifactSurfaceReport,
    PrimitiveConstructionVerifiedArtifactSurfaceRow,
};
