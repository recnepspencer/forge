use wasm_bindgen::prelude::*;

use crate::runtime::core::new_shared_core;
use crate::runtime::policy::RuntimePolicySpec;

use super::types::{SignalApp, SignalRuntime, Signals};

fn new_default_core() -> Result<crate::runtime::core::SharedCore, JsValue> {
    new_shared_core(RuntimePolicySpec::default()).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = createSignals)]
pub fn create_signals() -> Result<Signals, JsValue> {
    Ok(Signals {
        core: new_default_core()?,
    })
}

#[wasm_bindgen]
impl SignalApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignalApp, JsValue> {
        Ok(Self {
            core: new_default_core()?,
        })
    }
}

#[wasm_bindgen]
impl SignalRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignalRuntime, JsValue> {
        Ok(Self {
            core: new_default_core()?,
        })
    }
}
