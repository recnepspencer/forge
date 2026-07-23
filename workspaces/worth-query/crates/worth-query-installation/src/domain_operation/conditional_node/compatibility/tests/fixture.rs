use worth_foundational::facade::{
    AbsenceLaw, AspectBinding, AspectContract, AspectContractRevision, AspectEvolutionPolicy,
    AspectIdentity, AspectKey, AspectMask, AuthoritativeAspectChangeKind, FieldDeclaration,
    FieldKey, FieldRequirement, ProjectionMask, ScalarAspectType, StructAspectShape,
};
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::canonicalize_request;

use crate::domain_operation::*;

pub(super) struct TriggerA;
impl WorthQueryOnDemandTriggerFamily for TriggerA {
    const PORTABLE_IDENTITY: &'static str = "test.trigger.a";
}

pub(super) struct TriggerB;
impl WorthQueryOnDemandTriggerFamily for TriggerB {
    const PORTABLE_IDENTITY: &'static str = "test.trigger.b";
}

pub(super) fn operation_node<Trigger: WorthQueryOnDemandTriggerFamily>(
    identity: &str,
    comparator: WorthQueryComparatorRequirement,
) -> WorthQueryPortableConditionalNodeDeclaration {
    WorthQueryPortableConditionalNodeDeclaration::declare(
        identity,
        WorthQueryConditionalNodeRole::OperationGate,
    )
    .dependencies([])
    .outputs([WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([WorthQueryConditionalNodeContext::OperationInput])
    .evaluation(
        WorthQueryConditionalEvaluationCondition::on_demand(),
        WorthQueryConditionalTrigger::on_demand::<Trigger>(),
    )
    .comparison(
        comparator,
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        WorthQueryArtifactReuseEquivalence::NotReusable,
        WorthQueryMaintenancePosture::OnDemandOnly,
        WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(WorthQueryOutputRelationship::IsOperationOutput)
    .finish()
    .unwrap()
}

pub(super) fn workflow_node(identity: &str) -> WorthQueryPortableConditionalNodeDeclaration {
    WorthQueryPortableConditionalNodeDeclaration::declare(
        identity,
        WorthQueryConditionalNodeRole::WorkflowStage,
    )
    .dependencies([])
    .outputs([WorthQueryConditionalNodeOutput::WorkflowStageOutput {
        contract: WorthQueryWorkflowValueContract::Bool,
    }])
    .required_context([WorthQueryConditionalNodeContext::WorkflowRun])
    .evaluation(
        WorthQueryConditionalEvaluationCondition::on_demand(),
        WorthQueryConditionalTrigger::on_demand::<TriggerA>(),
    )
    .comparison(
        WorthQueryComparatorRequirement::ExactCanonicalValue,
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        WorthQueryArtifactReuseEquivalence::NotReusable,
        WorthQueryMaintenancePosture::OnDemandOnly,
        WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(WorthQueryOutputRelationship::IsWorkflowStageOutput)
    .finish()
    .unwrap()
}

pub(super) fn dependency_node(revision: u64) -> WorthQueryPortableConditionalNodeDeclaration {
    let dependency = WorthQuerySemanticTruthDependency::new(
        WorthQueryConditionalGraphReadRole::new("model").unwrap(),
        contract_with_revision(revision),
        AspectMask::<ProjectionMask>::whole_aspect(),
        AspectBinding::EntityField {
            field: FieldKey::new("name").unwrap(),
        },
        WorthQuerySemanticLocality::SourceRecord,
        [AuthoritativeAspectChangeKind::FieldSet],
    )
    .unwrap();
    WorthQueryPortableConditionalNodeDeclaration::declare(
        "computed",
        WorthQueryConditionalNodeRole::Computed,
    )
    .dependencies([dependency.clone()])
    .outputs([WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([WorthQueryConditionalNodeContext::Snapshot])
    .evaluation(
        WorthQueryConditionalEvaluationCondition::aspect_filtered([dependency]).unwrap(),
        WorthQueryConditionalTrigger::DependencyChange,
    )
    .comparison(
        WorthQueryComparatorRequirement::FoundationalContractEquivalence,
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
    )
    .artifact_policy(
        WorthQueryArtifactReuseEquivalence::NotReusable,
        WorthQueryMaintenancePosture::LazyUntilObserved,
        WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(WorthQueryOutputRelationship::ContributesToOperationOutput)
    .finish()
    .unwrap()
}

pub(super) fn operation_definition(
    conditional_nodes: Vec<WorthQueryPortableConditionalNodeDeclaration>,
    stage_node: WorthQueryPortableConditionalNodeDeclaration,
) -> WorthQueryPortableDomainOperationDefinition {
    let contract = contract_with_revision(1);
    let native_projection = WorthQueryOperationNativeProjectionContract::new(
        contract.clone(),
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .unwrap();
    let stage = WorthQueryPortableWorkflowStage::new("stage", Vec::<String>::new(), true, true, [])
        .with_semantics(WorthQueryWorkflowStageSemantics {
            output: WorthQueryWorkflowValueContract::Bool,
            conditional_nodes: vec![stage_node],
            ..WorthQueryWorkflowStageSemantics::default()
        });
    let semantics = WorthQueryDomainOperationSemanticClosure {
        parameters: WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection.clone(),
        canonical_query: canonical_bundle(),
        collection: WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: WorthQueryOperationWorkflowContract::Declared(
            WorthQueryPortableWorkflowDefinition::new("stage", [stage]),
        ),
        conditional_nodes,
        graph_reads: WorthQueryOperationGraphReadContract::Declared {
            roles: vec![WorthQueryOperationGraphReadRole {
                role: "model".into(),
                participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![native_projection],
            }],
        },
        touches: WorthQueryOperationTouchContract::NotRequired,
        effects: WorthQueryOperationEffectContract::NotRequired,
        invariants: WorthQueryOperationInvariantContract::NotRequired,
        replay: WorthQueryOperationReplayContract::ReExecutable,
        reversal: WorthQueryOperationReversalContract::Irreversible,
        lineage: WorthQueryOperationLineageContract::NotRequired,
        promotion: WorthQueryOperationPromotionContract::NotRequired,
        publication: WorthQueryOperationPublicationContract::DerivedProjection {
            projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
        },
        projection_consumption:
            WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        terminal: WorthQueryOperationTerminalContract {
            result_states: vec![WorthQueryOperationResultState::Ready],
            failure_classes: vec![WorthQueryOperationFailureClass::Dependency],
        },
        cost: WorthQueryOperationCostContract {
            lookup: WorthQueryOperationCostClass::Constant,
            execution: WorthQueryOperationCostClass::DeclaredWidth,
            result_width: WorthQueryOperationCostClass::DeclaredWidth,
        },
        support: support(),
        lowering: WorthQueryOperationLoweringContract {
            family: "conditional-comparison-test".into(),
            deterministic: true,
        },
    };
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        WorthQueryDomainOperationIdentity::new("conditional-test", 1),
        semantics,
    )
    .into_portable()
}

fn support() -> WorthQueryOperationSupportRequirements {
    WorthQueryOperationSupportRequirements {
        live: WorthQuerySupportRequirement::NotRequired,
        continuation: WorthQuerySupportRequirement::NotRequired,
        async_result_state: WorthQuerySupportRequirement::NotRequired,
        recovery: WorthQuerySupportRequirement::NotRequired,
        inspection: WorthQuerySupportRequirement::NotRequired,
        projection_consumption: WorthQuerySupportRequirement::Required,
        dependency_impact: WorthQuerySupportRequirement::NotRequired,
        sharing: WorthQuerySupportRequirement::NotRequired,
        invalidation: WorthQuerySupportRequirement::NotRequired,
        collection_delivery: WorthQuerySupportRequirement::NotRequired,
        conditional_evaluation: WorthQuerySupportRequirement::Required,
        conditional_comparator: WorthQuerySupportRequirement::Required,
        conditional_trigger: WorthQuerySupportRequirement::Required,
        conditional_temporal_or_on_demand: WorthQuerySupportRequirement::Required,
    }
}

fn contract_with_revision(revision: u64) -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new("name").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    AspectContract::struct_aspect(
        AspectKey::new("profile").unwrap(),
        AspectIdentity(1601),
        AspectContractRevision(revision),
        StructAspectShape::new([field]).unwrap(),
    )
}

fn canonical_bundle() -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let selector = AspectFieldSelector::new("profile", "name").unwrap();
    let query = DetailQueryBuilder::new(RootEntityKey::new("ConditionalTest").unwrap())
        .project(selector)
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("profile", "name", "name").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .unwrap(),
    )
    .unwrap()
}
