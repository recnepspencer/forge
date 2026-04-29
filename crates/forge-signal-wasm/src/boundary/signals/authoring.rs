use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::from_js;
use crate::expression::model::SignalValue;
use crate::recipe::model::SetValueWithRegions;
use crate::runtime::compute_callbacks;

use super::super::signals_model::{ComputedSpec, OutputSpec};
use super::super::types::{ComputedSignal, InputSignal, OutputSignal, Signals, SignalsTransaction};
use super::helpers::{apply_transaction_ops, output_callback_deferred_error, read_signal_value};

#[wasm_bindgen]
impl Signals {
    pub fn input(
        &self,
        id: String,
        initial: JsValue,
        options: Option<JsValue>,
    ) -> Result<InputSignal, JsValue> {
        let initial: SignalValue = from_js(initial)?;
        let options = options.map(from_js).transpose()?;
        self.core
            .borrow_mut()
            .define_web_input(id.clone(), initial, options)
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

    #[wasm_bindgen(js_name = computedSpec)]
    pub fn computed_spec(&self, id: String, spec: JsValue) -> Result<ComputedSignal, JsValue> {
        self.computed(id, spec)
    }

    #[wasm_bindgen(js_name = computedCallback)]
    pub fn computed_callback(
        &self,
        id: String,
        callback: Function,
    ) -> Result<ComputedSignal, JsValue> {
        let token = compute_callbacks::register_wasm_compute(callback);
        let invocation = match compute_callbacks::invoke_compute(token) {
            Ok(invocation) => invocation,
            Err(failure) => {
                let _ = compute_callbacks::dispose_compute(token);
                return Err(JsValue::from(
                    crate::boundary::errors::ForgeSignalJsError::from_compute_callback_failure(
                        failure,
                    ),
                ));
            }
        };
        self.core
            .borrow_mut()
            .install_web_computed_callback_recipe(id.clone(), token, invocation)
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

    #[wasm_bindgen(js_name = outputSpec)]
    pub fn output_spec(&self, id: String, spec: JsValue) -> Result<OutputSignal, JsValue> {
        self.output(id, spec)
    }

    #[wasm_bindgen(js_name = outputCallback)]
    pub fn output_callback(
        &self,
        id: String,
        _callback: Function,
    ) -> Result<OutputSignal, JsValue> {
        Err(JsValue::from(output_callback_deferred_error(id)))
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

#[cfg(test)]
impl Signals {
    pub(crate) fn output_callback_deferred_error_for_test(
        &self,
        id: &str,
    ) -> crate::boundary::errors::ForgeSignalJsError {
        output_callback_deferred_error(id.to_owned())
    }

    pub(crate) fn input_for_test(
        &self,
        id: &str,
        initial: SignalValue,
    ) -> Result<InputSignal, crate::boundary::errors::ForgeSignalJsError> {
        super::helpers::define_test_input(&self.core, id, initial)
    }

    pub(crate) fn computed_for_test(
        &self,
        id: &str,
        spec: ComputedSpec,
    ) -> Result<ComputedSignal, crate::boundary::errors::ForgeSignalJsError> {
        self.core
            .borrow_mut()
            .define_web_computed(id.to_owned(), spec.into_recipe(id.to_owned()))?;
        Ok(ComputedSignal {
            core: self.core.clone(),
            id: id.to_owned(),
        })
    }

    pub(crate) fn computed_spec_for_test(
        &self,
        id: &str,
        spec: ComputedSpec,
    ) -> Result<ComputedSignal, crate::boundary::errors::ForgeSignalJsError> {
        self.computed_for_test(id, spec)
    }

    pub(crate) fn output_for_test(
        &self,
        id: &str,
        spec: OutputSpec,
    ) -> Result<OutputSignal, crate::boundary::errors::ForgeSignalJsError> {
        self.core
            .borrow_mut()
            .define_web_output(id.to_owned(), spec.into_recipe(id.to_owned()))?;
        Ok(OutputSignal {
            core: self.core.clone(),
            id: id.to_owned(),
        })
    }

    pub(crate) fn output_spec_for_test(
        &self,
        id: &str,
        spec: OutputSpec,
    ) -> Result<OutputSignal, crate::boundary::errors::ForgeSignalJsError> {
        self.output_for_test(id, spec)
    }

    pub(crate) fn apply_transaction_for_test(
        &self,
        builder: &SignalsTransaction,
    ) -> Result<(), crate::boundary::errors::ForgeSignalJsError> {
        super::helpers::apply_transaction_for_test(&self.core, builder)
    }
}

#[allow(dead_code)]
fn _keep_imports_alive(
    _value: fn(&crate::runtime::core::SharedCore, &str) -> Result<JsValue, JsValue>,
) {
    let _ = read_signal_value;
    let _ = SetValueWithRegions {
        id: String::new(),
        value: SignalValue::Null,
        changed_regions: Vec::new(),
        aspect: None,
        aspects: None,
    };
}
