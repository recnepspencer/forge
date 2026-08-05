use crate::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryLiveCauseBinding,
    ApplicationQueryLiveResourceContract, ApplicationQueryOrderingDirection,
    ApplicationQueryReference, ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
    ApplicationQueryResultShapeBuilder, ErasedApplicationQueryDefinition, ForwardResultTraversal,
    ManyResults,
};

use super::{
    ApplicationEffectPayload, ApplicationEffectRef, ApplicationEntityRef, ApplicationFieldRef,
    ApplicationRelationRef, EqualityPredicate, NoApplicationCurrency, ReadOnly,
};

struct Schema;
struct Root;
struct Child;
struct RootAspect;
struct ChildAspect;
struct RootIdentity;
struct ChildIdentity;
struct Relation;
struct Query;
struct OtherQuery;
struct Parameters;
struct OtherParameters;
struct QueryResult;
struct OtherQueryResult;
struct OtherScope;
struct RootIdentitySlot;
struct ChildIdentitySlot;
struct ChildRelationSlot;
struct Effect;

impl crate::application_schema::DeclaredApplicationFieldValue for RootIdentity {
    type Value = u64;
    const PRESENCE: crate::application_schema::ApplicationFieldPresence =
        crate::application_schema::ApplicationFieldPresence::Required;
}

impl crate::application_schema::RequiredApplicationFieldValue for RootIdentity {}

impl crate::application_schema::DeclaredApplicationFieldValue for ChildIdentity {
    type Value = u64;
    const PRESENCE: crate::application_schema::ApplicationFieldPresence =
        crate::application_schema::ApplicationFieldPresence::Required;
}

impl crate::application_schema::RequiredApplicationFieldValue for ChildIdentity {}

#[derive(Clone)]
struct Cause {
    root: u64,
    child: u64,
}

impl ApplicationEffectPayload for Cause {
    fn retained_bytes(&self) -> u64 {
        std::mem::size_of::<Self>() as u64
    }
}

struct LiveBinding;
struct OtherLiveBinding;

impl ApplicationQueryLiveCauseBinding<Schema, Query, Root, Child> for LiveBinding {
    type Effect = Effect;
    type Payload = Cause;
    type ScopeIdentity = u64;
    type TargetIdentity = u64;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
        ApplicationEffectRef::from_schema_identifier("Cause")
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.root
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.child
    }
}

impl ApplicationQueryLiveCauseBinding<Schema, Query, Root, Child> for OtherLiveBinding {
    type Effect = Effect;
    type Payload = Cause;
    type ScopeIdentity = u64;
    type TargetIdentity = u64;

    fn effect() -> ApplicationEffectRef<Schema, Self::Effect, Self::Payload> {
        ApplicationEffectRef::from_schema_identifier("Cause")
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.root
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.child
    }
}

#[test]
fn every_query_marker_type_is_identity_bearing() {
    let baseline = typed_definition::<Query, Parameters, QueryResult, Root>();

    for (dimension, changed) in [
        (
            "query marker",
            typed_definition::<OtherQuery, Parameters, QueryResult, Root>(),
        ),
        (
            "parameter-set marker",
            typed_definition::<Query, OtherParameters, QueryResult, Root>(),
        ),
        (
            "result marker",
            typed_definition::<Query, Parameters, OtherQueryResult, Root>(),
        ),
        (
            "scope marker",
            typed_definition::<Query, Parameters, QueryResult, OtherScope>(),
        ),
    ] {
        assert_ne!(
            baseline.canonical_basis(),
            changed.canonical_basis(),
            "{dimension} must change the canonical query basis"
        );
    }
}

#[test]
fn continuation_presence_is_identity_bearing() {
    let without_continuation = collection_definition(false);
    let with_continuation = collection_definition(true);

    assert_ne!(
        without_continuation.canonical_basis(),
        with_continuation.canonical_basis()
    );
}

#[test]
fn live_binding_and_each_resource_bound_are_identity_bearing() {
    let baseline =
        live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(8, 64, 512));
    let variants = [
        (
            "binding type",
            live_definition::<OtherLiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 64, 512,
            )),
        ),
        (
            "buffered cause bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                9, 64, 512,
            )),
        ),
        (
            "delivery work bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 65, 512,
            )),
        ),
        (
            "retained payload bound",
            live_definition::<LiveBinding>(ApplicationQueryLiveResourceContract::bounded(
                8, 64, 513,
            )),
        ),
    ];

    for (dimension, changed) in variants {
        assert_ne!(
            baseline.canonical_basis(),
            changed.canonical_basis(),
            "{dimension} must change the canonical query basis"
        );
    }
}

fn typed_definition<QueryMarker: 'static, ParameterMarker, ResultMarker, ScopeMarker>(
) -> ErasedApplicationQueryDefinition {
    let root = ApplicationEntityRef::<Schema, Root>::from_schema_identifier("Root");
    let scope = ApplicationEntityRef::<Schema, ScopeMarker>::from_schema_identifier("Root");
    let identity = ApplicationQueryResultFieldRef::<
        QueryMarker,
        RootIdentitySlot,
        Schema,
        Root,
        RootAspect,
        RootIdentity,
        u64,
        ReadOnly,
        EqualityPredicate,
        NoApplicationCurrency,
    >::new("root", root_identity_field());
    let shape =
        ApplicationQueryResultShapeBuilder::<Schema, QueryMarker, Root, ResultMarker>::new(root)
            .field(identity)
            .build();
    ApplicationQueryDefinitionBuilder::public(
        ApplicationQueryReference::<
            Schema,
            QueryMarker,
            ParameterMarker,
            ResultMarker,
            ScopeMarker,
        >::from_schema_identifier("query"),
        root,
        scope,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 1),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .build()
    .expect("typed identity fixture must be lawful")
    .into_erased()
}

fn collection_definition(continuation: bool) -> ErasedApplicationQueryDefinition {
    let root = root_entity();
    let child = child_entity();
    let child_shape = ApplicationQueryResultShapeBuilder::<Schema, Query, Child, ()>::new(child)
        .field(child_identity());
    let shape = ApplicationQueryResultShapeBuilder::<Schema, Query, Root, QueryResult>::new(root)
        .field(root_identity())
        .relation(children(), child_shape)
        .build();
    let builder = ApplicationQueryDefinitionBuilder::public(
        query_reference(),
        root,
        root,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 2),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
    )
    .order_by(
        child_identity(),
        ApplicationQueryOrderingDirection::Ascending,
    );
    let builder = if continuation {
        builder.continue_by(children())
    } else {
        builder
    };
    builder
        .build()
        .expect("continuation identity fixture must be lawful")
        .into_erased()
}

fn live_definition<Binding>(
    resources: ApplicationQueryLiveResourceContract,
) -> ErasedApplicationQueryDefinition
where
    Binding: ApplicationQueryLiveCauseBinding<
        Schema,
        Query,
        Root,
        Child,
        ScopeIdentity = u64,
        TargetIdentity = u64,
    >,
{
    let root = root_entity();
    let child = child_entity();
    let child_shape = ApplicationQueryResultShapeBuilder::<Schema, Query, Child, ()>::new(child)
        .field(child_identity());
    let shape = ApplicationQueryResultShapeBuilder::<Schema, Query, Root, QueryResult>::new(root)
        .field(root_identity())
        .relation(children(), child_shape)
        .build();
    ApplicationQueryDefinitionBuilder::public(
        query_reference(),
        root,
        root,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 2),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot().with_live(),
    )
    .order_by(
        child_identity(),
        ApplicationQueryOrderingDirection::Ascending,
    )
    .continue_by(children())
    .live_by::<Child, Binding, _, _, _, _, _, _, _, _>(root_identity(), child_identity(), resources)
    .build()
    .expect("live identity fixture must be lawful")
    .into_erased()
}

fn query_reference() -> ApplicationQueryReference<Schema, Query, Parameters, QueryResult, Root> {
    ApplicationQueryReference::from_schema_identifier("query")
}

fn root_entity() -> ApplicationEntityRef<Schema, Root> {
    ApplicationEntityRef::from_schema_identifier("Root")
}

fn child_entity() -> ApplicationEntityRef<Schema, Child> {
    ApplicationEntityRef::from_schema_identifier("Child")
}

fn root_identity_field() -> ApplicationFieldRef<
    Schema,
    Root,
    RootAspect,
    RootIdentity,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationFieldRef::from_schema_identifiers("Root", "Identity", "Id")
}

fn root_identity() -> ApplicationQueryResultFieldRef<
    Query,
    RootIdentitySlot,
    Schema,
    Root,
    RootAspect,
    RootIdentity,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("root", root_identity_field())
}

fn child_identity() -> ApplicationQueryResultFieldRef<
    Query,
    ChildIdentitySlot,
    Schema,
    Child,
    ChildAspect,
    ChildIdentity,
    u64,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new(
        "child",
        ApplicationFieldRef::from_schema_identifiers("Child", "Identity", "Id"),
    )
}

fn children() -> ApplicationQueryResultRelationRef<
    Query,
    ChildRelationSlot,
    Schema,
    Relation,
    Root,
    Child,
    ForwardResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::forward_many(
        "children",
        ApplicationRelationRef::from_schema_identifiers("Children", "Root", "Child"),
    )
}
