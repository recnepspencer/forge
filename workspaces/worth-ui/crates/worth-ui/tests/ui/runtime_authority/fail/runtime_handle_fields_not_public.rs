use worth_ui::facade::{
    WorthUiHandleArenaIdentity,
    WorthUiHandleSlotGeneration,
    WorthUiPlanNodeInputFamily,
    WorthUiRuntimeHandleLocator,
    runtime::WorthUiRuntimeHandle,
};

fn main() {
    let arena_identity = WorthUiHandleArenaIdentity { value: 1 };
    let slot_generation = WorthUiHandleSlotGeneration { value: 1 };
    let locator = WorthUiRuntimeHandleLocator {
        arena_identity,
        plan_index: 0,
        slot_generation,
    };
    let _handle = WorthUiRuntimeHandle {
        family: WorthUiPlanNodeInputFamily::ComponentInvocation,
        locator,
    };
}
