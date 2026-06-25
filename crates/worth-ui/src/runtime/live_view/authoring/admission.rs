use std::collections::BTreeSet;

use crate::runtime::{WorthUiLiveViewTargetBinding, WorthUiRuntimeHost};

use super::super::admission::target_binding_stale_denial;
use super::super::{
    WorthUiLiveViewAdmissionCounters, WorthUiLiveViewAdmissionReport,
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewDenial, WorthUiLiveViewStateAccess,
    WorthUiLiveViewStateFactId,
};
use super::document::WorthUiAuthoredLiveViewDeclaration;
use super::lowering::lower_authored_live_view_declaration;
use super::tokens::{
    authored_live_view_access, authored_live_view_value_kind, invalid_live_view_identity,
};

impl WorthUiRuntimeHost {
    pub fn admit_authored_live_view_declaration(
        &self,
        authored: &WorthUiAuthoredLiveViewDeclaration,
        target_binding: WorthUiLiveViewTargetBinding,
    ) -> Result<WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewAdmissionReport> {
        let denials = authored_live_view_denials(self, authored, &target_binding);
        let counters =
            WorthUiLiveViewAdmissionCounters::new(authored.bindings().len(), denials.len());
        if !denials.is_empty() {
            return Err(WorthUiLiveViewAdmissionReport::denied(denials, counters));
        }
        let lowered_declaration = lower_authored_live_view_declaration(authored, target_binding);
        self.admit_live_view_declaration(lowered_declaration)
    }
}

fn authored_live_view_denials(
    runtime: &WorthUiRuntimeHost,
    authored: &WorthUiAuthoredLiveViewDeclaration,
    target_binding: &WorthUiLiveViewTargetBinding,
) -> Vec<WorthUiLiveViewDenial> {
    let mut denials = Vec::new();
    append_declaration_denials(&mut denials, authored);
    if let Some(denial) = target_binding_stale_denial(runtime, target_binding) {
        denials.push(denial);
    }
    append_state_binding_denials(&mut denials, authored);
    denials
}

fn append_declaration_denials(
    denials: &mut Vec<WorthUiLiveViewDenial>,
    authored: &WorthUiAuthoredLiveViewDeclaration,
) {
    if invalid_live_view_identity(authored.live_view_id()) {
        denials.push(WorthUiLiveViewDenial::InvalidLiveViewId {
            live_view_id: authored.live_view_id().to_owned(),
        });
    }
    if authored.bindings().is_empty() {
        denials.push(WorthUiLiveViewDenial::EmptyStateBindings {
            live_view_id: authored.live_view_id().to_owned(),
        });
    }
}

fn append_state_binding_denials(
    denials: &mut Vec<WorthUiLiveViewDenial>,
    authored: &WorthUiAuthoredLiveViewDeclaration,
) {
    let mut seen_binding_ids = BTreeSet::new();
    for binding in authored.bindings() {
        if invalid_live_view_identity(binding.binding_id()) {
            denials.push(WorthUiLiveViewDenial::InvalidBindingId {
                binding_id: binding.binding_id().to_owned(),
            });
        }
        if !seen_binding_ids.insert(binding.binding_id().to_owned()) {
            denials.push(WorthUiLiveViewDenial::DuplicateBindingId {
                binding_id: binding.binding_id().to_owned(),
            });
        }
        if WorthUiLiveViewStateFactId::new(binding.state_fact()).is_err() {
            denials.push(WorthUiLiveViewDenial::InvalidStateFact {
                binding_id: binding.binding_id().to_owned(),
                state_fact: binding.state_fact().to_owned(),
            });
        }
        if !authored_live_view_value_kind(binding.value_kind()).is_supported() {
            denials.push(WorthUiLiveViewDenial::UnsupportedValueKind {
                binding_id: binding.binding_id().to_owned(),
                value_kind: binding.value_kind().to_owned(),
            });
        }
        if authored_live_view_access(binding.access()) != WorthUiLiveViewStateAccess::ReadWrite {
            denials.push(WorthUiLiveViewDenial::UnsupportedWritePosture {
                binding_id: binding.binding_id().to_owned(),
            });
        }
    }
}
