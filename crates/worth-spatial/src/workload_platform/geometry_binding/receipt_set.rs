use super::{
    BoundPlanarEdgeGeometry, BoundPlanarFaceGeometry, BoundPlanarLoopGeometry,
    GeometryCarrierIdentity, TopologyBindingTarget,
};
use crate::workload_platform::vocabulary::{
    GeometryBindingWorkloadReceipt, SpatialWorkloadStage, WorkloadStageIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryBindingWorkloadCounters {
    topology_targets: usize,
    geometry_carriers: usize,
    face_bindings: usize,
    edge_bindings: usize,
    loop_bindings: usize,
}

impl GeometryBindingWorkloadCounters {
    pub(crate) fn new(
        topology_targets: usize,
        geometry_carriers: usize,
        face_bindings: usize,
        edge_bindings: usize,
        loop_bindings: usize,
    ) -> Self {
        Self {
            topology_targets,
            geometry_carriers,
            face_bindings,
            edge_bindings,
            loop_bindings,
        }
    }

    pub fn topology_targets(self) -> usize {
        self.topology_targets
    }

    pub fn geometry_carriers(self) -> usize {
        self.geometry_carriers
    }

    pub fn face_bindings(self) -> usize {
        self.face_bindings
    }

    pub fn edge_bindings(self) -> usize {
        self.edge_bindings
    }

    pub fn loop_bindings(self) -> usize {
        self.loop_bindings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryBindingReceiptSet {
    stage_receipt: GeometryBindingWorkloadReceipt,
    topology_target: TopologyBindingTarget,
    carrier_receipts: Vec<GeometryCarrierIdentity>,
    counters: GeometryBindingWorkloadCounters,
}

impl GeometryBindingReceiptSet {
    pub(crate) fn new(
        stage_receipt: GeometryBindingWorkloadReceipt,
        topology_target: TopologyBindingTarget,
        faces: &[BoundPlanarFaceGeometry],
        edges: &[BoundPlanarEdgeGeometry],
        loops: &[BoundPlanarLoopGeometry],
    ) -> Self {
        let carrier_receipts = faces
            .iter()
            .map(|face| face.carrier_identity().clone())
            .chain(edges.iter().map(|edge| edge.carrier_identity().clone()))
            .chain(
                loops
                    .iter()
                    .map(|loop_geometry| loop_geometry.carrier_identity().clone()),
            )
            .collect::<Vec<_>>();
        let counters = GeometryBindingWorkloadCounters::new(
            topology_target.face_targets().len()
                + topology_target.edge_targets().len()
                + topology_target.loop_targets().len(),
            carrier_receipts.len(),
            faces.len(),
            edges.len(),
            loops.len(),
        );
        Self {
            stage_receipt,
            topology_target,
            carrier_receipts,
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &GeometryBindingWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn topology_target(&self) -> &TopologyBindingTarget {
        &self.topology_target
    }

    pub fn topology_identity(&self) -> &str {
        self.topology_target.topology_receipt_identity()
    }

    pub fn topology_query_surface(&self) -> &str {
        self.topology_target.topology_query_surface()
    }

    pub fn carrier_receipts(&self) -> &[GeometryCarrierIdentity] {
        &self.carrier_receipts
    }

    pub fn counters(&self) -> GeometryBindingWorkloadCounters {
        self.counters
    }

    pub fn has_binding_declaration_receipt(&self) -> bool {
        self.stage_identity().stage() == SpatialWorkloadStage::GeometryBinding
            && !self.stage_identity().declaration().trim().is_empty()
    }

    pub fn has_geometry_carrier_receipts(&self) -> bool {
        !self.carrier_receipts.is_empty()
            && self
                .carrier_receipts
                .iter()
                .all(GeometryCarrierIdentity::is_distinct_from_topology_identity)
    }
}
