use crate::facade::{
    WorthQueryCollection, WorthQueryLiveDeclarationBuilder, WorthQueryLiveField,
    WorthQuerySchemaBasis, WorthRelationKind, WorthTopologyRelationKind,
};

#[test]
fn worth_live_query_declarations_can_admit_traversal_relations_in_schema_view() {
    let domain_declaration = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.domain-query-schema",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
    ])
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        64,
    )
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex),
        1,
    )
    .build()
    .expect("traversal-aware domain declaration should lower");
    let live_declaration = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.entity-live-schema",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
    ])
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        64,
    )
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex),
        1,
    )
    .build()
    .expect("entity live declaration should lower");

    let next = domain_declaration
        .schema_view()
        .relation(WorthTopologyRelationKind::HalfEdgeNext.kind_name())
        .expect("next traversal relation should be registered");
    let end = domain_declaration
        .schema_view()
        .relation(WorthTopologyRelationKind::HalfEdgeEndsAtVertex.kind_name())
        .expect("end traversal relation should be registered");

    assert_eq!(next.max_depth(), 64);
    assert_eq!(end.max_depth(), 1);
    assert_ne!(
        domain_declaration.schema_view().basis(),
        live_declaration.schema_view().basis()
    );
}

#[test]
fn worth_live_query_declarations_reject_zero_depth_traversal_relations() {
    let error = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.domain-query-schema",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([WorthQueryLiveField::IdentityId])
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        0,
    )
    .build()
    .expect_err("zero-depth traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("must declare a non-zero max depth"));
}

#[test]
fn worth_live_query_declarations_reject_duplicate_traversal_relations() {
    let error = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.domain-query-schema",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([WorthQueryLiveField::IdentityId])
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        2,
    )
    .allow_traversal_relation(
        WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
        4,
    )
    .build()
    .expect_err("duplicate traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("may only be declared once per live view"));
}
