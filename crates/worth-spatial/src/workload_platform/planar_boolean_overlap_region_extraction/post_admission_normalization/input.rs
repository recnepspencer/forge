use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCandidateBoundaryBundle;

#[derive(Clone, Copy)]
pub struct PlanarBooleanPostAdmissionNormalizationInput<'a> {
    region_candidate_boundary: &'a PlanarBooleanOverlapRegionCandidateBoundaryBundle,
}

impl<'a> PlanarBooleanPostAdmissionNormalizationInput<'a> {
    pub fn new(
        region_candidate_boundary: &'a PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    ) -> Self {
        Self {
            region_candidate_boundary,
        }
    }

    pub fn from_region_candidate_boundary(
        region_candidate_boundary: &'a PlanarBooleanOverlapRegionCandidateBoundaryBundle,
    ) -> Self {
        Self::new(region_candidate_boundary)
    }

    pub fn region_candidate_boundary(
        self,
    ) -> &'a PlanarBooleanOverlapRegionCandidateBoundaryBundle {
        self.region_candidate_boundary
    }
}
