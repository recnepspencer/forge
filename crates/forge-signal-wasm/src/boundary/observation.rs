use js_sys::Function;
use wasm_bindgen::prelude::*;

#[cfg(test)]
use crate::runtime::web_callbacks;

use super::signals::signal_id_from_js;
use super::types::{DisposableHandle, Signals};

#[wasm_bindgen]
impl Signals {
    pub fn watch(&self, target: JsValue, callback: Function) -> Result<DisposableHandle, JsValue> {
        let signal_id = signal_id_from_js(&target)?;
        let callback_token = self
            .core
            .borrow_mut()
            .register_wasm_watch_callback(callback);
        match self
            .core
            .borrow_mut()
            .watch_signal(&signal_id, callback_token)
        {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_token: Some(callback_token),
                diagnostics_callback_token: None,
            }),
            Err(err) => {
                let _ = self
                    .core
                    .borrow_mut()
                    .dispose_observation_callback(callback_token);
                Err(JsValue::from(err))
            }
        }
    }

    pub fn effect(&self, target: JsValue, callback: Function) -> Result<DisposableHandle, JsValue> {
        let signal_id = signal_id_from_js(&target)?;
        let callback_token = self
            .core
            .borrow_mut()
            .register_wasm_effect_callback(callback);
        match self
            .core
            .borrow_mut()
            .effect_signal(&signal_id, callback_token)
        {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_token: Some(callback_token),
                diagnostics_callback_token: None,
            }),
            Err(err) => {
                let _ = self
                    .core
                    .borrow_mut()
                    .dispose_observation_callback(callback_token);
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
        let callback_removed = handle.callback_token.take().is_some_and(|callback_token| {
            self.core
                .borrow_mut()
                .dispose_observation_callback(callback_token)
        });
        let diagnostics_removed =
            handle
                .diagnostics_callback_token
                .take()
                .is_some_and(|callback_token| {
                    self.core
                        .borrow_mut()
                        .dispose_diagnostics_callback(callback_token)
                });
        unsubscribed || callback_removed || diagnostics_removed
    }
}

impl Drop for DisposableHandle {
    fn drop(&mut self) {
        if let Some(observation_handle) = self.observation_handle.take() {
            let _ = self.core.borrow_mut().unobserve_handle(observation_handle);
        }
        if let Some(callback_token) = self.callback_token.take() {
            let _ = self
                .core
                .borrow_mut()
                .dispose_observation_callback(callback_token);
        }
        if let Some(callback_token) = self.diagnostics_callback_token.take() {
            let _ = self
                .core
                .borrow_mut()
                .dispose_diagnostics_callback(callback_token);
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
        let callback_token = self
            .core
            .borrow_mut()
            .register_native_watch_callback(Box::new(callback));
        match self
            .core
            .borrow_mut()
            .watch_signal(signal_id, callback_token)
        {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_token: Some(callback_token),
                diagnostics_callback_token: None,
            }),
            Err(err) => {
                let _ = self
                    .core
                    .borrow_mut()
                    .dispose_observation_callback(callback_token);
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
        let callback_token = self
            .core
            .borrow_mut()
            .register_native_effect_callback(Box::new(callback));
        match self
            .core
            .borrow_mut()
            .effect_signal(signal_id, callback_token)
        {
            Ok(observation_handle) => Ok(DisposableHandle {
                core: self.core.clone(),
                observation_handle: Some(observation_handle),
                callback_token: Some(callback_token),
                diagnostics_callback_token: None,
            }),
            Err(err) => {
                let _ = self
                    .core
                    .borrow_mut()
                    .dispose_observation_callback(callback_token);
                Err(err)
            }
        }
    }
}
