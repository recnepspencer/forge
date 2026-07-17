use super::{ViewBindingDescriptor, WorthUiViewBindingIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenViewBindingEntry {
    descriptor: ViewBindingDescriptor,
    identity: WorthUiViewBindingIdentity,
}

impl FrozenViewBindingEntry {
    pub(crate) fn new(
        descriptor: ViewBindingDescriptor,
        identity: WorthUiViewBindingIdentity,
    ) -> Self {
        Self {
            descriptor,
            identity,
        }
    }

    pub fn descriptor(&self) -> &ViewBindingDescriptor {
        &self.descriptor
    }

    pub fn identity(&self) -> WorthUiViewBindingIdentity {
        self.identity
    }
}
