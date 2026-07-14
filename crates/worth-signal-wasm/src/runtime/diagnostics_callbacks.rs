use std::cell::RefCell;
use std::collections::BTreeMap;
#[cfg(test)]
use std::panic::{catch_unwind, AssertUnwindSafe};

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[cfg(test)]
use std::rc::Rc;

thread_local! {
    static WEB_RUNTIME_DIAGNOSTICS_CALLBACKS: RefCell<WebRuntimeDiagnosticsCallbackState> =
        RefCell::new(WebRuntimeDiagnosticsCallbackState::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticsCallbackToken {
    pub slot: u64,
    pub generation: u64,
}

#[derive(Clone)]
enum RegisteredDiagnosticsCallback {
    Wasm(Function),
    #[cfg(test)]
    Native(Rc<dyn Fn()>),
}

#[derive(Default)]
struct DiagnosticsCallbackRegistry {
    next_id: u64,
    free_slots: Vec<u64>,
    callbacks: BTreeMap<u64, RegisteredDiagnosticsCallback>,
    generations: BTreeMap<u64, u64>,
}

#[derive(Default)]
struct WebRuntimeDiagnosticsCallbackState {
    next_runtime_scope_id: u64,
    registries: BTreeMap<u64, DiagnosticsCallbackRegistry>,
}

pub fn allocate_runtime_diagnostics_callback_scope() -> u64 {
    WEB_RUNTIME_DIAGNOSTICS_CALLBACKS.with(|state| {
        let mut state = state.borrow_mut();
        let scope_id = state.next_runtime_scope_id;
        state.next_runtime_scope_id = state.next_runtime_scope_id.saturating_add(1);
        state.registries.entry(scope_id).or_default();
        scope_id
    })
}

pub fn dispose_runtime_diagnostics_callback_scope(scope_id: u64) {
    WEB_RUNTIME_DIAGNOSTICS_CALLBACKS.with(|state| {
        state.borrow_mut().registries.remove(&scope_id);
    });
}

pub fn register_wasm_diagnostics_callback(
    scope_id: u64,
    callback: Function,
) -> DiagnosticsCallbackToken {
    with_registry_mut(scope_id, |registry| {
        registry.allocate_token(RegisteredDiagnosticsCallback::Wasm(callback))
    })
}

pub fn dispose_diagnostics_callback(scope_id: u64, token: DiagnosticsCallbackToken) -> bool {
    with_registry_mut(scope_id, |registry| registry.dispose_callback(token))
}

pub fn notify_diagnostics_callbacks(scope_id: u64) {
    with_registry_mut(scope_id, DiagnosticsCallbackRegistry::notify_all);
}

fn with_registry_mut<R>(scope_id: u64, f: impl FnOnce(&mut DiagnosticsCallbackRegistry) -> R) -> R {
    WEB_RUNTIME_DIAGNOSTICS_CALLBACKS.with(|state| {
        let mut state = state.borrow_mut();
        let registry = state.registries.entry(scope_id).or_default();
        f(registry)
    })
}

impl DiagnosticsCallbackRegistry {
    fn allocate_token(
        &mut self,
        callback: RegisteredDiagnosticsCallback,
    ) -> DiagnosticsCallbackToken {
        let (slot, generation) = if let Some(slot) = self.free_slots.pop() {
            let generation = self
                .generations
                .get(&slot)
                .copied()
                .unwrap_or(0)
                .saturating_add(1);
            (slot, generation)
        } else {
            let slot = self.next_id;
            self.next_id = self.next_id.saturating_add(1);
            (slot, 1)
        };
        self.callbacks.insert(slot, callback);
        self.generations.insert(slot, generation);
        DiagnosticsCallbackToken { slot, generation }
    }

    fn dispose_callback(&mut self, token: DiagnosticsCallbackToken) -> bool {
        let Some(generation) = self.generations.get(&token.slot).copied() else {
            return false;
        };
        if generation != token.generation || !self.callbacks.contains_key(&token.slot) {
            return false;
        }
        self.callbacks.remove(&token.slot);
        self.free_slots.push(token.slot);
        true
    }

    fn notify_all(&mut self) {
        let callbacks = self.callbacks.values().cloned().collect::<Vec<_>>();
        for callback in callbacks {
            match callback {
                RegisteredDiagnosticsCallback::Wasm(function) => {
                    let _ = function.call0(&JsValue::NULL);
                }
                #[cfg(test)]
                RegisteredDiagnosticsCallback::Native(callback) => {
                    let _ = catch_unwind(AssertUnwindSafe(|| callback()));
                }
            }
        }
    }
}

#[cfg(test)]
pub fn register_native_diagnostics_callback(
    scope_id: u64,
    callback: Box<dyn Fn()>,
) -> DiagnosticsCallbackToken {
    with_registry_mut(scope_id, |registry| {
        registry.allocate_token(RegisteredDiagnosticsCallback::Native(Rc::from(callback)))
    })
}
