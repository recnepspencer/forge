use wasm_bindgen::prelude::*;

use crate::boundary::serde::to_js;
use crate::runtime::worker_host::WorkerPhase5CloseoutCertificationPackage;

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = certifyWorkerPhase5Closeout)]
    pub fn certify_worker_phase5_closeout(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_phase5_closeout_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn certify_worker_phase5_closeout_for_test(
        &self,
    ) -> Result<WorkerPhase5CloseoutCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_phase5_closeout()
            .map_err(JsValue::from)
    }
}
