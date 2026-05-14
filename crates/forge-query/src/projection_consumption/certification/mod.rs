mod boundary;
mod bundle;
mod bundle_outputs;
mod fixtures;
mod oracles;
mod proof_artifacts;
mod proof_shape;
mod seeded;
mod slopes;
mod support_matrix;
mod surfaces;

pub use boundary::{
    projection_consumption_public_boundary_audit, ProjectionConsumptionPublicBoundaryAudit,
    ProjectionConsumptionPublicBoundaryAuditRow, ProjectionConsumptionPublicBoundarySurface,
};
pub use bundle::{
    certify_projection_consumption_closeout_core, ProjectionConsumptionCertificationBundle,
    ProjectionConsumptionCertificationLane, ProjectionConsumptionCertificationRow,
};
#[cfg(test)]
pub(crate) use proof_artifacts::{
    compile_fail_boundary_bundle_digest, golden_transcript_bundle_digest,
    projection_consumption_compile_fail_proofs, projection_consumption_golden_transcripts,
};
pub use proof_shape::{
    projection_consumption_phase_progression_digest, projection_consumption_proof_shape_audit,
    ProjectionConsumptionProofShapeAudit, ProjectionConsumptionProofShapeAuditRow,
    ProjectionConsumptionProofShapeEnforcement, ProjectionConsumptionProofShapeViolation,
};
pub use slopes::ProjectionConsumptionCertificationCounterSnapshot;
pub use support_matrix::{
    projection_consumption_family_inventory, projection_consumption_support_matrix,
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionFamilyInventoryRow,
    ProjectionConsumptionSupportMatrix, ProjectionConsumptionSupportMatrixRow,
};
pub use surfaces::ProjectionConsumptionCertifiedSourceSurface;
