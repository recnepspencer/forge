use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use worth_signal::facade::ChangedRegion;

use crate::boundary::serde::{from_js, to_js};

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
    pub fn mark_changed_with_regions(
        &self,
        id: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_changed_with_regions(&id, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = markChanged)]
    pub fn mark_changed(&self, id: String, aspects: JsValue) -> Result<JsValue, JsValue> {
        let aspects = from_js(aspects)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_changed_on_aspects(&id, aspects)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = markChangedWithRegionsAndAspects)]
    pub fn mark_changed_with_regions_and_aspects(
        &self,
        id: String,
        changed_regions: JsValue,
        aspects: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let aspects = from_js(aspects)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_changed_with_regions_for_aspect_ids(&id, changed_regions, aspects)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn mark_keyed_changed_with_regions(
        &self,
        family_id: String,
        key: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_keyed_changed_with_regions(&family_id, &key, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = markKeyedChanged)]
    pub fn mark_keyed_changed(
        &self,
        family_id: String,
        key: String,
        aspects: JsValue,
    ) -> Result<JsValue, JsValue> {
        let aspects = from_js(aspects)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_keyed_changed_on_aspects(&family_id, &key, aspects)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }
}
