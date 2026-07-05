use super::classification::promote_region_candidate_boundary_bundle;
use super::counters::PlanarBooleanOverlapRegionCandidateBoundaryCounters;
use super::denial::PlanarBooleanOverlapRegionCandidateBoundaryDenial;
use super::input::PlanarBooleanOverlapRegionCandidateBoundaryInput;
use super::rows::{
    PlanarBooleanAdmittedOverlapRegionRow, PlanarBooleanBoundaryOnlyOverlapOutcomeRow,
    PlanarBooleanDeniedOverlapRegionCandidateRow, PlanarBooleanOverlapRegionCandidateRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanPostAdmissionNormalizationDenial,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCandidateSet {
    set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionCandidateRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDeniedOverlapRegionCandidateSet {
    set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanDeniedOverlapRegionCandidateRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAdmittedOverlapRegionSet {
    set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanAdmittedOverlapRegionRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryOnlyOverlapOutcomeSet {
    set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanBoundaryOnlyOverlapOutcomeRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    bundle_identity: String,
    overlap_region_candidates: PlanarBooleanOverlapRegionCandidateSet,
    denied_overlap_region_candidates: PlanarBooleanDeniedOverlapRegionCandidateSet,
    admitted_overlap_regions: PlanarBooleanAdmittedOverlapRegionSet,
    boundary_only_overlap_outcomes: PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
    counters: PlanarBooleanOverlapRegionCandidateBoundaryCounters,
}

macro_rules! impl_product_set {
    ($name:ident, $row:ty) => {
        impl $name {
            pub(crate) fn new(
                set_identity: String,
                request_identity: String,
                arrangement_graph_identity: String,
                cell_set_identity: String,
                ordering_basis_identity: String,
                rows: Vec<$row>,
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

            pub fn rows(&self) -> &[$row] { &self.rows }
            pub fn set_identity(&self) -> &str { &self.set_identity }
            pub fn request_identity(&self) -> &str { &self.request_identity }
            pub fn arrangement_graph_identity(&self) -> &str { &self.arrangement_graph_identity }
            pub fn cell_set_identity(&self) -> &str { &self.cell_set_identity }
            pub fn ordering_basis_identity(&self) -> &str { &self.ordering_basis_identity }
        }
    };
}

impl_product_set!(PlanarBooleanOverlapRegionCandidateSet, PlanarBooleanOverlapRegionCandidateRow);
impl_product_set!(PlanarBooleanDeniedOverlapRegionCandidateSet, PlanarBooleanDeniedOverlapRegionCandidateRow);
impl_product_set!(PlanarBooleanAdmittedOverlapRegionSet, PlanarBooleanAdmittedOverlapRegionRow);
impl_product_set!(PlanarBooleanBoundaryOnlyOverlapOutcomeSet, PlanarBooleanBoundaryOnlyOverlapOutcomeRow);

impl PlanarBooleanOverlapRegionCandidateBoundaryBundle {
    pub fn from_pre_region_normalization(
        pre_region_normalization: &PlanarBooleanPreRegionNormalizationBundle,
        shared_area_admission: &PlanarBooleanSharedAreaAdmissionBundle,
    ) -> Result<Self, PlanarBooleanOverlapRegionCandidateBoundaryDenial> {
        Self::admit(PlanarBooleanOverlapRegionCandidateBoundaryInput::from_pre_region_normalization(
            pre_region_normalization,
            shared_area_admission,
        ))
    }

    pub fn admit(
        input: PlanarBooleanOverlapRegionCandidateBoundaryInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionCandidateBoundaryDenial> {
        promote_region_candidate_boundary_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        overlap_region_candidates: PlanarBooleanOverlapRegionCandidateSet,
        denied_overlap_region_candidates: PlanarBooleanDeniedOverlapRegionCandidateSet,
        admitted_overlap_regions: PlanarBooleanAdmittedOverlapRegionSet,
        boundary_only_overlap_outcomes: PlanarBooleanBoundaryOnlyOverlapOutcomeSet,
        counters: PlanarBooleanOverlapRegionCandidateBoundaryCounters,
    ) -> Self {
        Self {
            bundle_identity,
            overlap_region_candidates,
            denied_overlap_region_candidates,
            admitted_overlap_regions,
            boundary_only_overlap_outcomes,
            counters,
        }
    }

    pub fn overlap_region_candidates(&self) -> &PlanarBooleanOverlapRegionCandidateSet {
        &self.overlap_region_candidates
    }

    pub fn denied_overlap_region_candidates(&self) -> &PlanarBooleanDeniedOverlapRegionCandidateSet {
        &self.denied_overlap_region_candidates
    }

    pub fn admitted_overlap_regions(&self) -> &PlanarBooleanAdmittedOverlapRegionSet {
        &self.admitted_overlap_regions
    }

    pub fn boundary_only_overlap_outcomes(&self) -> &PlanarBooleanBoundaryOnlyOverlapOutcomeSet {
        &self.boundary_only_overlap_outcomes
    }

    pub fn normalize_post_admission_canonical_winding(
        &self,
    ) -> Result<
        PlanarBooleanPostAdmissionNormalizationBundle,
        PlanarBooleanPostAdmissionNormalizationDenial,
    > {
        PlanarBooleanPostAdmissionNormalizationBundle::from_region_candidate_boundary(self)
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionCandidateBoundaryCounters {
        self.counters
    }
}
