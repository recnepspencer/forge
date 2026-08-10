use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::KeyedSetValue;

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
    pub fn set_keyed(
        &self,
        family_id: String,
        key: String,
        value: JsValue,
    ) -> Result<JsValue, JsValue> {
        let value = from_js(value)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_value(&family_id, &key, value)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = setKeyedWithAspects)]
    pub fn set_keyed_with_aspects(
        &self,
        family_id: String,
        key: String,
        value: JsValue,
        aspects: JsValue,
    ) -> Result<JsValue, JsValue> {
        let value = from_js(value)?;
        let aspects = from_js(aspects)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_value_with_aspects(&family_id, &key, value, aspects)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn set_keyed_many(&self, family_id: String, values: JsValue) -> Result<JsValue, JsValue> {
        let values: Vec<KeyedSetValue> = from_js(values)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_values(&family_id, values)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn clear_keyed_family_cache(&self, family_id: String) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .clear_keyed_family_cache(&family_id)
            .map_err(JsValue::from)
    }
}
