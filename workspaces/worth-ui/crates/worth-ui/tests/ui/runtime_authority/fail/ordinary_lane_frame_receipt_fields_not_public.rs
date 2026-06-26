use worth_ui::facade::{WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameReceipt};

fn main() {
    let _receipt = WorthUiOrdinaryLaneFrameReceipt {
        target: WorthUiOrdinaryFrameTarget::root_shell(),
        touched_plan_indexes: Vec::new(),
        touched_runtime_handles: Vec::new(),
        counters: Default::default(),
        certification: todo!(),
    };
}
