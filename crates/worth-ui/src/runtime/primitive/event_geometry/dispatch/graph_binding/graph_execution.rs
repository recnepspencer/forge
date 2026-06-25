use super::super::outcome_receipt::WorthUiPrimitiveEventDispatchOutcome;
use super::super::region_receipt::WorthUiPrimitiveEventRegionReceipt;
use crate::runtime::{
    WorthUiPrimitiveEventGraphDispatchPosture, WorthUiQueryGraphExecutionReceipt,
    WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

pub(in crate::runtime::primitive::event_geometry::dispatch) fn primitive_event_dispatch_execution(
    outcome: &WorthUiPrimitiveEventDispatchOutcome,
    regions: &[WorthUiPrimitiveEventRegionReceipt],
) -> WorthUiQueryGraphExecutionReceipt {
    let posture = event_graph_dispatch_posture(outcome);
    let surface_id = outcome
        .primary_surface_id()
        .unwrap_or("worth.ui.event.no-hit");
    let interaction_id = regions
        .iter()
        .find(|region| Some(region.surface_id()) == outcome.primary_surface_id())
        .map(|region| region.interaction_id())
        .unwrap_or("worth.ui.event.no-interaction");
    WorthUiRuntimeGraphAuthority::new()
        .plan_primitive_event_dispatch_graph_operation(
            surface_id,
            interaction_id,
            event_dependency_facts(regions),
            posture,
        )
        .into_execution_receipt()
}

fn event_graph_dispatch_posture(
    outcome: &WorthUiPrimitiveEventDispatchOutcome,
) -> WorthUiPrimitiveEventGraphDispatchPosture {
    match outcome {
        WorthUiPrimitiveEventDispatchOutcome::NoHit(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::NoHit
        }
        WorthUiPrimitiveEventDispatchOutcome::EnabledHit(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::EnabledHit
        }
        WorthUiPrimitiveEventDispatchOutcome::DisabledHit(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::DisabledHit
        }
        WorthUiPrimitiveEventDispatchOutcome::Bubbled(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::Bubbled
        }
        WorthUiPrimitiveEventDispatchOutcome::Captured(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::Captured
        }
        WorthUiPrimitiveEventDispatchOutcome::Denied(_) => {
            WorthUiPrimitiveEventGraphDispatchPosture::Denied
        }
    }
}

fn event_dependency_facts(
    regions: &[WorthUiPrimitiveEventRegionReceipt],
) -> Vec<WorthUiRuntimeFactId> {
    let mut facts = Vec::new();
    for region in regions {
        facts.push(region.graph_basis().produced_fact().clone());
        facts.extend(region.graph_basis().consumed_facts().iter().cloned());
    }
    facts.sort();
    facts.dedup();
    facts
}
