mod canonical;
mod ingestion;
mod plan;
mod replay;
mod summaries;

use crate::identity::{BridgeIdentity, RouteIdentityTag};

pub type BridgeRouteIdentity = BridgeIdentity<RouteIdentityTag>;

pub use summaries::{
    BridgeExecutionCounts, BridgePlanningProvenance, BridgePlanningSummary,
    BridgeRouteSourceSummary, BridgeRoutingSummary,
};
pub use plan::BridgePlannedRoute;

pub(crate) use ingestion::IngestedBridgePatch;
pub(crate) use plan::{BridgePreparedDelivery, plan_ingested_patch};
pub(crate) use replay::replay_route_record;
