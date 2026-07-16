#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiAllocationAxisAlignedBounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    coordinate_space: crate::evidence::UiMeasurementCoordinateSpace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationGeometryKnowledge<T> {
    Known(T),
    NotKnownAtAllocation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationAnchorPosture {
    NotAnchored,
    PortalAnchored(crate::runtime::portal_anchored_allocation::UiPortalAnchorIdentity),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiAllocationEdgeReference {
    target: crate::graph::UiGraphNodeIdentity,
    axis: UiAllocationAxis,
    delta: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiCommittedAllocationGeometryEvidence {
    bounds: UiAllocationGeometryKnowledge<UiAllocationAxisAlignedBounds>,
    anchor_posture: UiAllocationAnchorPosture,
    portal_anchor_observation: Option<UiPortalAnchorObservationGeometryEvidence>,
    parent_edges: UiAllocationGeometryKnowledge<Box<[UiAllocationEdgeReference]>>,
    sibling_edges: UiAllocationGeometryKnowledge<Box<[UiAllocationEdgeReference]>>,
    spacing_relationship_ids: UiAllocationGeometryKnowledge<Box<[u64]>>,
    baseline_relationships: UiAllocationGeometryKnowledge<Box<[u64]>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiPortalAnchorObservationGeometryEvidence {
    identity: crate::runtime::portal_anchored_allocation::UiPortalAnchorIdentity,
    observed_bounds: UiAllocationAxisAlignedBounds,
}

impl UiCommittedAllocationGeometryEvidence {
    pub(super) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        let portal_anchor_observation = portal_anchor_observation(candidate);
        Self {
            // The host-observed anchor rectangle is an input, not the portal's
            // derived placement. Phase 11's solver must establish allocation
            // bounds before this field may become Known.
            bounds: UiAllocationGeometryKnowledge::NotKnownAtAllocation,
            anchor_posture: portal_anchor_observation
                .map(|observation| UiAllocationAnchorPosture::PortalAnchored(observation.identity))
                .unwrap_or(UiAllocationAnchorPosture::NotAnchored),
            portal_anchor_observation,
            parent_edges: UiAllocationGeometryKnowledge::NotKnownAtAllocation,
            sibling_edges: UiAllocationGeometryKnowledge::NotKnownAtAllocation,
            spacing_relationship_ids: UiAllocationGeometryKnowledge::NotKnownAtAllocation,
            baseline_relationships: UiAllocationGeometryKnowledge::NotKnownAtAllocation,
        }
    }

    pub fn bounds(&self) -> UiAllocationGeometryKnowledge<UiAllocationAxisAlignedBounds> {
        self.bounds
    }

    pub fn anchor_posture(&self) -> UiAllocationAnchorPosture {
        self.anchor_posture
    }

    pub fn portal_anchor_observation(&self) -> Option<UiPortalAnchorObservationGeometryEvidence> {
        self.portal_anchor_observation
    }

    pub fn parent_edges(&self) -> &UiAllocationGeometryKnowledge<Box<[UiAllocationEdgeReference]>> {
        &self.parent_edges
    }

    pub fn sibling_edges(
        &self,
    ) -> &UiAllocationGeometryKnowledge<Box<[UiAllocationEdgeReference]>> {
        &self.sibling_edges
    }

    pub fn spacing_relationship_ids(&self) -> &UiAllocationGeometryKnowledge<Box<[u64]>> {
        &self.spacing_relationship_ids
    }

    pub fn baseline_relationships(&self) -> &UiAllocationGeometryKnowledge<Box<[u64]>> {
        &self.baseline_relationships
    }
}

impl UiAllocationAxisAlignedBounds {
    pub fn x(self) -> f32 {
        self.x
    }
    pub fn y(self) -> f32 {
        self.y
    }
    pub fn width(self) -> f32 {
        self.width
    }
    pub fn height(self) -> f32 {
        self.height
    }
    pub fn coordinate_space(self) -> crate::evidence::UiMeasurementCoordinateSpace {
        self.coordinate_space
    }
}

impl UiAllocationEdgeReference {
    pub fn target(self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
    pub fn axis(self) -> UiAllocationAxis {
        self.axis
    }
    pub fn delta(self) -> f32 {
        self.delta
    }
}

impl UiPortalAnchorObservationGeometryEvidence {
    pub fn identity(self) -> crate::runtime::UiPortalAnchorIdentity {
        self.identity
    }

    pub fn observed_bounds(self) -> UiAllocationAxisAlignedBounds {
        self.observed_bounds
    }
}

fn portal_anchor_observation(
    candidate: &super::UiAllocationCandidate,
) -> Option<UiPortalAnchorObservationGeometryEvidence> {
    if let Some(basis) = candidate.portal_allocation_input() {
        let observation = basis.observation();
        let rect = observation.rect();
        return Some(UiPortalAnchorObservationGeometryEvidence {
            identity: observation.identity(),
            observed_bounds: UiAllocationAxisAlignedBounds {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                coordinate_space: observation.identity().coordinate_space(),
            },
        });
    }
    let input = candidate
        .allocation_constraint_set()?
        .portal_anchor_planning_input()?;
    if input.posture()
        != crate::evidence::UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly
        || !input.is_planning_time_only()
    {
        return None;
    }
    candidate.measurement_basis().evidence_inputs().iter().find_map(|evidence| {
        let result = evidence.as_host_measurement_result()?;
        let crate::evidence::UiMeasurementValue::PortalAnchorRect(rect) = result.value() else {
            return None;
        };
        let identity = crate::runtime::portal_anchored_allocation::UiPortalAnchorIdentity::from_measurement_result(result)?;
        Some(UiPortalAnchorObservationGeometryEvidence {
            identity,
            observed_bounds: UiAllocationAxisAlignedBounds {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                coordinate_space: result.coordinate_space(),
            },
        })
    })
}
