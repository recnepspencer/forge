use std::any::TypeId;

pub use worth_query_installation::facade::{
    WorthQueryAftermathPostcondition, WorthQueryArtifactPosture,
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorFamily,
    WorthQueryComparatorRequirement, WorthQueryConditionalConditionClass,
    WorthQueryConditionalConsequenceRole, WorthQueryConditionalEvaluationCondition,
    WorthQueryConditionalGraphReadRole, WorthQueryConditionalNodeContext,
    WorthQueryConditionalNodeLocation, WorthQueryConditionalNodeOutput,
    WorthQueryConditionalNodeRole, WorthQueryConditionalTouchRole, WorthQueryConditionalTrigger,
    WorthQueryDeltaComparisonDomain, WorthQueryDeltaThreshold, WorthQueryDomainConditionFamily,
    WorthQueryDomainOperationDefinition, WorthQueryDomainOperationIdentity,
    WorthQueryDomainOperationSemanticClosure, WorthQueryMaintenancePosture,
    WorthQueryOnDemandTriggerFamily, WorthQueryOperationCapabilityRequirement,
    WorthQueryOperationCollectionContract, WorthQueryOperationContinuationPosture,
    WorthQueryOperationCostClass, WorthQueryOperationCostContract,
    WorthQueryOperationEffectContract, WorthQueryOperationEffectFamily,
    WorthQueryOperationFailureClass, WorthQueryOperationGraphAccess,
    WorthQueryOperationGraphParticipation, WorthQueryOperationGraphReadContract,
    WorthQueryOperationGraphReadRole, WorthQueryOperationInvariantContract,
    WorthQueryOperationLineageContract, WorthQueryOperationLoweringContract,
    WorthQueryOperationNativeProjectionContract, WorthQueryOperationParameterContract,
    WorthQueryOperationParameterField, WorthQueryOperationProjectionConsumptionContract,
    WorthQueryOperationProjectionRole, WorthQueryOperationPromotionContract,
    WorthQueryOperationPublicationContract, WorthQueryOperationReplayComparatorContract,
    WorthQueryOperationReplayContract, WorthQueryOperationReplayNoiseContract,
    WorthQueryOperationRequiredDomainRole, WorthQueryOperationResultState,
    WorthQueryOperationReversalContract, WorthQueryOperationSupportRequirements,
    WorthQueryOperationTerminalContract, WorthQueryOperationTouchContract,
    WorthQueryOperationValueFamily, WorthQueryOperationWorkflowContract,
    WorthQueryOutputEquivalenceRequirement, WorthQueryOutputRelationship,
    WorthQueryPortableConditionParameter, WorthQueryPortableConditionParameterValue,
    WorthQueryPortableConditionalNodeDeclaration, WorthQueryPortableWorkflowDefinition,
    WorthQueryPortableWorkflowStage, WorthQueryQuantityUnit, WorthQueryQuantityValueFamily,
    WorthQuerySemanticDependencyCanonicalBasis, WorthQuerySemanticLocality,
    WorthQuerySemanticTruthDependency, WorthQuerySemanticTruthDependencyDenial,
    WorthQuerySupportRequirement, WorthQueryTemporalCondition, WorthQueryTemporalWake,
    WorthQueryThresholdBoundary, WorthQueryTruthPartitionRole, WorthQueryTypedFamilyIdentity,
    WorthQueryWorkflowCostRole, WorthQueryWorkflowStageSemantics, WorthQueryWorkflowValueContract,
};

#[derive(Clone)]
pub(crate) struct WorthQueryDomainOperationDefinitionRecord {
    operation_marker: TypeId,
    family_marker: TypeId,
    definition: worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition,
}

impl WorthQueryDomainOperationDefinitionRecord {
    pub(crate) fn from_typed<D, O, F>(
        definition: WorthQueryDomainOperationDefinition<D, O, F>,
    ) -> Self
    where
        O: 'static,
        F: 'static,
    {
        Self {
            operation_marker: TypeId::of::<O>(),
            family_marker: TypeId::of::<F>(),
            definition: definition.into_portable(),
        }
    }

    pub(crate) fn operation_marker(&self) -> TypeId {
        self.operation_marker
    }

    pub(crate) fn family_marker(&self) -> TypeId {
        self.family_marker
    }

    pub(crate) fn definition(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition {
        &self.definition
    }
}
