use super::classification::build_post_admission_normalization_bundle;
use super::counters::PlanarBooleanPostAdmissionNormalizationCounters;
use super::denial::PlanarBooleanPostAdmissionNormalizationDenial;
use super::input::PlanarBooleanPostAdmissionNormalizationInput;
use super::rows::PlanarBooleanOverlapRegionCanonicalWindingRow;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    PlanarBooleanOverlapRegionIdentityLineageBundle,
    PlanarBooleanOverlapRegionIdentityLineageDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCanonicalWindingSet {
    set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionCanonicalWindingRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPostAdmissionNormalizationBundle {
    bundle_identity: String,
    overlap_region_canonical_winding: PlanarBooleanOverlapRegionCanonicalWindingSet,
    source_region_candidate_boundary: PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    counters: PlanarBooleanPostAdmissionNormalizationCounters,
}

impl PlanarBooleanOverlapRegionCanonicalWindingSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionCanonicalWindingRow>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionCanonicalWindingRow] {
        &self.rows
    }

    pub fn set_identity(&self) -> &str {
        &self.set_identity
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

impl PlanarBooleanPostAdmissionNormalizationBundle {
    pub fn from_region_candidate_boundary(
        region_candidate_boundary: &PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    ) -> Result<Self, PlanarBooleanPostAdmissionNormalizationDenial> {
        Self::admit(
            PlanarBooleanPostAdmissionNormalizationInput::from_region_candidate_boundary(
                region_candidate_boundary,
            ),
        )
    }

    pub fn admit(
        input: PlanarBooleanPostAdmissionNormalizationInput<'_>,
    ) -> Result<Self, PlanarBooleanPostAdmissionNormalizationDenial> {
        build_post_admission_normalization_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        overlap_region_canonical_winding: PlanarBooleanOverlapRegionCanonicalWindingSet,
        source_region_candidate_boundary: PlanarBooleanOverlapRegionCandidateBoundaryBundle,
        counters: PlanarBooleanPostAdmissionNormalizationCounters,
    ) -> Self {
        Self {
            bundle_identity,
            overlap_region_canonical_winding,
            source_region_candidate_boundary,
            counters,
        }
    }

    pub fn overlap_region_canonical_winding(
        &self,
    ) -> &PlanarBooleanOverlapRegionCanonicalWindingSet {
        &self.overlap_region_canonical_winding
    }

    pub(crate) fn source_region_candidate_boundary(
        &self,
    ) -> &PlanarBooleanOverlapRegionCandidateBoundaryBundle {
        &self.source_region_candidate_boundary
    }

    pub fn mint_overlap_region_identity_lineage(
        &self,
    ) -> Result<
        PlanarBooleanOverlapRegionIdentityLineageBundle,
        PlanarBooleanOverlapRegionIdentityLineageDenial,
    > {
        PlanarBooleanOverlapRegionIdentityLineageBundle::from_post_admission_normalization(self)
    }

    pub fn counters(&self) -> PlanarBooleanPostAdmissionNormalizationCounters {
        self.counters
    }
}
