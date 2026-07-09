use wasm_bindgen::prelude::*;

mod boundary;
mod expression;
mod recipe;
mod runtime;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

pub use boundary::facade::{
    createSignals, WorthSignalCoreProfile, WorthSignalMaxAspects, ComputedSignal, DisposableHandle,
    InputSignal, OutputSignal, SignalAdapters, SignalApp, SignalDiagnostics, SignalHistory,
    SignalRuntime, SignalSpecialist, Signals, SignalsTransaction,
};
