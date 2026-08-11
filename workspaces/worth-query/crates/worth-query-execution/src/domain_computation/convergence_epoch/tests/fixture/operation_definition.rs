use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, FieldDeclaration, FieldKey, FieldRequirement, ProjectionMask,
    ScalarAspectType, StructAspectShape,
};
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::canonicalize_request;
use worth_query_installation::facade::{
    WorthQueryArtifactContractReference, WorthQueryDomainEvidenceContract,
    WorthQueryDomainOperationDefinition, WorthQueryDomainOperationIdentity,
    WorthQueryDomainOperationSemanticClosure, WorthQueryInvariantExecutionContract,
    WorthQueryOperationCollectionContract, WorthQueryOperationCostClass,
    WorthQueryOperationCostContract, WorthQueryOperationDecisionFactContract,
    WorthQueryOperationEffectContract, WorthQueryOperationGraphAccess,
    WorthQueryOperationGraphParticipation, WorthQueryOperationGraphReadContract,
    WorthQueryOperationGraphReadRole, WorthQueryOperationInvariantContract,
    WorthQueryOperationLineageContract, WorthQueryOperationLoweringContract,
    WorthQueryOperationNativeProjectionContract, WorthQueryOperationParameterContract,
    WorthQueryOperationProjectionConsumptionContract, WorthQueryOperationPromotionContract,
    WorthQueryOperationPublicationContract, WorthQueryOperationReplayContract,
    WorthQueryOperationResultState, WorthQueryOperationSupportRequirements,
    WorthQueryOperationTerminalContract, WorthQueryOperationTouchContract,
    WorthQueryOperationWorkflowContract, WorthQueryPortableDomainOperationDefinition,
    WorthQueryPortableWorkflowDefinition, WorthQueryPortableWorkflowStage,
    WorthQuerySupportRequirement, WorthQueryWorkflowStageSemantics,
    WorthQueryWorkflowValueContract,
};

use super::fixture_identity::{GRAPH_ROLE, WORKFLOW_STAGE};

pub(super) fn workflow_operation(
    reference: WorthQueryArtifactContractReference,
    operation_resources: worth_query_installation::facade::WorthQueryExecutionResourceContract,
    stage_resources: worth_query_installation::facade::WorthQueryExecutionResourceContract,
    graph_access: WorthQueryOperationGraphAccess,
) -> WorthQueryPortableDomainOperationDefinition {
    let direct = direct_operation(reference.clone(), operation_resources.clone(), graph_access);
    let mut semantics = direct.semantics().clone();
    let stage = WorthQueryPortableWorkflowStage::new(
        WORKFLOW_STAGE,
        std::iter::empty::<&str>(),
        true,
        false,
        [],
    )
    .with_semantics(WorthQueryWorkflowStageSemantics {
        evidence: WorthQueryDomainEvidenceContract::installed_artifact(reference.clone()),
        output: WorthQueryWorkflowValueContract::installed_artifact(reference),
        graph_read_roles: vec![GRAPH_ROLE.into()],
        resources: stage_resources,
        terminal_result_states: vec![WorthQueryOperationResultState::Ready],
        ..WorthQueryWorkflowStageSemantics::default()
    });
    semantics.workflow = WorthQueryOperationWorkflowContract::Declared(
        WorthQueryPortableWorkflowDefinition::new(WORKFLOW_STAGE, [stage]),
    );
    semantics.evidence = WorthQueryDomainEvidenceContract::not_required();
    semantics.resources = operation_resources;
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        WorthQueryDomainOperationIdentity::new("iterate-workflow", 1),
        semantics,
    )
    .into_portable()
}

pub(super) fn direct_operation(
    reference: WorthQueryArtifactContractReference,
    resources: worth_query_installation::facade::WorthQueryExecutionResourceContract,
    graph_access: WorthQueryOperationGraphAccess,
) -> WorthQueryPortableDomainOperationDefinition {
    let projection = native_projection();
    let semantics = WorthQueryDomainOperationSemanticClosure {
        parameters: WorthQueryOperationParameterContract::NotRequired,
        native_projection: projection.clone(),
        canonical_query: canonical_query(),
        collection: WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: WorthQueryOperationWorkflowContract::NotRequired,
        evidence: WorthQueryDomainEvidenceContract::installed_artifact(reference),
        conditional_nodes: Vec::new(),
        graph_reads: WorthQueryOperationGraphReadContract::Declared {
            roles: vec![WorthQueryOperationGraphReadRole {
                role: GRAPH_ROLE.into(),
                participation: WorthQueryOperationGraphParticipation::SeparateAuthority {
                    role: GRAPH_ROLE.into(),
                },
                access: graph_access,
                semantic_reads: vec![projection],
            }],
        },
        decision_facts: WorthQueryOperationDecisionFactContract::NotRequired,
        touches: WorthQueryOperationTouchContract::NotRequired,
        effects: WorthQueryOperationEffectContract::NotRequired,
        invariants: WorthQueryOperationInvariantContract::NotRequired,
        invariant_execution: WorthQueryInvariantExecutionContract::NotRequired,
        replay: WorthQueryOperationReplayContract::ReExecutable,
        aftermath: None,
        lineage: WorthQueryOperationLineageContract::NotRequired,
        promotion: WorthQueryOperationPromotionContract::NotRequired,
        publication: WorthQueryOperationPublicationContract::NotRequired,
        projection_consumption: WorthQueryOperationProjectionConsumptionContract::NotRequired,
        terminal: WorthQueryOperationTerminalContract {
            result_states: vec![WorthQueryOperationResultState::Ready],
            failure_classes: Vec::new(),
        },
        cost: WorthQueryOperationCostContract {
            lookup: WorthQueryOperationCostClass::Constant,
            execution: WorthQueryOperationCostClass::GraphBreadth,
            result_width: WorthQueryOperationCostClass::Constant,
        },
        resources,
        support: no_support(),
        lowering: WorthQueryOperationLoweringContract {
            family: "worth.convergence.iterate".into(),
            deterministic: true,
        },
    };
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        WorthQueryDomainOperationIdentity::new("iterate", 1),
        semantics,
    )
    .into_portable()
}

fn native_projection() -> WorthQueryOperationNativeProjectionContract {
    let field = FieldDeclaration::new(
        FieldKey::new("state").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    let contract = AspectContract::struct_aspect(
        AspectKey::new("candidate").unwrap(),
        AspectIdentity(9_156),
        AspectContractRevision(1),
        StructAspectShape::new([field]).unwrap(),
    );
    WorthQueryOperationNativeProjectionContract::new(
        contract,
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .unwrap()
}

fn canonical_query() -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let query = DetailQueryBuilder::new(RootEntityKey::new("Candidate").unwrap())
        .project(AspectFieldSelector::new("candidate", "state").unwrap())
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("candidate", "state", "state").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .unwrap(),
    )
    .unwrap()
}

fn no_support() -> WorthQueryOperationSupportRequirements {
    WorthQueryOperationSupportRequirements {
        live: WorthQuerySupportRequirement::NotRequired,
        continuation: WorthQuerySupportRequirement::NotRequired,
        async_result_state: WorthQuerySupportRequirement::NotRequired,
        recovery: WorthQuerySupportRequirement::NotRequired,
        inspection: WorthQuerySupportRequirement::NotRequired,
        projection_consumption: WorthQuerySupportRequirement::NotRequired,
        dependency_impact: WorthQuerySupportRequirement::NotRequired,
        sharing: WorthQuerySupportRequirement::NotRequired,
        invalidation: WorthQuerySupportRequirement::NotRequired,
        collection_delivery: WorthQuerySupportRequirement::NotRequired,
        conditional_evaluation: WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: WorthQuerySupportRequirement::NotRequired,
    }
}
