use worth_query_declaration::facade::application_schema::ApplicationRelationRef;
use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationEntityKey, WorthQueryApplicationRelationSeed,
    WorthQueryPrimaryGraphBootstrap,
};

use super::super::declaration::*;
use super::super::CompositionScenario;

pub(super) fn bind(
    graph: &mut WorthQueryPrimaryGraphBootstrap<PublicationAuthorizationSchema>,
    scenario: CompositionScenario,
) {
    if !matches!(scenario, CompositionScenario::MissingAuthorization) {
        bind_resource_relation(graph, ResourceOwner::reference(), "resource-owner");
    }
    if matches!(
        scenario,
        CompositionScenario::ExplicitDeny | CompositionScenario::AccumulatedProhibitions
    ) {
        bind_resource_relation(graph, ExplicitDeny::reference(), "explicit-deny");
    }
    if matches!(scenario, CompositionScenario::AccumulatedProhibitions) {
        bind_accumulated_prohibitions(graph);
    }
}

fn bind_accumulated_prohibitions(
    graph: &mut WorthQueryPrimaryGraphBootstrap<PublicationAuthorizationSchema>,
) {
    bind_resource_relation(graph, ConflictingActor::reference(), "conflicting-actor");
    bind_action_actor(
        graph,
        RequestActor::reference(),
        "request-actor",
        "selected-request",
    );
    bind_action_actor(
        graph,
        PriorActor::reference(),
        "prior-actor",
        "selected-prior",
    );
}

fn bind_resource_relation<Relation>(
    graph: &mut WorthQueryPrimaryGraphBootstrap<PublicationAuthorizationSchema>,
    relation: ApplicationRelationRef<PublicationAuthorizationSchema, Relation, Principal, Resource>,
    key: &str,
) {
    graph
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            relation,
            key,
            entity_key("principal-1"),
            entity_key("resource-1"),
        ))
        .unwrap();
}

fn bind_action_actor<Relation>(
    graph: &mut WorthQueryPrimaryGraphBootstrap<PublicationAuthorizationSchema>,
    relation: ApplicationRelationRef<
        PublicationAuthorizationSchema,
        Relation,
        Principal,
        ActionRecord,
    >,
    key: &str,
    record: &str,
) {
    graph
        .bind_relation(WorthQueryApplicationRelationSeed::new(
            relation,
            key,
            entity_key("principal-1"),
            entity_key(record),
        ))
        .unwrap();
}

fn entity_key<Entity>(
    value: &str,
) -> WorthQueryApplicationEntityKey<PublicationAuthorizationSchema, Entity> {
    WorthQueryApplicationEntityKey::new(value).unwrap()
}
