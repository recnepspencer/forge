use super::{
    BoundPlanarEdgeGeometry, BoundPlanarFaceGeometry, BoundPlanarLoopGeometry,
    GeometryBindingReceiptSet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BoundGeometryWorkload {
    receipts: GeometryBindingReceiptSet,
    planar_faces: Vec<BoundPlanarFaceGeometry>,
    planar_edges: Vec<BoundPlanarEdgeGeometry>,
    planar_loops: Vec<BoundPlanarLoopGeometry>,
}

impl BoundGeometryWorkload {
    pub(crate) fn new(
        receipts: GeometryBindingReceiptSet,
        planar_faces: Vec<BoundPlanarFaceGeometry>,
        planar_edges: Vec<BoundPlanarEdgeGeometry>,
        planar_loops: Vec<BoundPlanarLoopGeometry>,
    ) -> Self {
        Self {
            receipts,
            planar_faces,
            planar_edges,
            planar_loops,
        }
    }

    pub fn receipts(&self) -> &GeometryBindingReceiptSet {
        &self.receipts
    }

    pub fn planar_faces(&self) -> &[BoundPlanarFaceGeometry] {
        &self.planar_faces
    }

    pub fn planar_edges(&self) -> &[BoundPlanarEdgeGeometry] {
        &self.planar_edges
    }

    pub fn planar_loops(&self) -> &[BoundPlanarLoopGeometry] {
        &self.planar_loops
    }

    pub fn can_enter_surface_support(&self) -> bool {
        self.receipts.has_binding_declaration_receipt()
            && self.receipts.has_geometry_carrier_receipts()
    }
}
