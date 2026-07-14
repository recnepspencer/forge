use wasm_bindgen::prelude::*;
use worth_signal::facade::ChangedRegion;

#[cfg(test)]
use crate::boundary::errors::WorthSignalJsError;
use crate::boundary::serde::from_js;
use crate::expression::model::SignalValue;
use crate::recipe::model::{SetValueWithRegions, TransactionOp, WasmAspectId};

use super::super::types::{InputSignal, SignalsTransaction};
use super::helpers::assert_same_runtime;

impl Clone for SignalsTransaction {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            ops: self.ops.clone(),
        }
    }
}

impl SignalsTransaction {
    pub(super) fn drain_ops(&self) -> Vec<TransactionOp> {
        self.ops.borrow_mut().drain(..).collect()
    }

    pub(super) fn push_set(&self, input: &InputSignal, value: SignalValue) {
        self.ops.borrow_mut().push(TransactionOp::Set {
            id: input.id.clone(),
            value,
            aspect: None,
            aspects: None,
        });
    }

    fn push_set_with_aspects(
        &self,
        input: &InputSignal,
        value: SignalValue,
        aspects: Vec<WasmAspectId>,
    ) {
        self.ops.borrow_mut().push(TransactionOp::Set {
            id: input.id.clone(),
            value,
            aspect: None,
            aspects: Some(aspects),
        });
    }

    fn push_set_with_regions(
        &self,
        input: &InputSignal,
        value: SignalValue,
        changed_regions: Vec<ChangedRegion>,
    ) {
        self.ops
            .borrow_mut()
            .push(TransactionOp::SetManyWithRegions {
                values: vec![SetValueWithRegions {
                    id: input.id.clone(),
                    value,
                    changed_regions,
                    aspect: None,
                    aspects: None,
                }],
            });
    }

    fn push_set_with_regions_and_aspects(
        &self,
        input: &InputSignal,
        value: SignalValue,
        changed_regions: Vec<ChangedRegion>,
        aspects: Vec<WasmAspectId>,
    ) {
        self.ops
            .borrow_mut()
            .push(TransactionOp::SetManyWithRegions {
                values: vec![SetValueWithRegions {
                    id: input.id.clone(),
                    value,
                    changed_regions,
                    aspect: None,
                    aspects: Some(aspects),
                }],
            });
    }
}

#[wasm_bindgen]
impl SignalsTransaction {
    pub fn set(&self, input: &InputSignal, value: JsValue) -> Result<(), JsValue> {
        assert_same_runtime(&self.core, &input.core, "input handle")?;
        let value: SignalValue = from_js(value)?;
        self.push_set(input, value);
        Ok(())
    }

    #[wasm_bindgen(js_name = setWithAspects)]
    pub fn set_with_aspects(
        &self,
        input: &InputSignal,
        value: JsValue,
        aspects: JsValue,
    ) -> Result<(), JsValue> {
        assert_same_runtime(&self.core, &input.core, "input handle")?;
        let value: SignalValue = from_js(value)?;
        let aspects: Vec<WasmAspectId> = from_js(aspects)?;
        self.push_set_with_aspects(input, value, aspects);
        Ok(())
    }

    #[wasm_bindgen(js_name = setWithRegions)]
    pub fn set_with_regions(
        &self,
        input: &InputSignal,
        value: JsValue,
        changed_regions: JsValue,
    ) -> Result<(), JsValue> {
        assert_same_runtime(&self.core, &input.core, "input handle")?;
        let value: SignalValue = from_js(value)?;
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        self.push_set_with_regions(input, value, changed_regions);
        Ok(())
    }

    #[wasm_bindgen(js_name = setWithRegionsAndAspects)]
    pub fn set_with_regions_and_aspects(
        &self,
        input: &InputSignal,
        value: JsValue,
        changed_regions: JsValue,
        aspects: JsValue,
    ) -> Result<(), JsValue> {
        assert_same_runtime(&self.core, &input.core, "input handle")?;
        let value: SignalValue = from_js(value)?;
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let aspects: Vec<WasmAspectId> = from_js(aspects)?;
        self.push_set_with_regions_and_aspects(input, value, changed_regions, aspects);
        Ok(())
    }
}

#[cfg(test)]
impl SignalsTransaction {
    pub(crate) fn set_for_test(
        &self,
        input: &InputSignal,
        value: SignalValue,
    ) -> Result<(), WorthSignalJsError> {
        super::helpers::set_for_test(self, input, value)
    }
}
