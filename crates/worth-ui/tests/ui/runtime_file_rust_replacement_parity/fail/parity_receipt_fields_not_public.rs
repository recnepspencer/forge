use worth_ui::facade::{
    WorthUiFileRustReplacementParityCounters, WorthUiFileRustReplacementParityReceipt,
};

fn main() {
    let _receipt = WorthUiFileRustReplacementParityReceipt {
        file_report: uninitialized_field(),
        rust_report: uninitialized_field(),
        semantic_receipt: uninitialized_field(),
        counters: WorthUiFileRustReplacementParityCounters::default(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
