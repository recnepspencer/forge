use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiLiveViewStateBindingGraphPosture, WorthUiLiveViewTargetBinding,
    WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
    WorthUiRuntimeHost,
};

use super::{
    WorthUiLiveViewAdmissionCounters, WorthUiLiveViewAdmissionReport, WorthUiLiveViewDeclaration,
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewDenial, WorthUiLiveViewStateAccess,
    WorthUiLiveViewStateBindingReceipt,
};

pub(super) fn admit_live_view_declaration(
    runtime: &WorthUiRuntimeHost,
    declaration: WorthUiLiveViewDeclaration,
) -> Result<WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewAdmissionReport> {
    let denials = declaration_denials(runtime, &declaration);
    let counters =
        WorthUiLiveViewAdmissionCounters::new(declaration.bindings().len(), denials.len());
    if !denials.is_empty() {
        return Err(WorthUiLiveViewAdmissionReport::denied(denials, counters));
    }
    let bindings = declaration
        .bindings()
        .iter()
        .map(|binding| {
            WorthUiLiveViewStateBindingReceipt::new(
                declaration.live_view_id(),
                declaration.target_binding(),
                binding,
            )
        })
        .collect::<Vec<_>>();
    let graph_execution = live_view_graph_execution(
        runtime.graph_authority(),
        declaration.live_view_id(),
        declaration.target_binding(),
        &bindings,
        WorthUiLiveViewStateBindingGraphPosture::Admitted,
    );
    Ok(WorthUiLiveViewDeclarationReceipt::new(
        declaration,
        bindings,
        graph_execution,
        counters,
    ))
}

pub(super) fn live_view_graph_execution(
    graph_authority: &WorthUiRuntimeGraphAuthority,
    live_view_id: &str,
    target_binding: &WorthUiLiveViewTargetBinding,
    bindings: &[WorthUiLiveViewStateBindingReceipt],
    posture: WorthUiLiveViewStateBindingGraphPosture,
) -> WorthUiQueryGraphExecutionReceipt {
    let mut facts = vec![
        WorthUiRuntimeFactId::active_artifact(),
        WorthUiRuntimeFactId::user_intent_target_binding(target_binding.slot_name()),
        WorthUiRuntimeFactId::live_view_declaration(live_view_id),
        WorthUiRuntimeFactId::surface_mount(target_binding.surface_id()),
        WorthUiRuntimeFactId::component(target_binding.component_id()),
    ];
    for binding in bindings {
        facts.push(WorthUiRuntimeFactId::live_view_state_binding(format!(
            "{}:{}",
            live_view_id,
            binding.binding_id()
        )));
        facts.push(WorthUiRuntimeFactId::live_view_state_value(
            binding.state_fact().as_str(),
        ));
    }
    graph_authority
        .plan_live_view_state_binding_graph_operation(
            live_view_id,
            target_binding.binding_digest(),
            facts,
            posture,
        )
        .into_execution_receipt()
}

pub(crate) fn target_binding_stale_denial(
    runtime: &WorthUiRuntimeHost,
    target_binding: &WorthUiLiveViewTargetBinding,
) -> Option<WorthUiLiveViewDenial> {
    let actual_component_id = runtime
        .inspect_active_surface_descriptor(target_binding.surface_id())
        .map(|surface| {
            runtime
                .inspect_active_authored_surface_component_id(target_binding.surface_id())
                .unwrap_or_else(|| surface.component_id().as_str())
                .to_owned()
        });
    if actual_component_id.as_deref() == Some(target_binding.component_id().as_str()) {
        return None;
    }
    Some(WorthUiLiveViewDenial::StaleTargetBinding {
        slot_name: target_binding.slot_name().to_owned(),
        surface_id: target_binding.surface_id().as_str().to_owned(),
        expected_component_id: target_binding.component_id().as_str().to_owned(),
        actual_component_id,
    })
}

fn declaration_denials(
    runtime: &WorthUiRuntimeHost,
    declaration: &WorthUiLiveViewDeclaration,
) -> Vec<WorthUiLiveViewDenial> {
    let mut denials = Vec::new();
    if invalid_identity(declaration.live_view_id()) {
        denials.push(WorthUiLiveViewDenial::InvalidLiveViewId {
            live_view_id: declaration.live_view_id().to_owned(),
        });
    }
    if declaration.bindings().is_empty() {
        denials.push(WorthUiLiveViewDenial::EmptyStateBindings {
            live_view_id: declaration.live_view_id().to_owned(),
        });
    }
    if let Some(denial) = target_binding_stale_denial(runtime, declaration.target_binding()) {
        denials.push(denial);
    }
    let mut seen = BTreeSet::new();
    for binding in declaration.bindings() {
        if invalid_identity(binding.binding_id()) {
            denials.push(WorthUiLiveViewDenial::InvalidBindingId {
                binding_id: binding.binding_id().to_owned(),
            });
        }
        if !seen.insert(binding.binding_id().to_owned()) {
            denials.push(WorthUiLiveViewDenial::DuplicateBindingId {
                binding_id: binding.binding_id().to_owned(),
            });
        }
        if !binding.value_kind().is_supported() {
            denials.push(WorthUiLiveViewDenial::UnsupportedValueKind {
                binding_id: binding.binding_id().to_owned(),
                value_kind: binding.value_kind().token().to_owned(),
            });
        }
        if binding.access() != WorthUiLiveViewStateAccess::ReadWrite {
            denials.push(WorthUiLiveViewDenial::UnsupportedWritePosture {
                binding_id: binding.binding_id().to_owned(),
            });
        }
    }
    denials
}

fn invalid_identity(value: &str) -> bool {
    value.trim().is_empty() || value.chars().any(char::is_whitespace)
}
