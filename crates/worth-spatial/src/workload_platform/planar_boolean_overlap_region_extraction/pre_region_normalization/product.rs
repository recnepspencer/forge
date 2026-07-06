use super::classification::build_pre_region_normalization_bundle;
use super::counters::PlanarBooleanPreRegionNormalizationCounters;
use super::denial::PlanarBooleanPreRegionNormalizationDenial;
use super::input::PlanarBooleanPreRegionNormalizationInput;
use super::rows::PlanarBooleanOppositeSenseOverlapNormalizationRow;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionCandidateBoundaryDenial, PlanarBooleanSharedAreaAdmissionBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOppositeSenseOverlapNormalizationSet {
    normalization_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOppositeSenseOverlapNormalizationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPreRegionNormalizationBundle {
    bundle_identity: String,
    opposite_sense_overlap_normalizations: PlanarBooleanOppositeSenseOverlapNormalizationSet,
    counters: PlanarBooleanPreRegionNormalizationCounters,
}

impl PlanarBooleanOppositeSenseOverlapNormalizationSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        normalization_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOppositeSenseOverlapNormalizationRow>,
    ) -> Self {
        Self {
            normalization_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanOppositeSenseOverlapNormalizationRow] {
        &self.rows
    }

    pub fn normalization_set_identity(&self) -> &str {
        &self.normalization_set_identity
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

impl PlanarBooleanPreRegionNormalizationBundle {
    pub fn from_shared_area_admission(
        shared_area_admission: &PlanarBooleanSharedAreaAdmissionBundle,
        chain_lineage_map: &PlanarBooleanOverlapChainRegionLineageMap,
    ) -> Result<Self, PlanarBooleanPreRegionNormalizationDenial> {
        Self::admit(
            PlanarBooleanPreRegionNormalizationInput::from_shared_area_admission(
                shared_area_admission,
                chain_lineage_map,
            ),
        )
    }

    pub fn admit(
        input: PlanarBooleanPreRegionNormalizationInput<'_>,
    ) -> Result<Self, PlanarBooleanPreRegionNormalizationDenial> {
        build_pre_region_normalization_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        opposite_sense_overlap_normalizations: PlanarBooleanOppositeSenseOverlapNormalizationSet,
        counters: PlanarBooleanPreRegionNormalizationCounters,
    ) -> Self {
        Self {
            bundle_identity,
            opposite_sense_overlap_normalizations,
            counters,
        }
    }

    pub fn opposite_sense_overlap_normalizations(
        &self,
    ) -> &PlanarBooleanOppositeSenseOverlapNormalizationSet {
        &self.opposite_sense_overlap_normalizations
    }

    pub fn promote_overlap_region_candidates(
        &self,
        shared_area_admission: &PlanarBooleanSharedAreaAdmissionBundle,
    ) -> Result<
        PlanarBooleanOverlapRegionCandidateBoundaryBundle,
        PlanarBooleanOverlapRegionCandidateBoundaryDenial,
    > {
        PlanarBooleanOverlapRegionCandidateBoundaryBundle::from_pre_region_normalization(
            self,
            shared_area_admission,
        )
    }

    pub fn counters(&self) -> PlanarBooleanPreRegionNormalizationCounters {
        self.counters
    }
}
