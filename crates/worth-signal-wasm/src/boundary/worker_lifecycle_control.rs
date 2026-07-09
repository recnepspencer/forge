use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::worker_host::{
    WorkerLifecycleControlCertificationPackage, WorkerLifecycleControlPacket,
    WorkerObservationDeliveryAttachRequest, WorkerObservationDeliveryDetachRequest,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = attachObservationDelivery)]
    pub fn attach_observation_delivery(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerObservationDeliveryAttachRequest =
            from_js(request).map_err(JsValue::from)?;
        let packet = self.attach_observation_delivery_for_test(request)?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = detachObservationDelivery)]
    pub fn detach_observation_delivery(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerObservationDeliveryDetachRequest =
            from_js(request).map_err(JsValue::from)?;
        let packet = self.detach_observation_delivery_for_test(request)?;
        to_js(&packet).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerLifecycleControl)]
    pub fn certify_worker_lifecycle_control(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_lifecycle_control_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn attach_observation_delivery_for_test(
        &self,
        request: WorkerObservationDeliveryAttachRequest,
    ) -> Result<WorkerLifecycleControlPacket, JsValue> {
        self.shell
            .borrow_mut()
            .attach_observation_delivery(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn detach_observation_delivery_for_test(
        &self,
        request: WorkerObservationDeliveryDetachRequest,
    ) -> Result<WorkerLifecycleControlPacket, JsValue> {
        self.shell
            .borrow_mut()
            .detach_observation_delivery(request)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_lifecycle_control_for_test(
        &self,
    ) -> Result<WorkerLifecycleControlCertificationPackage, JsValue> {
        self.shell
            .borrow()
            .certify_worker_lifecycle_control()
            .map_err(JsValue::from)
    }
}
