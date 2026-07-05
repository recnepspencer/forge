use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapCellContainmentEvidenceKind {
    Inside,
    Outside,
    BoundaryOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapCellWindingEvidenceKind {
    NoTopologySupport,
    BoundaryTopology,
    SupportingIslandTopology,
    BoundaryTopologyAndSupportingIslandTopology,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellContainmentRow {
    cell_identity: String,
    arrangement_identity: String,
    neighborhood_identity: String,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    supporting_island_identity: Option<String>,
    source_loop_identities: Vec<String>,
    evidence_kind: PlanarBooleanOverlapCellContainmentEvidenceKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellWindingRow {
    cell_identity: String,
    arrangement_identity: String,
    neighborhood_identity: String,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    supporting_island_identity: Option<String>,
    source_loop_identities: Vec<String>,
    evidence_kind: PlanarBooleanOverlapCellWindingEvidenceKind,
    winding_number: i8,
}

impl PlanarBooleanOverlapCellContainmentRow {
    pub(crate) fn new(
        cell_identity: String,
        arrangement_identity: String,
        neighborhood_identity: String,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        supporting_island_identity: Option<String>,
        source_loop_identities: Vec<String>,
        evidence_kind: PlanarBooleanOverlapCellContainmentEvidenceKind,
    ) -> Self {
        Self {
            cell_identity,
            arrangement_identity,
            neighborhood_identity,
            operand_side,
            supporting_island_identity,
            source_loop_identities,
            evidence_kind,
        }
    }

    pub fn cell_identity(&self) -> &str {
        &self.cell_identity
    }

    pub fn arrangement_identity(&self) -> &str {
        &self.arrangement_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.operand_side
    }

    pub fn supporting_island_identity(&self) -> Option<&str> {
        self.supporting_island_identity.as_deref()
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn evidence_kind(&self) -> PlanarBooleanOverlapCellContainmentEvidenceKind {
        self.evidence_kind
    }
}

impl PlanarBooleanOverlapCellWindingRow {
    pub(crate) fn new(
        cell_identity: String,
        arrangement_identity: String,
        neighborhood_identity: String,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        supporting_island_identity: Option<String>,
        source_loop_identities: Vec<String>,
        evidence_kind: PlanarBooleanOverlapCellWindingEvidenceKind,
        winding_number: i8,
    ) -> Self {
        Self {
            cell_identity,
            arrangement_identity,
            neighborhood_identity,
            operand_side,
            supporting_island_identity,
            source_loop_identities,
            evidence_kind,
            winding_number,
        }
    }

    pub fn cell_identity(&self) -> &str {
        &self.cell_identity
    }

    pub fn arrangement_identity(&self) -> &str {
        &self.arrangement_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.operand_side
    }

    pub fn supporting_island_identity(&self) -> Option<&str> {
        self.supporting_island_identity.as_deref()
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn evidence_kind(&self) -> PlanarBooleanOverlapCellWindingEvidenceKind {
        self.evidence_kind
    }

    pub fn winding_number(&self) -> i8 {
        self.winding_number
    }
}
