use crate::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryReference, ApplicationQueryResultRelationRef,
        ApplicationQueryResultShapeBuilder, ExactlyOneResult, ForwardResultTraversal, ManyResults,
        OptionalOneResult,
    },
    application_schema::{ApplicationEntityRef, ApplicationRelationRef, ApplicationSchemaMember},
};

use super::{identity, QueryEntity, QueryMarker, QueryParameters, QueryResult, QuerySchema};

struct QueryChild;
struct QueryRelation;
struct QueryChildSlot;

#[test]
fn nested_relation_cardinality_changes_schema_identity() {
    let optional = identity(&[application_query_with_relation(
        ApplicationQueryCardinality::OptionalOne,
    )]);
    let one = identity(&[application_query_with_relation(
        ApplicationQueryCardinality::ExactlyOne,
    )]);
    let many = identity(&[application_query_with_relation(
        ApplicationQueryCardinality::Many,
    )]);

    assert_ne!(optional, one);
    assert_ne!(one, many);
    assert_ne!(optional, many);
}

fn application_query_with_relation(
    cardinality: ApplicationQueryCardinality,
) -> ApplicationSchemaMember {
    let entity =
        ApplicationEntityRef::<QuerySchema, QueryEntity>::from_schema_identifier("QueryEntity");
    let child =
        ApplicationEntityRef::<QuerySchema, QueryChild>::from_schema_identifier("QueryChild");
    let relation = ApplicationRelationRef::<
        QuerySchema,
        QueryRelation,
        QueryEntity,
        QueryChild,
    >::from_schema_identifiers("QueryRelation", "QueryEntity", "QueryChild");
    let nested =
        ApplicationQueryResultShapeBuilder::<QuerySchema, QueryMarker, QueryChild, ()>::new(child);
    let shape = ApplicationQueryResultShapeBuilder::<
        QuerySchema,
        QueryMarker,
        QueryEntity,
        QueryResult,
    >::new(entity);
    let shape = match cardinality {
        ApplicationQueryCardinality::OptionalOne => shape.relation(
            ApplicationQueryResultRelationRef::<
                QueryMarker,
                QueryChildSlot,
                QuerySchema,
                QueryRelation,
                QueryEntity,
                QueryChild,
                ForwardResultTraversal,
                OptionalOneResult,
            >::forward_optional("child", relation),
            nested,
        ),
        ApplicationQueryCardinality::ExactlyOne => shape.relation(
            ApplicationQueryResultRelationRef::<
                QueryMarker,
                QueryChildSlot,
                QuerySchema,
                QueryRelation,
                QueryEntity,
                QueryChild,
                ForwardResultTraversal,
                ExactlyOneResult,
            >::forward_one("child", relation),
            nested,
        ),
        ApplicationQueryCardinality::Many => shape.relation(
            ApplicationQueryResultRelationRef::<
                QueryMarker,
                QueryChildSlot,
                QuerySchema,
                QueryRelation,
                QueryEntity,
                QueryChild,
                ForwardResultTraversal,
                ManyResults,
            >::forward_many("child", relation),
            nested,
        ),
    }
    .build();
    let definition =
        ApplicationQueryDefinitionBuilder::declare(ApplicationQueryReference::<
            QuerySchema,
            QueryMarker,
            QueryParameters,
            QueryResult,
            QueryEntity,
        >::from_schema_identifier("query"))
        .root(entity)
        .scope(entity)
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(1, 1, 0))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .public()
        .build()
        .unwrap();
    ApplicationSchemaMember::ApplicationQuery {
        definition: definition.into_erased(),
    }
}
