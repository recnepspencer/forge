mod boundary;
mod forbidden_fallback;
#[cfg(test)]
mod lane_local_hostile;
mod proof_shape;
mod support_matrix;
mod surfaces;

pub use boundary::{
    projection_consumption_public_boundary_audit, ProjectionConsumptionPublicBoundaryAudit,
    ProjectionConsumptionPublicBoundaryAuditRow, ProjectionConsumptionPublicBoundarySurface,
};
pub use forbidden_fallback::{
    projection_consumption_forbidden_fallback_audit, ProjectionConsumptionForbiddenFallbackAudit,
};
#[cfg(test)]
pub(crate) use forbidden_fallback::{
    ProjectionConsumptionForbiddenFallbackSeam, ProjectionConsumptionOrdinaryPathSurface,
};
pub use proof_shape::{
    projection_consumption_phase_progression_digest, projection_consumption_proof_shape_audit,
    ProjectionConsumptionProofShapeAudit, ProjectionConsumptionProofShapeAuditRow,
    ProjectionConsumptionProofShapeEnforcement, ProjectionConsumptionProofShapeViolation,
};
pub use support_matrix::{
    projection_consumption_family_inventory, projection_consumption_support_matrix,
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionFamilyInventoryRow,
    ProjectionConsumptionSupportMatrix, ProjectionConsumptionSupportMatrixRow,
};
pub(super) use surfaces::representative_source;
pub use surfaces::ProjectionConsumptionCertifiedSourceSurface;
