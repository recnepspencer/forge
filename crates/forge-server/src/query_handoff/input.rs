use crate::ForgeServerAdmission;

use super::ForgeServerQueryHandoffOperation;

#[derive(Clone, Debug)]
pub struct ForgeServerQueryHandoffInput {
    admission: ForgeServerAdmission,
    operation: ForgeServerQueryHandoffOperation,
}

impl ForgeServerQueryHandoffInput {
    pub fn new(
        admission: ForgeServerAdmission,
        operation: ForgeServerQueryHandoffOperation,
    ) -> Self {
        Self {
            admission,
            operation,
        }
    }

    pub(crate) fn into_parts(self) -> (ForgeServerAdmission, ForgeServerQueryHandoffOperation) {
        (self.admission, self.operation)
    }
}
