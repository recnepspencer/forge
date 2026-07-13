use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use worth_signal::facade::ChangedRegion;

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeSpec, SourceSpec,
    TransactionOp,
};

use super::types::{SignalAdapters, SignalApp, SignalDiagnostics, SignalHistory, SignalSpecialist};

#[wasm_bindgen]
impl SignalApp {
    pub fn source(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: SourceSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source(spec)
            .map_err(JsValue::from)
    }

    pub fn recipe(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: RecipeSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_recipe(spec)
            .map_err(JsValue::from)
    }

    pub fn source_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedSourceFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source_family(spec)
            .map_err(JsValue::from)
    }

    pub fn recipe_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedRecipeFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_keyed_recipe_family(spec)
            .map_err(JsValue::from)
    }

    pub fn batch(&self, ops: JsValue) -> Result<JsValue, JsValue> {
        let ops: Vec<TransactionOp> = from_js(ops)?;
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn transaction_with_packed_grid_rgba(
        &self,
        prefix_ops: JsValue,
        family_id: String,
        width: u32,
        height: u32,
        rgba: JsValue,
        suffix_ops: JsValue,
    ) -> Result<JsValue, JsValue> {
        let mut ops: Vec<TransactionOp> = from_js(prefix_ops)?;
        let rgba = Uint8Array::new(&rgba).to_vec();
        ops.push(TransactionOp::SetPackedGridRgba {
            family_id,
            width,
            height,
            rgba,
        });
        let suffix_ops: Vec<TransactionOp> = from_js(suffix_ops)?;
        ops.extend(suffix_ops);
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

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

    pub fn read_keyed(&self, family_id: String, key: String) -> Result<JsValue, JsValue> {
        let value = {
            let mut core = self.core.borrow_mut();
            let value = core
                .read_keyed_value(&family_id, &key)
                .map_err(JsValue::from)?;
            core.note_compatibility_read(1);
            value
        };
        to_js(&value).map_err(JsValue::from)
    }

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

    pub fn read_keyed_many(&self, family_id: String, keys: JsValue) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let values = {
            let mut core = self.core.borrow_mut();
            let values = core
                .read_keyed_values(&family_id, keys.clone())
                .map_err(JsValue::from)?;
            core.note_compatibility_read(keys.len());
            values
        };
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_many_packed_fields(
        &self,
        family_id: String,
        keys: JsValue,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let fields: Vec<String> = from_js(fields)?;
        let values = {
            let mut core = self.core.borrow_mut();
            let values = core
                .read_keyed_values_packed_fields(&family_id, keys.clone(), fields)
                .map_err(JsValue::from)?;
            core.note_compatibility_read(keys.len());
            values
        };
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_grid_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = {
            let mut core = self.core.borrow_mut();
            let values = core
                .read_keyed_grid_packed_fields(&family_id, columns, rows, fields)
                .map_err(JsValue::from)?;
            core.note_compatibility_read((columns as usize).saturating_mul(rows as usize));
            values
        };
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_rect_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        row: u32,
        start_column: u32,
        width: u32,
        height: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = {
            let mut core = self.core.borrow_mut();
            let values = core
                .read_keyed_rect_packed_fields(
                    &family_id,
                    columns,
                    rows,
                    row,
                    start_column,
                    width,
                    height,
                    fields,
                )
                .map_err(JsValue::from)?;
            core.note_compatibility_read((width as usize).saturating_mul(height as usize));
            values
        };
        to_js(&values).map_err(JsValue::from)
    }

    pub fn prewarm_keyed_grid(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .prewarm_keyed_grid(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn seed_keyed_grid_coords(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .seed_keyed_grid_coords(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn take_debug_events(&self) -> Result<JsValue, JsValue> {
        let events = self.core.borrow_mut().take_debug_events();
        to_js(&events).map_err(JsValue::from)
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

#[cfg(test)]
impl SignalApp {
    pub(super) fn read_for_test(
        &self,
        id: &str,
    ) -> Result<crate::expression::model::SignalValue, crate::boundary::errors::WorthSignalJsError>
    {
        let value = self.core.borrow_mut().read_value(id)?;
        {
            let mut core = self.core.borrow_mut();
            core.note_compatibility_read(1);
            core.note_compatibility_signal_serialization(id, &value);
        }
        Ok(value)
    }
}
