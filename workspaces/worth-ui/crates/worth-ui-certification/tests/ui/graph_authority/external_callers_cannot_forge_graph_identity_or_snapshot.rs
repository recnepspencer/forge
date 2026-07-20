use worth_ui::facade::graph::{
    UiGraphCoreIndexes, UiGraphMountedReceiptAuthoritySeedStore, UiGraphNode,
    UiGraphNodeTopology, UiGraphSnapshot, UiGraphTopology,
};

fn main() {
    let _ = std::mem::MaybeUninit::<UiGraphSnapshot>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNode>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphNodeTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphTopology>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphCoreIndexes>::uninit();
    let _ = std::mem::MaybeUninit::<UiGraphMountedReceiptAuthoritySeedStore>::uninit();
}

// graph and obligation denials share one compiler process.
mod covered_001 { include!("external_callers_cannot_mint_graph_successor_from_snapshot.rs"); }
mod covered_002 { include!("../graph_instantiation/external_callers_cannot_construct_or_substitute_graph_instantiation_plan.rs"); }
mod covered_003 { include!("../graph_touch/external_callers_cannot_forge_touch_descriptors.rs"); }
mod covered_004 { include!("../obligation_boundary/external_callers_cannot_construct_or_substitute_obligation_handoffs.rs"); }
mod covered_005 { include!("../obligation_dispatch/external_callers_cannot_mint_dispatch_plans_or_verdicts.rs"); }
mod covered_006 { include!("../obligation_selection/external_callers_cannot_mint_selected_obligation_artifacts.rs"); }
