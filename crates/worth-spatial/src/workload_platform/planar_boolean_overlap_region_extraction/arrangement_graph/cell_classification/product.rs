use super::containment::build_containment_map;
use super::counters::PlanarBooleanOverlapCellClassificationCounters;
use super::denial::PlanarBooleanOverlapCellClassificationDenial;
use super::input::{
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellWindingFieldInput,
};
use super::rows::{PlanarBooleanOverlapCellContainmentRow, PlanarBooleanOverlapCellWindingRow};
use super::winding::build_winding_field;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellContainmentMap {
    containment_map_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapCellContainmentRow>,
    counters: PlanarBooleanOverlapCellClassificationCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapCellWindingField {
    winding_field_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapCellWindingRow>,
    counters: PlanarBooleanOverlapCellClassificationCounters,
}

impl PlanarBooleanOverlapCellContainmentMap {
    pub fn admit(
        input: PlanarBooleanOverlapCellContainmentInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapCellClassificationDenial> {
        build_containment_map(input)
    }

    pub(crate) fn new(
        containment_map_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapCellContainmentRow>,
        counters: PlanarBooleanOverlapCellClassificationCounters,
    ) -> Self {
        Self {
            containment_map_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
            counters,
        }
    }

    pub fn containment_map_identity(&self) -> &str {
        &self.containment_map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapCellContainmentRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanOverlapCellClassificationCounters {
        self.counters
    }
}

impl PlanarBooleanOverlapCellWindingField {
    pub fn admit(
        input: PlanarBooleanOverlapCellWindingFieldInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapCellClassificationDenial> {
        build_winding_field(input)
    }

    pub(crate) fn new(
        winding_field_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapCellWindingRow>,
        counters: PlanarBooleanOverlapCellClassificationCounters,
    ) -> Self {
        Self {
            winding_field_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
            counters,
        }
    }

    pub fn winding_field_identity(&self) -> &str {
        &self.winding_field_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapCellWindingRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanOverlapCellClassificationCounters {
        self.counters
    }
}
