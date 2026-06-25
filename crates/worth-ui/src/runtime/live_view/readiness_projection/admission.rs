use super::super::expression::{
    lower_live_view_expression_output, readiness_expression_declaration,
};
use crate::runtime::{
    WorthUiLiveViewConditionalProjectionReceipt, WorthUiLiveViewDeclarationReceipt,
    WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::{
    WorthUiLiveViewReadinessProjectionDeclaration, WorthUiLiveViewReadinessProjectionDenial,
    WorthUiLiveViewReadinessProjectionReceipt, WorthUiLiveViewValuePresenceReceipt,
};

pub(crate) fn readiness_denials(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewReadinessProjectionDeclaration],
) -> Vec<WorthUiLiveViewReadinessProjectionDenial> {
    let mut denials = Vec::new();
    for declaration in declarations {
        if invalid_identity(declaration.readiness_id()) {
            denials.push(
                WorthUiLiveViewReadinessProjectionDenial::InvalidReadinessId {
                    readiness_id: declaration.readiness_id().to_owned(),
                },
            );
        }
        if declaration.required_bindings().is_empty() {
            denials.push(WorthUiLiveViewReadinessProjectionDenial::EmptyRequiredSet {
                readiness_id: declaration.readiness_id().to_owned(),
            });
        }
        for binding_id in declaration.required_bindings() {
            if live_view.binding(binding_id).is_none() {
                denials.push(
                    WorthUiLiveViewReadinessProjectionDenial::UnknownRequiredBinding {
                        readiness_id: declaration.readiness_id().to_owned(),
                        binding_id: binding_id.to_owned(),
                    },
                );
            }
        }
    }
    denials
}

pub(crate) fn lower_live_view_readiness_receipts(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    conditionals: &[WorthUiLiveViewConditionalProjectionReceipt],
    declarations: &[WorthUiLiveViewReadinessProjectionDeclaration],
) -> Vec<WorthUiLiveViewReadinessProjectionReceipt> {
    declarations
        .iter()
        .map(|declaration| {
            let presence = declaration
                .required_bindings()
                .iter()
                .map(|binding_id| {
                    let binding = live_view
                        .binding(binding_id)
                        .expect("readiness binding was admitted before lowering")
                        .clone();
                    let participates = binding_participates(conditionals, binding_id);
                    WorthUiLiveViewValuePresenceReceipt::new(
                        binding.clone(),
                        runtime.live_view_state_value(&binding),
                        participates,
                    )
                })
                .collect::<Vec<_>>();
            let mut dependency_facts = readiness_dependency_facts(live_view, declaration);
            let expression_declaration =
                readiness_expression_declaration(live_view.live_view_id(), declaration);
            let expression_projection = lower_live_view_expression_output(
                runtime,
                live_view,
                &expression_declaration,
                dependency_facts.clone(),
            )
            .expect("readiness expression was admitted before lowering");
            dependency_facts.push(expression_projection.output_fact().clone());
            dependency_facts.sort();
            dependency_facts.dedup();
            let graph_execution = runtime
                .graph_authority()
                .plan_live_view_readiness_projection_graph_operation(
                    live_view.live_view_id(),
                    declaration.readiness_id(),
                    dependency_facts.clone(),
                )
                .into_execution_receipt();
            WorthUiLiveViewReadinessProjectionReceipt::new(
                live_view.live_view_id(),
                live_view.target_binding().clone(),
                declaration,
                presence,
                expression_projection,
                dependency_facts,
                graph_execution,
            )
        })
        .collect()
}

fn binding_participates(
    conditionals: &[WorthUiLiveViewConditionalProjectionReceipt],
    binding_id: &str,
) -> bool {
    conditionals
        .iter()
        .find(|conditional| conditional.control().binding().binding_id() == binding_id)
        .map(|conditional| conditional.participation().participates_in_events())
        .unwrap_or(true)
}

fn readiness_dependency_facts(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewReadinessProjectionDeclaration,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::user_intent_target_binding(live_view.target_binding().slot_name()),
        WorthUiRuntimeFactId::surface_mount(live_view.target_binding().surface_id()),
        WorthUiRuntimeFactId::component(live_view.target_binding().component_id()),
        WorthUiRuntimeFactId::live_view_readiness_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            declaration.readiness_id()
        )),
    ];
    for binding_id in declaration.required_bindings() {
        if let Some(binding) = live_view.binding(binding_id) {
            facts.push(WorthUiRuntimeFactId::live_view_state_binding(format!(
                "{}:{}",
                live_view.live_view_id(),
                binding.binding_id()
            )));
            facts.push(WorthUiRuntimeFactId::live_view_state_value(
                binding.state_fact().as_str(),
            ));
        }
    }
    facts
}

fn invalid_identity(value: &str) -> bool {
    value.trim().is_empty() || value.chars().any(char::is_whitespace)
}
