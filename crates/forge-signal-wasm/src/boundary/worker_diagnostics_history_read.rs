use wasm_bindgen::prelude::*;

use crate::boundary::serde::to_js;
use crate::runtime::worker_host::{
    WorkerDiagnosticsHistoryReadPacket, WorkerDiagnosticsSummaryReadCertificationPackage,
    WorkerDiagnosticsSummaryReadPacket,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = readDiagnosticsSummary)]
    pub fn read_diagnostics_summary(&self) -> Result<JsValue, JsValue> {
        let packet = self.read_diagnostics_summary_for_test()?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = readDiagnosticsHistory)]
    pub fn read_diagnostics_history(&self) -> Result<JsValue, JsValue> {
        let packet = self.read_diagnostics_history_for_test()?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerDiagnosticsSummaryRead)]
    pub fn certify_worker_diagnostics_summary_read(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_diagnostics_summary_read_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn read_diagnostics_summary_for_test(
        &self,
    ) -> Result<WorkerDiagnosticsSummaryReadPacket, JsValue> {
        self.shell
            .borrow_mut()
            .read_diagnostics_summary()
            .map_err(JsValue::from)
    }

    pub(crate) fn read_diagnostics_history_for_test(
        &self,
    ) -> Result<WorkerDiagnosticsHistoryReadPacket, JsValue> {
        self.shell
            .borrow_mut()
            .read_diagnostics_history()
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_diagnostics_summary_read_for_test(
        &self,
    ) -> Result<WorkerDiagnosticsSummaryReadCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_diagnostics_summary_read()
            .map_err(JsValue::from)
    }
}
