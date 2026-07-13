mod certification;
mod consumed;
mod contracts;
mod declaration;
mod declaration_authoring;
mod downstream_authority;
mod dx;
mod eligibility;
mod envelope;
mod extraction;
mod facts;
mod identity;
mod receipt;
mod receipt_transitions;
mod source;
mod support;

#[allow(unused_imports)]
pub use certification::{
    certify_consumed_projection_authority, certify_projection_consumption_closeout_core,
    consumed_projection_authority_support_matrix, projection_consumption_family_inventory,
    projection_consumption_forbidden_fallback_audit,
    projection_consumption_phase_progression_digest, projection_consumption_proof_shape_audit,
    projection_consumption_public_boundary_audit, projection_consumption_support_matrix,
    ConsumedProjectionAuthorityCertificationBundle, ConsumedProjectionAuthorityCertificationLane,
    ConsumedProjectionAuthorityCertificationRow, ConsumedProjectionAuthorityComplexityEvidence,
    ConsumedProjectionAuthoritySupportMatrix, ConsumedProjectionAuthoritySupportRow,
    ConsumedProjectionAuthoritySupportStatus, ProjectionConsumptionCertificationBundle,
    ProjectionConsumptionCertificationCounterSnapshot, ProjectionConsumptionCertificationLane,
    ProjectionConsumptionCertificationRow, ProjectionConsumptionCertifiedSourceSurface,
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionFamilyInventoryRow,
    ProjectionConsumptionForbiddenFallbackAudit, ProjectionConsumptionForbiddenFallbackAuditRow,
    ProjectionConsumptionForbiddenFallbackSeam, ProjectionConsumptionOrdinaryPathSurface,
    ProjectionConsumptionProofShapeAudit, ProjectionConsumptionProofShapeAuditRow,
    ProjectionConsumptionProofShapeEnforcement, ProjectionConsumptionProofShapeViolation,
    ProjectionConsumptionPublicBoundaryAudit, ProjectionConsumptionPublicBoundaryAuditRow,
    ProjectionConsumptionPublicBoundarySurface, ProjectionConsumptionSupportMatrix,
    ProjectionConsumptionSupportMatrixRow,
};
pub(crate) use certification::{
    intent_admission_admitted_projection_declaration,
    intent_admission_warning_projection_declaration,
};
pub use consumed::{
    ConsumedContinuityAuthorityIdentity, ConsumedEffectContinuityFact, ConsumedEntityIdentityFact,
    ConsumedFieldValueFact, ConsumedMembershipFact, ConsumedProjectionFactSet,
    ConsumedRelationEndpointFact, ConsumedSourceReferenceFact, ConsumedTargetIdentityFact,
    ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
pub use contracts::{
    BoundProjectionFactFamily, MaterializedProjectionContract, ProjectionContractSourcePosture,
    ProjectionContractSupportPosture,
};
#[cfg(test)]
pub(crate) use declaration::test_authorized_field_paths;
pub use declaration::{
    declare_projection_consumption, ProjectionConsumptionBindingContext,
    ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError,
};
pub use declaration_authoring::{
    ProjectionConsumptionAuthoringSurface, ProjectionConsumptionDeclarationBuilder,
};
pub use downstream_authority::{
    downstream_authority_closure_contract, ConsumedProjectionAuthorityCounters,
    ConsumedProjectionAuthorityDenial, ConsumedProjectionAuthorityDenialKind,
    ConsumedProjectionAuthorityEvidence, DownstreamAuthorityClosureContract,
    DownstreamAuthorityClosureRole, DownstreamAuthorityClosureRow, ProjectionAuthorityContract,
    ProjectionAuthorityOutcome, ProjectionAuthorityRequirement,
    WorthQueryConsumedProjectionAuthority,
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
pub(crate) use facts::projection_fact_field_path_from_segments;
#[allow(unused_imports)]
pub use facts::{
    ProjectMaterializedFacts, ProjectionFactFieldPath, ProjectionFactKind, ProjectionFactRequest,
    ProjectionMaterializedFactPosture, ProjectionMaterializedFactPostureKind,
};
pub use receipt::ProjectionConsumptionReceipt;
pub use receipt_transitions::{
    ProjectionConsumptionDeferredNeighborFamily, ProjectionConsumptionTransitionKind,
    ProjectionConsumptionTransitionPosture, ProjectionConsumptionTransitionRule,
    ProjectionConsumptionTransitionRules,
};
pub use source::{
    ProjectionConsumptionSource, ProjectionSourceBasisAuthority, ProjectionSourceFamily,
    ProjectionSourceIdentity, ProjectionSourceReferenceIdentity,
};
pub use support::{
    discover_projection_consumption_support, ProjectionConsumptionSupportPosture,
    ProjectionConsumptionSupportReport, ProjectionConsumptionSupportRow,
};

#[cfg(test)]
mod tests;
