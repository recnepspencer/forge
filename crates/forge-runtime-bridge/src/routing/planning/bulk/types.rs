use std::sync::Arc;

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::facade::BridgeRouteRequest;
use crate::identity::{
    BridgeIdentity, BulkAdmissionProfileIdentityTag, BulkPlanningIdentityTag,
    ContinuityPacketIdentityTag, FallbackPacketIdentityTag, ReducedContinuityIdentityTag,
    ReducedFallbackIdentityTag, ReducedPublicationIdentityTag, ReducedRoutingTargetIdentityTag,
    ReducedTruthViewIdentityTag, ReductionPacketIdentityTag, RoutingPacketIdentityTag,
    TruthViewPacketIdentityTag, WorkloadIdentityTag,
};
use crate::routing::canonicalization::digest_string;
use crate::routing::context::BridgeMappingContext;
use crate::routing::planning::BridgePlannedRoute;

pub type BridgeWorkloadIdentity = BridgeIdentity<WorkloadIdentityTag>;
pub type BridgeCanonicalPlanningIdentity = BridgeIdentity<BulkPlanningIdentityTag>;
pub type BridgeAdmissionProfileIdentity = BridgeIdentity<BulkAdmissionProfileIdentityTag>;
pub type ReducedPublicationIdentity = BridgeIdentity<ReducedPublicationIdentityTag>;
pub type ReducedRoutingTargetIdentity = BridgeIdentity<ReducedRoutingTargetIdentityTag>;
pub type ReducedContinuityIdentity = BridgeIdentity<ReducedContinuityIdentityTag>;
pub type ReducedTruthViewIdentity = BridgeIdentity<ReducedTruthViewIdentityTag>;
pub type ReducedFallbackIdentity = BridgeIdentity<ReducedFallbackIdentityTag>;
pub type ContinuityPacketIdentity = BridgeIdentity<ContinuityPacketIdentityTag>;
pub type FallbackPacketIdentity = BridgeIdentity<FallbackPacketIdentityTag>;
pub type RoutingPacketIdentity = BridgeIdentity<RoutingPacketIdentityTag>;
pub type TruthViewPacketIdentity = BridgeIdentity<TruthViewPacketIdentityTag>;
pub type ReductionPacketIdentity = BridgeIdentity<ReductionPacketIdentityTag>;

pub const BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.bulk-plan-record.v1";

mod admission;
mod decisions;
mod execution_plan;
mod packet_families;
mod planning_surfaces;
mod plan_surface;
mod records;
mod reductions;
mod request_surfaces;
mod route_packets;
mod summary_surfaces;

pub use admission::*;
pub use decisions::*;
pub use execution_plan::*;
pub use packet_families::*;
pub use planning_surfaces::*;
pub use plan_surface::*;
pub use records::*;
pub use reductions::*;
pub use request_surfaces::*;
pub use route_packets::*;
pub use summary_surfaces::*;
