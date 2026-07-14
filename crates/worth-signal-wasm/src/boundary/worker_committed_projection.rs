use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::worker_host::{
    WorkerCommittedProjectionPacket, WorkerCommittedProjectionRequest,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = applyTransactionProjection)]
    pub fn apply_transaction_projection(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerCommittedProjectionRequest = from_js(request).map_err(JsValue::from)?;
        let packet = self.apply_transaction_projection_for_test(request)?;
        to_js(&packet).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn apply_transaction_projection_for_test(
        &self,
        request: WorkerCommittedProjectionRequest,
    ) -> Result<WorkerCommittedProjectionPacket, JsValue> {
        self.shell
            .borrow_mut()
            .apply_committed_projection(request)
            .map_err(JsValue::from)
    }
}
