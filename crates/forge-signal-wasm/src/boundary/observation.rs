use js_sys::Function;
use wasm_bindgen::prelude::*;

use crate::runtime::web_callbacks;

use super::signals::signal_id_from_js;
use super::types::{DisposableHandle, Signals};

#[wasm_bindgen]
impl Signals {
    pub fn watch(&self, target: JsValue, callback: Function) -> Result<DisposableHandle, JsValue> {
        let signal_id = signal_id_from_js(&target)?;
        let callback_id = web_callbacks::register_wasm_watch(callback);
        match self.core.borrow_mut().watch_signal(&signal_id, callback_id) {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_id: Some(callback_id),
            }),
            Err(err) => {
                let _ = web_callbacks::remove_callback(callback_id);
                Err(JsValue::from(err))
            }
        }
    }

    pub fn effect(&self, target: JsValue, callback: Function) -> Result<DisposableHandle, JsValue> {
        let signal_id = signal_id_from_js(&target)?;
        let callback_id = web_callbacks::register_wasm_effect(callback);
        match self
            .core
            .borrow_mut()
            .effect_signal(&signal_id, callback_id)
        {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_id: Some(callback_id),
            }),
            Err(err) => {
                let _ = web_callbacks::remove_callback(callback_id);
                Err(JsValue::from(err))
            }
        }
    }

    pub fn nuke(&self, mut handle: DisposableHandle) -> bool {
        if !std::rc::Rc::ptr_eq(&self.core, &handle.core) {
            return false;
        }
        let unsubscribed = handle
            .observation_handle
            .take()
            .is_some_and(|observation_handle| {
                self.core.borrow_mut().unobserve_handle(observation_handle)
            });
        let callback_removed = handle
            .callback_id
            .take()
            .is_some_and(web_callbacks::remove_callback);
        unsubscribed || callback_removed
    }
}

impl Drop for DisposableHandle {
    fn drop(&mut self) {
        if let Some(observation_handle) = self.observation_handle.take() {
            let _ = self.core.borrow_mut().unobserve_handle(observation_handle);
        }
        if let Some(callback_id) = self.callback_id.take() {
            let _ = web_callbacks::remove_callback(callback_id);
        }
    }
}

#[cfg(test)]
impl Signals {
    pub(super) fn watch_for_test<F>(
        &self,
        signal_id: &str,
        callback: F,
    ) -> Result<DisposableHandle, crate::boundary::errors::ForgeSignalJsError>
    where
        F: Fn(web_callbacks::WebObservationNotice) + 'static,
    {
        let callback_id = web_callbacks::register_native_watch(Box::new(callback));
        match self.core.borrow_mut().watch_signal(signal_id, callback_id) {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_id: Some(callback_id),
            }),
            Err(err) => {
                let _ = web_callbacks::remove_callback(callback_id);
                Err(err)
            }
        }
    }

    pub(super) fn effect_for_test<F>(
        &self,
        signal_id: &str,
        callback: F,
    ) -> Result<DisposableHandle, crate::boundary::errors::ForgeSignalJsError>
    where
        F: Fn() + 'static,
    {
        let callback_id = web_callbacks::register_native_effect(Box::new(callback));
        match self.core.borrow_mut().effect_signal(signal_id, callback_id) {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_id: Some(callback_id),
            }),
            Err(err) => {
                let _ = web_callbacks::remove_callback(callback_id);
                Err(err)
            }
        }
    }
}
