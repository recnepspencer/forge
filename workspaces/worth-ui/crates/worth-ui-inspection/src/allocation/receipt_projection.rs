use super::{UiAllocationInspectionEvidenceRef, UiAllocationInspectionGeometry};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionReceiptIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionPlanningBasisIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationInspectionNeighborhoodIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionStreamFamily {
    TextInput,
    QueryProjection,
    ResizePreview,
    DurableResize,
    ViewportObservation,
    ScrollExtentObservation,
    PortalAnchorObservation,
    HostMeasurementReplacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionInvalidationFamily {
    TextContentChange,
    QueryMeasurementFactChange,
    ContentExtentChange,
    ResizePreviewDelta,
    DurableLocalResizeChange,
    ViewportExtentChange,
    ScrollExtentObservation,
    ScrollOwnedExtentChange,
    PortalAnchorMovement,
    HostMeasurementResultReplacement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionReusePosture {
    NewCommit,
    FullReuse,
    StructureReuseLeafRemeasure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionFreshnessPosture {
    Current,
    Coalescing,
    StaleButBounded,
    RecomputePending,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationInspectionSelection {
    primary_neighborhood: UiAllocationInspectionNeighborhoodIdentity,
    ordered_neighborhoods: Box<[UiAllocationInspectionNeighborhoodIdentity]>,
    widening_count: u16,
    evidence_ref: UiAllocationInspectionEvidenceRef,
}

/// Receipt-backed local explanation. Its typed citations point back to runtime-owned evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationInspectionReceipt {
    receipt_identity: UiAllocationInspectionReceiptIdentity,
    planning_basis_identity: UiAllocationInspectionPlanningBasisIdentity,
    stream_families: Box<[UiAllocationInspectionStreamFamily]>,
    invalidation_families: Box<[UiAllocationInspectionInvalidationFamily]>,
    reuse: UiAllocationInspectionReusePosture,
    freshness: UiAllocationInspectionFreshnessPosture,
    invalidation_evidence_ref: UiAllocationInspectionEvidenceRef,
    reuse_evidence_ref: UiAllocationInspectionEvidenceRef,
    selection: UiAllocationInspectionSelection,
    geometry: UiAllocationInspectionGeometry,
}

pub struct UiAllocationInspectionReceiptProjection {
    pub receipt_identity: UiAllocationInspectionReceiptIdentity,
    pub planning_basis_identity: UiAllocationInspectionPlanningBasisIdentity,
    pub stream_families: Box<[UiAllocationInspectionStreamFamily]>,
    pub invalidation_families: Box<[UiAllocationInspectionInvalidationFamily]>,
    pub reuse: UiAllocationInspectionReusePosture,
    pub freshness: UiAllocationInspectionFreshnessPosture,
    pub invalidation_evidence_ref: UiAllocationInspectionEvidenceRef,
    pub reuse_evidence_ref: UiAllocationInspectionEvidenceRef,
    pub selection: UiAllocationInspectionSelection,
    pub geometry: UiAllocationInspectionGeometry,
}

impl UiAllocationInspectionReceipt {
    pub fn from_runtime_projection(projection: UiAllocationInspectionReceiptProjection) -> Self {
        let UiAllocationInspectionReceiptProjection {
            receipt_identity,
            planning_basis_identity,
            stream_families,
            invalidation_families,
            reuse,
            freshness,
            invalidation_evidence_ref,
            reuse_evidence_ref,
            selection,
            geometry,
        } = projection;
        Self {
            receipt_identity,
            planning_basis_identity,
            stream_families,
            invalidation_families,
            reuse,
            freshness,
            invalidation_evidence_ref,
            reuse_evidence_ref,
            selection,
            geometry,
        }
    }
    pub fn receipt_identity(&self) -> UiAllocationInspectionReceiptIdentity {
        self.receipt_identity
    }
    pub fn planning_basis_identity(&self) -> UiAllocationInspectionPlanningBasisIdentity {
        self.planning_basis_identity
    }
    pub fn stream_families(&self) -> &[UiAllocationInspectionStreamFamily] {
        &self.stream_families
    }
    pub fn invalidation_families(&self) -> &[UiAllocationInspectionInvalidationFamily] {
        &self.invalidation_families
    }
    pub fn reuse(&self) -> UiAllocationInspectionReusePosture {
        self.reuse
    }
    pub fn freshness(&self) -> UiAllocationInspectionFreshnessPosture {
        self.freshness
    }
    pub fn invalidation_evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.invalidation_evidence_ref
    }
    pub fn reuse_evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.reuse_evidence_ref
    }
    pub fn selection(&self) -> &UiAllocationInspectionSelection {
        &self.selection
    }
    pub fn geometry(&self) -> &UiAllocationInspectionGeometry {
        &self.geometry
    }
}

impl UiAllocationInspectionSelection {
    pub fn new(
        primary_neighborhood: UiAllocationInspectionNeighborhoodIdentity,
        ordered_neighborhoods: Box<[UiAllocationInspectionNeighborhoodIdentity]>,
        widening_count: u16,
        evidence_ref: UiAllocationInspectionEvidenceRef,
    ) -> Self {
        Self {
            primary_neighborhood,
            ordered_neighborhoods,
            widening_count,
            evidence_ref,
        }
    }
    pub fn primary_neighborhood(&self) -> UiAllocationInspectionNeighborhoodIdentity {
        self.primary_neighborhood
    }
    pub fn ordered_neighborhoods(&self) -> &[UiAllocationInspectionNeighborhoodIdentity] {
        &self.ordered_neighborhoods
    }
    pub fn widening_count(&self) -> u16 {
        self.widening_count
    }
    pub fn evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.evidence_ref
    }
}

impl UiAllocationInspectionReceiptIdentity {
    pub const fn diagnostic(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiAllocationInspectionPlanningBasisIdentity {
    pub const fn diagnostic(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiAllocationInspectionNeighborhoodIdentity {
    pub const fn diagnostic(value: u64) -> Self {
        Self(value)
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}
