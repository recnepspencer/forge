use crate::{
    ForgeServerOperationConcurrencyClass, ForgeServerOperationPreconditionPosture,
    ForgeServerOperationSupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationReadinessClosure {
    support_posture: ForgeServerOperationSupportPosture,
    precondition_posture: ForgeServerOperationPreconditionPosture,
    concurrency_class: ForgeServerOperationConcurrencyClass,
}

impl ForgeServerOperationReadinessClosure {
    pub(crate) fn new(
        support_posture: ForgeServerOperationSupportPosture,
        precondition_posture: ForgeServerOperationPreconditionPosture,
        concurrency_class: ForgeServerOperationConcurrencyClass,
    ) -> Self {
        Self {
            support_posture,
            precondition_posture,
            concurrency_class,
        }
    }

    pub fn support_posture(&self) -> &ForgeServerOperationSupportPosture {
        &self.support_posture
    }

    pub fn precondition_posture(&self) -> &ForgeServerOperationPreconditionPosture {
        &self.precondition_posture
    }

    pub fn concurrency_class(&self) -> ForgeServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }
}
