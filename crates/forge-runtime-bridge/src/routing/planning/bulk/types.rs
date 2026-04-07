use std::sync::Arc;

use crate::error::{BridgeErrorContext, BridgeReplayError, BridgeReplayErrorKind};
use crate::facade::BridgeRouteRequest;
use crate::identity::{
    BridgeIdentity, BulkAdmissionProfileIdentityTag, BulkPlanningIdentityTag,
    ContinuityPacketIdentityTag, FallbackPacketIdentityTag, ReducedContinuityIdentityTag,
    ReducedPublicationIdentityTag, ReducedTruthViewIdentityTag, ReductionPacketIdentityTag,
    RoutingPacketIdentityTag, TruthViewPacketIdentityTag, WorkloadIdentityTag,
};
use crate::routing::canonicalization::digest_string;
use crate::routing::context::BridgeMappingContext;
use crate::routing::planning::BridgePlannedRoute;

pub type BridgeWorkloadIdentity = BridgeIdentity<WorkloadIdentityTag>;
pub type BridgeCanonicalPlanningIdentity = BridgeIdentity<BulkPlanningIdentityTag>;
pub type BridgeAdmissionProfileIdentity = BridgeIdentity<BulkAdmissionProfileIdentityTag>;
pub type ReducedPublicationIdentity = BridgeIdentity<ReducedPublicationIdentityTag>;
pub type ReducedContinuityIdentity = BridgeIdentity<ReducedContinuityIdentityTag>;
pub type ReducedTruthViewIdentity = BridgeIdentity<ReducedTruthViewIdentityTag>;
pub type ContinuityPacketIdentity = BridgeIdentity<ContinuityPacketIdentityTag>;
pub type FallbackPacketIdentity = BridgeIdentity<FallbackPacketIdentityTag>;
pub type RoutingPacketIdentity = BridgeIdentity<RoutingPacketIdentityTag>;
pub type TruthViewPacketIdentity = BridgeIdentity<TruthViewPacketIdentityTag>;
pub type ReductionPacketIdentity = BridgeIdentity<ReductionPacketIdentityTag>;

pub const BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1: &str =
    "forge-runtime-bridge.bulk-plan-record.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityRemapPacket {
    packet_identity: ContinuityPacketIdentity,
    workload_identity: BridgeWorkloadIdentity,
    originating_route_identity: Arc<str>,
    continuity_authority_digest: Arc<str>,
    branch_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    prior_slice_count: usize,
    packet_index: usize,
    digest: Arc<str>,
}

impl ContinuityRemapPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        originating_route_identity: Arc<str>,
        continuity_authority_digest: Arc<str>,
        branch_identity: Arc<str>,
        snapshot_identity: Arc<str>,
        prior_slice_count: usize,
        packet_index: usize,
    ) -> Self {
        let basis = format!(
            "continuity-remap-packet|workload={}|route={}|authority={}|branch={}|snapshot={}|prior-slice-count={}|packet-index={}",
            workload_identity.as_str(),
            originating_route_identity,
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            prior_slice_count,
            packet_index,
        );
        Self {
            packet_identity: ContinuityPacketIdentity::new(digest_string("continuity-packet", &basis)),
            workload_identity,
            originating_route_identity,
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            prior_slice_count,
            packet_index,
            digest: digest_string("continuity-remap-packet", &basis),
        }
    }

    pub fn packet_identity(&self) -> &ContinuityPacketIdentity { &self.packet_identity }
    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn originating_route_identity(&self) -> &str { self.originating_route_identity.as_ref() }
    pub fn continuity_authority_digest(&self) -> &str { self.continuity_authority_digest.as_ref() }
    pub fn branch_identity(&self) -> &str { self.branch_identity.as_ref() }
    pub fn snapshot_identity(&self) -> &str { self.snapshot_identity.as_ref() }
    pub fn prior_slice_count(&self) -> usize { self.prior_slice_count }
    pub fn packet_index(&self) -> usize { self.packet_index }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthViewMaterializationPacket {
    packet_identity: TruthViewPacketIdentity,
    workload_identity: BridgeWorkloadIdentity,
    source_branch: Arc<str>,
    source_commit: Arc<str>,
    source_snapshot: Arc<str>,
    planned_route_count: usize,
    snapshot_read_count: usize,
    packet_index: usize,
    digest: Arc<str>,
}

impl TruthViewMaterializationPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        source_branch: Arc<str>,
        source_commit: Arc<str>,
        source_snapshot: Arc<str>,
        planned_route_count: usize,
        snapshot_read_count: usize,
        packet_index: usize,
    ) -> Self {
        let basis = format!(
            "truth-view-materialization-packet|workload={}|branch={}|commit={}|snapshot={}|planned-route-count={}|snapshot-read-count={}|packet-index={}",
            workload_identity.as_str(),
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
            packet_index,
        );
        Self {
            packet_identity: TruthViewPacketIdentity::new(digest_string("truth-view-packet", &basis)),
            workload_identity,
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
            packet_index,
            digest: digest_string("truth-view-materialization-packet", &basis),
        }
    }

    pub fn packet_identity(&self) -> &TruthViewPacketIdentity { &self.packet_identity }
    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn source_branch(&self) -> &str { self.source_branch.as_ref() }
    pub fn source_commit(&self) -> &str { self.source_commit.as_ref() }
    pub fn source_snapshot(&self) -> &str { self.source_snapshot.as_ref() }
    pub fn planned_route_count(&self) -> usize { self.planned_route_count }
    pub fn snapshot_read_count(&self) -> usize { self.snapshot_read_count }
    pub fn packet_index(&self) -> usize { self.packet_index }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackAggregationPacket {
    packet_identity: FallbackPacketIdentity,
    workload_identity: BridgeWorkloadIdentity,
    originating_route_identity: Arc<str>,
    fallback_class: Arc<str>,
    bounded_scope_identity: Arc<str>,
    packet_index: usize,
    digest: Arc<str>,
}

impl FallbackAggregationPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        originating_route_identity: Arc<str>,
        fallback_class: Arc<str>,
        bounded_scope_identity: Arc<str>,
        packet_index: usize,
    ) -> Self {
        let basis = format!(
            "fallback-aggregation-packet|workload={}|route={}|fallback-class={}|bounded-scope={}|packet-index={}",
            workload_identity.as_str(),
            originating_route_identity,
            fallback_class,
            bounded_scope_identity,
            packet_index,
        );
        Self {
            packet_identity: FallbackPacketIdentity::new(digest_string("fallback-packet", &basis)),
            workload_identity,
            originating_route_identity,
            fallback_class,
            bounded_scope_identity,
            packet_index,
            digest: digest_string("fallback-aggregation-packet", &basis),
        }
    }

    pub fn packet_identity(&self) -> &FallbackPacketIdentity { &self.packet_identity }
    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn originating_route_identity(&self) -> &str { self.originating_route_identity.as_ref() }
    pub fn fallback_class(&self) -> &str { self.fallback_class.as_ref() }
    pub fn bounded_scope_identity(&self) -> &str { self.bounded_scope_identity.as_ref() }
    pub fn packet_index(&self) -> usize { self.packet_index }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningCounters {
    bulk_workload_count: usize,
    bulk_routed_item_count: usize,
    bulk_normalized_workload_width: usize,
    bulk_packet_count: usize,
    bulk_packet_entry_count: usize,
    bulk_reduction_input_count: usize,
    bulk_reduction_output_count: usize,
    bulk_fallback_count: usize,
    bulk_unsupported_path_count: usize,
    bulk_serial_required_count: usize,
    bulk_parallel_legal_count: usize,
    bulk_parallel_profitable_count: usize,
    bulk_parallel_preparation_admitted_count: usize,
    bulk_parallel_preparation_rejected_count: usize,
    bulk_parallel_fallback_to_serial_count: usize,
}

impl BridgeBulkPlanningCounters {
    pub(crate) fn new(
        bulk_routed_item_count: usize,
        bulk_normalized_workload_width: usize,
        bulk_packet_count: usize,
        bulk_packet_entry_count: usize,
        bulk_reduction_input_count: usize,
        bulk_reduction_output_count: usize,
        bulk_fallback_count: usize,
        bulk_unsupported_path_count: usize,
        legality_class: BridgeParallelLegalityClass,
        profitability_class: BridgeParallelProfitabilityClass,
        admission_class: BridgeParallelAdmissionClass,
    ) -> Self {
        let parallel_legal =
            matches!(legality_class, BridgeParallelLegalityClass::ParallelPreparationLegal);
        let parallel_profitable =
            matches!(profitability_class, BridgeParallelProfitabilityClass::Profitable);
        let parallel_admitted =
            matches!(admission_class, BridgeParallelAdmissionClass::ParallelPreparationAdmitted);
        let parallel_rejected =
            matches!(admission_class, BridgeParallelAdmissionClass::ParallelPreparationRejected);
        Self {
            bulk_workload_count: 1,
            bulk_routed_item_count,
            bulk_normalized_workload_width,
            bulk_packet_count,
            bulk_packet_entry_count,
            bulk_reduction_input_count,
            bulk_reduction_output_count,
            bulk_fallback_count,
            bulk_unsupported_path_count,
            bulk_serial_required_count: usize::from(!parallel_admitted),
            bulk_parallel_legal_count: usize::from(parallel_legal),
            bulk_parallel_profitable_count: usize::from(parallel_profitable),
            bulk_parallel_preparation_admitted_count: usize::from(parallel_admitted),
            bulk_parallel_preparation_rejected_count: usize::from(parallel_rejected),
            bulk_parallel_fallback_to_serial_count: usize::from(parallel_legal && !parallel_profitable),
        }
    }

    pub fn bulk_workload_count(&self) -> usize { self.bulk_workload_count }
    pub fn bulk_routed_item_count(&self) -> usize { self.bulk_routed_item_count }
    pub fn bulk_normalized_workload_width(&self) -> usize { self.bulk_normalized_workload_width }
    pub fn bulk_packet_count(&self) -> usize { self.bulk_packet_count }
    pub fn bulk_packet_entry_count(&self) -> usize { self.bulk_packet_entry_count }
    pub fn bulk_reduction_input_count(&self) -> usize { self.bulk_reduction_input_count }
    pub fn bulk_reduction_output_count(&self) -> usize { self.bulk_reduction_output_count }
    pub fn bulk_fallback_count(&self) -> usize { self.bulk_fallback_count }
    pub fn bulk_unsupported_path_count(&self) -> usize { self.bulk_unsupported_path_count }
    pub fn bulk_serial_required_count(&self) -> usize { self.bulk_serial_required_count }
    pub fn bulk_parallel_legal_count(&self) -> usize { self.bulk_parallel_legal_count }
    pub fn bulk_parallel_profitable_count(&self) -> usize { self.bulk_parallel_profitable_count }
    pub fn bulk_parallel_preparation_admitted_count(&self) -> usize {
        self.bulk_parallel_preparation_admitted_count
    }
    pub fn bulk_parallel_preparation_rejected_count(&self) -> usize {
        self.bulk_parallel_preparation_rejected_count
    }
    pub fn bulk_parallel_fallback_to_serial_count(&self) -> usize {
        self.bulk_parallel_fallback_to_serial_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLocalityFootprint {
    branch_scope_count: usize,
    snapshot_scope_count: usize,
    publication_scope_count: usize,
    digest: Arc<str>,
}

impl BridgeLocalityFootprint {
    pub(crate) fn new(
        branch_scope_count: usize,
        snapshot_scope_count: usize,
        publication_scope_count: usize,
    ) -> Self {
        let basis = format!(
            "bridge-locality-footprint|branch-scope-count={}|snapshot-scope-count={}|publication-scope-count={}",
            branch_scope_count, snapshot_scope_count, publication_scope_count
        );
        Self {
            branch_scope_count,
            snapshot_scope_count,
            publication_scope_count,
            digest: digest_string("bridge-locality-footprint", &basis),
        }
    }

    pub fn branch_scope_count(&self) -> usize {
        self.branch_scope_count
    }

    pub fn snapshot_scope_count(&self) -> usize {
        self.snapshot_scope_count
    }

    pub fn publication_scope_count(&self) -> usize {
        self.publication_scope_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthDeltaRoutingPacket {
    packet_identity: RoutingPacketIdentity,
    workload_identity: BridgeWorkloadIdentity,
    route_identity: Arc<str>,
    source_branch: Arc<str>,
    source_commit: Arc<str>,
    source_snapshot: Arc<str>,
    subscription_slice_identity: Arc<str>,
    invalidation_target_count: usize,
    packet_index: usize,
    digest: Arc<str>,
}

impl TruthDeltaRoutingPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_identity: Arc<str>,
        source_branch: Arc<str>,
        source_commit: Arc<str>,
        source_snapshot: Arc<str>,
        subscription_slice_identity: Arc<str>,
        invalidation_target_count: usize,
        packet_index: usize,
    ) -> Self {
        let basis = format!(
            "truth-delta-routing-packet|workload={}|route={}|branch={}|commit={}|snapshot={}|subscription-slice={}|invalidation-target-count={}|packet-index={}",
            workload_identity.as_str(),
            route_identity,
            source_branch,
            source_commit,
            source_snapshot,
            subscription_slice_identity,
            invalidation_target_count,
            packet_index,
        );
        let packet_identity = RoutingPacketIdentity::new(digest_string("routing-packet", &basis));
        Self {
            packet_identity,
            workload_identity,
            route_identity,
            source_branch,
            source_commit,
            source_snapshot,
            subscription_slice_identity,
            invalidation_target_count,
            packet_index,
            digest: digest_string("truth-delta-routing-packet", &basis),
        }
    }

    pub fn packet_identity(&self) -> &RoutingPacketIdentity {
        &self.packet_identity
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn route_identity(&self) -> &str {
        self.route_identity.as_ref()
    }

    pub fn source_commit(&self) -> &str {
        self.source_commit.as_ref()
    }

    pub fn source_branch(&self) -> &str {
        self.source_branch.as_ref()
    }

    pub fn source_snapshot(&self) -> &str {
        self.source_snapshot.as_ref()
    }

    pub fn subscription_slice_identity(&self) -> &str {
        self.subscription_slice_identity.as_ref()
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn packet_index(&self) -> usize {
        self.packet_index
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationReductionPacket {
    packet_identity: ReductionPacketIdentity,
    workload_identity: BridgeWorkloadIdentity,
    reduction_family: Arc<str>,
    reduced_target_scope: Arc<str>,
    reduced_target_identity: ReducedPublicationIdentity,
    packet_index: usize,
    digest: Arc<str>,
}

impl InvalidationReductionPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        reduction_family: Arc<str>,
        reduced_target_scope: Arc<str>,
        reduced_target_identity: ReducedPublicationIdentity,
        packet_index: usize,
    ) -> Self {
        let basis = format!(
            "invalidation-reduction-packet|workload={}|reduction-family={}|reduced-target-scope={}|reduced-target-identity={}|packet-index={}",
            workload_identity.as_str(),
            reduction_family,
            reduced_target_scope,
            reduced_target_identity.as_str(),
            packet_index,
        );
        let packet_identity =
            ReductionPacketIdentity::new(digest_string("reduction-packet", &basis));
        Self {
            packet_identity,
            workload_identity,
            reduction_family,
            reduced_target_scope,
            reduced_target_identity,
            packet_index,
            digest: digest_string("invalidation-reduction-packet", &basis),
        }
    }

    pub fn packet_identity(&self) -> &ReductionPacketIdentity {
        &self.packet_identity
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn reduction_family(&self) -> &str {
        self.reduction_family.as_ref()
    }

    pub fn reduced_target_scope(&self) -> &str {
        self.reduced_target_scope.as_ref()
    }

    pub fn reduced_target_identity(&self) -> &ReducedPublicationIdentity {
        &self.reduced_target_identity
    }

    pub fn packet_index(&self) -> usize {
        self.packet_index
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedBridgePacketSet {
    workload_identity: BridgeWorkloadIdentity,
    routing_packets: Arc<[TruthDeltaRoutingPacket]>,
    truth_view_packets: Arc<[TruthViewMaterializationPacket]>,
    continuity_packets: Arc<[ContinuityRemapPacket]>,
    fallback_packets: Arc<[FallbackAggregationPacket]>,
    reduction_packets: Arc<[InvalidationReductionPacket]>,
    counters: BridgeBulkPlanningCounters,
    digest: Arc<str>,
}

impl PlannedBridgePacketSet {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        routing_packets: Vec<TruthDeltaRoutingPacket>,
        truth_view_packets: Vec<TruthViewMaterializationPacket>,
        continuity_packets: Vec<ContinuityRemapPacket>,
        fallback_packets: Vec<FallbackAggregationPacket>,
        reduction_packets: Vec<InvalidationReductionPacket>,
        counters: BridgeBulkPlanningCounters,
    ) -> Self {
        let mut basis = format!(
            "planned-bridge-packet-set|workload={}|routing-count={}|truth-view-count={}|continuity-count={}|fallback-count={}|reduction-count={}",
            workload_identity.as_str(),
            routing_packets.len(),
            truth_view_packets.len(),
            continuity_packets.len(),
            fallback_packets.len(),
            reduction_packets.len(),
        );
        for packet in &routing_packets {
            basis.push_str("|routing=");
            basis.push_str(packet.digest());
        }
        for packet in &truth_view_packets {
            basis.push_str("|truth-view=");
            basis.push_str(packet.digest());
        }
        for packet in &continuity_packets {
            basis.push_str("|continuity=");
            basis.push_str(packet.digest());
        }
        for packet in &fallback_packets {
            basis.push_str("|fallback=");
            basis.push_str(packet.digest());
        }
        for packet in &reduction_packets {
            basis.push_str("|reduction=");
            basis.push_str(packet.digest());
        }
        Self {
            workload_identity,
            routing_packets: routing_packets.into(),
            truth_view_packets: truth_view_packets.into(),
            continuity_packets: continuity_packets.into(),
            fallback_packets: fallback_packets.into(),
            reduction_packets: reduction_packets.into(),
            counters,
            digest: digest_string("planned-bridge-packet-set", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn routing_packets(&self) -> &[TruthDeltaRoutingPacket] {
        &self.routing_packets
    }

    pub fn truth_view_packets(&self) -> &[TruthViewMaterializationPacket] {
        &self.truth_view_packets
    }

    pub fn continuity_packets(&self) -> &[ContinuityRemapPacket] {
        &self.continuity_packets
    }

    pub fn fallback_packets(&self) -> &[FallbackAggregationPacket] {
        &self.fallback_packets
    }

    pub fn reduction_packets(&self) -> &[InvalidationReductionPacket] {
        &self.reduction_packets
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedContinuityRemap {
    continuity_identity: ReducedContinuityIdentity,
    continuity_authority_digest: Arc<str>,
    branch_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    reduced_route_count: usize,
    prior_slice_count: usize,
    digest: Arc<str>,
}

impl ReducedContinuityRemap {
    pub(crate) fn new(
        continuity_identity: ReducedContinuityIdentity,
        continuity_authority_digest: Arc<str>,
        branch_identity: Arc<str>,
        snapshot_identity: Arc<str>,
        reduced_route_count: usize,
        prior_slice_count: usize,
    ) -> Self {
        let basis = format!(
            "reduced-continuity-remap|identity={}|authority={}|branch={}|snapshot={}|reduced-route-count={}|prior-slice-count={}",
            continuity_identity.as_str(),
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            reduced_route_count,
            prior_slice_count,
        );
        Self {
            continuity_identity,
            continuity_authority_digest,
            branch_identity,
            snapshot_identity,
            reduced_route_count,
            prior_slice_count,
            digest: digest_string("reduced-continuity-remap", &basis),
        }
    }

    pub fn continuity_identity(&self) -> &ReducedContinuityIdentity { &self.continuity_identity }
    pub fn continuity_authority_digest(&self) -> &str { self.continuity_authority_digest.as_ref() }
    pub fn branch_identity(&self) -> &str { self.branch_identity.as_ref() }
    pub fn snapshot_identity(&self) -> &str { self.snapshot_identity.as_ref() }
    pub fn reduced_route_count(&self) -> usize { self.reduced_route_count }
    pub fn prior_slice_count(&self) -> usize { self.prior_slice_count }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedTruthViewMaterialization {
    truth_view_identity: ReducedTruthViewIdentity,
    source_branch: Arc<str>,
    source_commit: Arc<str>,
    source_snapshot: Arc<str>,
    planned_route_count: usize,
    snapshot_read_count: usize,
    digest: Arc<str>,
}

impl ReducedTruthViewMaterialization {
    pub(crate) fn new(
        truth_view_identity: ReducedTruthViewIdentity,
        source_branch: Arc<str>,
        source_commit: Arc<str>,
        source_snapshot: Arc<str>,
        planned_route_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        let basis = format!(
            "reduced-truth-view-materialization|identity={}|branch={}|commit={}|snapshot={}|planned-route-count={}|snapshot-read-count={}",
            truth_view_identity.as_str(),
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
        );
        Self {
            truth_view_identity,
            source_branch,
            source_commit,
            source_snapshot,
            planned_route_count,
            snapshot_read_count,
            digest: digest_string("reduced-truth-view-materialization", &basis),
        }
    }

    pub fn truth_view_identity(&self) -> &ReducedTruthViewIdentity { &self.truth_view_identity }
    pub fn source_branch(&self) -> &str { self.source_branch.as_ref() }
    pub fn source_commit(&self) -> &str { self.source_commit.as_ref() }
    pub fn source_snapshot(&self) -> &str { self.source_snapshot.as_ref() }
    pub fn planned_route_count(&self) -> usize { self.planned_route_count }
    pub fn snapshot_read_count(&self) -> usize { self.snapshot_read_count }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedBridgePublication {
    publication_identity: ReducedPublicationIdentity,
    subscription_slice_identity: Arc<str>,
    reduced_route_identities: Arc<[Arc<str>]>,
    invalidation_target_count: usize,
    digest: Arc<str>,
}

impl ReducedBridgePublication {
    pub(crate) fn new(
        publication_identity: ReducedPublicationIdentity,
        subscription_slice_identity: Arc<str>,
        reduced_route_identities: Vec<Arc<str>>,
        invalidation_target_count: usize,
    ) -> Self {
        let mut basis = format!(
            "reduced-bridge-publication|publication={}|subscription-slice={}|route-count={}|invalidation-target-count={}",
            publication_identity.as_str(),
            subscription_slice_identity,
            reduced_route_identities.len(),
            invalidation_target_count,
        );
        for route_identity in &reduced_route_identities {
            basis.push_str("|route=");
            basis.push_str(route_identity);
        }
        Self {
            publication_identity,
            subscription_slice_identity,
            reduced_route_identities: reduced_route_identities.into(),
            invalidation_target_count,
            digest: digest_string("reduced-bridge-publication", &basis),
        }
    }

    pub fn publication_identity(&self) -> &ReducedPublicationIdentity {
        &self.publication_identity
    }

    pub fn subscription_slice_identity(&self) -> &str {
        self.subscription_slice_identity.as_ref()
    }

    pub fn reduced_route_identities(&self) -> &[Arc<str>] {
        &self.reduced_route_identities
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedBridgeWorkloadArtifact {
    workload_identity: BridgeWorkloadIdentity,
    reduced_continuity_remaps: Arc<[ReducedContinuityRemap]>,
    reduced_truth_views: Arc<[ReducedTruthViewMaterialization]>,
    reduced_publications: Arc<[ReducedBridgePublication]>,
    counters: BridgeBulkPlanningCounters,
    digest: Arc<str>,
}

impl ReducedBridgeWorkloadArtifact {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        reduced_continuity_remaps: Vec<ReducedContinuityRemap>,
        reduced_truth_views: Vec<ReducedTruthViewMaterialization>,
        reduced_publications: Vec<ReducedBridgePublication>,
        counters: BridgeBulkPlanningCounters,
    ) -> Self {
        let reduction_output_count =
            reduced_continuity_remaps.len() + reduced_truth_views.len() + reduced_publications.len();
        let mut basis = format!(
            "reduced-bridge-workload-artifact|workload={}|reduction-input-count={}|reduction-output-count={}",
            workload_identity.as_str(),
            counters.bulk_reduction_input_count(),
            reduction_output_count,
        );
        for continuity in &reduced_continuity_remaps {
            basis.push_str("|continuity=");
            basis.push_str(continuity.digest());
        }
        for truth_view in &reduced_truth_views {
            basis.push_str("|truth-view=");
            basis.push_str(truth_view.digest());
        }
        for publication in &reduced_publications {
            basis.push_str("|publication=");
            basis.push_str(publication.digest());
        }
        Self {
            workload_identity,
            reduced_continuity_remaps: reduced_continuity_remaps.into(),
            reduced_truth_views: reduced_truth_views.into(),
            reduced_publications: reduced_publications.into(),
            counters,
            digest: digest_string("reduced-bridge-workload-artifact", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn reduced_continuity_remaps(&self) -> &[ReducedContinuityRemap] {
        &self.reduced_continuity_remaps
    }

    pub fn reduced_truth_views(&self) -> &[ReducedTruthViewMaterialization] {
        &self.reduced_truth_views
    }

    pub fn reduced_publications(&self) -> &[ReducedBridgePublication] {
        &self.reduced_publications
    }

    pub fn reduction_input_count(&self) -> usize {
        self.counters.bulk_reduction_input_count()
    }

    pub fn reduction_output_count(&self) -> usize {
        self.counters.bulk_reduction_output_count()
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelLegalityClass {
    SerialOnly,
    ParallelPreparationLegal,
    ParallelPreparationIllegal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelLegalityReason {
    BelowMinWorkloadWidth,
    SharedTruthViewMaterializationTarget,
    ContinuityRemapRequiresSerialPreparation,
    DisjointPacketRegionsCertified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelLegalityDecision {
    class: BridgeParallelLegalityClass,
    reason: BridgeParallelLegalityReason,
    digest: Arc<str>,
}

impl BridgeParallelLegalityDecision {
    pub(crate) fn new(class: BridgeParallelLegalityClass, reason: BridgeParallelLegalityReason) -> Self {
        let basis = format!(
            "bridge-parallel-legality-decision|class={}|reason={}",
            super::planner::parallel_legality_class_label(class),
            super::planner::parallel_legality_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-legality-decision", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelLegalityClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelLegalityReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelProfitabilityClass {
    NotApplicable,
    Profitable,
    Unprofitable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelProfitabilityReason {
    SerialOnlyWorkload,
    SharedPublicationReductionTarget,
    AdmittedOperational,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelProfitabilityDecision {
    class: BridgeParallelProfitabilityClass,
    reason: BridgeParallelProfitabilityReason,
    digest: Arc<str>,
}

impl BridgeParallelProfitabilityDecision {
    pub(crate) fn new(class: BridgeParallelProfitabilityClass, reason: BridgeParallelProfitabilityReason) -> Self {
        let basis = format!(
            "bridge-parallel-profitability-decision|class={}|reason={}",
            super::planner::parallel_profitability_class_label(class),
            super::planner::parallel_profitability_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-profitability-decision", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelProfitabilityClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelProfitabilityReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelAdmissionClass {
    SerialRequired,
    ParallelPreparationAdmitted,
    ParallelPreparationRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeParallelAdmissionReason {
    SerialExecutor,
    BelowMinWorkloadWidth,
    SharedPublicationReductionTarget,
    SharedTruthViewMaterializationTarget,
    ContinuityRemapRequiresSerialPreparation,
    AdmittedOperational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgePreparationMode {
    Serial,
    ParallelPreparation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisjointPacketRegionSet {
    regions: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl DisjointPacketRegionSet {
    pub(crate) fn new(regions: Vec<Arc<str>>) -> Self {
        let mut basis = format!("disjoint-packet-region-set|region-count={}", regions.len());
        for region in &regions {
            basis.push_str("|region=");
            basis.push_str(region);
        }
        Self {
            regions: regions.into(),
            digest: digest_string("disjoint-packet-region-set", &basis),
        }
    }

    pub fn regions(&self) -> &[Arc<str>] {
        &self.regions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPreparationPartitionSet {
    partitions: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl AdmittedPreparationPartitionSet {
    pub(crate) fn new(partitions: Vec<Arc<str>>) -> Self {
        let mut basis = format!(
            "admitted-preparation-partition-set|partition-count={}",
            partitions.len()
        );
        for partition in &partitions {
            basis.push_str("|partition=");
            basis.push_str(partition);
        }
        Self {
            partitions: partitions.into(),
            digest: digest_string("admitted-preparation-partition-set", &basis),
        }
    }

    pub fn partitions(&self) -> &[Arc<str>] {
        &self.partitions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelPreparationLegalityProof {
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    disjoint_packet_regions: DisjointPacketRegionSet,
    admitted_partitions: AdmittedPreparationPartitionSet,
    digest: Arc<str>,
}

impl ParallelPreparationLegalityProof {
    pub(crate) fn new(
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        disjoint_packet_regions: DisjointPacketRegionSet,
        admitted_partitions: AdmittedPreparationPartitionSet,
    ) -> Self {
        let basis = format!(
            "parallel-preparation-legality-proof|planning={}|regions={}|partitions={}",
            canonical_planning_identity.as_str(),
            disjoint_packet_regions.digest(),
            admitted_partitions.digest(),
        );
        Self {
            canonical_planning_identity,
            disjoint_packet_regions,
            admitted_partitions,
            digest: digest_string("parallel-preparation-legality-proof", &basis),
        }
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn disjoint_packet_regions(&self) -> &DisjointPacketRegionSet {
        &self.disjoint_packet_regions
    }

    pub fn admitted_partitions(&self) -> &AdmittedPreparationPartitionSet {
        &self.admitted_partitions
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeParallelAdmission {
    class: BridgeParallelAdmissionClass,
    reason: BridgeParallelAdmissionReason,
    digest: Arc<str>,
}

impl BridgeParallelAdmission {
    pub(crate) fn new(class: BridgeParallelAdmissionClass, reason: BridgeParallelAdmissionReason) -> Self {
        let basis = format!(
            "bridge-parallel-admission|class={}|reason={}",
            super::planner::parallel_admission_class_label(class),
            super::planner::parallel_admission_reason_label(reason),
        );
        Self {
            class,
            reason,
            digest: digest_string("bridge-parallel-admission", &basis),
        }
    }

    pub fn class(&self) -> BridgeParallelAdmissionClass {
        self.class
    }

    pub fn reason(&self) -> BridgeParallelAdmissionReason {
        self.reason
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBulkDecisionRecordKind {
    ParallelLegality,
    ParallelProfitability,
    ParallelAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkDecisionRecord {
    kind: BridgeBulkDecisionRecordKind,
    class_label: Arc<str>,
    reason_label: Arc<str>,
    digest: Arc<str>,
}

impl BridgeBulkDecisionRecord {
    pub(crate) fn new(
        kind: BridgeBulkDecisionRecordKind,
        class_label: Arc<str>,
        reason_label: Arc<str>,
    ) -> Self {
        let basis = format!(
            "bridge-bulk-decision-record|kind={}|class={}|reason={}",
            super::planner::bulk_decision_kind_label(kind),
            class_label,
            reason_label,
        );
        Self {
            kind,
            class_label,
            reason_label,
            digest: digest_string("bridge-bulk-decision-record", &basis),
        }
    }

    pub fn kind(&self) -> BridgeBulkDecisionRecordKind {
        self.kind
    }

    pub fn class_label(&self) -> &str {
        self.class_label.as_ref()
    }

    pub fn reason_label(&self) -> &str {
        self.reason_label.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkDecisionLog {
    records: Arc<[BridgeBulkDecisionRecord]>,
    digest: Arc<str>,
}

impl BridgeBulkDecisionLog {
    pub(crate) fn new(records: Vec<BridgeBulkDecisionRecord>) -> Self {
        let mut basis = format!("bridge-bulk-decision-log|record-count={}", records.len());
        for record in &records {
            basis.push_str("|record=");
            basis.push_str(record.digest());
        }
        Self {
            records: records.into(),
            digest: digest_string("bridge-bulk-decision-log", &basis),
        }
    }

    pub fn records(&self) -> &[BridgeBulkDecisionRecord] {
        &self.records
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBulkPlanningFailureKind {
    WorkloadSummaryConstructionFailure,
    UnsupportedPacketClass,
    InvalidLegalityBasis,
    LegalButUnprofitableParallelFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningFailure {
    kind: BridgeBulkPlanningFailureKind,
    boundary: Arc<str>,
    detail: Arc<str>,
    digest: Arc<str>,
}

impl BridgeBulkPlanningFailure {
    pub(crate) fn new(
        kind: BridgeBulkPlanningFailureKind,
        boundary: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let boundary = boundary.into();
        let detail = detail.into();
        let basis = format!(
            "bridge-bulk-planning-failure|kind={}|boundary={}|detail={}",
            super::planner::planning_failure_kind_label(kind),
            boundary,
            detail,
        );
        Self {
            kind,
            boundary,
            detail,
            digest: digest_string("bridge-bulk-planning-failure", &basis),
        }
    }

    pub fn kind(&self) -> BridgeBulkPlanningFailureKind {
        self.kind
    }

    pub fn boundary(&self) -> &str {
        self.boundary.as_ref()
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgeExecutionPlan {
    workload_identity: BridgeWorkloadIdentity,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    reduced_artifact: ReducedBridgeWorkloadArtifact,
    counters: BridgeBulkPlanningCounters,
    locality_footprint: BridgeLocalityFootprint,
    selected_mode: BridgePreparationMode,
    legality_decision: BridgeParallelLegalityDecision,
    profitability_decision: BridgeParallelProfitabilityDecision,
    parallel_admission: BridgeParallelAdmission,
    legality_proof: ParallelPreparationLegalityProof,
    decision_log: BridgeBulkDecisionLog,
    planning_failures: Arc<[BridgeBulkPlanningFailure]>,
    digest: Arc<str>,
}

impl AdmittedBridgeExecutionPlan {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        admission_profile_identity: BridgeAdmissionProfileIdentity,
        reduced_artifact: ReducedBridgeWorkloadArtifact,
        counters: BridgeBulkPlanningCounters,
        locality_footprint: BridgeLocalityFootprint,
        selected_mode: BridgePreparationMode,
        legality_decision: BridgeParallelLegalityDecision,
        profitability_decision: BridgeParallelProfitabilityDecision,
        parallel_admission: BridgeParallelAdmission,
        legality_proof: ParallelPreparationLegalityProof,
        decision_log: BridgeBulkDecisionLog,
        planning_failures: Vec<BridgeBulkPlanningFailure>,
    ) -> Self {
        let planning_failures: Arc<[BridgeBulkPlanningFailure]> = planning_failures.into();
        let failure_count = planning_failures.len();
        let basis = format!(
            "admitted-bridge-execution-plan|workload={}|planning={}|profile={}|reduced-artifact={}|packet-count={}|reduction-output-count={}|locality={}|mode={}|legality={}|profitability={}|parallel-admission={}|legality-proof={}|decision-log={}|failure-count={}",
            workload_identity.as_str(),
            canonical_planning_identity.as_str(),
            admission_profile_identity.as_str(),
            reduced_artifact.digest(),
            counters.bulk_packet_count(),
            counters.bulk_reduction_output_count(),
            locality_footprint.digest(),
            super::planner::preparation_mode_label(selected_mode),
            legality_decision.digest(),
            profitability_decision.digest(),
            parallel_admission.digest(),
            legality_proof.digest(),
            decision_log.digest(),
            failure_count,
        );
        Self {
            workload_identity,
            canonical_planning_identity,
            admission_profile_identity,
            reduced_artifact,
            counters,
            locality_footprint,
            selected_mode,
            legality_decision,
            profitability_decision,
            parallel_admission,
            legality_proof,
            decision_log,
            planning_failures,
            digest: digest_string("admitted-bridge-execution-plan", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn reduced_artifact(&self) -> &ReducedBridgeWorkloadArtifact {
        &self.reduced_artifact
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn locality_footprint(&self) -> &BridgeLocalityFootprint {
        &self.locality_footprint
    }

    pub fn selected_mode(&self) -> BridgePreparationMode {
        self.selected_mode
    }

    pub fn legality_decision(&self) -> &BridgeParallelLegalityDecision {
        &self.legality_decision
    }

    pub fn profitability_decision(&self) -> &BridgeParallelProfitabilityDecision {
        &self.profitability_decision
    }

    pub fn parallel_admission(&self) -> &BridgeParallelAdmission {
        &self.parallel_admission
    }

    pub fn legality_proof(&self) -> &ParallelPreparationLegalityProof {
        &self.legality_proof
    }

    pub fn decision_log(&self) -> &BridgeBulkDecisionLog {
        &self.decision_log
    }

    pub fn planning_failures(&self) -> &[BridgeBulkPlanningFailure] {
        &self.planning_failures
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkWorkloadSegment {
    request: BridgeRouteRequest,
    mapping_context: BridgeMappingContext,
}

impl BridgeBulkWorkloadSegment {
    pub fn new(request: BridgeRouteRequest) -> Self {
        Self {
            request,
            mapping_context: BridgeMappingContext::default(),
        }
    }

    pub fn with_mapping_context(mut self, mapping_context: BridgeMappingContext) -> Self {
        self.mapping_context = mapping_context;
        self
    }

    pub fn request(&self) -> &BridgeRouteRequest {
        &self.request
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        &self.mapping_context
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkWorkloadRequest {
    segments: Vec<BridgeBulkWorkloadSegment>,
}

impl BridgeBulkWorkloadRequest {
    pub fn new(segments: Vec<BridgeBulkWorkloadSegment>) -> Self {
        Self { segments }
    }

    pub fn from_requests(requests: Vec<BridgeRouteRequest>) -> Self {
        Self {
            segments: requests
                .into_iter()
                .map(BridgeBulkWorkloadSegment::new)
                .collect(),
        }
    }

    pub fn segments(&self) -> &[BridgeBulkWorkloadSegment] {
        &self.segments
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalBridgeWorkloadRequest {
    workload_identity: BridgeWorkloadIdentity,
    route_members: Arc<[Arc<str>]>,
    subscription_slice_members: Arc<[Arc<str>]>,
    continuity_members: Arc<[Arc<str>]>,
    truth_view_members: Arc<[Arc<str>]>,
    commit_members: Arc<[Arc<str>]>,
    snapshot_members: Arc<[Arc<str>]>,
    branch_members: Arc<[Arc<str>]>,
    workload_segment_digests: Arc<[Arc<str>]>,
    digest: Arc<str>,
}

impl CanonicalBridgeWorkloadRequest {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_members: Vec<Arc<str>>,
        subscription_slice_members: Vec<Arc<str>>,
        continuity_members: Vec<Arc<str>>,
        truth_view_members: Vec<Arc<str>>,
        commit_members: Vec<Arc<str>>,
        snapshot_members: Vec<Arc<str>>,
        branch_members: Vec<Arc<str>>,
        workload_segment_digests: Vec<Arc<str>>,
    ) -> Self {
        let mut basis = format!(
            "canonical-bridge-workload-request|workload={}|route-count={}|slice-count={}|continuity-count={}|truth-view-count={}|commit-count={}|snapshot-count={}|branch-count={}|segment-count={}",
            workload_identity.as_str(),
            route_members.len(),
            subscription_slice_members.len(),
            continuity_members.len(),
            truth_view_members.len(),
            commit_members.len(),
            snapshot_members.len(),
            branch_members.len(),
            workload_segment_digests.len(),
        );
        for member in &route_members {
            basis.push_str("|route=");
            basis.push_str(member);
        }
        for member in &subscription_slice_members {
            basis.push_str("|slice=");
            basis.push_str(member);
        }
        for member in &continuity_members {
            basis.push_str("|continuity=");
            basis.push_str(member);
        }
        for member in &truth_view_members {
            basis.push_str("|truth-view=");
            basis.push_str(member);
        }
        for member in &commit_members {
            basis.push_str("|commit=");
            basis.push_str(member);
        }
        for member in &snapshot_members {
            basis.push_str("|snapshot=");
            basis.push_str(member);
        }
        for member in &branch_members {
            basis.push_str("|branch=");
            basis.push_str(member);
        }
        for segment in &workload_segment_digests {
            basis.push_str("|segment=");
            basis.push_str(segment);
        }
        Self {
            workload_identity,
            route_members: route_members.into(),
            subscription_slice_members: subscription_slice_members.into(),
            continuity_members: continuity_members.into(),
            truth_view_members: truth_view_members.into(),
            commit_members: commit_members.into(),
            snapshot_members: snapshot_members.into(),
            branch_members: branch_members.into(),
            workload_segment_digests: workload_segment_digests.into(),
            digest: digest_string("canonical-bridge-workload-request", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn route_members(&self) -> &[Arc<str>] { &self.route_members }
    pub fn subscription_slice_members(&self) -> &[Arc<str>] { &self.subscription_slice_members }
    pub fn continuity_members(&self) -> &[Arc<str>] { &self.continuity_members }
    pub fn truth_view_members(&self) -> &[Arc<str>] { &self.truth_view_members }
    pub fn commit_members(&self) -> &[Arc<str>] { &self.commit_members }
    pub fn snapshot_members(&self) -> &[Arc<str>] { &self.snapshot_members }
    pub fn branch_members(&self) -> &[Arc<str>] { &self.branch_members }
    pub fn workload_segment_digests(&self) -> &[Arc<str>] { &self.workload_segment_digests }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedBridgeWorkloadSummary {
    workload_identity: BridgeWorkloadIdentity,
    route_count: usize,
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
    truth_view_member_count: usize,
    continuity_member_count: usize,
    branch_scope_count: usize,
    snapshot_scope_count: usize,
    counters: BridgeBulkPlanningCounters,
    digest: Arc<str>,
}

impl NormalizedBridgeWorkloadSummary {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_count: usize,
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
        truth_view_member_count: usize,
        continuity_member_count: usize,
        branch_scope_count: usize,
        snapshot_scope_count: usize,
        counters: BridgeBulkPlanningCounters,
    ) -> Self {
        let basis = format!(
            "normalized-bridge-workload-summary|workload={}|route-count={}|invalidation-target-count={}|subscription-slice-count={}|snapshot-read-count={}|truth-view-member-count={}|continuity-member-count={}|branch-scope-count={}|snapshot-scope-count={}|counter-digest={}{}{}{}{}{}",
            workload_identity.as_str(),
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            truth_view_member_count,
            continuity_member_count,
            branch_scope_count,
            snapshot_scope_count,
            counters.bulk_workload_count(),
            counters.bulk_routed_item_count(),
            counters.bulk_packet_count(),
            counters.bulk_packet_entry_count(),
            counters.bulk_reduction_input_count(),
            counters.bulk_reduction_output_count(),
        );
        Self {
            workload_identity,
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            truth_view_member_count,
            continuity_member_count,
            branch_scope_count,
            snapshot_scope_count,
            counters,
            digest: digest_string("normalized-bridge-workload-summary", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity { &self.workload_identity }
    pub fn route_count(&self) -> usize { self.route_count }
    pub fn invalidation_target_count(&self) -> usize { self.invalidation_target_count }
    pub fn subscription_slice_count(&self) -> usize { self.subscription_slice_count }
    pub fn snapshot_read_count(&self) -> usize { self.snapshot_read_count }
    pub fn truth_view_member_count(&self) -> usize { self.truth_view_member_count }
    pub fn continuity_member_count(&self) -> usize { self.continuity_member_count }
    pub fn branch_scope_count(&self) -> usize { self.branch_scope_count }
    pub fn snapshot_scope_count(&self) -> usize { self.snapshot_scope_count }
    pub fn counters(&self) -> &BridgeBulkPlanningCounters { &self.counters }
    pub fn digest(&self) -> &str { self.digest.as_ref() }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningSummary {
    workload_identity: BridgeWorkloadIdentity,
    route_count: usize,
    invalidation_target_count: usize,
    subscription_slice_count: usize,
    snapshot_read_count: usize,
    digest: Arc<str>,
}

impl BridgeBulkPlanningSummary {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        route_count: usize,
        invalidation_target_count: usize,
        subscription_slice_count: usize,
        snapshot_read_count: usize,
    ) -> Self {
        let basis = format!(
            "bulk-planning-summary|workload={}|route-count={}|invalidation-target-count={}|subscription-slice-count={}|snapshot-read-count={}",
            workload_identity.as_str(),
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
        );
        Self {
            workload_identity,
            route_count,
            invalidation_target_count,
            subscription_slice_count,
            snapshot_read_count,
            digest: digest_string("bulk-planning-summary", &basis),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn route_count(&self) -> usize {
        self.route_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn subscription_slice_count(&self) -> usize {
        self.subscription_slice_count
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkWorkloadPlan {
    request: BridgeBulkWorkloadRequest,
    workload_identity: BridgeWorkloadIdentity,
    canonical_request: CanonicalBridgeWorkloadRequest,
    normalized_summary: NormalizedBridgeWorkloadSummary,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    packet_set: PlannedBridgePacketSet,
    execution_plan: AdmittedBridgeExecutionPlan,
    planned_routes: Vec<BridgePlannedRoute>,
    summary: BridgeBulkPlanningSummary,
}

impl BridgeBulkWorkloadPlan {
    pub(crate) fn new(
        request: BridgeBulkWorkloadRequest,
        workload_identity: BridgeWorkloadIdentity,
        canonical_request: CanonicalBridgeWorkloadRequest,
        normalized_summary: NormalizedBridgeWorkloadSummary,
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        admission_profile_identity: BridgeAdmissionProfileIdentity,
        packet_set: PlannedBridgePacketSet,
        execution_plan: AdmittedBridgeExecutionPlan,
        planned_routes: Vec<BridgePlannedRoute>,
        summary: BridgeBulkPlanningSummary,
    ) -> Self {
        Self {
            request,
            workload_identity,
            canonical_request,
            normalized_summary,
            canonical_planning_identity,
            admission_profile_identity,
            packet_set,
            execution_plan,
            planned_routes,
            summary,
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn request(&self) -> &BridgeBulkWorkloadRequest {
        &self.request
    }

    pub fn canonical_request(&self) -> &CanonicalBridgeWorkloadRequest {
        &self.canonical_request
    }

    pub fn normalized_summary(&self) -> &NormalizedBridgeWorkloadSummary {
        &self.normalized_summary
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn execution_plan(&self) -> &AdmittedBridgeExecutionPlan {
        &self.execution_plan
    }

    pub fn packet_set(&self) -> &PlannedBridgePacketSet {
        &self.packet_set
    }

    pub fn planned_routes(&self) -> &[BridgePlannedRoute] {
        &self.planned_routes
    }

    pub fn summary(&self) -> &BridgeBulkPlanningSummary {
        &self.summary
    }

    #[cfg(test)]
    pub(crate) fn with_selected_mode_for_test(
        mut self,
        selected_mode: BridgePreparationMode,
    ) -> Self {
        self.execution_plan.selected_mode = selected_mode;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalBulkPlanRecord {
    schema_version: Arc<str>,
    request: BridgeBulkWorkloadRequest,
    workload_identity: BridgeWorkloadIdentity,
    canonical_request_digest: Arc<str>,
    normalized_summary_digest: Arc<str>,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    packet_set_digest: Arc<str>,
    execution_plan_digest: Arc<str>,
    reduced_artifact_digest: Arc<str>,
    selected_mode: BridgePreparationMode,
    decision_log: BridgeBulkDecisionLog,
    counters: BridgeBulkPlanningCounters,
    planning_failures: Arc<[BridgeBulkPlanningFailure]>,
}

impl BridgeCanonicalBulkPlanRecord {
    pub(crate) fn from_bulk_workload_plan(plan: &BridgeBulkWorkloadPlan) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1),
            request: plan.request().clone(),
            workload_identity: plan.workload_identity().clone(),
            canonical_request_digest: Arc::from(plan.canonical_request().digest().to_owned()),
            normalized_summary_digest: Arc::from(plan.normalized_summary().digest().to_owned()),
            canonical_planning_identity: plan.canonical_planning_identity().clone(),
            admission_profile_identity: plan.admission_profile_identity().clone(),
            packet_set_digest: Arc::from(plan.packet_set().digest().to_owned()),
            execution_plan_digest: Arc::from(plan.execution_plan().digest().to_owned()),
            reduced_artifact_digest: Arc::from(
                plan.execution_plan().reduced_artifact().digest().to_owned(),
            ),
            selected_mode: plan.execution_plan().selected_mode(),
            decision_log: plan.execution_plan().decision_log().clone(),
            counters: plan.execution_plan().counters().clone(),
            planning_failures: Arc::from(plan.execution_plan().planning_failures().to_vec()),
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn request(&self) -> &BridgeBulkWorkloadRequest {
        &self.request
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn canonical_request_digest(&self) -> &str {
        self.canonical_request_digest.as_ref()
    }

    pub fn normalized_summary_digest(&self) -> &str {
        self.normalized_summary_digest.as_ref()
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn packet_set_digest(&self) -> &str {
        self.packet_set_digest.as_ref()
    }

    pub fn execution_plan_digest(&self) -> &str {
        self.execution_plan_digest.as_ref()
    }

    pub fn reduced_artifact_digest(&self) -> &str {
        self.reduced_artifact_digest.as_ref()
    }

    pub fn selected_mode(&self) -> BridgePreparationMode {
        self.selected_mode
    }

    pub fn decision_log(&self) -> &BridgeBulkDecisionLog {
        &self.decision_log
    }

    pub fn decision_log_digest(&self) -> &str {
        self.decision_log.digest()
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn planning_failures(&self) -> &[BridgeBulkPlanningFailure] {
        &self.planning_failures
    }

    pub fn planning_failure_count(&self) -> usize {
        self.planning_failures.len()
    }

    #[cfg(test)]
    pub(crate) fn with_schema_version_for_test(
        mut self,
        schema_version: impl Into<Arc<str>>,
    ) -> Self {
        self.schema_version = schema_version.into();
        self
    }

    pub(crate) fn decode(&self) -> Result<Self, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical bulk plan record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.clone())
    }
}


