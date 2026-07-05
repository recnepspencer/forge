use topology::facade::{
    PlanarBooleanOverlapOperatorClassificationMatrix, PlanarBooleanOverlapValidatorRegistrationPlan,
};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap,
    PlanarBooleanOverlapRegionExtractionRequest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegistrationContractError {
    RegistryIdentityMismatch,
    InvalidDirectoryCutoverMap,
    UncertifiedOverlapRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegistrationContract {
    request_identity: String,
    blueprint_registry_identity: String,
    cutover_row_count: usize,
}

impl PlanarBooleanOverlapRegistrationContract {
    pub fn freeze_phase_2(
        request: &PlanarBooleanOverlapRegionExtractionRequest,
        cutover_map: &PlanarBooleanOverlapRegionExtractionDirectoryCutoverMap,
        operator_matrix: &PlanarBooleanOverlapOperatorClassificationMatrix,
        validator_plan: &PlanarBooleanOverlapValidatorRegistrationPlan,
    ) -> Result<Self, PlanarBooleanOverlapRegistrationContractError> {
        if !request.certifies_overlap_region_extraction_request() {
            return Err(PlanarBooleanOverlapRegistrationContractError::UncertifiedOverlapRequest);
        }
        if operator_matrix.registry_identity() != validator_plan.registry_identity() {
            return Err(PlanarBooleanOverlapRegistrationContractError::RegistryIdentityMismatch);
        }
        if !cutover_map.certifies_one_owner_per_artifact()
            || !cutover_map.certifies_one_consuming_phase_per_artifact()
            || cutover_map.certifies_live_artifact_contracts().is_err()
            || cutover_map.certifies_legacy_surface_contracts().is_err()
            || cutover_map.certify_live_overlap_lane().is_err()
        {
            return Err(PlanarBooleanOverlapRegistrationContractError::InvalidDirectoryCutoverMap);
        }
        Ok(Self {
            request_identity: request.request_identity().to_string(),
            blueprint_registry_identity: operator_matrix.registry_identity().digest().to_string(),
            cutover_row_count: cutover_map.artifact_rows().len(),
        })
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn blueprint_registry_identity(&self) -> &str {
        &self.blueprint_registry_identity
    }

    pub fn cutover_row_count(&self) -> usize {
        self.cutover_row_count
    }
}
