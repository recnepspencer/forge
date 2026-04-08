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
    reduced_target_identity: ReducedRoutingTargetIdentity,
    packet_index: usize,
    digest: Arc<str>,
}

impl InvalidationReductionPacket {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        reduction_family: Arc<str>,
        reduced_target_scope: Arc<str>,
        reduced_target_identity: ReducedRoutingTargetIdentity,
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

    pub fn reduced_target_identity(&self) -> &ReducedRoutingTargetIdentity {
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

    pub(crate) fn with_counters(&self, counters: BridgeBulkPlanningCounters) -> Self {
        Self {
            workload_identity: self.workload_identity.clone(),
            routing_packets: Arc::clone(&self.routing_packets),
            truth_view_packets: Arc::clone(&self.truth_view_packets),
            continuity_packets: Arc::clone(&self.continuity_packets),
            fallback_packets: Arc::clone(&self.fallback_packets),
            reduction_packets: Arc::clone(&self.reduction_packets),
            counters,
            digest: Arc::clone(&self.digest),
        }
    }
}

use super::*;
