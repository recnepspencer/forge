use crate::capability::ComponentDescriptor;

use super::digest::fold_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiComponentPlanMeaning {
    descriptor: ComponentDescriptor,
    child_range_identity: Option<String>,
}

impl WorthUiComponentPlanMeaning {
    pub(crate) fn new(
        descriptor: ComponentDescriptor,
        child_range_identity: Option<String>,
    ) -> Self {
        Self {
            descriptor,
            child_range_identity,
        }
    }

    pub(crate) fn child_range_identity(&self) -> Option<&str> {
        self.child_range_identity.as_deref()
    }

    pub(crate) fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        fold_text(0x636f_6d70_6f6e_656e, self.descriptor.id().as_str())
    }
}
