use crate::facade::{
    QueryCollection, QueryLiveDeclarationBuilder, QueryLiveField, QuerySchemaBasis, RelationKind,
    TopologyRelationKind,
};

#[test]
fn live_query_declarations_can_admit_traversal_relations_in_schema_view() {
    let domain_declaration = QueryLiveDeclarationBuilder::new(
        ".topology.domain-query-schema",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([QueryLiveField::IdentityId, QueryLiveField::TopologyKind])
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
        64,
    )
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex),
        1,
    )
    .build()
    .expect("traversal-aware domain declaration should lower");
    let live_declaration = QueryLiveDeclarationBuilder::new(
        ".topology.entity-live-schema",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .select_fields([QueryLiveField::IdentityId, QueryLiveField::TopologyKind])
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
        64,
    )
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex),
        1,
    )
    .build()
    .expect("entity live declaration should lower");

    let next = domain_declaration
        .schema_view()
        .relation(TopologyRelationKind::HalfEdgeNext.kind_name())
        .expect("next traversal relation should be registered");
    let end = domain_declaration
        .schema_view()
        .relation(TopologyRelationKind::HalfEdgeEndsAtVertex.kind_name())
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
    let error = QueryLiveDeclarationBuilder::new(
        ".topology.domain-query-schema",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([QueryLiveField::IdentityId])
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
        0,
    )
    .build()
    .expect_err("zero-depth traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("must declare a non-zero max depth"));
}

#[test]
fn live_query_declarations_reject_duplicate_traversal_relations() {
    let error = QueryLiveDeclarationBuilder::new(
        ".topology.domain-query-schema",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([QueryLiveField::IdentityId])
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
        2,
    )
    .allow_traversal_relation(
        RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
        4,
    )
    .build()
    .expect_err("duplicate traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("may only be declared once per live view"));
}
