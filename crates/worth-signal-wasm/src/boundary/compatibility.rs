use wasm_bindgen::prelude::*;

use super::types::{
    SignalAdapters, SignalApp, SignalDiagnostics, SignalHistory, SignalRuntime, SignalSpecialist,
    Signals,
};

#[wasm_bindgen]
impl Signals {
    #[wasm_bindgen(js_name = compatibilityApp)]
    pub fn compatibility_app_public(&self) -> SignalApp {
        SignalApp {
            core: self.core.clone(),
        }
    }

    #[wasm_bindgen(js_name = compatibilityRuntime)]
    pub fn compatibility_runtime_public(&self) -> SignalRuntime {
        SignalRuntime {
            core: self.core.clone(),
        }
    }

    #[wasm_bindgen(js_name = diagnostics)]
    pub fn diagnostics_compat(&self) -> SignalDiagnostics {
        self.compatibility_app_public().diagnostics()
    }

    #[wasm_bindgen(js_name = history)]
    pub fn history_compat(&self) -> SignalHistory {
        self.compatibility_app_public().history()
    }

    #[wasm_bindgen(js_name = specialist)]
    pub fn specialist_compat(&self) -> SignalSpecialist {
        self.compatibility_app_public().specialist()
    }

    #[wasm_bindgen(js_name = adapters)]
    pub fn adapters_compat(&self) -> SignalAdapters {
        self.compatibility_app_public().adapters()
    }
}
