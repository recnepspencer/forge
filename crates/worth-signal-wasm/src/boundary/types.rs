use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use worth_signal::facade::runtime::ObservationHandle;

use crate::recipe::model::TransactionOp;
use crate::runtime::core::SharedCore;
use crate::runtime::diagnostics_callbacks::DiagnosticsCallbackToken;
use crate::runtime::web_callbacks::ObservationCallbackToken;
use crate::runtime::worker_host::WorkerRuntimeShell;

#[wasm_bindgen(js_name = Signals)]
pub struct Signals {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = InputSignal)]
pub struct InputSignal {
    pub(crate) core: SharedCore,
    pub(crate) id: String,
}

#[wasm_bindgen(js_name = ComputedSignal)]
pub struct ComputedSignal {
    pub(crate) core: SharedCore,
    pub(crate) id: String,
}

#[wasm_bindgen(js_name = OutputSignal)]
pub struct OutputSignal {
    pub(crate) core: SharedCore,
    pub(crate) id: String,
}

#[wasm_bindgen(js_name = SignalsTransaction)]
pub struct SignalsTransaction {
    pub(crate) core: SharedCore,
    pub(crate) ops: Rc<RefCell<Vec<TransactionOp>>>,
}

#[wasm_bindgen(js_name = DisposableHandle)]
pub struct DisposableHandle {
    pub(crate) core: SharedCore,
    pub(crate) observation_handle: Option<ObservationHandle>,
    pub(crate) callback_token: Option<ObservationCallbackToken>,
    pub(crate) diagnostics_callback_token: Option<DiagnosticsCallbackToken>,
}

#[wasm_bindgen(js_name = SignalApp)]
pub struct SignalApp {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalRuntime)]
pub struct SignalRuntime {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalDiagnostics)]
pub struct SignalDiagnostics {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalHistory)]
pub struct SignalHistory {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalSpecialist)]
pub struct SignalSpecialist {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalAdapters)]
pub struct SignalAdapters {
    pub(crate) core: SharedCore,
}

#[wasm_bindgen(js_name = SignalWorkerRuntime)]
pub struct SignalWorkerRuntime {
    pub(crate) shell: RefCell<WorkerRuntimeShell>,
}
