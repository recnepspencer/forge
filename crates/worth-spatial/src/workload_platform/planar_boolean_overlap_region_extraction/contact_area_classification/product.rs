use super::classification::build_boundary_contact_classification_bundle;
use super::counters::PlanarBooleanBoundaryContactClassificationCounters;
use super::denial::PlanarBooleanBoundaryContactClassificationDenial;
use super::input::PlanarBooleanBoundaryContactClassificationInput;
use super::rows::{
    PlanarBooleanPureBoundaryOnlyOutcomeRow, PlanarBooleanSharedBoundaryContactOutcomeRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapIslandComponentBundle;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentSet, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanSharedAreaAdmissionBundle,
    PlanarBooleanSharedAreaAdmissionDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSharedBoundaryContactOutcomeSet {
    outcome_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanSharedBoundaryContactOutcomeRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPureBoundaryOnlyOutcomeSet {
    outcome_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanPureBoundaryOnlyOutcomeRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryContactClassificationBundle {
    bundle_identity: String,
    shared_boundary_contact_outcomes: PlanarBooleanSharedBoundaryContactOutcomeSet,
    pure_boundary_only_outcomes: PlanarBooleanPureBoundaryOnlyOutcomeSet,
    area_overlap_components: PlanarBooleanAreaOverlapComponentSet,
    counters: PlanarBooleanBoundaryContactClassificationCounters,
}

impl PlanarBooleanSharedBoundaryContactOutcomeSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanSharedBoundaryContactOutcomeRow>,
    ) -> Self {
        Self {
            outcome_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanSharedBoundaryContactOutcomeRow] {
        &self.rows
    }
}

impl PlanarBooleanPureBoundaryOnlyOutcomeSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanPureBoundaryOnlyOutcomeRow>,
    ) -> Self {
        Self {
            outcome_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanPureBoundaryOnlyOutcomeRow] {
        &self.rows
    }
}

impl PlanarBooleanBoundaryContactClassificationBundle {
    pub fn from_island_component_bundle(
        island_component_bundle: &PlanarBooleanOverlapIslandComponentBundle,
    ) -> Result<Self, PlanarBooleanBoundaryContactClassificationDenial> {
        Self::admit(PlanarBooleanBoundaryContactClassificationInput::from_island_component_bundle(
            island_component_bundle,
        ))
    }

    pub fn admit(
        input: PlanarBooleanBoundaryContactClassificationInput<'_>,
    ) -> Result<Self, PlanarBooleanBoundaryContactClassificationDenial> {
        build_boundary_contact_classification_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        shared_boundary_contact_outcomes: PlanarBooleanSharedBoundaryContactOutcomeSet,
        pure_boundary_only_outcomes: PlanarBooleanPureBoundaryOnlyOutcomeSet,
        area_overlap_components: PlanarBooleanAreaOverlapComponentSet,
        counters: PlanarBooleanBoundaryContactClassificationCounters,
    ) -> Self {
        Self {
            bundle_identity,
            shared_boundary_contact_outcomes,
            pure_boundary_only_outcomes,
            area_overlap_components,
            counters,
        }
    }

    pub fn bundle_identity(&self) -> &str {
        &self.bundle_identity
    }

    pub fn shared_boundary_contact_outcomes(&self) -> &PlanarBooleanSharedBoundaryContactOutcomeSet {
        &self.shared_boundary_contact_outcomes
    }

    pub fn pure_boundary_only_outcomes(&self) -> &PlanarBooleanPureBoundaryOnlyOutcomeSet {
        &self.pure_boundary_only_outcomes
    }

    pub fn area_overlap_components(&self) -> &PlanarBooleanAreaOverlapComponentSet {
        &self.area_overlap_components
    }

    pub fn request_identity(&self) -> &str {
        self.area_overlap_components.request_identity()
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        self.area_overlap_components.arrangement_graph_identity()
    }

    pub fn cell_set_identity(&self) -> &str {
        self.area_overlap_components.cell_set_identity()
    }

    pub fn ordering_basis_identity(&self) -> &str {
        self.area_overlap_components.ordering_basis_identity()
    }

    pub fn admit_shared_area_components(
        &self,
        containment_map: &PlanarBooleanOverlapCellContainmentMap,
        winding_field: &PlanarBooleanOverlapCellWindingField,
    ) -> Result<PlanarBooleanSharedAreaAdmissionBundle, PlanarBooleanSharedAreaAdmissionDenial> {
        PlanarBooleanSharedAreaAdmissionBundle::from_boundary_contact_classification(
            self,
            containment_map,
            winding_field,
        )
    }

    pub fn counters(&self) -> PlanarBooleanBoundaryContactClassificationCounters {
        self.counters
    }
}
