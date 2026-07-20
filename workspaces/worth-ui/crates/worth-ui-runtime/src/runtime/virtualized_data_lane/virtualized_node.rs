use std::rc::Rc;

use crate::runtime::{WorthUiQueryBindingIdentity, WorthUiRuntimeHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataNode {
    runtime_handle: WorthUiRuntimeHandle,
    binding_identity: Rc<WorthUiQueryBindingIdentity>,
    installed_reference: Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
}

impl WorthUiVirtualizedDataNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        binding_identity: Rc<WorthUiQueryBindingIdentity>,
        installed_reference: Rc<worth_ui_query_binding::WorthUiInstalledQueryBindingReference>,
    ) -> Self {
        Self {
            runtime_handle,
            binding_identity,
            installed_reference,
        }
    }

    pub fn runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn plan_index(&self) -> u32 {
        self.runtime_handle.plan_index()
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub(crate) fn binding_identity_reference(&self) -> Rc<WorthUiQueryBindingIdentity> {
        Rc::clone(&self.binding_identity)
    }

    pub fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        self.installed_reference.definition()
    }

    pub(crate) fn installed_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiInstalledQueryBindingReference {
        &self.installed_reference
    }
}
