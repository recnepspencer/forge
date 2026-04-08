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

use super::*;
