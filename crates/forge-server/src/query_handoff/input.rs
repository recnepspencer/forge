use crate::ForgeServerOperationAdmissionPosture;

use super::ForgeServerQueryHandoffOperation;

#[derive(Clone, Debug)]
pub struct ForgeServerQueryHandoffInput {
    operation_admission: ForgeServerOperationAdmissionPosture,
    operation: ForgeServerQueryHandoffOperation,
}

impl ForgeServerQueryHandoffInput {
    pub fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Self {
        Self {
            operation_admission,
            operation,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeServerOperationAdmissionPosture,
        ForgeServerQueryHandoffOperation,
    ) {
        (self.operation_admission, self.operation)
    }
}
