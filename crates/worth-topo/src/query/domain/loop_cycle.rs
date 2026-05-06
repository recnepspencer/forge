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
use super::views::WorthTopologyLoopCycleView;

const TOPOLOGY_ENTITY_ROOT: &str = "WorthTopologyEntity";
const IDENTITY_ASPECT: &str = "identity";
const IDENTITY_FIELD: &str = "id";
const TOPOLOGY_ASPECT: &str = "topology";
const TOPOLOGY_KIND_FIELD: &str = "kind";

impl WorthTopologyDomainQuery {
    #[allow(dead_code)]
    pub(crate) fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Result<WorthTopologyLoopCycleView, WorthTopologyDomainQueryError> {
        let request = WorthTopologyDomainQueryRequest::LoopCycleNeighborhood {
            start_half_edge_identity: start_identity.to_string(),
            depth: u8::try_from(count).expect("supported traversal depth must fit in u8"),
        };
        Self::require_supported_traversal_depth(request.family(), count)?;
        let request_report = self.record_report(self.query_native_loop_cycle_report(
            workspace,
            &request,
            start_identity,
            count,
        )?);
        let cycle_identities = self
            .snapshot_index
            .successor_cycle_identities(start_identity, count)
            .map_err(|error| {
                WorthTopologyDomainQueryError::snapshot_indexed_resolution(error.to_string())
            })?;
        Ok(WorthTopologyLoopCycleView {
            request_report,
            start_half_edge_identity: start_identity.to_string(),
            cycle_identities,
        })
    }

    fn query_native_loop_cycle_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &WorthTopologyDomainQueryRequest,
        start_identity: &str,
        count: usize,
    ) -> Result<WorthTopologyDomainQueryRequestReport, WorthTopologyDomainQueryError> {
        let lowering_artifact = lower_topology_domain_query(request)?;
        let schema_view = worth_topology_domain_query_schema_view().map_err(|error| {
            WorthTopologyDomainQueryError::canonical_lowering_resolution(error.to_string())
        })?;
        let family = workspace
            .define_read_family(
                format!("worth.topology.domain_query.loop_cycle:{start_identity}:{count}"),
                |read| {
                    if count == 1 {
                        read.local_direct_edge_collection(
                            TOPOLOGY_ENTITY_ROOT,
                            schema_view,
                            successor_relation_name(),
                            |query| {
                                query
                                    .project(identity_selector())
                                    .project(topology_kind_selector())
                                    .where_equal(identity_anchor_predicate(start_identity))
                                    .order_by(identity_ordering())
                            },
                            |shape| {
                                shape
                                    .field(identity_result_field())
                                    .field(topology_kind_result_field())
                            },
                        )
                    } else {
                        read.explicit_broad_search_frontier_collection(
                            TOPOLOGY_ENTITY_ROOT,
                            schema_view,
                            [successor_relation_name()],
                            u8::try_from(count).expect("supported traversal depth fits in u8"),
                            |query| {
                                query
                                    .project(identity_selector())
                                    .project(topology_kind_selector())
                                    .where_equal(identity_anchor_predicate(start_identity))
                                    .order_by(identity_ordering())
                            },
                            |shape| {
                                shape
                                    .field(identity_result_field())
                                    .field(topology_kind_result_field())
                            },
                        )
                    }
                },
            )
            .map_err(query_native_execution_denied)?;
        let result = workspace
            .execute_read_family(&family)
            .map_err(query_native_execution_denied)?;
        let receipt = result.receipt();
        if receipt.fallback_class() != &ForgeQueryReadFallbackClass::None {
            return Err(
                WorthTopologyDomainQueryError::query_native_execution_denied(format!(
                    "loop cycle read family unexpectedly executed with fallback `{:?}`",
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

fn query_native_execution_denied(error: ForgeQueryRuntimeError) -> WorthTopologyDomainQueryError {
    WorthTopologyDomainQueryError::query_native_execution_denied(format!("{error:?}"))
}

fn successor_relation_name() -> RelationName {
    WorthTopologyDomainTraversalRelation::HalfEdgeNext.relation_name()
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
