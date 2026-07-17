use super::UiAllocationInspectionEvidenceRef;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionGraphNodeIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionPortalAnchorTargetIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionCoordinateSpace {
    Viewport,
    Window,
    GraphNodeLocal,
    HostSurface,
    PortalLayer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiAllocationInspectionBounds {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    coordinate_space: UiAllocationInspectionCoordinateSpace,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionKnowledge<T> {
    Known(T),
    NotKnownAtAllocation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionAxis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiAllocationInspectionEdgeReference {
    target: UiAllocationInspectionGraphNodeIdentity,
    axis: UiAllocationInspectionAxis,
    delta: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionAnchorPosture {
    NotAnchored,
    PortalAnchored {
        target: UiAllocationInspectionPortalAnchorTargetIdentity,
        coordinate_space: UiAllocationInspectionCoordinateSpace,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationInspectionGeometry {
    bounds: UiAllocationInspectionKnowledge<UiAllocationInspectionBounds>,
    anchor: UiAllocationInspectionAnchorPosture,
    parent_edges: UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>>,
    sibling_edges: UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>>,
    spacing_relationship_ids: UiAllocationInspectionKnowledge<Box<[u64]>>,
    baseline_relationship_ids: UiAllocationInspectionKnowledge<Box<[u64]>>,
    evidence_ref: UiAllocationInspectionEvidenceRef,
}

impl UiAllocationInspectionGeometry {
    pub fn from_runtime_projection(
        bounds: UiAllocationInspectionKnowledge<UiAllocationInspectionBounds>,
        anchor: UiAllocationInspectionAnchorPosture,
        parent_edges: UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>>,
        sibling_edges: UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>>,
        spacing_relationship_ids: UiAllocationInspectionKnowledge<Box<[u64]>>,
        baseline_relationship_ids: UiAllocationInspectionKnowledge<Box<[u64]>>,
        evidence_ref: UiAllocationInspectionEvidenceRef,
    ) -> Self {
        Self {
            bounds,
            anchor,
            parent_edges,
            sibling_edges,
            spacing_relationship_ids,
            baseline_relationship_ids,
            evidence_ref,
        }
    }

    pub fn bounds(&self) -> &UiAllocationInspectionKnowledge<UiAllocationInspectionBounds> {
        &self.bounds
    }
    pub fn anchor(&self) -> UiAllocationInspectionAnchorPosture {
        self.anchor
    }
    pub fn parent_edges(
        &self,
    ) -> &UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>> {
        &self.parent_edges
    }
    pub fn sibling_edges(
        &self,
    ) -> &UiAllocationInspectionKnowledge<Box<[UiAllocationInspectionEdgeReference]>> {
        &self.sibling_edges
    }
    pub fn spacing_relationship_ids(&self) -> &UiAllocationInspectionKnowledge<Box<[u64]>> {
        &self.spacing_relationship_ids
    }
    pub fn baseline_relationship_ids(&self) -> &UiAllocationInspectionKnowledge<Box<[u64]>> {
        &self.baseline_relationship_ids
    }
    pub fn evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.evidence_ref
    }
}

impl UiAllocationInspectionBounds {
    pub const fn from_runtime_projection(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        coordinate_space: UiAllocationInspectionCoordinateSpace,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            coordinate_space,
        }
    }
    pub const fn x(self) -> f32 {
        self.x
    }
    pub const fn y(self) -> f32 {
        self.y
    }
    pub const fn width(self) -> f32 {
        self.width
    }
    pub const fn height(self) -> f32 {
        self.height
    }
    pub const fn coordinate_space(self) -> UiAllocationInspectionCoordinateSpace {
        self.coordinate_space
    }
}

impl UiAllocationInspectionEdgeReference {
    pub const fn from_runtime_projection(
        target: UiAllocationInspectionGraphNodeIdentity,
        axis: UiAllocationInspectionAxis,
        delta: f32,
    ) -> Self {
        Self {
            target,
            axis,
            delta,
        }
    }
    pub const fn target(self) -> UiAllocationInspectionGraphNodeIdentity {
        self.target
    }
    pub const fn axis(self) -> UiAllocationInspectionAxis {
        self.axis
    }
    pub const fn delta(self) -> f32 {
        self.delta
    }
}

impl UiAllocationInspectionGraphNodeIdentity {
    pub const fn diagnostic(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiAllocationInspectionPortalAnchorTargetIdentity {
    pub const fn diagnostic(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}
