use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::{from_js, to_js};

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
    pub fn read(&self, id: String) -> Result<JsValue, JsValue> {
        let value = self
            .core
            .borrow_mut()
            .read_value(&id)
            .map_err(JsValue::from)?;
        {
            let mut core = self.core.borrow_mut();
            core.note_compatibility_read(1);
            core.note_compatibility_signal_serialization(&id, &value);
        }
        to_js(&value).map_err(JsValue::from)
    }

    pub fn read_many(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let values = self
            .core
            .borrow_mut()
            .read_values(ids.clone())
            .map_err(JsValue::from)?;
        {
            let mut core = self.core.borrow_mut();
            core.note_compatibility_read(ids.len());
            for (id, value) in ids.iter().zip(values.iter()) {
                core.note_compatibility_signal_serialization(id, value);
            }
        }
        to_js(&values).map_err(JsValue::from)
    }
}
