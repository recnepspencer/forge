use std::rc::Rc;

use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::boundary::errors::ForgeSignalJsError;
use crate::boundary::serde::{from_js, to_js};
#[cfg(test)]
use crate::expression::model::SignalValue;
use crate::recipe::model::TransactionOp;

#[cfg(test)]
use super::super::signals_model::InputOptions;
#[cfg(test)]
use super::super::types::{InputSignal, SignalsTransaction};

pub(super) fn read_signal_value(
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

pub(super) fn peek_signal_value(
    core: &crate::runtime::core::SharedCore,
    id: &str,
) -> Result<JsValue, JsValue> {
    let value = core.borrow().peek_value(id).map_err(JsValue::from)?;
    to_js(&value).map_err(JsValue::from)
}

pub(super) fn apply_transaction_ops(
    core: &crate::runtime::core::SharedCore,
    ops: Vec<TransactionOp>,
) -> Result<JsValue, JsValue> {
    let summary = core
        .borrow_mut()
        .apply_transaction(ops)
        .map_err(JsValue::from)?;
    to_js(&summary).map_err(JsValue::from)
}

pub(super) fn output_callback_deferred_error(id: String) -> ForgeSignalJsError {
    ForgeSignalJsError::callback_deferred(
        "outputCallbackDeferred",
        "output callback authoring is intentionally deferred; use outputSpec(...) for now",
        Some(id),
    )
}

pub(super) fn assert_same_runtime(
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

pub(crate) fn signal_id_from_js(target: &JsValue) -> Result<String, JsValue> {
    if let Ok(id) = from_js::<String>(target.clone()) {
        return Ok(id);
    }

    let id = Reflect::get(target, &JsValue::from_str("id")).map_err(|_| {
        JsValue::from_str("watch/effect target must be a signal id or signal handle")
    })?;
    from_js(id)
        .map_err(|_| JsValue::from_str("watch/effect target must expose a string `id` property"))
}

#[cfg(test)]
pub(super) fn define_test_input(
    core: &crate::runtime::core::SharedCore,
    id: &str,
    initial: SignalValue,
) -> Result<InputSignal, ForgeSignalJsError> {
    core.borrow_mut()
        .define_web_input(id.to_owned(), initial, None::<InputOptions>)?;
    Ok(InputSignal {
        core: core.clone(),
        id: id.to_owned(),
    })
}

#[cfg(test)]
pub(super) fn apply_transaction_for_test(
    core: &crate::runtime::core::SharedCore,
    builder: &SignalsTransaction,
) -> Result<(), ForgeSignalJsError> {
    let ops = builder.drain_ops();
    core.borrow_mut().apply_transaction(ops)?;
    Ok(())
}

#[cfg(test)]
pub(super) fn set_for_test(
    tx: &SignalsTransaction,
    input: &InputSignal,
    value: SignalValue,
) -> Result<(), ForgeSignalJsError> {
    if !Rc::ptr_eq(&tx.core, &input.core) {
        return Err(ForgeSignalJsError::invalid_input(
            "input handle belongs to a different Signals runtime",
        ));
    }
    tx.push_set(input, value);
    Ok(())
}
