use forge_query::facade::{QuerySchemaView, RelationName};
use worth_schema::facade::{
    WorthQueryCollection, WorthQueryDeclarationError, WorthQueryLiveDeclarationBuilder,
    WorthQueryLiveField, WorthQuerySchemaBasis, WorthRelationKind, WorthTopologyRelationKind,
};

pub(crate) const TOPOLOGY_DOMAIN_QUERY_MAX_CYCLE_DEPTH: u8 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum WorthTopologyDomainTraversalRelation {
    HalfEdgeStartsAtVertex,
    HalfEdgeEndsAtVertex,
    HalfEdgeUsesEdge,
    HalfEdgeRadialNext,
    HalfEdgeNext,
    HalfEdgePrev,
}

impl WorthTopologyDomainTraversalRelation {
    pub(crate) const ALL: [Self; 6] = [
        Self::HalfEdgeStartsAtVertex,
        Self::HalfEdgeEndsAtVertex,
        Self::HalfEdgeUsesEdge,
        Self::HalfEdgeRadialNext,
        Self::HalfEdgeNext,
        Self::HalfEdgePrev,
    ];

    pub(crate) const fn topology_relation_kind(self) -> WorthTopologyRelationKind {
        match self {
            Self::HalfEdgeStartsAtVertex => WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
            Self::HalfEdgeEndsAtVertex => WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
            Self::HalfEdgeUsesEdge => WorthTopologyRelationKind::HalfEdgeUsesEdge,
            Self::HalfEdgeRadialNext => WorthTopologyRelationKind::HalfEdgeRadialNext,
            Self::HalfEdgeNext => WorthTopologyRelationKind::HalfEdgeNext,
            Self::HalfEdgePrev => WorthTopologyRelationKind::HalfEdgePrev,
        }
    }

    pub(crate) fn relation_name(self) -> RelationName {
        RelationName::new(self.topology_relation_kind().kind_name())
            .expect("worth topology traversal relations must be valid relation names")
    }

    pub(crate) const fn max_depth(self) -> u8 {
        match self {
            Self::HalfEdgeStartsAtVertex
            | Self::HalfEdgeEndsAtVertex
            | Self::HalfEdgeUsesEdge
            | Self::HalfEdgeRadialNext
            | Self::HalfEdgePrev => 1,
            Self::HalfEdgeNext => TOPOLOGY_DOMAIN_QUERY_MAX_CYCLE_DEPTH,
        }
    }
}

pub(crate) fn worth_topology_domain_query_schema_view(
) -> Result<QuerySchemaView, WorthQueryDeclarationError> {
    let mut builder = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.domain_query.schema",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyDomainQuery,
    )
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
    ]);
    for relation in WorthTopologyDomainTraversalRelation::ALL {
        builder = builder.allow_traversal_relation(
            WorthRelationKind::Topology(relation.topology_relation_kind()),
            relation.max_depth(),
        );
    }
    builder
        .build()
        .map(|declaration| declaration.schema_view().clone())
}
