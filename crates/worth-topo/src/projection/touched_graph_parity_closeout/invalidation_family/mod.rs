mod declaration_row;

use crate::projection::planner_owned_routing::invalidation_route::current_topology_invalidation_route_input;
use crate::projection::touched_graph_parity_closeout::{
    TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError,
    TopologyTouchedGraphParityQuerySurfaceKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

pub use declaration_row::current_topology_invalidation_declaration_row;

pub const TOPOLOGY_INVALIDATION_CLAIM_PATH: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/invalidation_family/mod.rs";
pub const TOPOLOGY_INVALIDATION_REPLACEMENT_LANE: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/invalidation_family/";

const SOURCE_PATH: &str =
    "crates/worth-topo/src/projection/planner_owned_routing/invalidation_route/route_input.rs";
const ORDINARY_CALLER_PATH: &str =
    "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs";
const SELECTED_FIELDS: &[&str] = &[
    "touched_closure_digest",
    "selected_plan_digest",
    "selected_row_family_identities",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyInvalidationRoutePacket {
    packet_identity: String,
    touched_closure_digest: String,
    selected_plan_digest: String,
    routing_contract_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    selected_row_family_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyInvalidationRoutePacketCurrentError {
    detail: String,
}

pub fn current_topology_invalidation_coverage_contributor(
) -> Result<TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError>
{
    current_topology_invalidation_route_input()
        .map_err(|error| TopologyTouchedGraphParityCoverageError::new(error.detail()))?;

    Ok(TopologyTouchedGraphParityCoverageContributor::new(
        "current_topology_invalidation_route_input",
        SOURCE_PATH,
        "current_topology_invalidation_route_input",
        "current_topology_invalidation_route_input",
        "invalidation_current_builder",
        TOPOLOGY_INVALIDATION_REPLACEMENT_LANE,
        SELECTED_FIELDS,
        TopologyTouchedGraphParityQuerySurfaceKind::NotQuery,
        "current_worth_touched_graph_conflict_selected_route_packet",
        ORDINARY_CALLER_PATH,
    ))
}

pub fn current_topology_invalidation_route_packet(
) -> Result<TopologyInvalidationRoutePacket, TopologyInvalidationRoutePacketCurrentError> {
    let route_input = current_topology_invalidation_route_input()
        .map_err(TopologyInvalidationRoutePacketCurrentError::from_input)?;
    let touched_closure_digest = route_input.touched_closure_digest().to_string();
    let selected_plan_digest = route_input.selected_plan_digest().to_string();
    let routing_contract_digest = route_input.routing_contract_digest().to_string();
    let query_support_digest = route_input.query_support_digest().to_string();
    let legality_support_digest = route_input.legality_support_digest().to_string();
    let selected_row_family_identities = route_input
        .selected_rows()
        .iter()
        .map(|row| row.family_identity().as_str().to_string())
        .collect::<Vec<_>>();
    let packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:invalidation-route-packet:v1".to_string(),
            format!("touched-closure:{touched_closure_digest}"),
            format!("selected-plan:{selected_plan_digest}"),
            format!("routing-contract:{routing_contract_digest}"),
            format!("query-support:{query_support_digest}"),
            format!("legality-support:{legality_support_digest}"),
            format!(
                "selected-row-families:{}",
                selected_row_family_identities.join(",")
            ),
        ],
    );

    Ok(TopologyInvalidationRoutePacket {
        packet_identity,
        touched_closure_digest,
        selected_plan_digest,
        routing_contract_digest,
        query_support_digest,
        legality_support_digest,
        selected_row_family_identities,
    })
}

impl TopologyInvalidationRoutePacket {
    pub fn packet_identity(&self) -> &str {
        &self.packet_identity
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn routing_contract_digest(&self) -> &str {
        &self.routing_contract_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn selected_row_family_identities(&self) -> &[String] {
        &self.selected_row_family_identities
    }
}

impl TopologyInvalidationRoutePacketCurrentError {
    fn from_input(
        error: crate::projection::planner_owned_routing::invalidation_route::TopologyInvalidationRouteInputCurrentError,
    ) -> Self {
        Self {
            detail: error.detail().to_string(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
