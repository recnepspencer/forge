use crate::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryOrderingDirection,
    ApplicationQueryParameterRef, ApplicationQueryReference, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder, ErasedApplicationQueryDefinition,
};

use super::{
    ApplicationEntityRef, ApplicationFieldRef, EqualityPredicate, NoApplicationCurrency, ReadOnly,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;
struct Query;
struct Parameters;
struct QueryResult;
struct Parameter;
struct ResultSlot;

impl crate::application_schema::DeclaredApplicationFieldValue for Field {
    type Value = u64;
    const PRESENCE: crate::application_schema::ApplicationFieldPresence =
        crate::application_schema::ApplicationFieldPresence::Required;
}

impl crate::application_schema::RequiredApplicationFieldValue for Field {}

#[derive(Clone)]
struct QueryControlFixture {
    name: &'static str,
    root: &'static str,
    scope: &'static str,
    output: &'static str,
    parameter: &'static str,
    cardinality: ApplicationQueryCardinality,
    ceiling: ApplicationQueryDependencyCeiling,
    disclosure: ApplicationQueryDisclosureContract,
    basis: ApplicationQueryBasisSupport,
    lanes: ApplicationQueryLaneEligibility,
    predicate: bool,
    ordering: ApplicationQueryOrderingDirection,
}

impl QueryControlFixture {
    fn baseline() -> Self {
        Self {
            name: "query",
            root: "Entity",
            scope: "Entity",
            output: "value",
            parameter: "parameter",
            cardinality: ApplicationQueryCardinality::ExactlyOne,
            ceiling: ApplicationQueryDependencyCeiling::bounded(0, 0, 1),
            disclosure: ApplicationQueryDisclosureContract::public(),
            basis: ApplicationQueryBasisSupport::current_and_pinned(),
            lanes: ApplicationQueryLaneEligibility::one_shot(),
            predicate: true,
            ordering: ApplicationQueryOrderingDirection::Ascending,
        }
    }
}

#[test]
fn every_scalar_query_control_is_identity_bearing() {
    let baseline = QueryControlFixture::baseline();
    let baseline_basis = definition(&baseline).canonical_basis().clone();
    let mut variants = Vec::new();

    let mut changed = baseline.clone();
    changed.name = "other_query";
    variants.push(("name", changed));
    let mut changed = baseline.clone();
    changed.root = "OtherRoot";
    variants.push(("root", changed));
    let mut changed = baseline.clone();
    changed.scope = "OtherScope";
    variants.push(("scope", changed));
    let mut changed = baseline.clone();
    changed.output = "other_value";
    variants.push(("output", changed));
    let mut changed = baseline.clone();
    changed.parameter = "other_parameter";
    variants.push(("parameter", changed));
    let mut changed = baseline.clone();
    changed.cardinality = ApplicationQueryCardinality::OptionalOne;
    variants.push(("cardinality", changed));
    let mut changed = baseline.clone();
    changed.ceiling = ApplicationQueryDependencyCeiling::bounded(1, 0, 1);
    variants.push(("traversal ceiling", changed));
    let mut changed = baseline.clone();
    changed.ceiling = ApplicationQueryDependencyCeiling::bounded(0, 1, 1);
    variants.push(("relation ceiling", changed));
    let mut changed = baseline.clone();
    changed.ceiling = ApplicationQueryDependencyCeiling::bounded(0, 0, 2);
    variants.push(("projection ceiling", changed));
    let mut changed = baseline.clone();
    changed.disclosure = ApplicationQueryDisclosureContract::installed_policy("private");
    variants.push(("disclosure posture", changed));
    let mut changed = baseline.clone();
    changed.disclosure = ApplicationQueryDisclosureContract::installed_policy("secret");
    variants.push(("disclosure classification", changed));
    let mut changed = baseline.clone();
    changed.basis = ApplicationQueryBasisSupport::current_and_pinned().with_preview();
    variants.push(("basis support", changed));
    let mut changed = baseline.clone();
    changed.lanes = ApplicationQueryLaneEligibility::one_shot().with_historical();
    variants.push(("lane eligibility", changed));
    let mut changed = baseline.clone();
    changed.predicate = false;
    variants.push(("predicate", changed));
    let mut changed = baseline;
    changed.ordering = ApplicationQueryOrderingDirection::Descending;
    variants.push(("ordering", changed));

    for (dimension, variant) in variants {
        assert_ne!(
            baseline_basis,
            definition(&variant).canonical_basis().clone(),
            "{dimension} must change the canonical query basis"
        );
    }
}

fn definition(fixture: &QueryControlFixture) -> ErasedApplicationQueryDefinition {
    let root = ApplicationEntityRef::<Schema, Entity>::from_schema_identifier(fixture.root);
    let scope = ApplicationEntityRef::<Schema, Entity>::from_schema_identifier(fixture.scope);
    let field = ApplicationFieldRef::<
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >::from_schema_identifiers(fixture.root, "Aspect", "Field");
    let result = ApplicationQueryResultFieldRef::<
        Query,
        ResultSlot,
        Schema,
        Entity,
        Aspect,
        Field,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >::new(fixture.output, field);
    let parameter = ApplicationQueryParameterRef::<Query, Parameter, u64>::from_query_identifier(
        fixture.parameter,
    );
    let shape = ApplicationQueryResultShapeBuilder::<Schema, Query, Entity, QueryResult>::new(root)
        .field(result)
        .build();
    let builder = ApplicationQueryDefinitionBuilder::public(
        ApplicationQueryReference::<Schema, Query, Parameters, QueryResult, Entity>::
            from_schema_identifier(fixture.name),
        root,
        scope,
        shape,
        fixture.cardinality,
        fixture.ceiling,
        fixture.disclosure.clone(),
        fixture.basis,
        fixture.lanes,
    )
    .parameter(parameter)
    .order_by(result, fixture.ordering);
    let builder = if fixture.predicate {
        builder.where_equal(field, parameter)
    } else {
        builder
    };
    builder
        .build()
        .expect("one-axis identity fixture should remain valid")
        .into_erased()
}
