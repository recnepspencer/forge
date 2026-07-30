use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query::facade::domain;
use worth_query_decl::facade::{
    authoring::{
        AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
        CollectionQueryBuilder, CollectionResultShapeBuilder, OrderingSelector, RootEntityKey,
    },
    binding::QueryBindingDescriptor,
    canonicalization::canonicalize_request,
};

use crate::{native_aspect_contracts, WorthUiDomainEntry};

use super::{
    WorthUiCollectionTextProjection, WorthUiCollectionTextProjectionFamily, LOWERING_FAMILY,
};

pub(crate) fn collection_text_projection_definition() -> domain::WorthQueryDomainOperationDefinition<
    WorthUiDomainEntry,
    WorthUiCollectionTextProjection,
    WorthUiCollectionTextProjectionFamily,
> {
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("collection-text-projection", 1),
        semantic_closure(),
    )
}

fn semantic_closure() -> domain::WorthQueryDomainOperationSemanticClosure {
    let identity = native_projection("identity");
    let projection = native_projection("query_text");
    domain::WorthQueryDomainOperationSemanticClosure {
        parameters: domain::WorthQueryOperationParameterContract::NotRequired,
        native_projection: projection.clone(),
        canonical_query: canonical_bundle(),
        collection: domain::WorthQueryOperationCollectionContract::Collection {
            row_identity_field: collection_field("identity.id"),
            ordering_fields: vec![collection_field("identity.id")],
            grouping: domain::WorthQueryOperationGroupingContract::Ungrouped,
            window: domain::WorthQueryOperationWindowPolicy::ContinuationBounded,
            continuation: domain::WorthQueryOperationContinuationPosture::LiveCursor,
        },
        required_capabilities: vec![
            domain::WorthQueryOperationCapabilityRequirement::QueryRead,
            domain::WorthQueryOperationCapabilityRequirement::QueryComposition,
        ],
        required_domains: Vec::new(),
        workflow: domain::WorthQueryOperationWorkflowContract::NotRequired,
        evidence: domain::WorthQueryDomainEvidenceContract::not_required(),
        conditional_nodes: Vec::new(),
        graph_reads: domain::WorthQueryOperationGraphReadContract::Declared {
            roles: vec![domain::WorthQueryOperationGraphReadRole {
                role: "collection-text".into(),
                participation: domain::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: domain::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![identity, projection],
            }],
        },
        decision_facts: domain::WorthQueryOperationDecisionFactContract::NotRequired,
        touches: domain::WorthQueryOperationTouchContract::NotRequired,
        effects: domain::WorthQueryOperationEffectContract::NotRequired,
        invariants: domain::WorthQueryOperationInvariantContract::NotRequired,
        invariant_execution: domain::WorthQueryInvariantExecutionContract::NotRequired,
        replay: domain::WorthQueryOperationReplayContract::ReExecutable,
        reversal: domain::WorthQueryOperationReversalContract::Irreversible,
        lineage: domain::WorthQueryOperationLineageContract::NotRequired,
        promotion: domain::WorthQueryOperationPromotionContract::NotRequired,
        publication: domain::WorthQueryOperationPublicationContract::DerivedProjection {
            projection_role: domain::WorthQueryOperationProjectionRole::new("collection-text")
                .expect("static projection role must admit"),
        },
        projection_consumption:
            domain::WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        terminal: domain::WorthQueryOperationTerminalContract {
            result_states: vec![
                domain::WorthQueryOperationResultState::Ready,
                domain::WorthQueryOperationResultState::Partial,
            ],
            failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
        },
        cost: domain::WorthQueryOperationCostContract {
            lookup: domain::WorthQueryOperationCostClass::Constant,
            execution: domain::WorthQueryOperationCostClass::DeclaredWidth,
            result_width: domain::WorthQueryOperationCostClass::DeclaredWidth,
        },
        resources:
            crate::installed_domain::execution_resources::operation_execution_resource_contract(),
        support: support_requirements(),
        lowering: domain::WorthQueryOperationLoweringContract {
            family: LOWERING_FAMILY.into(),
            deterministic: true,
        },
    }
}

fn canonical_bundle() -> worth_query_decl::facade::canonicalization::CanonicalQueryBundle {
    let query = CollectionQueryBuilder::new(
        RootEntityKey::new(crate::installed_domain::query_text::QUERY_TEXT_ROOT)
            .expect("static projection root must admit"),
    )
    .project(selector("identity", "id"))
    .project(selector("query_text", "status"))
    .order_by(
        OrderingSelector::ascending("identity", "id")
            .expect("static collection ordering must admit"),
    )
    .build()
    .expect("static collection text query must admit")
    .into_raw();
    let shape = CollectionResultShapeBuilder::new()
        .field(result_field("identity", "id", "identity.id"))
        .field(result_field("query_text", "status", "query_text.status"))
        .build()
        .expect("static collection text result shape must admit")
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .expect("static collection text bundle must admit"),
    )
    .expect("static collection text bundle must canonicalize")
}

fn support_requirements() -> domain::WorthQueryOperationSupportRequirements {
    domain::WorthQueryOperationSupportRequirements {
        live: domain::WorthQuerySupportRequirement::Required,
        continuation: domain::WorthQuerySupportRequirement::Required,
        async_result_state: domain::WorthQuerySupportRequirement::NotRequired,
        recovery: domain::WorthQuerySupportRequirement::NotRequired,
        inspection: domain::WorthQuerySupportRequirement::NotRequired,
        projection_consumption: domain::WorthQuerySupportRequirement::Required,
        dependency_impact: domain::WorthQuerySupportRequirement::Required,
        sharing: domain::WorthQuerySupportRequirement::Required,
        invalidation: domain::WorthQuerySupportRequirement::Required,
        collection_delivery: domain::WorthQuerySupportRequirement::Required,
        conditional_evaluation: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: domain::WorthQuerySupportRequirement::NotRequired,
    }
}

fn selector(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("static selector must admit")
}

fn result_field(aspect: &str, field: &str, alias: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, alias).expect("static result field must admit")
}

fn native_projection(key: &'static str) -> domain::WorthQueryOperationNativeProjectionContract {
    domain::WorthQueryOperationNativeProjectionContract::new(
        native_aspect_contracts::worth_ui_native_aspect_contract(key)
            .expect("installed native contract must exist"),
        AspectMask::<ProjectionMask>::whole_aspect(),
    )
    .expect("whole projection aspect must admit")
}

fn collection_field(value: &str) -> domain::WorthQueryOperationCollectionField {
    domain::WorthQueryOperationCollectionField::from_dotted(value)
        .expect("static collection field must admit")
}
