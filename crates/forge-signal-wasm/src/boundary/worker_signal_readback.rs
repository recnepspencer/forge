use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::worker_host::{WorkerSignalReadbackPacket, WorkerSignalReadbackRequest};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = readSignals)]
    pub fn read_signals(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerSignalReadbackRequest = from_js(request).map_err(JsValue::from)?;
        let packet = self.read_signals_for_test(request)?;
        to_js(&packet).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn read_signals_for_test(
        &self,
        request: WorkerSignalReadbackRequest,
    ) -> Result<WorkerSignalReadbackPacket, JsValue> {
        self.shell
            .borrow_mut()
            .read_signals(request)
            .map_err(JsValue::from)
    }
}
