pub use crate::aspects::{
    admit_authoritative_record_aspect_state, classify_aspect_contract_evolution,
    validate_aspect_value, AbsenceLaw, AspectContract, AspectContractRevision,
    AspectEquivalenceBasis, AspectEvolutionClassified, AspectEvolutionClassifiedContractArtifact,
    AspectEvolutionClassifiedContracts, AspectEvolutionKind, AspectEvolutionPolicy,
    AspectEvolutionVerdict, AspectIdentity, AspectKey, AspectMask, AspectMaskContract, AspectShape,
    AuthoritativePatchApplicationDenial, AuthoritativePatchConstructionDenial,
    AuthoritativeRecordAspectPatch, AuthoritativeRecordAspectState,
    AuthoritativeRecordAspectStateAdmitted, AuthoritativeRecordAspectStateArtifact,
    AuthoritativeStateAdmissionDenial, CanonicalAspectStateMap, CanonicalFieldPath,
    ContractValidatedAspectArtifact, ContractValidatedAspectValue,
    ContractValidatedAspectValueView, ContractValidationDenial, ContractValidationInput,
    DiagnosticMask, FieldDeclaration, FieldKey, FieldLevelAspectPatch, FieldRequirement,
    MaskAdmissibilityDenial, MutationMask, OpaqueAspectType, ProjectionMask, ReferenceAspectType,
    StructAspectShape, StructAspectValue, StructAspectValueConstructionDenial,
};
pub use crate::boundary::{
    declared_foundational_boundary, milestone1_compatibility_debt_inventory,
    milestone1_migration_readiness_report, milestone1_proof_seed_inventory,
    milestone1_public_api_inventory, FoundationalBoundaryArtifact, FoundationalBoundaryDeclaration,
    FoundationalBoundaryDeclared, Milestone1CompatibilityDebt, Milestone1MigrationReadinessReport,
    Milestone1ProofSeed, Milestone1PublicApiSurface,
};
pub use crate::canonicalization::{
    aspect_contract_digest_preparation_basis, aspect_mask_digest_preparation_basis,
    aspect_patch_digest_preparation_basis, aspect_state_digest_preparation_basis,
    prepare_aspect_contract_for_digest, prepare_aspect_mask_for_digest,
    prepare_aspect_patch_for_digest, prepare_aspect_state_for_digest,
    CanonicalDigestAspectShapeKind, CanonicalDigestMaskMode, CanonicalDigestPreparationEntry,
    DigestPreparationMaskMode, DigestPreparationReady, DigestPreparationReadyAspectContract,
    DigestPreparationReadyAspectContractArtifact, DigestPreparationReadyAspectMask,
    DigestPreparationReadyAspectMaskArtifact, DigestPreparationReadyAspectPatch,
    DigestPreparationReadyAspectPatchArtifact, DigestPreparationReadyAspectState,
    DigestPreparationReadyAspectStateArtifact, Milestone2DigestReadinessNote,
};
pub use crate::compatibility::{
    lower_json_aspect_value, lower_json_record_aspect_state, JsonCompatibilityAspectInput,
    JsonCompatibilityLoweringDeferred, JsonCompatibilityLoweringDenial,
    JsonCompatibilityLoweringFailure, JsonCompatibilityLoweringOutcome,
    JsonCompatibilityLoweringStale, JsonCompatibilityRebindRequired,
};
pub use crate::identities::{
    BoundaryArtifactId, BoundaryEpoch, BoundaryHandle, CanonicalDigestId, EquivalenceBasisId,
};
pub use crate::locators::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectMaskLocator,
    AspectValueLocator, BoundaryArtifactField, BoundaryArtifactLocator, BoundaryMismatchLocator,
    BoundarySourceLocator, LocatorAuthority,
};
pub use crate::values::{
    AspectValue, CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalString, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz,
    ContentRefId, EntityId, Generation, InternedString, LocalSlot, PartitionId, ScalarAspectType,
    Symbol,
};

/// A named implementation home that Milestone 1 expects to remain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponsibilityArea {
    name: &'static str,
    owns: &'static str,
    does_not_own: &'static str,
}

impl ResponsibilityArea {
    pub const fn new(name: &'static str, owns: &'static str, does_not_own: &'static str) -> Self {
        Self {
            name,
            owns,
            does_not_own,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn owns(&self) -> &'static str {
        self.owns
    }

    pub const fn does_not_own(&self) -> &'static str {
        self.does_not_own
    }
}

/// Responsibility topology exposed for Phase 1 certification.
pub fn foundational_responsibilities() -> [ResponsibilityArea; 6] {
    [
        crate::values::responsibility(),
        crate::aspects::responsibility(),
        crate::identities::responsibility(),
        crate::locators::responsibility(),
        crate::compatibility::responsibility(),
        crate::canonicalization::responsibility(),
    ]
}
