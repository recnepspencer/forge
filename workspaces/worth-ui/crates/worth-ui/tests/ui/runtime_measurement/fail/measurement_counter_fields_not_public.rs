use worth_ui::facade::{WorthUiCounterValueKind, WorthUiFrameCostCounter};

fn main() {
    let _counter = WorthUiFrameCostCounter {
        name: "frame.fake",
        value: 1,
        value_kind: WorthUiCounterValueKind::CountedWork,
        work_class: forge_foundational::FoundationalPerformanceWorkClass::ValidationPlanning,
    };
}
