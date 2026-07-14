use crate::WorthServerOperationAdmissionPosture;

use super::WorthServerQueryHandoffOperation;

#[derive(Clone, Debug)]
pub struct WorthServerQueryHandoffInput {
    operation_admission: WorthServerOperationAdmissionPosture,
    operation: WorthServerQueryHandoffOperation,
}

impl WorthServerQueryHandoffInput {
    pub fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        operation: WorthServerQueryHandoffOperation,
    ) -> Self {
        Self {
            operation_admission,
            operation,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthServerOperationAdmissionPosture,
        WorthServerQueryHandoffOperation,
    ) {
        (self.operation_admission, self.operation)
    }
}
