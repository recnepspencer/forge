use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::TransactionOp;

use super::super::types::SignalRuntime;

#[wasm_bindgen]
impl SignalRuntime {
    pub fn transaction(&self, ops: JsValue) -> Result<JsValue, JsValue> {
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
}
