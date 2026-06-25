use forge_query::facade::{
    ForgeQueryLiveViewBuilder, ForgeQueryRuntimeError, QuerySchemaView, RelationName,
};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::{QueryCollection, QueryLiveField, QuerySchemaBasis};

use crate::query_native_runtime_boundary::query_live_field_key;

pub(crate) const TOPOLOGY_READ_MAX_CYCLE_DEPTH: u8 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TopologyDomainTraversalRelation {
    HalfEdgeStartsAtVertex,
    HalfEdgeEndsAtVertex,
    HalfEdgeUsesEdge,
    HalfEdgeRadialNext,
    HalfEdgeNext,
    HalfEdgePrev,
}

impl TopologyDomainTraversalRelation {
    pub(crate) const ALL: [Self; 6] = [
        Self::HalfEdgeStartsAtVertex,
        Self::HalfEdgeEndsAtVertex,
        Self::HalfEdgeUsesEdge,
        Self::HalfEdgeRadialNext,
        Self::HalfEdgeNext,
        Self::HalfEdgePrev,
    ];

    pub(crate) const fn topology_relation_kind(self) -> TopologyRelationKind {
        match self {
            Self::HalfEdgeStartsAtVertex => TopologyRelationKind::HalfEdgeStartsAtVertex,
            Self::HalfEdgeEndsAtVertex => TopologyRelationKind::HalfEdgeEndsAtVertex,
            Self::HalfEdgeUsesEdge => TopologyRelationKind::HalfEdgeUsesEdge,
            Self::HalfEdgeRadialNext => TopologyRelationKind::HalfEdgeRadialNext,
            Self::HalfEdgeNext => TopologyRelationKind::HalfEdgeNext,
            Self::HalfEdgePrev => TopologyRelationKind::HalfEdgePrev,
        }
    }

    pub(crate) fn relation_name(self) -> RelationName {
        RelationName::new(self.topology_relation_kind().kind_name())
            .expect(" topology traversal relations must be valid relation names")
    }

    pub(crate) const fn max_depth(self) -> u8 {
        match self {
            Self::HalfEdgeStartsAtVertex
            | Self::HalfEdgeEndsAtVertex
            | Self::HalfEdgeUsesEdge
            | Self::HalfEdgeRadialNext
            | Self::HalfEdgePrev => 1,
            Self::HalfEdgeNext => TOPOLOGY_READ_MAX_CYCLE_DEPTH,
        }
    }
}

pub(crate) fn topology_read_schema_view() -> Result<QuerySchemaView, ForgeQueryRuntimeError> {
    let mut builder = ForgeQueryLiveViewBuilder::surface(".topology.topology_read.schema")
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyDomainQuery.as_str())
        .select([
            query_live_field_key(QueryLiveField::IdentityId),
            query_live_field_key(QueryLiveField::TopologyKind),
        ]);
    for relation in TopologyDomainTraversalRelation::ALL {
        builder = builder.allow_traversal_relation(
            RelationKind::Topology(relation.topology_relation_kind()).kind_name(),
            relation.max_depth(),
        );
    }
    builder
        .build()
        .map(|declaration| declaration.schema_view().clone())
}
