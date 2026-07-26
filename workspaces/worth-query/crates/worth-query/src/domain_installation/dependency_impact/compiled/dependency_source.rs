use super::conditional_observation_evidence::WorthQueryConditionalObservationEvidence;

#[path = "dependency_source/replay_semantics.rs"]
mod replay_semantics;

#[derive(Clone, Debug)]
pub(crate) enum WorthQuerySemanticAspectDependencySource {
    InstalledOperationIdentity {
        identity: worth_query_installation::facade::WorthQueryDomainOperationIdentity,
        canonical_identity: String,
    },
    NativeProjection(worth_query_installation::facade::WorthQueryOperationNativeProjectionContract),
    CollectionField(worth_query_installation::facade::WorthQueryOperationCollectionField),
    CollectionWindowPolicy(worth_query_installation::facade::WorthQueryOperationWindowPolicy),
    ResultShape(worth_query_declaration::facade::canonicalization::CanonicalQueryBundle),
    TouchGraphRole(String),
    TouchScope(String),
    EffectFamily(worth_query_installation::facade::WorthQueryOperationEffectFamily),
    InstalledInvariant(String),
    ReplayContract(worth_query_installation::facade::WorthQueryOperationReplayContract),
    LineageContract(worth_query_installation::facade::WorthQueryOperationLineageContract),
    SupportContract(worth_query_installation::facade::WorthQueryOperationSupportRequirements),
    WorkflowStageRead {
        graph_read_role: String,
    },
    WorkflowStage {
        predecessors: Vec<String>,
    },
    ConditionalNodeContract(
        worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    ),
    ConditionalTruth(worth_query_installation::facade::WorthQuerySemanticTruthDependency),
    RealizedGraphCall {
        role: String,
        call_kind: crate::domain_installation::WorthQueryGraphProviderCallKind,
        evidence_identity: String,
        projection_result_digest: Option<String>,
        commit_graph_roles: Vec<String>,
    },
    RealizedWorkflowRead(crate::domain_installation::WorthQueryWorkflowPrimaryReadEvidence),
    RealizedConditionalOutcome {
        class: crate::domain_installation::WorthQueryConditionalOutcomeClass,
        signal_projection: worth_signal::facade::SignalConditionalDecisionProjectionIdentity,
        observations: Vec<WorthQueryConditionalObservationEvidence>,
    },
    RealizedDirectOutput {
        result_state: crate::domain_installation::WorthQueryOperationResultState,
        output_identity: String,
        publication: crate::domain_installation::WorthQueryDerivedPublicationReceipt,
    },
    RealizedWorkflowEffect(crate::domain_installation::WorthQueryWorkflowEffectEvidence),
    RealizedWorkflowInvariant(crate::domain_installation::WorthQueryWorkflowInvariantOutcome),
    RealizedWorkflowLineage(crate::identity_evolution::InstalledIdentityEvolutionOutcome),
    RealizedWorkflowOutput {
        receipt_identity: String,
        semantic_output:
            crate::domain_installation::operation_execution::WorthQueryWorkflowSemanticValue,
        result_state: Option<worth_query_installation::facade::WorthQueryOperationResultState>,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum WorthQuerySemanticAspectDependencyView<'a> {
    InstalledOperationIdentity {
        identity: &'a worth_query_installation::facade::WorthQueryDomainOperationIdentity,
        canonical_identity: &'a str,
    },
    NativeProjection(
        &'a worth_query_installation::facade::WorthQueryOperationNativeProjectionContract,
    ),
    CollectionField(&'a worth_query_installation::facade::WorthQueryOperationCollectionField),
    CollectionWindowPolicy(worth_query_installation::facade::WorthQueryOperationWindowPolicy),
    ResultShape(&'a worth_query_declaration::facade::canonicalization::CanonicalQueryBundle),
    TouchGraphRole(&'a str),
    TouchScope(&'a str),
    EffectFamily(worth_query_installation::facade::WorthQueryOperationEffectFamily),
    InstalledInvariant(&'a str),
    ReplayContract(worth_query_installation::facade::WorthQueryOperationReplayContract),
    LineageContract(worth_query_installation::facade::WorthQueryOperationLineageContract),
    SupportContract(worth_query_installation::facade::WorthQueryOperationSupportRequirements),
    WorkflowStageRead {
        graph_read_role: &'a str,
    },
    WorkflowStage {
        predecessors: &'a [String],
    },
    ConditionalNodeContract(
        &'a worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    ),
    ConditionalTruth(&'a worth_query_installation::facade::WorthQuerySemanticTruthDependency),
    RealizedGraphCall {
        role: &'a str,
        call_kind: crate::domain_installation::WorthQueryGraphProviderCallKind,
        evidence_identity: &'a str,
        projection_result_digest: Option<&'a str>,
        commit_graph_roles: &'a [String],
    },
    RealizedWorkflowRead(&'a crate::domain_installation::WorthQueryWorkflowPrimaryReadEvidence),
    RealizedConditionalOutcome {
        class: crate::domain_installation::WorthQueryConditionalOutcomeClass,
        signal_projection: &'a worth_signal::facade::SignalConditionalDecisionProjectionIdentity,
        observations: &'a [WorthQueryConditionalObservationEvidence],
    },
    RealizedDirectOutput {
        result_state: crate::domain_installation::WorthQueryOperationResultState,
        output_identity: &'a str,
        publication: &'a crate::domain_installation::WorthQueryDerivedPublicationReceipt,
    },
    RealizedWorkflowEffect(&'a crate::domain_installation::WorthQueryWorkflowEffectEvidence),
    RealizedWorkflowInvariant(&'a crate::domain_installation::WorthQueryWorkflowInvariantOutcome),
    RealizedWorkflowLineage(&'a crate::identity_evolution::InstalledIdentityEvolutionOutcome),
    RealizedWorkflowOutput {
        receipt_identity: &'a str,
        semantic_output:
            &'a crate::domain_installation::operation_execution::WorthQueryWorkflowSemanticValue,
        result_state: Option<worth_query_installation::facade::WorthQueryOperationResultState>,
    },
}

impl WorthQuerySemanticAspectDependencySource {
    pub(super) fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        replay_semantics::dependency_source_semantics_eq(self, candidate)
    }
}
