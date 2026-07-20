use worth_foundational::facade::{
    AspectContractRevision, AspectIdentity, AspectKey, AspectMask, ProjectionMask,
};
use worth_query::facade::domain;
use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::canonicalize_request;

pub(crate) fn semantic_closure(
    bundle: worth_query_declaration::facade::canonicalization::CanonicalQueryBundle,
    projection_consumption: domain::WorthQuerySupportRequirement,
    publishes: bool,
) -> domain::WorthQueryDomainOperationSemanticClosure {
    let native_projection = domain::WorthQueryOperationNativeProjectionContract {
        aspect_key: AspectKey::new("identity").unwrap(),
        aspect_identity: AspectIdentity(0x9140_0001),
        contract_revision: AspectContractRevision(1),
        mask: AspectMask::<ProjectionMask>::whole_aspect(),
    };
    domain::WorthQueryDomainOperationSemanticClosure {
        parameters: domain::WorthQueryOperationParameterContract::NotRequired,
        native_projection: native_projection.clone(),
        canonical_query: bundle,
        collection: domain::WorthQueryOperationCollectionContract::NotCollection,
        required_capabilities: Vec::new(),
        required_domains: Vec::new(),
        workflow: domain::WorthQueryOperationWorkflowContract::NotRequired,
        conditional_nodes: Vec::new(),
        graph_reads: domain::WorthQueryOperationGraphReadContract::Declared {
            roles: vec![domain::WorthQueryOperationGraphReadRole {
                role: "model".into(),
                participation: domain::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: domain::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![native_projection],
            }],
        },
        touches: domain::WorthQueryOperationTouchContract::NotRequired,
        effects: domain::WorthQueryOperationEffectContract::NotRequired,
        invariants: domain::WorthQueryOperationInvariantContract::NotRequired,
        replay: domain::WorthQueryOperationReplayContract::ReExecutable,
        reversal: domain::WorthQueryOperationReversalContract::Irreversible,
        lineage: domain::WorthQueryOperationLineageContract::NotRequired,
        promotion: domain::WorthQueryOperationPromotionContract::NotRequired,
        publication: if publishes {
            domain::WorthQueryOperationPublicationContract::DerivedProjection {
                projection_role: domain::WorthQueryOperationProjectionRole::new("vertex").unwrap(),
            }
        } else {
            domain::WorthQueryOperationPublicationContract::NotRequired
        },
        projection_consumption: if publishes {
            domain::WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority
        } else {
            domain::WorthQueryOperationProjectionConsumptionContract::NotRequired
        },
        terminal: domain::WorthQueryOperationTerminalContract {
            result_states: vec![
                domain::WorthQueryOperationResultState::Ready,
                domain::WorthQueryOperationResultState::Advisory,
                domain::WorthQueryOperationResultState::Pending,
                domain::WorthQueryOperationResultState::Partial,
                domain::WorthQueryOperationResultState::Violation,
            ],
            failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
        },
        cost: domain::WorthQueryOperationCostContract {
            lookup: domain::WorthQueryOperationCostClass::Constant,
            execution: domain::WorthQueryOperationCostClass::DeclaredWidth,
            result_width: domain::WorthQueryOperationCostClass::DeclaredWidth,
        },
        support: domain::WorthQueryOperationSupportRequirements {
            live: domain::WorthQuerySupportRequirement::NotRequired,
            continuation: domain::WorthQuerySupportRequirement::NotRequired,
            async_result_state: domain::WorthQuerySupportRequirement::NotRequired,
            recovery: domain::WorthQuerySupportRequirement::NotRequired,
            inspection: domain::WorthQuerySupportRequirement::NotRequired,
            projection_consumption,
            dependency_impact: domain::WorthQuerySupportRequirement::NotRequired,
            sharing: domain::WorthQuerySupportRequirement::NotRequired,
            invalidation: domain::WorthQuerySupportRequirement::NotRequired,
            collection_delivery: domain::WorthQuerySupportRequirement::NotRequired,
            conditional_evaluation: domain::WorthQuerySupportRequirement::NotRequired,
            conditional_comparator: domain::WorthQuerySupportRequirement::NotRequired,
            conditional_trigger: domain::WorthQuerySupportRequirement::NotRequired,
            conditional_temporal_or_on_demand: domain::WorthQuerySupportRequirement::NotRequired,
        },
        lowering: domain::WorthQueryOperationLoweringContract {
            family: "read-vertex-v1".into(),
            deterministic: true,
        },
    }
}

pub(crate) fn canonical_bundle(
    root: &str,
) -> worth_query_declaration::facade::canonicalization::CanonicalQueryBundle {
    let selector = AspectFieldSelector::new("identity", "id").unwrap();
    let query = DetailQueryBuilder::new(RootEntityKey::new(root).unwrap())
        .project(selector)
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::new(query, shape, QueryBindingDescriptor::new()).unwrap(),
    )
    .unwrap()
}
