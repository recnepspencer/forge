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
    SignalAdapters, SignalApp, SignalDiagnostics, SignalHistory, SignalRuntime, SignalSpecialist,
};
