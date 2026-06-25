use crate::runtime::{
    WorthUiLiveViewTargetBinding, WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId,
};

use super::{
    WorthUiLiveViewAdmissionCounters, WorthUiLiveViewDeclaration, WorthUiLiveViewDenial,
    WorthUiLiveViewStateAccess, WorthUiLiveViewStateBindingDeclaration, WorthUiLiveViewStateFactId,
    WorthUiLiveViewStateValue, WorthUiLiveViewStateValueKind,
};
use crate::runtime::live_view::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewDeclarationReceipt {
    live_view_id: String,
    target_binding: WorthUiLiveViewTargetBinding,
    bindings: Vec<WorthUiLiveViewStateBindingReceipt>,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiLiveViewAdmissionCounters,
    declaration_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewStateBindingReceipt {
    live_view_id: String,
    target_binding: WorthUiLiveViewTargetBinding,
    binding_id: String,
    state_fact: WorthUiLiveViewStateFactId,
    value_kind: WorthUiLiveViewStateValueKind,
    access: WorthUiLiveViewStateAccess,
    binding_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewStateEditIntent {
    binding: WorthUiLiveViewStateBindingReceipt,
    value: WorthUiLiveViewStateValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewEditReceipt {
    binding: WorthUiLiveViewStateBindingReceipt,
    previous_value: Option<WorthUiLiveViewStateValue>,
    next_value: WorthUiLiveViewStateValue,
    changed_fact: WorthUiRuntimeFactId,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewAdmissionReport {
    denials: Vec<WorthUiLiveViewDenial>,
    counters: WorthUiLiveViewAdmissionCounters,
    denial_set_digest: u64,
}

impl WorthUiLiveViewDeclarationReceipt {
    pub(crate) fn new(
        declaration: WorthUiLiveViewDeclaration,
        bindings: Vec<WorthUiLiveViewStateBindingReceipt>,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
        counters: WorthUiLiveViewAdmissionCounters,
    ) -> Self {
        let (live_view_id, target_binding, _) = declaration.into_parts();
        let declaration_digest = digest_parts(
            std::iter::once(live_view_id.as_str())
                .chain(bindings.iter().map(|binding| binding.binding_id())),
        );
        Self {
            live_view_id,
            target_binding,
            bindings,
            graph_execution,
            counters,
            declaration_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn target_binding(&self) -> &WorthUiLiveViewTargetBinding {
        &self.target_binding
    }

    pub fn bindings(&self) -> &[WorthUiLiveViewStateBindingReceipt] {
        &self.bindings
    }

    pub fn binding(&self, binding_id: &str) -> Option<&WorthUiLiveViewStateBindingReceipt> {
        self.bindings
            .iter()
            .find(|binding| binding.binding_id() == binding_id)
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn counters(&self) -> WorthUiLiveViewAdmissionCounters {
        self.counters
    }

    pub fn declaration_digest(&self) -> u64 {
        self.declaration_digest
    }
}

impl WorthUiLiveViewStateBindingReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        target_binding: &WorthUiLiveViewTargetBinding,
        declaration: &WorthUiLiveViewStateBindingDeclaration,
    ) -> Self {
        let binding_digest = digest_parts([
            live_view_id,
            declaration.binding_id(),
            declaration.state_fact().as_str(),
            declaration.value_kind().token(),
            declaration.access().token(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            target_binding: target_binding.clone(),
            binding_id: declaration.binding_id().to_owned(),
            state_fact: declaration.state_fact().clone(),
            value_kind: declaration.value_kind().clone(),
            access: declaration.access(),
            binding_digest,
        }
    }

    pub fn edit(&self, value: WorthUiLiveViewStateValue) -> WorthUiLiveViewStateEditIntent {
        WorthUiLiveViewStateEditIntent {
            binding: self.clone(),
            value,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn target_binding(&self) -> &WorthUiLiveViewTargetBinding {
        &self.target_binding
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

    pub fn binding_digest(&self) -> u64 {
        self.binding_digest
    }
}

impl WorthUiLiveViewStateEditIntent {
    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiLiveViewStateBindingReceipt,
        WorthUiLiveViewStateValue,
    ) {
        (self.binding, self.value)
    }
}

impl WorthUiLiveViewEditReceipt {
    pub(crate) fn new(
        binding: WorthUiLiveViewStateBindingReceipt,
        previous_value: Option<WorthUiLiveViewStateValue>,
        next_value: WorthUiLiveViewStateValue,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let changed_fact = WorthUiRuntimeFactId::live_view_state_value(binding.state_fact.as_str());
        let next_digest_token = next_value.digest_token();
        let receipt_digest = digest_parts([
            binding.live_view_id(),
            binding.binding_id(),
            binding.state_fact().as_str(),
            next_digest_token.as_str(),
        ]);
        Self {
            binding,
            previous_value,
            next_value,
            changed_fact,
            graph_execution,
            receipt_digest,
        }
    }

    pub fn binding(&self) -> &WorthUiLiveViewStateBindingReceipt {
        &self.binding
    }

    pub fn previous_value(&self) -> Option<&WorthUiLiveViewStateValue> {
        self.previous_value.as_ref()
    }

    pub fn next_value(&self) -> &WorthUiLiveViewStateValue {
        &self.next_value
    }

    pub fn changed_fact(&self) -> &WorthUiRuntimeFactId {
        &self.changed_fact
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiLiveViewAdmissionReport {
    pub(crate) fn denied(
        denials: Vec<WorthUiLiveViewDenial>,
        counters: WorthUiLiveViewAdmissionCounters,
    ) -> Self {
        let denial_set_digest = digest_parts(denials.iter().map(WorthUiLiveViewDenial::code));
        Self {
            denials,
            counters,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiLiveViewDenial] {
        &self.denials
    }

    pub fn counters(&self) -> WorthUiLiveViewAdmissionCounters {
        self.counters
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}
