use crate::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryReference, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder,
};
use crate::application_schema::{
    ApplicationEntityRef, ApplicationFieldRef, ApplicationSchemaMember, EqualityPredicate,
    NoApplicationUnit, ReadOnly,
};

pub(super) struct QuerySchema;
pub(super) struct QueryEntity;
struct QueryAspect;
struct QueryField;
pub(super) struct QueryMarker;
pub(super) struct QueryParameters;
pub(super) struct QueryResult;
struct QueryFieldSlot;

impl crate::application_schema::DeclaredApplicationFieldValue for QueryField {
    type Value = u64;
    const PRESENCE: crate::application_schema::ApplicationFieldPresence =
        crate::application_schema::ApplicationFieldPresence::Required;
}

impl crate::application_schema::RequiredApplicationFieldValue for QueryField {}

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
        NoApplicationUnit,
    >::new(output_name, field);
    let shape = ApplicationQueryResultShapeBuilder::<
        QuerySchema,
        QueryMarker,
        QueryEntity,
        QueryResult,
    >::new(entity)
    .field(result_field)
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
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 1))
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
