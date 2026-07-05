use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanPostAdmissionNormalizationBundle;

#[derive(Clone, Copy)]
pub struct PlanarBooleanOverlapRegionIdentityLineageInput<'a> {
    post_admission_normalization: &'a PlanarBooleanPostAdmissionNormalizationBundle,
}

impl<'a> PlanarBooleanOverlapRegionIdentityLineageInput<'a> {
    pub fn new(
        post_admission_normalization: &'a PlanarBooleanPostAdmissionNormalizationBundle,
    ) -> Self {
        Self {
            post_admission_normalization,
        }
    }

    pub fn from_post_admission_normalization(
        post_admission_normalization: &'a PlanarBooleanPostAdmissionNormalizationBundle,
    ) -> Self {
        Self::new(post_admission_normalization)
    }

    pub fn post_admission_normalization(
        self,
    ) -> &'a PlanarBooleanPostAdmissionNormalizationBundle {
        self.post_admission_normalization
    }
}
