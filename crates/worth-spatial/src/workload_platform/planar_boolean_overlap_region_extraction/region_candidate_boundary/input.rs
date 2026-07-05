use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanOverlapRegionCandidateBoundaryInput<'a> {
    pre_region_normalization: &'a PlanarBooleanPreRegionNormalizationBundle,
    shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
}

impl<'a> PlanarBooleanOverlapRegionCandidateBoundaryInput<'a> {
    pub fn new(
        pre_region_normalization: &'a PlanarBooleanPreRegionNormalizationBundle,
        shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
    ) -> Self {
        Self {
            pre_region_normalization,
            shared_area_admission,
        }
    }

    pub fn from_pre_region_normalization(
        pre_region_normalization: &'a PlanarBooleanPreRegionNormalizationBundle,
        shared_area_admission: &'a PlanarBooleanSharedAreaAdmissionBundle,
    ) -> Self {
        Self::new(pre_region_normalization, shared_area_admission)
    }

    pub fn pre_region_normalization(self) -> &'a PlanarBooleanPreRegionNormalizationBundle {
        self.pre_region_normalization
    }

    pub fn shared_area_admission(self) -> &'a PlanarBooleanSharedAreaAdmissionBundle {
        self.shared_area_admission
    }
}
