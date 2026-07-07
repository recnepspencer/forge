use forge_query::facade::foundation::{AspectFieldKey, RelationName};
use forge_query::facade::ForgeQueryLiveViewBuilder;

use crate::facade::platform::relations::{RelationKind, TopologyRelationKind};
use crate::facade::{QueryCollection, QueryLiveField, QuerySchemaBasis};

fn live_field_key(field: QueryLiveField) -> AspectFieldKey {
    AspectFieldKey::from_authoring_parts(field.aspect(), field.field())
        .expect("worth schema live fields should admit as forge-query field keys")
}

fn relation_name(name: &str) -> RelationName {
    RelationName::new(name).expect("relation name should admit")
}

#[test]
fn live_query_declarations_can_admit_traversal_relations_in_schema_view() {
    let domain_declaration = ForgeQueryLiveViewBuilder::surface(".topology.domain-query-schema")
        .select([
            live_field_key(QueryLiveField::IdentityId),
            live_field_key(QueryLiveField::TopologyKind),
        ])
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext).kind_name(),
            64,
        )
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex).kind_name(),
            1,
        )
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyDomainQuery.as_str())
        .build()
        .expect("traversal-aware domain declaration should lower");
    let live_declaration = ForgeQueryLiveViewBuilder::surface(".topology.entity-live-schema")
        .select([
            live_field_key(QueryLiveField::IdentityId),
            live_field_key(QueryLiveField::TopologyKind),
        ])
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext).kind_name(),
            64,
        )
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex).kind_name(),
            1,
        )
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
        .build()
        .expect("entity live declaration should lower");

    let next = domain_declaration
        .schema_view()
        .relation(&relation_name(TopologyRelationKind::HalfEdgeNext.kind_name()))
        .expect("next traversal relation should be registered");
    let end = domain_declaration
        .schema_view()
        .relation(&relation_name(
            TopologyRelationKind::HalfEdgeEndsAtVertex.kind_name()
        ))
        .expect("end traversal relation should be registered");

    assert_eq!(next.max_depth(), 64);
    assert_eq!(end.max_depth(), 1);
    assert_ne!(
        domain_declaration.schema_view().basis(),
        live_declaration.schema_view().basis()
    );
}

#[test]
fn live_query_declarations_reject_zero_depth_traversal_relations() {
    let error = ForgeQueryLiveViewBuilder::surface(".topology.domain-query-schema")
        .select([live_field_key(QueryLiveField::IdentityId)])
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext).kind_name(),
            0,
        )
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyDomainQuery.as_str())
        .build()
        .expect_err("zero-depth traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("must declare a non-zero max depth"));
}

#[test]
fn live_query_declarations_reject_duplicate_traversal_relations() {
    let error = ForgeQueryLiveViewBuilder::surface(".topology.domain-query-schema")
        .select([live_field_key(QueryLiveField::IdentityId)])
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext).kind_name(),
            2,
        )
        .allow_traversal_relation(
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext).kind_name(),
            4,
        )
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyDomainQuery.as_str())
        .build()
        .expect_err("duplicate traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("may only be declared once per live view"));
}
