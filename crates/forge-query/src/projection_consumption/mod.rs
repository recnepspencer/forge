mod certification;
mod consumed;
mod contracts;
mod declaration;
mod declaration_authoring;
mod dx;
mod eligibility;
mod envelope;
mod extraction;
mod facts;
mod receipt;
mod receipt_transitions;
mod source;
mod support;

#[allow(unused_imports)]
pub use certification::{
    certify_projection_consumption_closeout_core, projection_consumption_family_inventory,
    projection_consumption_forbidden_fallback_audit,
    projection_consumption_phase_progression_digest, projection_consumption_proof_shape_audit,
    projection_consumption_public_boundary_audit, projection_consumption_support_matrix,
    ProjectionConsumptionCertificationBundle, ProjectionConsumptionCertificationCounterSnapshot,
    ProjectionConsumptionCertificationLane, ProjectionConsumptionCertificationRow,
    ProjectionConsumptionCertifiedSourceSurface, ProjectionConsumptionFamilyInventory,
    ProjectionConsumptionFamilyInventoryRow, ProjectionConsumptionForbiddenFallbackAudit,
    ProjectionConsumptionForbiddenFallbackAuditRow, ProjectionConsumptionForbiddenFallbackSeam,
    ProjectionConsumptionOrdinaryPathSurface, ProjectionConsumptionProofShapeAudit,
    ProjectionConsumptionProofShapeAuditRow, ProjectionConsumptionProofShapeEnforcement,
    ProjectionConsumptionProofShapeViolation, ProjectionConsumptionPublicBoundaryAudit,
    ProjectionConsumptionPublicBoundaryAuditRow, ProjectionConsumptionPublicBoundarySurface,
    ProjectionConsumptionSupportMatrix, ProjectionConsumptionSupportMatrixRow,
};
pub(crate) use certification::{
    intent_admission_admitted_projection_declaration,
    intent_admission_warning_projection_declaration,
};
pub use consumed::{
    ConsumedEffectContinuityFact, ConsumedEntityIdentityFact, ConsumedFieldValueFact,
    ConsumedMembershipFact, ConsumedProjectionFactSet, ConsumedRelationEndpointFact,
    ConsumedSourceReferenceFact, ConsumedTargetIdentityFact, ConsumedViewLocalIdentityFact,
    ProjectionFactExtractionCounters,
};
pub use contracts::{
    BoundProjectionFactFamily, MaterializedProjectionContract, ProjectionContractSourcePosture,
    ProjectionContractSupportPosture,
};
pub use declaration::{
    declare_projection_consumption, ProjectionConsumptionBindingContext,
    ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError,
};
pub use declaration_authoring::{
    ProjectionConsumptionAuthoringSurface, ProjectionConsumptionDeclarationBuilder,
};
pub use dx::{
    CompletedProjectionFactConsumption, ProjectionFactConsumptionAttempt,
    ProjectionFactConsumptionPathError,
};
pub use eligibility::{
    evaluate_projection_consumption_eligibility, AdmittedProjectionConsumption,
    DeferredProjectionConsumption, DeferredProjectionConsumptionReason,
    DeniedProjectionConsumption, ProjectionConsumptionDenialReason,
    ProjectionConsumptionEligibility, ProjectionConsumptionEligibilityCounters,
    ProjectionConsumptionEligibilityTrace, ProjectionConsumptionWarningKind,
    ProjectionConsumptionWarnings, SourceMismatchedProjectionConsumption,
};
pub use envelope::{
    ProjectionConsumptionEnvelopeSourceRefs, SelfDescribingProjectionConsumptionEnvelope,
};
pub use extraction::ProjectionFactExtractionError;
#[allow(unused_imports)]
pub use facts::{
    ProjectMaterializedFacts, ProjectionFactKind, ProjectionFactRequest,
    ProjectionMaterializedFactPosture, ProjectionMaterializedFactPostureKind,
};
pub use receipt::ProjectionConsumptionReceipt;
pub use receipt_transitions::{
    ProjectionConsumptionDeferredNeighborFamily, ProjectionConsumptionTransitionKind,
    ProjectionConsumptionTransitionPosture, ProjectionConsumptionTransitionRule,
    ProjectionConsumptionTransitionRules,
};
pub use source::{
    ProjectionConsumptionSource, ProjectionSourceFamily, ProjectionSourceReferenceIdentity,
};
pub use support::{
    discover_projection_consumption_support, ProjectionConsumptionSupportPosture,
    ProjectionConsumptionSupportReport, ProjectionConsumptionSupportRow,
};

#[cfg(test)]
mod tests;
