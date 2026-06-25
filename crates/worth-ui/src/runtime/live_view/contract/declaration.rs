use crate::runtime::WorthUiLiveViewTargetBinding;

use super::{
    WorthUiLiveViewStateAccess, WorthUiLiveViewStateFactId, WorthUiLiveViewStateValueKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewDeclaration {
    live_view_id: String,
    target_binding: WorthUiLiveViewTargetBinding,
    bindings: Vec<WorthUiLiveViewStateBindingDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewStateBindingDeclaration {
    binding_id: String,
    state_fact: WorthUiLiveViewStateFactId,
    value_kind: WorthUiLiveViewStateValueKind,
    access: WorthUiLiveViewStateAccess,
}

impl WorthUiLiveViewDeclaration {
    pub fn new(
        live_view_id: impl Into<String>,
        target_binding: WorthUiLiveViewTargetBinding,
    ) -> Self {
        Self {
            live_view_id: live_view_id.into(),
            target_binding,
            bindings: Vec::new(),
        }
    }

    pub fn with_state_binding(mut self, binding: WorthUiLiveViewStateBindingDeclaration) -> Self {
        self.bindings.push(binding);
        self
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn target_binding(&self) -> &WorthUiLiveViewTargetBinding {
        &self.target_binding
    }

    pub fn bindings(&self) -> &[WorthUiLiveViewStateBindingDeclaration] {
        &self.bindings
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        String,
        WorthUiLiveViewTargetBinding,
        Vec<WorthUiLiveViewStateBindingDeclaration>,
    ) {
        (self.live_view_id, self.target_binding, self.bindings)
    }
}

impl WorthUiLiveViewStateBindingDeclaration {
    pub fn new(
        binding_id: impl Into<String>,
        state_fact: WorthUiLiveViewStateFactId,
        value_kind: WorthUiLiveViewStateValueKind,
        access: WorthUiLiveViewStateAccess,
    ) -> Self {
        Self {
            binding_id: binding_id.into(),
            state_fact,
            value_kind,
            access,
        }
    }

    pub fn binding_id(&self) -> &str {
        &self.binding_id
    }

    pub fn state_fact(&self) -> &WorthUiLiveViewStateFactId {
        &self.state_fact
    }

    pub fn value_kind(&self) -> &WorthUiLiveViewStateValueKind {
        &self.value_kind
    }

    pub fn access(&self) -> WorthUiLiveViewStateAccess {
        self.access
    }
}
