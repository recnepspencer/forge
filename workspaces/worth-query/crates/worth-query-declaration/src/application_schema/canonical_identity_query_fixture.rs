use crate::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryReference, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder,
};
use crate::application_schema::{
    ApplicationEntityRef, ApplicationFieldRef, ApplicationSchemaMember, EqualityPredicate,
    NoApplicationCurrency, ReadOnly,
};

pub(super) struct QuerySchema;
pub(super) struct QueryEntity;
struct QueryAspect;
struct QueryField;
pub(super) struct QueryMarker;
pub(super) struct QueryParameters;
pub(super) struct QueryResult;
struct QueryFieldSlot;

pub(super) fn application_query(output_name: &'static str) -> ApplicationSchemaMember {
    let entity =
        ApplicationEntityRef::<QuerySchema, QueryEntity>::from_schema_identifier("QueryEntity");
    let field = ApplicationFieldRef::<
        QuerySchema,
        QueryEntity,
        QueryAspect,
        QueryField,
        u64,
        ReadOnly,
        EqualityPredicate,
    >::from_schema_identifiers("QueryEntity", "QueryAspect", "QueryField");
    let result_field = ApplicationQueryResultFieldRef::<
        QueryMarker,
        QueryFieldSlot,
        QuerySchema,
        QueryEntity,
        QueryAspect,
        QueryField,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >::new(output_name, field);
    let shape = ApplicationQueryResultShapeBuilder::<
        QuerySchema,
        QueryMarker,
        QueryEntity,
        QueryResult,
    >::new(entity)
    .field(result_field)
    .build();
    let definition = ApplicationQueryDefinitionBuilder::public(
        ApplicationQueryReference::<
            QuerySchema,
            QueryMarker,
            QueryParameters,
            QueryResult,
            QueryEntity,
        >::from_schema_identifier("query"),
        entity,
        entity,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .build()
    .unwrap();
    ApplicationSchemaMember::ApplicationQuery {
        definition: definition.into_erased(),
    }
}
