use super::classification::build_shared_area_admission_bundle;
use super::counters::PlanarBooleanSharedAreaAdmissionCounters;
use super::denial::PlanarBooleanSharedAreaAdmissionDenial;
use super::input::PlanarBooleanSharedAreaAdmissionInput;
use super::rows::{
    PlanarBooleanMixedBoundaryAreaOutcomeRow, PlanarBooleanSharedAreaAdmissionOutcomeRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapChainRegionLineageMap,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanPreRegionNormalizationDenial,
    PlanarBooleanPureBoundaryOnlyOutcomeSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSharedAreaAdmissionOutcomeSet {
    outcome_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanSharedAreaAdmissionOutcomeRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanMixedBoundaryAreaOutcomeSet {
    outcome_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanMixedBoundaryAreaOutcomeRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSharedAreaAdmissionBundle {
    bundle_identity: String,
    shared_area_admission_outcomes: PlanarBooleanSharedAreaAdmissionOutcomeSet,
    mixed_boundary_area_outcomes: PlanarBooleanMixedBoundaryAreaOutcomeSet,
    pure_boundary_only_outcomes: PlanarBooleanPureBoundaryOnlyOutcomeSet,
    counters: PlanarBooleanSharedAreaAdmissionCounters,
}

impl PlanarBooleanSharedAreaAdmissionOutcomeSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanSharedAreaAdmissionOutcomeRow>,
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

    pub fn rows(&self) -> &[PlanarBooleanSharedAreaAdmissionOutcomeRow] {
        &self.rows
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
}

impl PlanarBooleanMixedBoundaryAreaOutcomeSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanMixedBoundaryAreaOutcomeRow>,
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

    pub fn rows(&self) -> &[PlanarBooleanMixedBoundaryAreaOutcomeRow] {
        &self.rows
    }
}

impl PlanarBooleanSharedAreaAdmissionBundle {
    pub fn from_boundary_contact_classification(
        boundary_contact_classification: &PlanarBooleanBoundaryContactClassificationBundle,
        containment_map: &PlanarBooleanOverlapCellContainmentMap,
        winding_field: &PlanarBooleanOverlapCellWindingField,
    ) -> Result<Self, PlanarBooleanSharedAreaAdmissionDenial> {
        Self::admit(PlanarBooleanSharedAreaAdmissionInput::from_boundary_contact_classification(
            boundary_contact_classification,
            containment_map,
            winding_field,
        ))
    }

    pub fn admit(
        input: PlanarBooleanSharedAreaAdmissionInput<'_>,
    ) -> Result<Self, PlanarBooleanSharedAreaAdmissionDenial> {
        build_shared_area_admission_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        shared_area_admission_outcomes: PlanarBooleanSharedAreaAdmissionOutcomeSet,
        mixed_boundary_area_outcomes: PlanarBooleanMixedBoundaryAreaOutcomeSet,
        pure_boundary_only_outcomes: PlanarBooleanPureBoundaryOnlyOutcomeSet,
        counters: PlanarBooleanSharedAreaAdmissionCounters,
    ) -> Self {
        Self {
            bundle_identity,
            shared_area_admission_outcomes,
            mixed_boundary_area_outcomes,
            pure_boundary_only_outcomes,
            counters,
        }
    }

    pub fn shared_area_admission_outcomes(&self) -> &PlanarBooleanSharedAreaAdmissionOutcomeSet {
        &self.shared_area_admission_outcomes
    }

    pub fn mixed_boundary_area_outcomes(&self) -> &PlanarBooleanMixedBoundaryAreaOutcomeSet {
        &self.mixed_boundary_area_outcomes
    }

    pub fn pure_boundary_only_outcomes(&self) -> &PlanarBooleanPureBoundaryOnlyOutcomeSet {
        &self.pure_boundary_only_outcomes
    }

    pub fn request_identity(&self) -> &str {
        self.shared_area_admission_outcomes.request_identity()
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        self.shared_area_admission_outcomes.arrangement_graph_identity()
    }

    pub fn cell_set_identity(&self) -> &str {
        self.shared_area_admission_outcomes.cell_set_identity()
    }

    pub fn ordering_basis_identity(&self) -> &str {
        self.shared_area_admission_outcomes.ordering_basis_identity()
    }

    pub fn normalize_pre_region_coincidence(
        &self,
        chain_lineage_map: &PlanarBooleanOverlapChainRegionLineageMap,
    ) -> Result<PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanPreRegionNormalizationDenial>
    {
        PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
            self,
            chain_lineage_map,
        )
    }

    pub fn counters(&self) -> PlanarBooleanSharedAreaAdmissionCounters {
        self.counters
    }
}
