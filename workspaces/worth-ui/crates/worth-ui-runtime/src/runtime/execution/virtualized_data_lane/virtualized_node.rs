use std::rc::Rc;

use crate::runtime::{WorthUiQueryBindingIdentity, WorthUiRuntimeHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataNode {
    runtime_handle: WorthUiRuntimeHandle,
    binding_identity: Rc<WorthUiQueryBindingIdentity>,
    settled_fact_link: Rc<crate::runtime::WorthUiQuerySettledFactLink>,
}

impl WorthUiVirtualizedDataNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        binding_identity: Rc<WorthUiQueryBindingIdentity>,
        settled_fact_link: Rc<crate::runtime::WorthUiQuerySettledFactLink>,
    ) -> Self {
        Self {
            runtime_handle,
            binding_identity,
            settled_fact_link,
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
        self.settled_fact_link.definition()
    }

    pub fn settled_fact_link(&self) -> &crate::runtime::WorthUiQuerySettledFactLink {
        &self.settled_fact_link
    }

    pub(crate) fn installed_reference(
        &self,
    ) -> &worth_ui_query_binding::WorthUiInstalledQueryBindingReference {
        self.settled_fact_link.installed_reference()
    }
}
