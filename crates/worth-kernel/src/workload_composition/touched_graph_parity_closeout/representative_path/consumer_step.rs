use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;
use topology::facade::{TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerFamilyRow};
use worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

use crate::workload_composition::planner_owned_routing::{
    CompiledProductReusePlannerRoutePacket, ReplayUndoPlannerRoutePacket,
    WorthTouchedGraphConflictDerivedDiagnosticProjection,
    WorthTouchedGraphConflictPublicProofInspection, WorthTouchedGraphConflictSelectedRoutePacket,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepresentativeSelectedRouteConsumerKind {
    QueryBackedRead,
    EvidenceLookup,
    ReplayOrConflict,
    CompiledProductReuse,
    PublicProof,
    Diagnostic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteAuthority {
    packet: WorthTouchedGraphConflictSelectedRoutePacket,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteQueryBackedReadStep {
    cutover: TopologyQueryBackedConsumerCutover,
    selected_family_row: TopologyQueryBackedConsumerFamilyRow,
    evidence_lookup_query_boundary_support_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteEvidenceLookupStep {
    packet: EvidenceLookupRoutePacket,
    public_closeout_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteReplayConsumerStep {
    packet: ReplayUndoPlannerRoutePacket,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteReuseConsumerStep {
    packet: CompiledProductReusePlannerRoutePacket,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRoutePublicProofStep {
    inspection: WorthTouchedGraphConflictPublicProofInspection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepresentativeSelectedRouteDiagnosticStep {
    projection: WorthTouchedGraphConflictDerivedDiagnosticProjection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepresentativeSelectedRouteConsumerStep {
    QueryBackedRead(RepresentativeSelectedRouteQueryBackedReadStep),
    EvidenceLookup(RepresentativeSelectedRouteEvidenceLookupStep),
    ReplayOrConflict(RepresentativeSelectedRouteReplayConsumerStep),
    CompiledProductReuse(RepresentativeSelectedRouteReuseConsumerStep),
    PublicProof(RepresentativeSelectedRoutePublicProofStep),
    Diagnostic(RepresentativeSelectedRouteDiagnosticStep),
}

impl RepresentativeSelectedRouteAuthority {
    pub(crate) fn new(packet: WorthTouchedGraphConflictSelectedRoutePacket) -> Self {
        Self { packet }
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        self.packet.selected_route_identity_digest()
    }

    pub fn selected_family_identity(&self) -> &str {
        self.packet.selected_family_identity()
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        self.packet.selected_product_identity_digest()
    }

    pub fn spatial_selected_family_identity(&self) -> &str {
        self.packet.spatial_selected_family_identity()
    }

    pub fn spatial_selected_product_identity_digest(&self) -> &str {
        self.packet.spatial_selected_product_identity_digest()
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.packet.selected_witness_identity_digest()
    }

    pub fn source_firewall_digest(&self) -> &str {
        self.packet.source_firewall_digest()
    }

    pub fn evidence_lookup_query_support_digest(&self) -> &str {
        self.packet.evidence_lookup_query_support_digest()
    }

    pub fn replay_undo_route_packet_identity(&self) -> &str {
        self.packet.replay_undo_route_packet_identity()
    }

    pub const fn replay_undo_route_family(&self) -> ReplayUndoPlannerRouteFamily {
        self.packet.replay_undo_route_family()
    }

    pub fn compiled_product_reuse_route_packet_identity(&self) -> &str {
        self.packet.compiled_product_reuse_route_packet_identity()
    }
}

impl RepresentativeSelectedRouteQueryBackedReadStep {
    pub(crate) fn new(
        cutover: TopologyQueryBackedConsumerCutover,
        selected_family_row: TopologyQueryBackedConsumerFamilyRow,
        evidence_lookup_query_boundary_support_digest: String,
    ) -> Self {
        Self {
            cutover,
            selected_family_row,
            evidence_lookup_query_boundary_support_digest,
        }
    }

    pub fn cutover(&self) -> &TopologyQueryBackedConsumerCutover {
        &self.cutover
    }

    pub fn selected_family_row(&self) -> &TopologyQueryBackedConsumerFamilyRow {
        &self.selected_family_row
    }

    pub fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
    }
}

impl RepresentativeSelectedRouteEvidenceLookupStep {
    pub(crate) fn new(packet: EvidenceLookupRoutePacket, public_closeout_digest: String) -> Self {
        Self {
            packet,
            public_closeout_digest,
        }
    }

    pub fn packet(&self) -> &EvidenceLookupRoutePacket {
        &self.packet
    }

    pub fn public_closeout_digest(&self) -> &str {
        &self.public_closeout_digest
    }
}

impl RepresentativeSelectedRouteReplayConsumerStep {
    pub(crate) fn new(packet: ReplayUndoPlannerRoutePacket) -> Self {
        Self { packet }
    }

    pub fn route_packet_identity(&self) -> &str {
        self.packet.route_packet_identity()
    }

    pub fn route_authority_digest(&self) -> &str {
        self.packet.route_authority_digest()
    }

    pub const fn family(&self) -> ReplayUndoPlannerRouteFamily {
        self.packet.family()
    }
}

impl RepresentativeSelectedRouteReuseConsumerStep {
    pub(crate) fn new(packet: CompiledProductReusePlannerRoutePacket) -> Self {
        Self { packet }
    }

    pub fn packet(&self) -> &CompiledProductReusePlannerRoutePacket {
        &self.packet
    }
}

impl RepresentativeSelectedRoutePublicProofStep {
    pub(crate) fn new(inspection: WorthTouchedGraphConflictPublicProofInspection) -> Self {
        Self { inspection }
    }

    pub fn inspection(&self) -> &WorthTouchedGraphConflictPublicProofInspection {
        &self.inspection
    }
}

impl RepresentativeSelectedRouteDiagnosticStep {
    pub(crate) fn new(projection: WorthTouchedGraphConflictDerivedDiagnosticProjection) -> Self {
        Self { projection }
    }

    pub fn projection(&self) -> &WorthTouchedGraphConflictDerivedDiagnosticProjection {
        &self.projection
    }
}

impl RepresentativeSelectedRouteConsumerStep {
    pub const fn kind(&self) -> RepresentativeSelectedRouteConsumerKind {
        match self {
            Self::QueryBackedRead(_) => RepresentativeSelectedRouteConsumerKind::QueryBackedRead,
            Self::EvidenceLookup(_) => RepresentativeSelectedRouteConsumerKind::EvidenceLookup,
            Self::ReplayOrConflict(_) => RepresentativeSelectedRouteConsumerKind::ReplayOrConflict,
            Self::CompiledProductReuse(_) => {
                RepresentativeSelectedRouteConsumerKind::CompiledProductReuse
            }
            Self::PublicProof(_) => RepresentativeSelectedRouteConsumerKind::PublicProof,
            Self::Diagnostic(_) => RepresentativeSelectedRouteConsumerKind::Diagnostic,
        }
    }
}
