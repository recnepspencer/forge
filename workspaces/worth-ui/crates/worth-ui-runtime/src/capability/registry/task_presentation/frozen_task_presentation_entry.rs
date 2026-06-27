use super::{TaskPresentationDescriptor, TaskPresentationKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenTaskPresentationEntry {
    descriptor: TaskPresentationDescriptor,
    key: TaskPresentationKey,
}

impl FrozenTaskPresentationEntry {
    pub(crate) fn new(descriptor: TaskPresentationDescriptor, key: TaskPresentationKey) -> Self {
        Self { descriptor, key }
    }

    pub fn descriptor(&self) -> &TaskPresentationDescriptor {
        &self.descriptor
    }

    pub fn key(&self) -> &TaskPresentationKey {
        &self.key
    }
}
