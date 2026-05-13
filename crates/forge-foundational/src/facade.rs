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
    certify_milestone1_production_test_readiness, declared_foundational_boundary,
    milestone1_compatibility_debt_inventory, milestone1_migration_readiness_report,
    milestone1_proof_seed_inventory, milestone1_public_api_inventory,
    require_milestone1_production_test_readiness, FoundationalBoundaryArtifact,
    FoundationalBoundaryDeclaration, FoundationalBoundaryDeclared, Milestone1CompatibilityDebt,
    Milestone1MigrationReadinessReport, Milestone1ProductionReadinessAuthority,
    Milestone1ProductionReadinessCertified, Milestone1ProductionReadinessScope,
    Milestone1ProductionTestReady, Milestone1ProductionTestReadyArtifact, Milestone1ProofSeed,
    Milestone1PublicApiSurface,
};
pub use crate::canonicalization::{
    admit_canonical_bundle_digest_derivation, admit_canonical_export_digest_derivation,
    admit_canonical_sequence_digest_derivation, aspect_contract_digest_preparation_basis,
    aspect_mask_digest_preparation_basis, aspect_patch_digest_preparation_basis,
    aspect_state_digest_preparation_basis, bridge_canonical_export_trust_boundary,
    canonical_milestone2_production_readiness_report,
    certify_canonical_milestone2_production_readiness, compare_canonical_basis,
    compare_canonical_exports, derive_canonical_digest,
    diagnostic_mask_locator_canonical_basis_entries, identity_canonical_basis_entries,
    locator_canonical_basis_entries, mutation_mask_locator_canonical_basis_entries,
    prepare_aspect_contract_for_canonical_basis, prepare_aspect_contract_for_digest,
    prepare_aspect_mask_for_canonical_basis, prepare_aspect_mask_for_digest,
    prepare_aspect_patch_for_canonical_basis, prepare_aspect_patch_for_digest,
    prepare_aspect_state_for_canonical_basis, prepare_aspect_state_for_digest,
    prepare_canonical_basis_bundle, prepare_canonical_basis_sequence, prepare_canonical_comparison,
    prepare_canonical_export_bundle, prepare_identity_for_canonical_basis,
    prepare_locator_for_canonical_basis, projection_mask_locator_canonical_basis_entries,
    readmit_canonical_export_after_boundary, require_canonical_production_test_readiness,
    BoundaryBridgedCanonicalExportArtifact, CanonicalBasisBundle, CanonicalBasisConstructionDenial,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryId, CanonicalBasisEntryKind,
    CanonicalBasisLocus, CanonicalBasisReadinessProofs, CanonicalBasisReady,
    CanonicalBasisReadyArtifact, CanonicalBasisSequence, CanonicalBasisValue,
    CanonicalBundleReadinessProofs, CanonicalBundleReady, CanonicalBundleReadyArtifact,
    CanonicalCertifiedSurface, CanonicalCertifiedSurfaceEvidence, CanonicalComparisonInput,
    CanonicalComparisonOutcome, CanonicalComparisonReadinessProofs, CanonicalComparisonReady,
    CanonicalComparisonReadyArtifact, CanonicalCompileFailBoundary, CanonicalCostCounterEvidence,
    CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalDigestAlgorithmMetadata,
    CanonicalDigestAlgorithmSlot, CanonicalDigestAspectShapeKind, CanonicalDigestBasisBundle,
    CanonicalDigestBasisSequence, CanonicalDigestDebt, CanonicalDigestDerivationDenial,
    CanonicalDigestDerivationInput, CanonicalDigestDerivationReadinessProofs,
    CanonicalDigestDerivationReady, CanonicalDigestDerivationReadyArtifact,
    CanonicalDigestInputDomain, CanonicalDigestInputEvidence, CanonicalDigestInputId,
    CanonicalDigestInputShape, CanonicalDigestInputShapeBound, CanonicalDigestMaskMode,
    CanonicalDigestMetadata, CanonicalDigestOutputShape, CanonicalDigestPreparationEntry,
    CanonicalDigestValue, CanonicalDomainBundleDigestAlgorithmSlot,
    CanonicalDomainBundleDigestInput, CanonicalDomainCoherence, CanonicalEquivalenceBasis,
    CanonicalEquivalenceBasisDeclared, CanonicalEquivalentBasis, CanonicalExportBasisBundle,
    CanonicalExportBasisSequence, CanonicalExportBundle, CanonicalExportBundleDigestAlgorithmSlot,
    CanonicalExportBundleDigestInput, CanonicalExportComparisonOutcome, CanonicalExportDebt,
    CanonicalExportHarnessSeed, CanonicalExportManifest, CanonicalExportManifestBound,
    CanonicalExportManifestMismatch, CanonicalExportManifestMismatchKind,
    CanonicalExportManifestRow, CanonicalExportReadinessProofs,
    CanonicalExportReadmissionAuthority, CanonicalExportReady, CanonicalExportReadyArtifact,
    CanonicalFixtureManifestEvidence, CanonicalFloatWidth, CanonicalGoldenArtifactEvidence,
    CanonicalHarnessExpansionPoint, CanonicalIdentityInput, CanonicalIntegerWidth,
    CanonicalLocatorInput, CanonicalMilestone2PhaseGate, CanonicalMismatchBasis,
    CanonicalMismatchKind, CanonicalMismatchLociBound, CanonicalPhaseGateEvidence,
    CanonicalProducerShape, CanonicalProductionReadinessAuthority,
    CanonicalProductionReadinessCertified, CanonicalProductionReadinessReport,
    CanonicalProductionReadinessScope, CanonicalProductionTestReady,
    CanonicalProductionTestReadyArtifact, CanonicalPropertySeed, CanonicalPropertySeedEvidence,
    CanonicalResidualDebt, CanonicalRuleVersionBound, CanonicalRuntimeAssumption,
    CanonicalRuntimeNonAssumption, CanonicalSingleSequenceDigestAlgorithmSlot,
    CanonicalSingleSequenceDigestInput, CanonicalSyntheticRuntimePressure, CanonicalizationCost,
    CanonicalizationCostObserved, CanonicalizationRuleVersion, DigestPreparationMaskMode,
    DigestPreparationReady, DigestPreparationReadyAspectContract,
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
pub use crate::profiles::{
    admit_requested_foundational_profile, attach_boundary_profiled_artifact,
    attach_proof_bearing_profiled_artifact, attach_support_profiled_artifact,
    boundary_artifact_surface_inventory,
    bridge_evidence_backed_proof_bearing_artifact_trust_boundary,
    bridge_production_certified_proof_bearing_artifact_trust_boundary,
    certify_evidence_backed_proof_bearing_artifact,
    certify_foundational_profile_milestone3_production_test_readiness,
    certify_production_certified_proof_bearing_artifact,
    classify_foundational_profile_compatibility, compare_foundational_profile_identities,
    compare_foundational_profiles, derive_foundational_profile_identity,
    foundational_profile_applicability, foundational_profile_canonical_basis_entries,
    foundational_profile_certification_authority, foundational_profile_certification_proof_lane,
    foundational_profile_certification_readmission_authority,
    foundational_profile_milestone3_readiness_report, foundational_profile_progression_authority,
    materialize_admitted_foundational_profile, plan_foundational_profile_materialization,
    plan_foundational_profile_materialization_with_elision,
    plan_selected_foundational_profile_materialization,
    prepare_admitted_foundational_profile_for_canonical_basis,
    proof_bearing_artifact_surface_inventory,
    readmit_evidence_backed_proof_bearing_artifact_after_boundary,
    readmit_production_certified_proof_bearing_artifact_after_boundary,
    request_foundational_profile_set,
    require_foundational_profile_milestone3_production_test_readiness,
    support_artifact_surface_inventory, AdmissionReadinessProfile,
    AdmittedFoundationalProfileArtifact, AdmittedFoundationalProfilePhase,
    AdmittedFoundationalProfileSet, BoundaryArtifactTarget,
    BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact,
    BoundaryBridgedProductionCertifiedProofBearingArtifact, BoundaryProfiledArtifact,
    CertificationPostureProfile, CompatibilityPostureProfile, DiagnosticRichnessProfile,
    EvidenceBackedCertifiedProofBearingArtifact, FoundationalDescriptiveElisionProfile,
    FoundationalDescriptiveSurface, FoundationalMaterializationCost,
    FoundationalMaterializationPlanningDenial, FoundationalProfileApplicability,
    FoundationalProfileAttachmentDenial, FoundationalProfileAttachmentOutcome,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileAttachmentTargetMarker,
    FoundationalProfileCertificationAuthority, FoundationalProfileCertificationDenial,
    FoundationalProfileCertificationOutcome, FoundationalProfileCertificationProofLane,
    FoundationalProfileCertificationReadmissionAuthority, FoundationalProfileCertifiedSurface,
    FoundationalProfileCertifiedSurfaceEvidence, FoundationalProfileCompatibilityClass,
    FoundationalProfileCompileFailBoundary, FoundationalProfileCompositionDenial,
    FoundationalProfileDecisionKind, FoundationalProfileDifferenceReport,
    FoundationalProfileFamily, FoundationalProfileForgeProofApi,
    FoundationalProfileForgeProofForbiddenSurface, FoundationalProfileForgeProofSurface,
    FoundationalProfileIdentity, FoundationalProfileIdentityDenial,
    FoundationalProfileMaterializationPlan, FoundationalProfileMilestone3PhaseGate,
    FoundationalProfileNarrowingKind, FoundationalProfileNarrowingRecord,
    FoundationalProfilePhaseGateEvidence, FoundationalProfileProductionReadinessAuthority,
    FoundationalProfileProductionReadinessCertified, FoundationalProfileProductionReadinessReport,
    FoundationalProfileProductionReadinessScope, FoundationalProfileProductionTestReady,
    FoundationalProfileProductionTestReadyArtifact, FoundationalProfileProgressionAuthority,
    FoundationalProfileProgressionDeferred, FoundationalProfileProgressionDenial,
    FoundationalProfileProgressionFailure, FoundationalProfileProgressionOutcome,
    FoundationalProfileProgressionRebindRequired, FoundationalProfileProgressionStale,
    FoundationalProfileResidualDebt, FoundationalProfileRuntimeAssumption,
    FoundationalProfileRuntimeNonAssumption, FoundationalProfileSet, FoundationalProfileSetInput,
    FoundationalProfileSyntheticRuntimePressure, FoundationalProfiledArtifact,
    FoundationalSurfaceAbsenceCause, FoundationalSurfaceAvailabilityDecision,
    FoundationalTargetSurfaceInventory, MaterializedFoundationalProfileArtifact,
    MaterializedFoundationalProfilePhase, MaterializedFoundationalProfileSet,
    ProductionCertifiedProofBearingArtifact, ProofBearingArtifactTarget,
    ProofBearingProfiledArtifact, RequestedFoundationalProfileArtifact,
    RequestedFoundationalProfilePhase, RequestedFoundationalProfileSet, RetentionDeliveryProfile,
    SupportArtifactTarget, SupportPostureProfile, SupportProfiledArtifact,
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
pub fn foundational_responsibilities() -> [ResponsibilityArea; 7] {
    [
        crate::values::responsibility(),
        crate::aspects::responsibility(),
        crate::identities::responsibility(),
        crate::locators::responsibility(),
        crate::compatibility::responsibility(),
        crate::canonicalization::responsibility(),
        crate::profiles::responsibility(),
    ]
}
