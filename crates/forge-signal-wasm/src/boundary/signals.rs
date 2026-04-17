use std::cell::RefCell;
use std::rc::Rc;

use forge_signal::facade::ChangedRegion;
use js_sys::Function;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[cfg(test)]
use crate::boundary::errors::ForgeSignalJsError;
use crate::boundary::serde::{from_js, to_js};
use crate::expression::model::SignalValue;
use crate::recipe::model::{SetValueWithRegions, TransactionOp};

use super::signals_model::{ComputedSpec, OutputSpec};
use super::types::{ComputedSignal, InputSignal, OutputSignal, Signals, SignalsTransaction};

fn read_signal_value(
    core: &crate::runtime::core::SharedCore,
    id: &str,
) -> Result<JsValue, JsValue> {
    let value = {
        let mut borrowed = core.borrow_mut();
        let value = borrowed.read_value(id).map_err(JsValue::from)?;
        borrowed.note_app_signal_serialization(id, &value);
        value
    };
    to_js(&value).map_err(JsValue::from)
}

fn apply_transaction_ops(
    core: &crate::runtime::core::SharedCore,
    ops: Vec<TransactionOp>,
) -> Result<JsValue, JsValue> {
    let summary = core
        .borrow_mut()
        .apply_transaction(ops)
        .map_err(JsValue::from)?;
    to_js(&summary).map_err(JsValue::from)
}

fn assert_same_runtime(
    expected: &crate::runtime::core::SharedCore,
    actual: &crate::runtime::core::SharedCore,
    label: &str,
) -> Result<(), JsValue> {
    if Rc::ptr_eq(expected, actual) {
        return Ok(());
    }

    Err(JsValue::from_str(&format!(
        "{label} belongs to a different Signals runtime"
    )))
}

pub(super) fn signal_id_from_js(target: &JsValue) -> Result<String, JsValue> {
    if let Ok(id) = from_js::<String>(target.clone()) {
        return Ok(id);
    }

    let id = Reflect::get(target, &JsValue::from_str("id")).map_err(|_| {
        JsValue::from_str("watch/effect target must be a signal id or signal handle")
    })?;
    from_js(id)
        .map_err(|_| JsValue::from_str("watch/effect target must expose a string `id` property"))
}

#[wasm_bindgen]
impl Signals {
    pub fn input(&self, id: String, initial: JsValue) -> Result<InputSignal, JsValue> {
        let initial: SignalValue = from_js(initial)?;
        self.core
            .borrow_mut()
            .define_web_input(id.clone(), initial)
            .map_err(JsValue::from)?;
        Ok(InputSignal {
            core: self.core.clone(),
            id,
        })
    }

    pub fn computed(&self, id: String, spec: JsValue) -> Result<ComputedSignal, JsValue> {
        let spec: ComputedSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_web_computed(id.clone(), spec.into_recipe(id.clone()))
            .map_err(JsValue::from)?;
        Ok(ComputedSignal {
            core: self.core.clone(),
            id,
        })
    }

    pub fn output(&self, id: String, spec: JsValue) -> Result<OutputSignal, JsValue> {
        let spec: OutputSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_web_output(id.clone(), spec.into_recipe(id.clone()))
            .map_err(JsValue::from)?;
        Ok(OutputSignal {
            core: self.core.clone(),
            id,
        })
    }

    pub fn transaction(&self, callback: &Function) -> Result<JsValue, JsValue> {
        let builder = SignalsTransaction {
            core: self.core.clone(),
            ops: Rc::new(RefCell::new(Vec::new())),
        };
        callback.call1(&JsValue::NULL, &JsValue::from(builder.clone()))?;
        apply_transaction_ops(&self.core, builder.drain_ops())
    }

    pub fn batch(&self, callback: &Function) -> Result<JsValue, JsValue> {
        self.transaction(callback)
    }
}

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
}

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
}

#[cfg(test)]
impl Signals {
    pub(super) fn input_for_test(
        &self,
        id: &str,
        initial: SignalValue,
    ) -> Result<InputSignal, ForgeSignalJsError> {
        self.core
            .borrow_mut()
            .define_web_input(id.to_owned(), initial)?;
        Ok(InputSignal {
            core: self.core.clone(),
            id: id.to_owned(),
        })
    }

    pub(super) fn computed_for_test(
        &self,
        id: &str,
        spec: ComputedSpec,
    ) -> Result<ComputedSignal, ForgeSignalJsError> {
        self.core
            .borrow_mut()
            .define_web_computed(id.to_owned(), spec.into_recipe(id.to_owned()))?;
        Ok(ComputedSignal {
            core: self.core.clone(),
            id: id.to_owned(),
        })
    }

    pub(super) fn output_for_test(
        &self,
        id: &str,
        spec: OutputSpec,
    ) -> Result<OutputSignal, ForgeSignalJsError> {
        self.core
            .borrow_mut()
            .define_web_output(id.to_owned(), spec.into_recipe(id.to_owned()))?;
        Ok(OutputSignal {
            core: self.core.clone(),
            id: id.to_owned(),
        })
    }

    pub(super) fn apply_transaction_for_test(
        &self,
        builder: &SignalsTransaction,
    ) -> Result<(), ForgeSignalJsError> {
        let ops = builder.drain_ops();
        self.core.borrow_mut().apply_transaction(ops)?;
        Ok(())
    }
}

#[cfg(test)]
impl InputSignal {
    pub(super) fn read_for_test(&self) -> Result<SignalValue, ForgeSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
    }
}

#[cfg(test)]
impl ComputedSignal {
    pub(super) fn read_for_test(&self) -> Result<SignalValue, ForgeSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
    }
}

#[cfg(test)]
impl OutputSignal {
    pub(super) fn read_for_test(&self) -> Result<SignalValue, ForgeSignalJsError> {
        self.core.borrow_mut().read_value(&self.id)
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
}

impl SignalsTransaction {
    fn push_set(&self, input: &InputSignal, value: SignalValue) {
        self.ops.borrow_mut().push(TransactionOp::Set {
            id: input.id.clone(),
            value,
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
                }],
            });
    }
}

#[cfg(test)]
impl SignalsTransaction {
    pub(super) fn set_for_test(
        &self,
        input: &InputSignal,
        value: SignalValue,
    ) -> Result<(), ForgeSignalJsError> {
        if !Rc::ptr_eq(&self.core, &input.core) {
            return Err(ForgeSignalJsError::invalid_input(
                "input handle belongs to a different Signals runtime",
            ));
        }
        self.push_set(input, value);
        Ok(())
    }
}
