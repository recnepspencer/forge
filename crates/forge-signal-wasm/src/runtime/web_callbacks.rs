use std::cell::RefCell;
use std::collections::BTreeMap;
#[cfg(test)]
use std::panic::{catch_unwind, AssertUnwindSafe};

use forge_signal::facade::runtime::{ObservationNotice, ObservationPolicy, ObservationReadContext};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::boundary::serde::to_js;
#[cfg(test)]
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebObservationNotice {
    pub observer_id: u64,
    pub handle_id: u64,
    pub signal_id: String,
    pub branch_id: u64,
    pub policy: ObservationPolicy,
    pub touched: bool,
    pub recomputed: bool,
    pub meaningful_change: bool,
    pub trigger_matched: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebCallbackStats {
    pub active_callback_count: u64,
    pub js_callback_invocation_count: u64,
    pub js_callback_failure_count: u64,
}

#[derive(Clone)]
enum RegisteredWebCallback {
    WasmWatch(Function),
    WasmEffect(Function),
    #[cfg(test)]
    NativeWatch(Rc<dyn Fn(WebObservationNotice)>),
    #[cfg(test)]
    NativeEffect(Rc<dyn Fn()>),
}

#[derive(Default)]
struct WebCallbackRegistry {
    next_id: u64,
    callbacks: BTreeMap<u64, RegisteredWebCallback>,
    js_callback_invocation_count: u64,
    js_callback_failure_count: u64,
}

thread_local! {
    static WEB_CALLBACKS: RefCell<WebCallbackRegistry> = RefCell::new(WebCallbackRegistry {
        next_id: 1,
        callbacks: BTreeMap::new(),
        js_callback_invocation_count: 0,
        js_callback_failure_count: 0,
    });
}

fn register_callback(callback: RegisteredWebCallback) -> u64 {
    WEB_CALLBACKS.with(|registry| {
        let mut borrowed = registry.borrow_mut();
        let id = borrowed.next_id;
        borrowed.next_id = borrowed.next_id.saturating_add(1);
        borrowed.callbacks.insert(id, callback);
        id
    })
}

pub fn register_wasm_watch(callback: Function) -> u64 {
    register_callback(RegisteredWebCallback::WasmWatch(callback))
}

pub fn register_wasm_effect(callback: Function) -> u64 {
    register_callback(RegisteredWebCallback::WasmEffect(callback))
}

pub fn remove_callback(callback_id: u64) -> bool {
    WEB_CALLBACKS.with(|registry| {
        registry
            .borrow_mut()
            .callbacks
            .remove(&callback_id)
            .is_some()
    })
}

fn record_callback_invocation(success: bool) {
    WEB_CALLBACKS.with(|registry| {
        let mut borrowed = registry.borrow_mut();
        borrowed.js_callback_invocation_count =
            borrowed.js_callback_invocation_count.saturating_add(1);
        if !success {
            borrowed.js_callback_failure_count =
                borrowed.js_callback_failure_count.saturating_add(1);
        }
    });
}

fn invoke_wasm_watch(callback: &Function, notice: &WebObservationNotice) {
    if let Ok(payload) = to_js(notice) {
        record_callback_invocation(callback.call1(&JsValue::NULL, &payload).is_ok());
    }
}

fn invoke_wasm_effect(callback: &Function) {
    record_callback_invocation(callback.call0(&JsValue::NULL).is_ok());
}

pub fn invoke_watch(callback_id: u64, notice: WebObservationNotice) {
    let callback =
        WEB_CALLBACKS.with(|registry| registry.borrow().callbacks.get(&callback_id).cloned());
    let Some(callback) = callback else {
        return;
    };
    match callback {
        RegisteredWebCallback::WasmWatch(function) => invoke_wasm_watch(&function, &notice),
        #[cfg(test)]
        RegisteredWebCallback::NativeWatch(callback) => {
            record_callback_invocation(
                catch_unwind(AssertUnwindSafe(|| callback(notice))).is_ok(),
            );
        }
        RegisteredWebCallback::WasmEffect(_) => {}
        #[cfg(test)]
        RegisteredWebCallback::NativeEffect(_) => {}
    }
}

pub fn invoke_effect(callback_id: u64) {
    let callback =
        WEB_CALLBACKS.with(|registry| registry.borrow().callbacks.get(&callback_id).cloned());
    let Some(callback) = callback else {
        return;
    };
    match callback {
        RegisteredWebCallback::WasmEffect(function) => invoke_wasm_effect(&function),
        #[cfg(test)]
        RegisteredWebCallback::NativeEffect(callback) => {
            record_callback_invocation(catch_unwind(AssertUnwindSafe(|| callback())).is_ok());
        }
        RegisteredWebCallback::WasmWatch(_) => {}
        #[cfg(test)]
        RegisteredWebCallback::NativeWatch(_) => {}
    }
}

pub fn notice_from_runtime(
    signal_id: &str,
    ctx: ObservationReadContext<'_, (), (), (), crate::runtime::core::SharedStore, ()>,
    notice: &ObservationNotice<'_>,
) -> WebObservationNotice {
    WebObservationNotice {
        observer_id: notice.observer_id().get(),
        handle_id: notice.handle_id().get(),
        signal_id: signal_id.to_owned(),
        branch_id: ctx.current_branch().id.0,
        policy: notice.policy(),
        touched: notice.touched(),
        recomputed: notice.recomputed(),
        meaningful_change: notice.meaningful_change(),
        trigger_matched: notice.trigger_matched(),
    }
}

pub fn callback_stats() -> WebCallbackStats {
    WEB_CALLBACKS.with(|registry| {
        let borrowed = registry.borrow();
        WebCallbackStats {
            active_callback_count: borrowed.callbacks.len() as u64,
            js_callback_invocation_count: borrowed.js_callback_invocation_count,
            js_callback_failure_count: borrowed.js_callback_failure_count,
        }
    })
}

#[cfg(test)]
pub fn register_native_watch(callback: Box<dyn Fn(WebObservationNotice)>) -> u64 {
    register_callback(RegisteredWebCallback::NativeWatch(Rc::from(callback)))
}

#[cfg(test)]
pub fn register_native_effect(callback: Box<dyn Fn()>) -> u64 {
    register_callback(RegisteredWebCallback::NativeEffect(Rc::from(callback)))
}
