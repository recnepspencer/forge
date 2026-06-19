use crate::{
    ForgeServerOperationAdmissionPosture, ForgeServerOperationPreconditionPosture,
    ForgeServerQueryHandoffOperation,
};

pub struct ForgeServerOperationPlannerInput {
    operation_admission: ForgeServerOperationAdmissionPosture,
    operation: ForgeServerQueryHandoffOperation,
    precondition_posture: Option<ForgeServerOperationPreconditionPosture>,
    bound_workspace: Option<forge_query::facade::ForgeQueryWorkspace>,
}

impl ForgeServerOperationPlannerInput {
    pub fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Self {
        Self {
            operation_admission,
            operation,
            precondition_posture: None,
            bound_workspace: None,
        }
    }

    pub fn with_precondition_posture(
        mut self,
        precondition_posture: ForgeServerOperationPreconditionPosture,
    ) -> Self {
        self.precondition_posture = Some(precondition_posture);
        self
    }

    pub(crate) fn with_bound_workspace(
        mut self,
        bound_workspace: forge_query::facade::ForgeQueryWorkspace,
    ) -> Self {
        self.bound_workspace = Some(bound_workspace);
        self
    }

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn operation(&self) -> &ForgeServerQueryHandoffOperation {
        &self.operation
    }

    pub fn precondition_posture(&self) -> Option<&ForgeServerOperationPreconditionPosture> {
        self.precondition_posture.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeServerOperationAdmissionPosture,
        ForgeServerQueryHandoffOperation,
        Option<ForgeServerOperationPreconditionPosture>,
        Option<forge_query::facade::ForgeQueryWorkspace>,
    ) {
        (
            self.operation_admission,
            self.operation,
            self.precondition_posture,
            self.bound_workspace,
        )
    }
}
