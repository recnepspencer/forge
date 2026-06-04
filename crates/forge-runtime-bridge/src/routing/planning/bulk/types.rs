use std::sync::Arc;

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::facade::BridgeRouteRequest;
use crate::identity::{
    BridgeIdentity, BulkAdmissionProfileIdentityTag, BulkContinuityMemberIdentityTag,
    BulkPacketRegionIdentityTag, BulkPlanningIdentityTag, BulkTruthViewMemberIdentityTag,
    BulkWorkloadSegmentIdentityTag, ContinuityPacketIdentityTag, ReducedContinuityIdentityTag,
    ReducedPublicationIdentityTag, ReducedRoutingTargetIdentityTag, ReducedTruthViewIdentityTag,
    ReducedWideningIdentityTag, ReductionPacketIdentityTag, RoutingPacketIdentityTag,
    TruthViewPacketIdentityTag, WideningPacketIdentityTag, WorkloadIdentityTag,
};
use crate::input::envelope::{TruthBranchIdentity, TruthCommitIdentity};
use crate::mapping::BridgeMappingWideningClass;
use crate::routing::canonicalization::digest_string;
use crate::routing::context::BridgeMappingContext;
use crate::routing::planning::{BridgePlannedRoute, BridgeRouteIdentity};
use crate::routing::surfaces::TruthDeltaSurfaceIdentity;
use crate::routing::BridgeSubscriptionSliceIdentity;
use crate::snapshot::TruthSnapshotIdentity;

pub type BridgeWorkloadIdentity = BridgeIdentity<WorkloadIdentityTag>;
pub type BridgeCanonicalPlanningIdentity = BridgeIdentity<BulkPlanningIdentityTag>;
pub type BridgeAdmissionProfileIdentity = BridgeIdentity<BulkAdmissionProfileIdentityTag>;
pub type BulkPacketRegionIdentity = BridgeIdentity<BulkPacketRegionIdentityTag>;
pub type BulkContinuityMemberIdentity = BridgeIdentity<BulkContinuityMemberIdentityTag>;
pub type BulkTruthViewMemberIdentity = BridgeIdentity<BulkTruthViewMemberIdentityTag>;
pub type BulkWorkloadSegmentIdentity = BridgeIdentity<BulkWorkloadSegmentIdentityTag>;
pub type ReducedPublicationIdentity = BridgeIdentity<ReducedPublicationIdentityTag>;
pub type ReducedRoutingTargetIdentity = BridgeIdentity<ReducedRoutingTargetIdentityTag>;
pub type ReducedContinuityIdentity = BridgeIdentity<ReducedContinuityIdentityTag>;
pub type ReducedTruthViewIdentity = BridgeIdentity<ReducedTruthViewIdentityTag>;
pub type ReducedWideningIdentity = BridgeIdentity<ReducedWideningIdentityTag>;
pub type ContinuityPacketIdentity = BridgeIdentity<ContinuityPacketIdentityTag>;
pub type WideningPacketIdentity = BridgeIdentity<WideningPacketIdentityTag>;
pub type RoutingPacketIdentity = BridgeIdentity<RoutingPacketIdentityTag>;
pub type TruthViewPacketIdentity = BridgeIdentity<TruthViewPacketIdentityTag>;
pub type ReductionPacketIdentity = BridgeIdentity<ReductionPacketIdentityTag>;

pub(super) fn mapping_widening_class_basis(class: BridgeMappingWideningClass) -> &'static str {
    match class {
        BridgeMappingWideningClass::Entity => "entity",
        BridgeMappingWideningClass::Aspect => "aspect",
        BridgeMappingWideningClass::Surface => "surface",
        BridgeMappingWideningClass::EntityAspect => "entity-aspect",
        BridgeMappingWideningClass::EntitySurface => "entity-surface",
        BridgeMappingWideningClass::AspectSurface => "aspect-surface",
        BridgeMappingWideningClass::EntityAspectSurface => "entity-aspect-surface",
    }
}

pub const BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.bulk-plan-record.v1";

mod admission;
mod decisions;
mod execution_plan;
mod packet_families;
mod plan_surface;
mod planning_surfaces;
mod records;
mod reductions;
mod request_surfaces;
mod route_packets;
mod summary_surfaces;

pub use admission::*;
pub use decisions::*;
pub use execution_plan::*;
pub use packet_families::*;
pub use plan_surface::*;
pub use planning_surfaces::*;
pub use records::*;
pub use reductions::*;
pub use request_surfaces::*;
pub use route_packets::*;
pub use summary_surfaces::*;
