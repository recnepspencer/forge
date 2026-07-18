mod audits;
mod bundle;
mod bundle_outputs;
mod downstream_authority_bundle;
mod downstream_authority_complexity;
mod downstream_authority_support;
mod fixtures;
mod grouped_projection_contract;
mod intent_admission_fixtures;
mod oracle;
mod proof_artifacts;
mod seeded;
mod slopes;

pub use audits::{
    projection_consumption_family_inventory, projection_consumption_phase_progression_digest,
    projection_consumption_proof_shape_audit, projection_consumption_public_boundary_audit,
    projection_consumption_support_matrix, ProjectionConsumptionCertifiedSourceSurface,
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionFamilyInventoryRow,
    ProjectionConsumptionProofShapeAudit, ProjectionConsumptionProofShapeAuditRow,
    ProjectionConsumptionProofShapeEnforcement, ProjectionConsumptionProofShapeViolation,
    ProjectionConsumptionPublicBoundaryAudit, ProjectionConsumptionPublicBoundaryAuditRow,
    ProjectionConsumptionPublicBoundarySurface, ProjectionConsumptionSupportMatrix,
    ProjectionConsumptionSupportMatrixRow,
};
#[cfg(test)]
pub(crate) use audits::{
    projection_consumption_forbidden_fallback_audit, ProjectionConsumptionForbiddenFallbackSeam,
    ProjectionConsumptionOrdinaryPathSurface,
};
pub use bundle::{
    certify_projection_consumption_closeout_core, ProjectionConsumptionCertificationBundle,
    ProjectionConsumptionCertificationLane, ProjectionConsumptionCertificationRow,
};
pub use downstream_authority_bundle::{
    certify_consumed_projection_authority, ConsumedProjectionAuthorityCertificationBundle,
    ConsumedProjectionAuthorityCertificationLane, ConsumedProjectionAuthorityCertificationRow,
};
pub use downstream_authority_complexity::{
    ConsumedProjectionAuthorityComplexityAxis, ConsumedProjectionAuthorityComplexityEvidence,
    ConsumedProjectionAuthorityComplexityRow,
};
pub use downstream_authority_support::{
    consumed_projection_authority_support_matrix, ConsumedProjectionAuthoritySupportMatrix,
    ConsumedProjectionAuthoritySupportRow, ConsumedProjectionAuthoritySupportStatus,
};
pub(crate) use intent_admission_fixtures::{
    intent_admission_admitted_projection_declaration,
    intent_admission_warning_projection_declaration,
};
pub use slopes::ProjectionConsumptionCertificationCounterSnapshot;
