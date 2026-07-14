use crate::{
    WorthServerOperationConcurrencyClass, WorthServerOperationPreconditionPosture,
    WorthServerOperationSupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationReadinessClosure {
    support_posture: WorthServerOperationSupportPosture,
    precondition_posture: WorthServerOperationPreconditionPosture,
    concurrency_class: WorthServerOperationConcurrencyClass,
}

impl WorthServerOperationReadinessClosure {
    pub(crate) fn new(
        support_posture: WorthServerOperationSupportPosture,
        precondition_posture: WorthServerOperationPreconditionPosture,
        concurrency_class: WorthServerOperationConcurrencyClass,
    ) -> Self {
        Self {
            support_posture,
            precondition_posture,
            concurrency_class,
        }
    }

    pub fn support_posture(&self) -> &WorthServerOperationSupportPosture {
        &self.support_posture
    }

    pub fn precondition_posture(&self) -> &WorthServerOperationPreconditionPosture {
        &self.precondition_posture
    }

    pub fn concurrency_class(&self) -> WorthServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }
}
