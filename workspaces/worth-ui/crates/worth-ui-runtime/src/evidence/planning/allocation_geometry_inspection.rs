pub(crate) fn project_allocation_geometry(
    geometry: &crate::runtime::UiCommittedAllocationGeometryEvidence,
    identity: u64,
) -> worth_ui_inspection::UiAllocationInspectionGeometry {
    use worth_ui_inspection::{
        UiAllocationInspectionEvidenceFamily as Family,
        UiAllocationInspectionEvidenceRef as EvidenceRef,
        UiAllocationInspectionGeometry as InspectionGeometry,
    };
    InspectionGeometry::from_runtime_projection(
        project_bounds(geometry.bounds()),
        project_anchor(geometry.anchor_posture()),
        project_edges(geometry.parent_edges()),
        project_edges(geometry.sibling_edges()),
        project_relationship_ids(geometry.spacing_relationship_ids()),
        project_relationship_ids(geometry.baseline_relationships()),
        EvidenceRef::diagnostic(Family::GeometryArtifact, identity),
    )
}

fn project_bounds(
    bounds: crate::runtime::UiAllocationGeometryKnowledge<
        crate::runtime::UiAllocationAxisAlignedBounds,
    >,
) -> worth_ui_inspection::UiAllocationInspectionKnowledge<
    worth_ui_inspection::UiAllocationInspectionBounds,
> {
    use worth_ui_inspection::{
        UiAllocationInspectionBounds as InspectionBounds,
        UiAllocationInspectionKnowledge as Knowledge,
    };
    match bounds {
        crate::runtime::UiAllocationGeometryKnowledge::Known(bounds) => {
            Knowledge::Known(InspectionBounds::from_runtime_projection(
                bounds.x(),
                bounds.y(),
                bounds.width(),
                bounds.height(),
                project_coordinate_space(bounds.coordinate_space()),
            ))
        }
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation => {
            Knowledge::NotKnownAtAllocation
        }
    }
}

fn project_anchor(
    anchor: crate::runtime::UiAllocationAnchorPosture,
) -> worth_ui_inspection::UiAllocationInspectionAnchorPosture {
    use worth_ui_inspection::{
        UiAllocationInspectionAnchorPosture as InspectionAnchor,
        UiAllocationInspectionPortalAnchorTargetIdentity as TargetIdentity,
    };
    match anchor {
        crate::runtime::UiAllocationAnchorPosture::NotAnchored => InspectionAnchor::NotAnchored,
        crate::runtime::UiAllocationAnchorPosture::PortalAnchored(identity) => {
            InspectionAnchor::PortalAnchored {
                target: TargetIdentity::diagnostic(identity.target().raw()),
                coordinate_space: project_coordinate_space(identity.coordinate_space()),
            }
        }
    }
}

fn project_edges(
    edges: &crate::runtime::UiAllocationGeometryKnowledge<
        Box<[crate::runtime::UiAllocationEdgeReference]>,
    >,
) -> worth_ui_inspection::UiAllocationInspectionKnowledge<
    Box<[worth_ui_inspection::UiAllocationInspectionEdgeReference]>,
> {
    use worth_ui_inspection::{
        UiAllocationInspectionEdgeReference as InspectionEdge,
        UiAllocationInspectionGraphNodeIdentity as GraphNodeIdentity,
        UiAllocationInspectionKnowledge as Knowledge,
    };
    match edges {
        crate::runtime::UiAllocationGeometryKnowledge::Known(edges) => Knowledge::Known(
            edges
                .iter()
                .map(|edge| {
                    InspectionEdge::from_runtime_projection(
                        GraphNodeIdentity::diagnostic(edge.target().digest()),
                        match edge.axis() {
                            crate::runtime::UiAllocationAxis::Horizontal => {
                                worth_ui_inspection::UiAllocationInspectionAxis::Horizontal
                            }
                            crate::runtime::UiAllocationAxis::Vertical => {
                                worth_ui_inspection::UiAllocationInspectionAxis::Vertical
                            }
                        },
                        edge.delta(),
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation => {
            Knowledge::NotKnownAtAllocation
        }
    }
}

fn project_relationship_ids(
    relationships: &crate::runtime::UiAllocationGeometryKnowledge<Box<[u64]>>,
) -> worth_ui_inspection::UiAllocationInspectionKnowledge<Box<[u64]>> {
    match relationships {
        crate::runtime::UiAllocationGeometryKnowledge::Known(relationships) => {
            worth_ui_inspection::UiAllocationInspectionKnowledge::Known(relationships.clone())
        }
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation => {
            worth_ui_inspection::UiAllocationInspectionKnowledge::NotKnownAtAllocation
        }
    }
}

fn project_coordinate_space(
    space: crate::evidence::UiMeasurementCoordinateSpace,
) -> worth_ui_inspection::UiAllocationInspectionCoordinateSpace {
    use crate::evidence::UiMeasurementCoordinateSpace as Runtime;
    use worth_ui_inspection::UiAllocationInspectionCoordinateSpace as Inspection;
    match space {
        Runtime::Viewport => Inspection::Viewport,
        Runtime::Window => Inspection::Window,
        Runtime::GraphNodeLocal => Inspection::GraphNodeLocal,
        Runtime::HostSurface => Inspection::HostSurface,
        Runtime::PortalLayer => Inspection::PortalLayer,
    }
}
