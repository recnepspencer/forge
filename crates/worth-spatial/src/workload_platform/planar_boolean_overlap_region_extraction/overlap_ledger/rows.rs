use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSourceKind;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanOverlapRegionDecisionKind {
    Request,
    Participation,
    Adjacency,
    Arrangement,
    Contact,
    Area,
    Winding,
    Identity,
    PersistentNamePropagation,
    SubshapeSignature,
    BoundaryOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionDecisionLogRow {
    decision_identity: String,
    kind: PlanarBooleanOverlapRegionDecisionKind,
    focal_identity: String,
    related_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedgerRow {
    ledger_row_identity: String,
    region_identity: String,
    canonical_winding_identity: String,
    source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    source_identity: String,
    area_overlap_component_identity: Option<String>,
    correspondence_only: bool,
    persistent_name_identities: Vec<String>,
    subshape_signature_identity: String,
    lineage_identities: Vec<String>,
    canonical_boundary_segment_identities: Vec<String>,
    canonical_source_loop_identities: Vec<String>,
}

impl PlanarBooleanOverlapRegionDecisionLogRow {
    pub(crate) fn new(
        decision_identity: String,
        kind: PlanarBooleanOverlapRegionDecisionKind,
        focal_identity: String,
        related_identities: Vec<String>,
    ) -> Self {
        Self {
            decision_identity,
            kind,
            focal_identity,
            related_identities,
        }
    }

    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionDecisionKind {
        self.kind
    }

    pub fn focal_identity(&self) -> &str {
        &self.focal_identity
    }

    pub fn related_identities(&self) -> &[String] {
        &self.related_identities
    }
}

impl PlanarBooleanOverlapRegionLedgerRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        ledger_row_identity: String,
        region_identity: String,
        canonical_winding_identity: String,
        source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
        source_identity: String,
        area_overlap_component_identity: Option<String>,
        correspondence_only: bool,
        persistent_name_identities: Vec<String>,
        subshape_signature_identity: String,
        lineage_identities: Vec<String>,
        canonical_boundary_segment_identities: Vec<String>,
        canonical_source_loop_identities: Vec<String>,
    ) -> Self {
        Self {
            ledger_row_identity,
            region_identity,
            canonical_winding_identity,
            source_kind,
            source_identity,
            area_overlap_component_identity,
            correspondence_only,
            persistent_name_identities,
            subshape_signature_identity,
            lineage_identities,
            canonical_boundary_segment_identities,
            canonical_source_loop_identities,
        }
    }

    pub fn ledger_row_identity(&self) -> &str {
        &self.ledger_row_identity
    }

    pub fn region_identity(&self) -> &str {
        &self.region_identity
    }

    pub fn canonical_winding_identity(&self) -> &str {
        &self.canonical_winding_identity
    }

    pub fn source_kind(&self) -> PlanarBooleanOverlapRegionCanonicalWindingSourceKind {
        self.source_kind
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn area_overlap_component_identity(&self) -> Option<&str> {
        self.area_overlap_component_identity.as_deref()
    }

    pub fn correspondence_only(&self) -> bool {
        self.correspondence_only
    }

    pub fn persistent_name_identities(&self) -> &[String] {
        &self.persistent_name_identities
    }

    pub fn subshape_signature_identity(&self) -> &str {
        &self.subshape_signature_identity
    }

    pub fn lineage_identities(&self) -> &[String] {
        &self.lineage_identities
    }

    pub fn canonical_boundary_segment_identities(&self) -> &[String] {
        &self.canonical_boundary_segment_identities
    }

    pub fn canonical_source_loop_identities(&self) -> &[String] {
        &self.canonical_source_loop_identities
    }
}
