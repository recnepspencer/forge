use forge_query::facade::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, ForgeQueryReadFallbackClass,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, OrderingSelector, RelationName,
    ScalarPredicateValue,
};

use super::error::WorthTopologyDomainQueryError;
use super::lowering::lower_topology_domain_query;
use super::report::WorthTopologyDomainQueryRequestReport;
use super::request::WorthTopologyDomainQueryRequest;
use super::schema::{
    worth_topology_domain_query_schema_view, WorthTopologyDomainTraversalRelation,
};
use super::topology::WorthTopologyDomainQuery;
use super::views::{
    WorthTopologyHalfEdgeRadialNeighborhoodView, WorthTopologyHalfEdgeSharedVertexNeighborhoodView,
};
use worth_schema::facade::WorthTopologyRelationKind;

const TOPOLOGY_ENTITY_ROOT: &str = "WorthTopologyEntity";
const IDENTITY_ASPECT: &str = "identity";
const IDENTITY_FIELD: &str = "id";
const TOPOLOGY_ASPECT: &str = "topology";
const TOPOLOGY_KIND_FIELD: &str = "kind";

impl WorthTopologyDomainQuery {
    pub(crate) fn shared_vertex_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<WorthTopologyHalfEdgeSharedVertexNeighborhoodView, WorthTopologyDomainQueryError>
    {
        let request = WorthTopologyDomainQueryRequest::HalfEdgeSharedVertexNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let request_report = self.record_report(self.query_native_shared_vertex_report(
            workspace,
            &request,
            source_identity,
        )?);
        let source_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let source_vertex_identities = self
            .snapshot_index
            .half_edge_vertex_identities(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let vertex_adjacent_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_sharing_vertex(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let vertex_adjacent_different_edge_half_edge_identities =
            vertex_adjacent_half_edge_identities
                .iter()
                .filter(|identity| {
                    self.snapshot_index
                        .edge_identity_of_half_edge(identity)
                        .is_ok_and(|edge_identity| edge_identity != source_edge_identity)
                })
                .cloned()
                .collect();
        Ok(WorthTopologyHalfEdgeSharedVertexNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            source_vertex_identities,
            vertex_adjacent_half_edge_identities,
            vertex_adjacent_different_edge_half_edge_identities,
        })
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<WorthTopologyHalfEdgeRadialNeighborhoodView, WorthTopologyDomainQueryError> {
        let request = WorthTopologyDomainQueryRequest::HalfEdgeRadialNeighborhood {
            source_half_edge_identity: source_identity.to_string(),
        };
        let request_report = self.record_report(self.query_native_radial_report(
            workspace,
            &request,
            source_identity,
        )?);
        let source_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let current_target_half_edge_identity = self
            .snapshot_index
            .outgoing_target_identity(
                source_identity,
                WorthTopologyRelationKind::HalfEdgeRadialNext,
            )
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let current_target_edge_identity = self
            .snapshot_index
            .edge_identity_of_half_edge(&current_target_half_edge_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let same_edge_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_on_same_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        let different_edge_half_edge_identities = self
            .snapshot_index
            .half_edge_identities_on_different_edge(source_identity)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        Ok(WorthTopologyHalfEdgeRadialNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity.to_string(),
            source_edge_identity,
            current_target_half_edge_identity,
            current_target_edge_identity,
            same_edge_half_edge_identities,
            different_edge_half_edge_identities,
        })
    }

    fn query_native_shared_vertex_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &WorthTopologyDomainQueryRequest,
        source_identity: &str,
    ) -> Result<WorthTopologyDomainQueryRequestReport, WorthTopologyDomainQueryError> {
        self.query_native_local_shared_neighborhood_report(
            workspace,
            request,
            format!("worth.topology.domain_query.shared_vertex:{source_identity}"),
            [
                starts_at_vertex_relation_name(),
                ends_at_vertex_relation_name(),
            ],
            SharedNeighborhoodOperator::Endpoint,
            source_identity,
        )
    }

    fn query_native_radial_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &WorthTopologyDomainQueryRequest,
        source_identity: &str,
    ) -> Result<WorthTopologyDomainQueryRequestReport, WorthTopologyDomainQueryError> {
        self.query_native_local_shared_neighborhood_report(
            workspace,
            request,
            format!("worth.topology.domain_query.radial:{source_identity}"),
            [radial_next_relation_name(), uses_edge_relation_name()],
            SharedNeighborhoodOperator::Attachment,
            source_identity,
        )
    }

    fn query_native_local_shared_neighborhood_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &WorthTopologyDomainQueryRequest,
        family_name: String,
        relations: [RelationName; 2],
        operator: SharedNeighborhoodOperator,
        anchor_identity: &str,
    ) -> Result<WorthTopologyDomainQueryRequestReport, WorthTopologyDomainQueryError> {
        let lowering_artifact = lower_topology_domain_query(request)?;
        let schema_view = worth_topology_domain_query_schema_view().map_err(|error| {
            WorthTopologyDomainQueryError::canonical_lowering_resolution(error.to_string())
        })?;
        let family = workspace
            .define_read_family(family_name, |read| match operator {
                SharedNeighborhoodOperator::Endpoint => read.local_shared_endpoint_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    relations,
                    |query| {
                        query
                            .project(identity_selector())
                            .project(topology_kind_selector())
                            .where_equal(identity_anchor_predicate(anchor_identity))
                            .order_by(identity_ordering())
                    },
                    |shape| {
                        shape
                            .field(identity_result_field())
                            .field(topology_kind_result_field())
                    },
                ),
                SharedNeighborhoodOperator::Attachment => read.local_shared_attachment_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    relations,
                    |query| {
                        query
                            .project(identity_selector())
                            .project(topology_kind_selector())
                            .where_equal(identity_anchor_predicate(anchor_identity))
                            .order_by(identity_ordering())
                    },
                    |shape| {
                        shape
                            .field(identity_result_field())
                            .field(topology_kind_result_field())
                    },
                ),
            })
            .map_err(query_native_execution_denied)?;
        let result = workspace
            .execute_read_family(&family)
            .map_err(query_native_execution_denied)?;
        let receipt = result.receipt();
        if receipt.fallback_class() != &ForgeQueryReadFallbackClass::None {
            return Err(
                WorthTopologyDomainQueryError::query_native_execution_denied(format!(
                    "shared neighborhood read family unexpectedly executed with fallback `{:?}`",
                    receipt.fallback_class()
                )),
            );
        }
        Ok(
            WorthTopologyDomainQueryRequestReport::query_runtime_current_whole_view_debt(
                lowering_artifact,
                receipt,
            ),
        )
    }
}

#[derive(Clone, Copy)]
enum SharedNeighborhoodOperator {
    Endpoint,
    Attachment,
}

fn query_native_execution_denied(error: ForgeQueryRuntimeError) -> WorthTopologyDomainQueryError {
    WorthTopologyDomainQueryError::query_native_execution_denied(format!("{error:?}"))
}

fn starts_at_vertex_relation_name() -> RelationName {
    WorthTopologyDomainTraversalRelation::HalfEdgeStartsAtVertex.relation_name()
}

fn ends_at_vertex_relation_name() -> RelationName {
    WorthTopologyDomainTraversalRelation::HalfEdgeEndsAtVertex.relation_name()
}

fn radial_next_relation_name() -> RelationName {
    WorthTopologyDomainTraversalRelation::HalfEdgeRadialNext.relation_name()
}

fn uses_edge_relation_name() -> RelationName {
    WorthTopologyDomainTraversalRelation::HalfEdgeUsesEdge.relation_name()
}

fn identity_selector() -> AspectFieldSelector {
    AspectFieldSelector::new(IDENTITY_ASPECT, IDENTITY_FIELD)
        .expect("identity selector should build for worth topology domain reads")
}

fn topology_kind_selector() -> AspectFieldSelector {
    AspectFieldSelector::new(TOPOLOGY_ASPECT, TOPOLOGY_KIND_FIELD)
        .expect("topology kind selector should build for worth topology domain reads")
}

fn identity_anchor_predicate(anchor_identity: &str) -> EqualityPredicate {
    EqualityPredicate::new(
        IDENTITY_ASPECT,
        IDENTITY_FIELD,
        ScalarPredicateValue::String(anchor_identity.to_string()),
    )
    .expect("identity anchor predicate should build for worth topology domain reads")
}

fn identity_ordering() -> OrderingSelector {
    OrderingSelector::ascending(IDENTITY_ASPECT, IDENTITY_FIELD)
        .expect("identity ordering should build for worth topology domain reads")
}

fn identity_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(IDENTITY_ASPECT, IDENTITY_FIELD, IDENTITY_FIELD)
        .expect("identity result field should build for worth topology domain reads")
}

fn topology_kind_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(TOPOLOGY_ASPECT, TOPOLOGY_KIND_FIELD, TOPOLOGY_KIND_FIELD)
        .expect("topology kind result field should build for worth topology domain reads")
}
