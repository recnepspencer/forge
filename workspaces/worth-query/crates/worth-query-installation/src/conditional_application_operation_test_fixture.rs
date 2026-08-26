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

use crate::facade::*;

pub(crate) struct FixtureTrigger;

impl WorthQueryOnDemandTriggerFamily for FixtureTrigger {
    const PORTABLE_IDENTITY: &'static str = "test.conditional.trigger";
}

pub(crate) fn definition<D, O, F>() -> WorthQueryDomainOperationDefinition<D, O, F> {
    WorthQueryDomainOperationDefinition::new(
        WorthQueryDomainOperationIdentity::new("conditional-operation", 1),
        WorthQueryDomainOperationSemanticClosure {
            parameters: WorthQueryOperationParameterContract::NotRequired,
            native_projection: native_projection(),
            canonical_query: canonical_query(),
            collection: WorthQueryOperationCollectionContract::NotCollection,
            required_capabilities: Vec::new(),
            required_domains: Vec::new(),
            workflow: WorthQueryOperationWorkflowContract::NotRequired,
            evidence: WorthQueryDomainEvidenceContract::not_required(),
            conditional_nodes: vec![conditional_node()],
            graph_reads: WorthQueryOperationGraphReadContract::NotRequired,
            decision_facts: WorthQueryOperationDecisionFactContract::NotRequired,
            touches: WorthQueryOperationTouchContract::NotRequired,
            effects: WorthQueryOperationEffectContract::NotRequired,
            invariants: WorthQueryOperationInvariantContract::NotRequired,
            invariant_execution: WorthQueryInvariantExecutionContract::NotRequired,
            replay: WorthQueryOperationReplayContract::ReExecutable,
            aftermath: None,
            lineage: WorthQueryOperationLineageContract::NotRequired,
            promotion: WorthQueryOperationPromotionContract::NotRequired,
            publication: WorthQueryOperationPublicationContract::DerivedProjection {
                projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
            },
            projection_consumption:
                WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
            terminal: WorthQueryOperationTerminalContract {
                result_states: vec![WorthQueryOperationResultState::Ready],
                failure_classes: Vec::new(),
            },
            cost: WorthQueryOperationCostContract {
                lookup: WorthQueryOperationCostClass::Constant,
                execution: WorthQueryOperationCostClass::Constant,
                result_width: WorthQueryOperationCostClass::Constant,
            },
            resources: crate::domain_computation_workflow_test_support::resource_contract(),
            support: no_support(),
            lowering: WorthQueryOperationLoweringContract {
                family: "test.conditional-operation".into(),
                deterministic: true,
            },
        },
    )
}

fn conditional_node() -> WorthQueryPortableConditionalNodeDeclaration {
    WorthQueryPortableConditionalNodeDeclaration::declare(
        "ready-gate",
        WorthQueryConditionalNodeRole::OperationGate,
    )
    .dependencies([])
    .outputs([WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: WorthQueryOperationProjectionRole::new("profile").unwrap(),
    }])
    .required_context([WorthQueryConditionalNodeContext::OperationInput])
    .evaluation(
        WorthQueryConditionalEvaluationCondition::on_demand(),
        WorthQueryConditionalTrigger::on_demand::<FixtureTrigger>(),
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
    .output_relationship(WorthQueryOutputRelationship::IsOperationOutput)
    .finish()
    .unwrap()
}

fn no_support() -> WorthQueryOperationSupportRequirements {
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
        conditional_evaluation: WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: WorthQuerySupportRequirement::NotRequired,
    }
}

fn native_projection() -> WorthQueryOperationNativeProjectionContract {
    let field = FieldDeclaration::new(
        FieldKey::new("name").unwrap(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap();
    let contract = AspectContract::struct_aspect(
        AspectKey::new("profile").unwrap(),
        AspectIdentity(9_160),
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
    canonical_query_for_root("ConditionalEntity")
}

pub(crate) fn canonical_query_for_root(
    root: &str,
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let query = DetailQueryBuilder::new(RootEntityKey::new(root).unwrap())
        .project(AspectFieldSelector::new("profile", "name").unwrap())
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
