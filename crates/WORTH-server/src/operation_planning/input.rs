use crate::{
    WorthServerOperationAdmissionPosture, WorthServerOperationPreconditionPosture,
    WorthServerQueryHandoffOperation,
};

pub struct WorthServerOperationPlannerInput {
    operation_admission: WorthServerOperationAdmissionPosture,
    operation: WorthServerQueryHandoffOperation,
    precondition_posture: Option<WorthServerOperationPreconditionPosture>,
    bound_workspace: Option<worth_query::facade::WorthQueryWorkspace>,
}

impl WorthServerOperationPlannerInput {
    pub fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        operation: WorthServerQueryHandoffOperation,
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
        precondition_posture: WorthServerOperationPreconditionPosture,
    ) -> Self {
        self.precondition_posture = Some(precondition_posture);
        self
    }

    pub(crate) fn with_bound_workspace(
        mut self,
        bound_workspace: worth_query::facade::WorthQueryWorkspace,
    ) -> Self {
        self.bound_workspace = Some(bound_workspace);
        self
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn operation(&self) -> &WorthServerQueryHandoffOperation {
        &self.operation
    }

    pub fn precondition_posture(&self) -> Option<&WorthServerOperationPreconditionPosture> {
        self.precondition_posture.as_ref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthServerOperationAdmissionPosture,
        WorthServerQueryHandoffOperation,
        Option<WorthServerOperationPreconditionPosture>,
        Option<worth_query::facade::WorthQueryWorkspace>,
    ) {
        (
            self.operation_admission,
            self.operation,
            self.precondition_posture,
            self.bound_workspace,
        )
    }
}
