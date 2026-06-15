use crate::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanCommonPlaneReductionRequest,
    WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportReceipt,
};

use super::admission::admit_operand_scope;
use super::admitted_scope::PlanarBooleanCommonPlaneAdmittedOperandScope;
use super::denial::PlanarBooleanCommonPlaneScopeAdmissionError;
use super::identity::admitted_request_identity;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCommonPlaneScopeAdmittedRequest {
    reduction_request: PlanarBooleanCommonPlaneReductionRequest,
    admitted_scope: PlanarBooleanCommonPlaneAdmittedOperandScope,
    scope_admission_identity: String,
}

impl PlanarBooleanCommonPlaneScopeAdmittedRequest {
    pub fn from_reduction_request(
        reduction_request: PlanarBooleanCommonPlaneReductionRequest,
    ) -> Result<Self, PlanarBooleanCommonPlaneScopeAdmissionError> {
        let admitted_scope = admit_operand_scope(&reduction_request)?;
        let scope_admission_identity =
            admitted_request_identity(&reduction_request, admitted_scope);
        Ok(Self {
            reduction_request,
            admitted_scope,
            scope_admission_identity,
        })
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        self.reduction_request.declaration()
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        self.reduction_request.support()
    }

    pub fn operand_pair_identity(&self) -> &str {
        self.reduction_request.operand_pair_identity()
    }

    pub fn request_identity(&self) -> &str {
        self.reduction_request.request_identity()
    }

    pub fn scope_admission_identity(&self) -> &str {
        &self.scope_admission_identity
    }

    pub fn admitted_scope(&self) -> PlanarBooleanCommonPlaneAdmittedOperandScope {
        self.admitted_scope
    }

    pub fn reduction_request(&self) -> &PlanarBooleanCommonPlaneReductionRequest {
        &self.reduction_request
    }

    pub fn operand_pair_recipe(&self) -> &BuiltBooleanOperandPairRecipe {
        self.reduction_request.operand_pair_recipe()
    }
}
