use super::AspectContract;
use crate::aspects::masks::{
    AspectMask, DiagnosticMask, MaskAdmissibilityDenial, MutationMask, ProjectionMask,
};

impl AspectContract {
    pub fn admits_projection_mask(
        &self,
        mask: &AspectMask<ProjectionMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Projection)
    }

    pub fn admits_mutation_mask(
        &self,
        mask: &AspectMask<MutationMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Mutation)
    }

    pub fn admits_diagnostic_mask(
        &self,
        mask: &AspectMask<DiagnosticMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Diagnostic)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MaskModeAdmission {
    Projection,
    Mutation,
    Diagnostic,
}
