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
mod native_contract;
mod receipt;
mod receipt_transitions;
mod source;
mod support;
mod visibility;
pub use certification::{
    certify_consumed_projection_authority, certify_projection_consumption_closeout_core,
    consumed_projection_authority_support_matrix, projection_consumption_family_inventory,
    projection_consumption_phase_progression_digest, projection_consumption_proof_shape_audit,
    projection_consumption_public_boundary_audit, projection_consumption_support_matrix,
    ConsumedProjectionAuthorityCertificationBundle, ConsumedProjectionAuthorityCertificationLane,
    ConsumedProjectionAuthorityCertificationRow, ConsumedProjectionAuthorityComplexityAxis,
    ConsumedProjectionAuthorityComplexityEvidence, ConsumedProjectionAuthorityComplexityRow,
    ConsumedProjectionAuthoritySupportMatrix, ConsumedProjectionAuthoritySupportRow,
    ConsumedProjectionAuthoritySupportStatus, ProjectionConsumptionCertificationBundle,
    ProjectionConsumptionCertificationCounterSnapshot, ProjectionConsumptionCertificationLane,
    ProjectionConsumptionCertificationRow, ProjectionConsumptionCertifiedSourceSurface,
    ProjectionConsumptionFamilyInventory, ProjectionConsumptionFamilyInventoryRow,
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
pub(crate) use consumed::ConsumedNativeLayoutProof;
pub use consumed::{
    ConsumedContinuityAuthorityIdentity, ConsumedEffectContinuityFact, ConsumedEntityIdentityFact,
    ConsumedFieldValueFact, ConsumedMembershipFact, ConsumedNativeRefinementDenial,
    ConsumedNativeValueView, ConsumedProjectionFactSet, ConsumedRelationEndpointFact,
    ConsumedSourceReferenceFact, ConsumedTargetIdentityFact, ConsumedViewLocalIdentityFact,
    ProjectionFactExtractionCounters,
};
#[cfg(test)]
pub(crate) use consumed::{
    ConsumedNativeValue, ConsumedProjectionContractProvenance, ConsumedProjectionFactInventory,
    ConsumedProjectionSourceTruth,
};
#[cfg(test)]
pub(crate) use contracts::bind_materialized_projection_contract;
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
pub use declaration_authoring::ProjectionConsumptionAuthoringSurface;
pub use downstream_authority::{
    downstream_authority_closure_contract, load_projection_authority_contract_document,
    ConsumedProjectionAuthorityCounters, ConsumedProjectionAuthorityDenial,
    ConsumedProjectionAuthorityDenialKind, ConsumedProjectionAuthorityEvidence,
    DownstreamAuthorityClosureContract, DownstreamAuthorityClosureRole,
    DownstreamAuthorityClosureRow, ExternalProjectionAuthorityContractDocument,
    ProjectionAuthorityContract, ProjectionAuthorityContractDocument,
    ProjectionAuthorityContractDocumentError, ProjectionAuthorityContractDocumentErrorKind,
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
    ProjectionConsumptionEligibility, ProjectionConsumptionEligibilityTrace,
    ProjectionConsumptionWarningKind, ProjectionConsumptionWarnings,
    SourceMismatchedProjectionConsumption,
};
pub use envelope::{
    ProjectionConsumptionEnvelopeSourceRefs, SelfDescribingProjectionConsumptionEnvelope,
};
pub use extraction::ProjectionFactExtractionError;
pub(crate) use facts::{projection_fact_field_path_from_segments, NativeFactDeclarationConflict};
pub use facts::{
    ProjectMaterializedFacts, ProjectionFactFieldPath, ProjectionFactKind, ProjectionFactRequest,
    ProjectionMaterializedFactPosture, ProjectionMaterializedFactPostureKind,
};
pub(crate) use native_contract::{
    DeclaredNativeAspectContractBasis, DeclaredNativeFactContract, DeclaredNativeFactContractDenial,
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
