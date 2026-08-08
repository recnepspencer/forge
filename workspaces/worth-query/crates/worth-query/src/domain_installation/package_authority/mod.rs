mod admission;
mod admission_profile;
mod artifact_installation_support;
mod identity;
mod package;
mod package_definitions;
mod portable_validation;
mod validation;

use super::WorthQueryInstalledDomainAuthorityWitness;

#[cfg(test)]
pub(crate) use admission::admit_domain_package;
pub(crate) use admission::admit_domain_package_with_artifact_support;
pub(crate) use admission::WorthQueryAdmittedDomainPackage;
pub use admission::{
    WorthQueryDomainPackageAdmissionDenial, WorthQueryDomainPackageAdmissionDenialKind,
};
pub use artifact_installation_support::WorthQueryArtifactInstallationSupport;
pub use identity::{
    WorthQueryDomainIdentityComponentError, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace,
    WorthQueryDomainPackageIdentity, WorthQueryDomainSemanticVersion,
};
pub use package::WorthQueryDomainPackage;
pub use package_definitions::{
    aftermath_owner_identity_digest, AftermathLoweringCorrespondenceCatalog,
    InstalledAftermathPostcondition, InstalledCorrectionMechanism, InstalledLoweringCorrespondence,
    PublishedAftermathPosture, WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence,
    WorthQueryComparatorFamily, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryConditionalConsequenceRole,
    WorthQueryConditionalEvaluationCondition, WorthQueryConditionalGraphReadRole,
    WorthQueryConditionalNodeContext, WorthQueryConditionalNodeLocation,
    WorthQueryConditionalNodeOutput, WorthQueryConditionalNodeRole, WorthQueryConditionalTouchRole,
    WorthQueryConditionalTrigger, WorthQueryDeltaComparisonDomain, WorthQueryDeltaThreshold,
    WorthQueryDomainConditionFamily, WorthQueryDomainDeclarationFamilyDefinition,
    WorthQueryDomainEvidenceContract, WorthQueryDomainGraphReadOperationDefinition,
    WorthQueryDomainInvariantDefinition, WorthQueryDomainInvariantPredicate,
    WorthQueryDomainOperationDefinition, WorthQueryDomainOperationIdentity,
    WorthQueryDomainOperationSemanticClosure, WorthQueryInstalledAftermathContract,
    WorthQueryMaintenancePosture, WorthQueryOnDemandTriggerFamily,
    WorthQueryOperationCapabilityRequirement, WorthQueryOperationCollectionContract,
    WorthQueryOperationCollectionField, WorthQueryOperationConditionalDimension,
    WorthQueryOperationContinuationPosture, WorthQueryOperationCostClass,
    WorthQueryOperationCostContract, WorthQueryOperationEffectContract,
    WorthQueryOperationEffectFamily, WorthQueryOperationFailureClass,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadContract, WorthQueryOperationGraphReadRole,
    WorthQueryOperationGroupingContract, WorthQueryOperationInvariantContract,
    WorthQueryOperationLineageContract, WorthQueryOperationLoweringContract,
    WorthQueryOperationNativeProjectionContract, WorthQueryOperationParameterContract,
    WorthQueryOperationParameterField, WorthQueryOperationProjectionConsumptionContract,
    WorthQueryOperationProjectionRole, WorthQueryOperationPromotionContract,
    WorthQueryOperationPublicationContract, WorthQueryOperationReplayComparatorContract,
    WorthQueryOperationReplayContract, WorthQueryOperationReplayNoiseContract,
    WorthQueryOperationRequiredDomainRole, WorthQueryOperationResultState,
    WorthQueryOperationSupportRequirements, WorthQueryOperationTerminalContract,
    WorthQueryOperationTouchContract, WorthQueryOperationValueFamily,
    WorthQueryOperationWindowPolicy, WorthQueryOperationWorkflowContract,
    WorthQueryOutputEquivalenceRequirement, WorthQueryOutputRelationship,
    WorthQueryPortableConditionParameter, WorthQueryPortableConditionParameterValue,
    WorthQueryPortableConditionalDependencyLocation, WorthQueryPortableConditionalDependencyPart,
    WorthQueryPortableConditionalDimension, WorthQueryPortableConditionalNodeDeclaration,
    WorthQueryPortableConditionalOutputPart, WorthQueryPortableOperationComparisonMismatchCategory,
    WorthQueryPortableOperationCostDimension, WorthQueryPortableOperationDimension,
    WorthQueryPortableOperationSupportDimension, WorthQueryPortableWorkflowDefinition,
    WorthQueryPortableWorkflowStage, WorthQueryQuantityUnit, WorthQueryQuantityValueFamily,
    WorthQuerySemanticDependencyCanonicalBasis, WorthQuerySemanticLocality,
    WorthQuerySemanticTruthDependency, WorthQuerySemanticTruthDependencyDenial,
    WorthQuerySupportRequirement, WorthQueryTemporalCondition, WorthQueryTemporalWake,
    WorthQueryThresholdBoundary, WorthQueryTruthPartitionRole, WorthQueryTypedFamilyIdentity,
    WorthQueryWorkflowCostRole, WorthQueryWorkflowStageSemantics, WorthQueryWorkflowValueContract,
};
pub use package_definitions::{
    WorthQueryArtifactAccessPathContract, WorthQueryArtifactBorrowPosture,
    WorthQueryArtifactBulkProjectionContract, WorthQueryArtifactCarriageContract,
    WorthQueryArtifactChunkContract, WorthQueryArtifactClassification,
    WorthQueryArtifactCloneBoundary, WorthQueryArtifactCloneMechanism,
    WorthQueryArtifactClonePosture, WorthQueryArtifactComparatorFamily,
    WorthQueryArtifactComparisonAuthority, WorthQueryArtifactCompatibilityContract,
    WorthQueryArtifactCompatibilityWindow, WorthQueryArtifactContentIdentityContract,
    WorthQueryArtifactContractIdentity, WorthQueryArtifactContractReference,
    WorthQueryArtifactContractValidationDenial, WorthQueryArtifactContractValidationDenialKind,
    WorthQueryArtifactDeletionPosture, WorthQueryArtifactDeterminismPosture,
    WorthQueryArtifactDowngradePosture, WorthQueryArtifactEvidenceContract,
    WorthQueryArtifactFamily, WorthQueryArtifactFamilyIdentity,
    WorthQueryArtifactFieldSlicePosture, WorthQueryArtifactGovernanceContract,
    WorthQueryArtifactKeyFamily, WorthQueryArtifactLegalHoldPosture,
    WorthQueryArtifactLifecycleContract, WorthQueryArtifactMovePosture,
    WorthQueryArtifactNativeAccessContract, WorthQueryArtifactNativeAlignment,
    WorthQueryArtifactNativeFieldContract, WorthQueryArtifactNativeLayoutContract,
    WorthQueryArtifactNativeLayoutIdentity, WorthQueryArtifactNativeLayoutReference,
    WorthQueryArtifactNativeLayoutVersion, WorthQueryArtifactOccurrenceContract,
    WorthQueryArtifactOccurrenceIdentityPolicy, WorthQueryArtifactOwnershipContract,
    WorthQueryArtifactProtocolVersion, WorthQueryArtifactProviderTransferPosture,
    WorthQueryArtifactRedactionPosture, WorthQueryArtifactReproducibilityClass,
    WorthQueryArtifactReproducibilityContract, WorthQueryArtifactRetirementRule,
    WorthQueryArtifactRowBatchPosture, WorthQueryArtifactScalarFallbackPosture,
    WorthQueryArtifactSchemaVersion, WorthQueryArtifactSerializationPosture,
    WorthQueryArtifactSubstitutionPurpose, WorthQueryArtifactVersionSupport,
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchContract,
    WorthQueryCandidateSearchEvidenceFamilies, WorthQueryCandidateSearchPosture,
    WorthQueryConvergenceContract, WorthQueryConvergenceIncumbentPosture,
    WorthQueryConvergenceOscillationPosture, WorthQueryDecisionCausalParentShape,
    WorthQueryDecisionGovernance, WorthQueryDecisionIdentity, WorthQueryDecisionKind,
    WorthQueryDecisionPayloadVersion, WorthQueryDecisionReasonFamily,
    WorthQueryDecisionRecordContract, WorthQueryDecisionSchema,
    WorthQueryImmutableSourceOccurrenceContract, WorthQueryInstallationSupportStatus,
    WorthQueryInstalledArtifactContractAuthority, WorthQueryPortableArtifactContract,
    WorthQueryPortableArtifactContractBuilder, WorthQuerySourceOutputCorrespondence,
    WorthQueryStructuralCounterAggregation, WorthQueryStructuralCounterContract,
    WorthQueryStructuralCounterMonotonicity, WorthQueryStructuralCounterReplayPosture,
    WorthQueryStructuralCounterRequiredness, WorthQueryStructuralCounterResetBoundary,
    WorthQueryStructuralCounterRole, WorthQueryStructuralCounterSchema,
    WorthQueryStructuralCounterScope, WorthQueryStructuralCounterUnit,
    WorthQueryTransformationDisposition, WorthQueryTransformationErrorPosture,
    WorthQueryTransformationEvidenceContract, WorthQueryTransformationIdentity,
    WorthQueryTransformationLossPosture, WorthQueryTransformationOutcomeContract,
};
pub(crate) use package_definitions::{
    WorthQueryDomainOperationDefinitionRecord, WorthQueryDomainOperationGraphParticipationRecord,
    WorthQueryDomainOperationRequiredDomainRecord,
};
pub(crate) use validation::WorthQueryValidatedDomainPackage;
pub use validation::{
    WorthQueryDomainPackageValidationDenial, WorthQueryDomainPackageValidationDenialKind,
};
