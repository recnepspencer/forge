use worth_foundational::facade::{AspectMask, ProjectionMask};
use worth_query::facade::domain;
use worth_query_decl::facade::{
    authoring::{
        AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField,
        DetailQueryBuilder, DetailResultShapeBuilder, EqualityPredicate, RootEntityKey,
        WorthQueryPredicateOperand,
    },
    binding::QueryBindingDescriptor,
    canonicalization::canonicalize_request,
};

use crate::{native_aspect_contracts, WorthUiDomainEntry};

use super::{
    WorthUiScalarTextProjection, WorthUiScalarTextProjectionFamily, LOWERING_FAMILY,
    PLATFORM_PULSE_STATUS_IDENTITY,
};

pub(crate) fn scalar_text_projection_definition() -> domain::WorthQueryDomainOperationDefinition<
    WorthUiDomainEntry,
    WorthUiScalarTextProjection,
    WorthUiScalarTextProjectionFamily,
> {
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("scalar-text-projection", 1),
        semantic_closure(),
    )
}

fn semantic_closure() -> domain::WorthQueryDomainOperationSemanticClosure {
    let identity = native_projection("identity");
    let projection = native_projection("query_text");
    let revision = native_projection("query_revision");
    domain::WorthQueryDomainOperationSemanticClosure {
        parameters: domain::WorthQueryOperationParameterContract::NotRequired,
        native_projection: projection.clone(),
        canonical_query: canonical_bundle(),
        collection: domain::WorthQueryOperationCollectionContract::NotCollection,
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
                role: "scalar-text".into(),
                participation: domain::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: domain::WorthQueryOperationGraphAccess::Project,
                semantic_reads: vec![identity, projection, revision],
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
            projection_role: domain::WorthQueryOperationProjectionRole::new("scalar-text")
                .expect("static projection role must admit"),
        },
        projection_consumption:
            domain::WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        terminal: domain::WorthQueryOperationTerminalContract {
            result_states: vec![domain::WorthQueryOperationResultState::Ready],
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
    let query = DetailQueryBuilder::new(
        RootEntityKey::new(crate::installed_domain::query_text::QUERY_TEXT_ROOT)
            .expect("static projection root must admit"),
    )
    .project(selector("identity", "id"))
    .project(selector("query_text", "status"))
    .project(selector("query_revision", "value"))
    .where_equal(
        EqualityPredicate::new(
            "identity",
            "id",
            WorthQueryPredicateOperand::string(PLATFORM_PULSE_STATUS_IDENTITY.to_owned()),
        )
        .expect("static Pulse identity predicate must admit"),
    )
    .build()
    .expect("static scalar text query must admit")
    .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(result_field("identity", "id", "id"))
        .field(result_field("query_text", "status", "query_text.status"))
        .field(result_field(
            "query_revision",
            "value",
            "query_revision.value",
        ))
        .build()
        .expect("static scalar text result shape must admit")
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::for_ordinary_read(query, shape, QueryBindingDescriptor::new())
            .expect("static scalar text bundle must admit"),
    )
    .expect("static scalar text bundle must canonicalize")
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

fn support_requirements() -> domain::WorthQueryOperationSupportRequirements {
    domain::WorthQueryOperationSupportRequirements {
        live: domain::WorthQuerySupportRequirement::NotRequired,
        continuation: domain::WorthQuerySupportRequirement::NotRequired,
        async_result_state: domain::WorthQuerySupportRequirement::Required,
        recovery: domain::WorthQuerySupportRequirement::Required,
        inspection: domain::WorthQuerySupportRequirement::NotRequired,
        projection_consumption: domain::WorthQuerySupportRequirement::Required,
        dependency_impact: domain::WorthQuerySupportRequirement::NotRequired,
        sharing: domain::WorthQuerySupportRequirement::NotRequired,
        invalidation: domain::WorthQuerySupportRequirement::NotRequired,
        collection_delivery: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_evaluation: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_comparator: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_trigger: domain::WorthQuerySupportRequirement::NotRequired,
        conditional_temporal_or_on_demand: domain::WorthQuerySupportRequirement::NotRequired,
    }
}
