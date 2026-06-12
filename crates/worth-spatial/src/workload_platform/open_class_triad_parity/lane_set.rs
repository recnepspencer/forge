use topology::facade::{NmtTopologyConstructionCounters, NmtTopologyConstructionReceipt};

use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};
use super::open_class::OpenTopologyClass;
use crate::workload_platform::projection_fact_parity::{
    ProjectionFactParityLane, ProjectionFactParityReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassParityLaneSet {
    topology_class: OpenTopologyClass,
    topology_identity: String,
    parity: ProjectionFactParityReceipt,
    counters: NmtTopologyConstructionCounters,
    open_boundary_digest: String,
    radial_digest: String,
}

impl OpenClassParityLaneSet {
    pub fn from_topology_and_parity(
        topology: &NmtTopologyConstructionReceipt,
        parity: ProjectionFactParityReceipt,
    ) -> Result<Self, OpenClassTriadParityDenial> {
        let topology_class = OpenTopologyClass::from_topology(topology)?;
        require_complete_parity(topology_class, &parity)?;
        require_same_topology_authority(topology_class, topology, &parity)?;
        let set = Self {
            topology_class,
            topology_identity: topology.pattern_identity().identity_digest().to_string(),
            parity,
            counters: topology.counters(),
            open_boundary_digest: topology.open_boundary().boundary_digest().to_string(),
            radial_digest: topology.radial_adjacency().radial_digest().to_string(),
        };
        set.require_bounded_conversion_guard()?;
        Ok(set)
    }

    pub fn topology_class(&self) -> OpenTopologyClass {
        self.topology_class
    }

    pub fn topology_identity(&self) -> &str {
        &self.topology_identity
    }

    pub fn parity(&self) -> &ProjectionFactParityReceipt {
        &self.parity
    }

    pub fn counters(&self) -> NmtTopologyConstructionCounters {
        self.counters
    }

    pub fn lane_count(&self) -> usize {
        self.parity.counters().lanes_compared()
    }

    pub fn receipt_backed_lane_count(&self) -> usize {
        self.parity.counters().receipt_backed_lanes()
    }

    pub fn retained_lane_identity(&self) -> Option<&str> {
        self.parity
            .evidence_for_lane(ProjectionFactParityLane::Retained)
            .map(|lane| lane.source_receipt_identity())
    }

    pub fn projection_consumed_lane_identity(&self) -> Option<&str> {
        self.parity
            .evidence_for_lane(ProjectionFactParityLane::ProjectionConsumed)
            .map(|lane| lane.source_receipt_identity())
    }

    pub(crate) fn open_boundary_digest(&self) -> &str {
        &self.open_boundary_digest
    }

    pub(crate) fn radial_digest(&self) -> &str {
        &self.radial_digest
    }

    fn require_bounded_conversion_guard(&self) -> Result<(), OpenClassTriadParityDenial> {
        match self.topology_class {
            OpenTopologyClass::Wire if self.counters.face_count() == 0 => Ok(()),
            OpenTopologyClass::Sheet
                if self.counters.face_count() > 0
                    && self.counters.boundary_half_edge_count() > 0
                    && self.counters.non_manifold_edge_count() == 0 =>
            {
                Ok(())
            }
            OpenTopologyClass::NmtFan
                if self.counters.face_count() >= 3
                    && self.counters.boundary_half_edge_count() > 0
                    && self.counters.non_manifold_edge_count() > 0 =>
            {
                Ok(())
            }
            topology_class => Err(OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::BoundedConversionViolation,
                Some(topology_class),
                format!(
                    "{} failed bounded-conversion guards: faces={}, open boundary half-edges={}, non-manifold edges={}.",
                    topology_class.human_name(),
                    self.counters.face_count(),
                    self.counters.boundary_half_edge_count(),
                    self.counters.non_manifold_edge_count()
                ),
            )),
        }
    }
}

fn require_same_topology_authority(
    topology_class: OpenTopologyClass,
    topology: &NmtTopologyConstructionReceipt,
    parity: &ProjectionFactParityReceipt,
) -> Result<(), OpenClassTriadParityDenial> {
    let topology_evidence_identity = topology
        .topology_seed_receipt()
        .query_receipts()
        .declaration_receipt()
        .identity()
        .name();
    if parity.topology_evidence_identity() == topology_evidence_identity {
        return Ok(());
    }
    Err(OpenClassTriadParityDenial::new(
        OpenClassTriadParityDenialKind::TopologyParityMismatch,
        Some(topology_class),
        format!(
            "{} parity topology evidence does not match the topology construction receipt; parity used {}, construction requires {}.",
            topology_class.human_name(),
            parity.topology_evidence_identity(),
            topology_evidence_identity
        ),
    ))
}

fn require_complete_parity(
    topology_class: OpenTopologyClass,
    parity: &ProjectionFactParityReceipt,
) -> Result<(), OpenClassTriadParityDenial> {
    let counters = parity.counters();
    if counters.lanes_compared() == ProjectionFactParityLane::REQUIRED.len()
        && counters.receipt_backed_lanes() == ProjectionFactParityLane::REQUIRED.len()
    {
        return Ok(());
    }
    Err(OpenClassTriadParityDenial::new(
        OpenClassTriadParityDenialKind::ParityReceiptRejected,
        Some(topology_class),
        format!(
            "{} parity must compare nine receipt-backed lanes; got {} compared and {} receipt-backed.",
            topology_class.human_name(),
            counters.lanes_compared(),
            counters.receipt_backed_lanes()
        ),
    ))
}
