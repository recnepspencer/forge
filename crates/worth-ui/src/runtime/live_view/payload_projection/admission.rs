use super::super::expression::{lower_live_view_expression_output, payload_expression_declaration};
use crate::runtime::{WorthUiLiveViewDeclarationReceipt, WorthUiRuntimeFactId, WorthUiRuntimeHost};

use super::{
    WorthUiLiveViewPayloadProjectionDeclaration, WorthUiLiveViewPayloadProjectionDenial,
    WorthUiLiveViewPayloadProjectionReceipt,
};

pub(crate) fn payload_denials(
    declarations: &[WorthUiLiveViewPayloadProjectionDeclaration],
) -> Vec<WorthUiLiveViewPayloadProjectionDenial> {
    let mut denials = Vec::new();
    for declaration in declarations {
        if invalid_identity(declaration.payload_id()) {
            denials.push(WorthUiLiveViewPayloadProjectionDenial::InvalidPayloadId {
                payload_id: declaration.payload_id().to_owned(),
            });
        }
        if !declaration.shape().is_supported() {
            denials.push(
                WorthUiLiveViewPayloadProjectionDenial::UnsupportedPayloadShape {
                    payload_id: declaration.payload_id().to_owned(),
                    shape: declaration.shape().token().to_owned(),
                },
            );
        }
    }
    denials
}

pub(crate) fn lower_live_view_payload_projection_receipts_for_bindings<'a>(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewPayloadProjectionDeclaration],
    consumed_binding_ids: impl IntoIterator<Item = &'a str> + Clone,
) -> Vec<WorthUiLiveViewPayloadProjectionReceipt> {
    declarations
        .iter()
        .map(|declaration| {
            let consumed_ids = consumed_binding_ids.clone().into_iter().collect::<Vec<_>>();
            let mut facts = payload_dependency_facts(
                live_view,
                declaration.payload_id(),
                consumed_ids.iter().copied(),
            );
            let expression_declaration = payload_expression_declaration(
                live_view.live_view_id(),
                declaration,
                consumed_ids.iter().copied(),
            );
            let expression_projection = lower_live_view_expression_output(
                runtime,
                live_view,
                &expression_declaration,
                facts.clone(),
            )
            .expect("payload expression was admitted before lowering");
            facts.push(expression_projection.output_fact().clone());
            facts.sort();
            facts.dedup();
            let graph_execution = runtime
                .graph_authority()
                .plan_live_view_payload_projection_graph_operation(
                    live_view.live_view_id(),
                    declaration.payload_id(),
                    facts.clone(),
                )
                .into_execution_receipt();
            WorthUiLiveViewPayloadProjectionReceipt::new(
                live_view.live_view_id(),
                declaration,
                facts,
                expression_projection,
                graph_execution,
            )
        })
        .collect()
}

fn payload_dependency_facts<'a>(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    payload_id: &str,
    consumed_binding_ids: impl IntoIterator<Item = &'a str>,
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = vec![
        WorthUiRuntimeFactId::live_view_declaration(live_view.live_view_id()),
        WorthUiRuntimeFactId::live_view_payload_projection(format!(
            "{}:{}",
            live_view.live_view_id(),
            payload_id
        )),
    ];
    for binding_id in consumed_binding_ids {
        if let Some(binding) = live_view.binding(binding_id) {
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
