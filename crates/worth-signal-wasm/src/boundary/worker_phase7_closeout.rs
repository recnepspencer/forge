use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::worker_host::{
    certify_worker_unavailable_compatibility_artifact, WorkerCompatibilityCertificationScenario,
    WorkerPhase7CloseoutCertificationPackage,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = certifyWorkerPhase7Closeout)]
    pub fn certify_worker_phase7_closeout(&self, scenario: JsValue) -> Result<JsValue, JsValue> {
        let scenario: WorkerCompatibilityCertificationScenario =
            from_js(scenario).map_err(JsValue::from)?;
        let package = self.certify_worker_phase7_closeout_for_test(scenario)?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerPhase7CloseoutReadiness)]
    pub fn certify_worker_phase7_closeout_readiness(
        &self,
        scenario: JsValue,
    ) -> Result<JsValue, JsValue> {
        let scenario: WorkerCompatibilityCertificationScenario =
            from_js(scenario).map_err(JsValue::from)?;
        let package = self.certify_worker_phase7_closeout_readiness_for_test(scenario)?;
        to_js(&package).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn certify_worker_phase7_closeout_for_test(
        &self,
        scenario: WorkerCompatibilityCertificationScenario,
    ) -> Result<WorkerPhase7CloseoutCertificationPackage, JsValue> {
        let worker_unavailable =
            certify_worker_unavailable_compatibility_artifact(scenario).map_err(JsValue::from)?;
        self.shell
            .borrow()
            .certify_worker_phase7_closeout(worker_unavailable)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_phase7_closeout_readiness_for_test(
        &self,
        scenario: WorkerCompatibilityCertificationScenario,
    ) -> Result<WorkerPhase7CloseoutCertificationPackage, JsValue> {
        self.certify_worker_phase7_closeout_for_test(scenario)
    }
}
