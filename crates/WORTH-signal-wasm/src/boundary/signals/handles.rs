use wasm_bindgen::prelude::*;

#[cfg(test)]
use crate::boundary::errors::WORTHSignalJsError;
#[cfg(test)]
use crate::expression::model::SignalValue;

use super::super::types::{ComputedSignal, InputSignal, OutputSignal};
use super::helpers::{peek_signal_value, read_signal_value};

#[wasm_bindgen]
impl InputSignal {
    #[wasm_bindgen(getter, js_name = id)]
    pub fn id_public(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(js_name = get)]
    pub fn get_public(&self) -> Result<JsValue, JsValue> {
        read_signal_value(&self.core, &self.id)
    }

    #[wasm_bindgen(js_name = peek)]
    pub fn peek_public(&self) -> Result<JsValue, JsValue> {
        peek_signal_value(&self.core, &self.id)
    }
}

#[wasm_bindgen]
impl ComputedSignal {
    #[wasm_bindgen(getter, js_name = id)]
    pub fn id_public(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(js_name = get)]
    pub fn get_public(&self) -> Result<JsValue, JsValue> {
        read_signal_value(&self.core, &self.id)
    }

    #[wasm_bindgen(js_name = peek)]
    pub fn peek_public(&self) -> Result<JsValue, JsValue> {
        peek_signal_value(&self.core, &self.id)
    }
}

#[wasm_bindgen]
impl OutputSignal {
    #[wasm_bindgen(getter, js_name = id)]
    pub fn id_public(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(js_name = get)]
    pub fn get_public(&self) -> Result<JsValue, JsValue> {
        read_signal_value(&self.core, &self.id)
    }

    #[wasm_bindgen(js_name = peek)]
    pub fn peek_public(&self) -> Result<JsValue, JsValue> {
        peek_signal_value(&self.core, &self.id)
    }
}

#[cfg(test)]
impl InputSignal {
    pub(crate) fn read_for_test(&self) -> Result<SignalValue, WORTHSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
    }
}

#[cfg(test)]
impl ComputedSignal {
    pub(crate) fn read_for_test(&self) -> Result<SignalValue, WORTHSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
    }
}

#[cfg(test)]
impl OutputSignal {
    pub(crate) fn read_for_test(&self) -> Result<SignalValue, WORTHSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
    }
}
