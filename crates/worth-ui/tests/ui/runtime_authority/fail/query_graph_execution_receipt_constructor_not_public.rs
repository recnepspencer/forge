use worth_ui::facade::{
    WorthUiQueryGraphExecutionReceipt, WorthUiQueryGraphOperatingWorld,
    WorthUiQueryGraphTouchDescriptor, WorthUiRuntimeFactId,
};

fn main() {
    let touch = WorthUiQueryGraphTouchDescriptor::primitive_construction(
        "worth.surface.preview.primitive.proof",
        Vec::<WorthUiRuntimeFactId>::new(),
    );
    let _receipt = WorthUiQueryGraphExecutionReceipt::primitive_construction(
        touch,
        WorthUiQueryGraphOperatingWorld::runtime_preview(),
    );
}
