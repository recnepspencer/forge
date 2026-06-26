use super::{
    FrozenTaskPresentationCapabilities, TaskPresentationAcceptedRegistrationProof,
    TaskPresentationDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TaskPresentationRegistry {
    descriptors: Vec<TaskPresentationDescriptor>,
}

impl TaskPresentationRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: TaskPresentationDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_task_presentations: &TaskPresentationAcceptedRegistrationProof,
    ) -> FrozenTaskPresentationCapabilities {
        FrozenTaskPresentationCapabilities::from_accepted_descriptors(
            self.descriptors,
            accepted_task_presentations,
        )
    }
}
