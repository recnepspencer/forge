use super::{
    BoundGeometryWorkload, GeometryBindingReceiptSet, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet, TopologyBindingTarget, UnsupportedGeometryBinding,
    UnsupportedGeometryBindingReasonCode, UnsupportedGeometryCarrierFamily,
};
use topology::facade::TopologySeedReceipt;

pub struct GeometryBindingWorkload {
    topology_target: TopologyBindingTarget,
    declaration: String,
    planar_faces: Option<PlanarFaceCarrierSet>,
    planar_edges: Option<PlanarEdgeCarrierSet>,
    planar_loops: Option<PlanarLoopCarrierSet>,
    unsupported_family: Option<UnsupportedGeometryCarrierFamily>,
}

impl GeometryBindingWorkload {
    pub fn for_topology_seed(seed: &TopologySeedReceipt) -> Self {
        Self {
            topology_target: TopologyBindingTarget::from_seed(seed),
            declaration: "geometry binding workload".to_string(),
            planar_faces: None,
            planar_edges: None,
            planar_loops: None,
            unsupported_family: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_planar_faces(mut self, faces: PlanarFaceCarrierSet) -> Self {
        self.planar_faces = Some(faces);
        self
    }

    pub fn with_planar_edges(mut self, edges: PlanarEdgeCarrierSet) -> Self {
        self.planar_edges = Some(edges);
        self
    }

    pub fn with_planar_loops(mut self, loops: PlanarLoopCarrierSet) -> Self {
        self.planar_loops = Some(loops);
        self
    }

    pub fn with_unsupported_family(mut self, family: UnsupportedGeometryCarrierFamily) -> Self {
        self.unsupported_family = Some(family);
        self
    }

    pub fn admit(self) -> Result<BoundGeometryWorkload, UnsupportedGeometryBinding> {
        if self.declaration.trim().is_empty() {
            return Err(self.deny(
                UnsupportedGeometryBindingReasonCode::MissingBindingDeclaration,
                "Geometry binding requires a human-readable declaration before admission.",
            ));
        }
        if let Some(family) = self.unsupported_family {
            return Err(self.deny_requested_family(
                UnsupportedGeometryBindingReasonCode::UnsupportedCarrierFamily,
                format!(
                    "{} is not admitted for geometry binding in this workload phase.",
                    family.human_label()
                ),
                family,
            ));
        }
        if let Some(denial) = self.validate_carrier_origins() {
            return Err(denial);
        }
        let GeometryBindingWorkload {
            topology_target,
            declaration,
            planar_faces,
            planar_edges,
            planar_loops,
            unsupported_family: _,
        } = self;

        let faces = planar_faces
            .map(PlanarFaceCarrierSet::into_faces)
            .unwrap_or_default();
        let edges = planar_edges
            .map(PlanarEdgeCarrierSet::into_edges)
            .unwrap_or_default();
        let loops = planar_loops
            .map(PlanarLoopCarrierSet::into_loops)
            .unwrap_or_default();

        if faces.is_empty() && edges.is_empty() && loops.is_empty() {
            return Err(deny_from_target(
                &topology_target,
                declaration.clone(),
                UnsupportedGeometryBindingReasonCode::MissingGeometryCarrier,
                "Geometry binding requires at least one admitted planar carrier.",
            ));
        }
        if let Some(denial) = validate_targets(
            &topology_target,
            declaration.clone(),
            &faces,
            &edges,
            &loops,
        ) {
            return Err(denial);
        }

        let stage_receipt =
            crate::workload_platform::vocabulary::GeometryBindingWorkload::for_topology_receipt(
                topology_target.topology_stage_receipt(),
            )
            .declared(declaration.clone())
            .admit()
            .map_err(|_| {
                deny_from_target(
                    &topology_target,
                    declaration.clone(),
                    UnsupportedGeometryBindingReasonCode::MissingBindingDeclaration,
                    "Geometry binding declaration could not produce a stage receipt.",
                )
            })?;
        let receipts =
            GeometryBindingReceiptSet::new(stage_receipt, topology_target, &faces, &edges, &loops);
        Ok(BoundGeometryWorkload::new(receipts, faces, edges, loops))
    }

    fn validate_carrier_origins(&self) -> Option<UnsupportedGeometryBinding> {
        if let Some(faces) = &self.planar_faces {
            if !self.carrier_origin_matches_target(faces.topology_receipt_identity()) {
                return Some(self.deny(
                    UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
                    "Planar face carriers must originate from the same topology receipt as the binding workload.",
                ));
            }
        }
        if let Some(edges) = &self.planar_edges {
            if !self.carrier_origin_matches_target(edges.topology_receipt_identity()) {
                return Some(self.deny(
                    UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
                    "Planar edge carriers must originate from the same topology receipt as the binding workload.",
                ));
            }
        }
        if let Some(loops) = &self.planar_loops {
            if !self.carrier_origin_matches_target(loops.topology_receipt_identity()) {
                return Some(self.deny(
                    UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
                    "Planar loop carriers must originate from the same topology receipt as the binding workload.",
                ));
            }
        }
        None
    }

    fn carrier_origin_matches_target(&self, carrier_topology_receipt_identity: &str) -> bool {
        carrier_topology_receipt_identity == self.topology_target.topology_receipt_identity()
    }

    fn deny(
        &self,
        reason_code: UnsupportedGeometryBindingReasonCode,
        human_reason: impl Into<String>,
    ) -> UnsupportedGeometryBinding {
        UnsupportedGeometryBinding::from_target(
            &self.topology_target,
            self.declaration.clone(),
            reason_code,
            human_reason,
        )
    }

    fn deny_requested_family(
        &self,
        reason_code: UnsupportedGeometryBindingReasonCode,
        human_reason: impl Into<String>,
        requested_family: UnsupportedGeometryCarrierFamily,
    ) -> UnsupportedGeometryBinding {
        UnsupportedGeometryBinding::from_target_with_requested_family(
            &self.topology_target,
            self.declaration.clone(),
            reason_code,
            human_reason,
            Some(requested_family),
        )
    }
}

fn contains_exactly<'a>(expected: &[String], actual: impl Iterator<Item = &'a str>) -> bool {
    let mut expected = expected.iter().map(String::as_str).collect::<Vec<_>>();
    let mut actual = actual.collect::<Vec<_>>();
    expected.sort_unstable();
    actual.sort_unstable();
    expected == actual
}

fn validate_targets(
    topology_target: &TopologyBindingTarget,
    declaration: String,
    faces: &[super::BoundPlanarFaceGeometry],
    edges: &[super::BoundPlanarEdgeGeometry],
    loops: &[super::BoundPlanarLoopGeometry],
) -> Option<UnsupportedGeometryBinding> {
    if !contains_exactly(
        topology_target.face_targets(),
        faces.iter().map(|face| face.topology_face_identity()),
    ) {
        return Some(deny_from_target(
            topology_target,
            declaration,
            UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
            "Planar face carriers must exactly match the topology seed face targets.",
        ));
    }
    if !contains_exactly(
        topology_target.edge_targets(),
        edges.iter().map(|edge| edge.topology_edge_identity()),
    ) {
        return Some(deny_from_target(
            topology_target,
            declaration,
            UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
            "Planar edge carriers must exactly match the topology seed edge targets.",
        ));
    }
    if !contains_exactly(
        topology_target.loop_targets(),
        loops
            .iter()
            .map(|loop_geometry| loop_geometry.topology_loop_identity()),
    ) {
        return Some(deny_from_target(
            topology_target,
            declaration,
            UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget,
            "Planar loop carriers must exactly match the topology seed loop targets.",
        ));
    }
    None
}

fn deny_from_target(
    topology_target: &TopologyBindingTarget,
    declaration: String,
    reason_code: UnsupportedGeometryBindingReasonCode,
    human_reason: impl Into<String>,
) -> UnsupportedGeometryBinding {
    UnsupportedGeometryBinding::from_target(topology_target, declaration, reason_code, human_reason)
}

#[cfg(test)]
mod tests {
    use super::contains_exactly;

    #[test]
    fn exact_target_matching_rejects_duplicate_carriers() {
        let expected = vec!["face-a".to_string(), "face-b".to_string()];
        let actual = ["face-a", "face-a", "face-b"];

        assert!(!contains_exactly(&expected, actual.iter().copied()));
    }
}
