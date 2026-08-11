use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::to_js;

use super::super::types::{
    SignalAdapters, SignalDiagnostics, SignalHistory, SignalRuntime, SignalSpecialist,
};

#[wasm_bindgen]
impl SignalRuntime {
    pub fn take_debug_events(&self) -> Result<JsValue, JsValue> {
        let events = self.core.borrow_mut().take_debug_events();
        to_js(&events).map_err(JsValue::from)
    }

    pub fn diagnostics(&self) -> SignalDiagnostics {
        SignalDiagnostics {
            core: self.core.clone(),
        }
    }

    pub fn history(&self) -> SignalHistory {
        SignalHistory {
            core: self.core.clone(),
        }
    }

    pub fn specialist(&self) -> SignalSpecialist {
        SignalSpecialist {
            core: self.core.clone(),
        }
    }

    pub fn adapters(&self) -> SignalAdapters {
        SignalAdapters {
            core: self.core.clone(),
        }
    }
}
