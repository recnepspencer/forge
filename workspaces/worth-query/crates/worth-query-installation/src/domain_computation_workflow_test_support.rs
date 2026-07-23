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

pub(crate) fn artifact_workflow(
    reference: WorthQueryArtifactContractReference,
) -> WorthQueryPortableDomainOperationDefinition {
    let producer = WorthQueryPortableWorkflowStage::new(
        "collect",
        std::iter::empty::<&str>(),
        false,
        false,
        [],
    )
    .with_semantics(WorthQueryWorkflowStageSemantics {
        output: WorthQueryWorkflowValueContract::installed_artifact(reference.clone()),
        ..WorthQueryWorkflowStageSemantics::default()
    });
    let consumer = WorthQueryPortableWorkflowStage::new("rank", ["collect"], true, false, [])
        .with_semantics(WorthQueryWorkflowStageSemantics {
            input: WorthQueryWorkflowValueContract::installed_artifact(reference),
            output: WorthQueryWorkflowValueContract::Bool,
            terminal_result_states: vec![WorthQueryOperationResultState::Ready],
            ..WorthQueryWorkflowStageSemantics::default()
        });
    let workflow = WorthQueryOperationWorkflowContract::Declared(
        WorthQueryPortableWorkflowDefinition::new("collect", [producer, consumer]),
    );
    WorthQueryDomainOperationDefinition::<(), (), ()>::new(
        WorthQueryDomainOperationIdentity::new("rank-candidates", 1),
        semantics(workflow),
    )
    .into_portable()
}

fn semantics(
    workflow: WorthQueryOperationWorkflowContract,
) -> WorthQueryDomainOperationSemanticClosure {
    WorthQueryDomainOperationSemanticClosure {
        parameters: WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection(),
        canonical_query: canonical_query(),
        collection: WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow,
        conditional_nodes: Vec::new(),
        graph_reads: WorthQueryOperationGraphReadContract::NotRequired,
        touches: WorthQueryOperationTouchContract::NotRequired,
        effects: WorthQueryOperationEffectContract::NotRequired,
        invariants: WorthQueryOperationInvariantContract::NotRequired,
        replay: WorthQueryOperationReplayContract::ReExecutable,
        reversal: WorthQueryOperationReversalContract::Irreversible,
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
            execution: WorthQueryOperationCostClass::Constant,
            result_width: WorthQueryOperationCostClass::Constant,
        },
        support: no_support(),
        lowering: WorthQueryOperationLoweringContract {
            family: "worth.routing.rank-candidates".into(),
            deterministic: true,
        },
    }
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
        AspectIdentity(9_150),
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
