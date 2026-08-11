use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::{from_js, to_js};

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
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
}
