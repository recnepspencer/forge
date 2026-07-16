use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::core::{
    WorkerApplyTransactionToBranchRequest, WorkerCloseoutEffectBranchRequest,
    WorkerForkBranchRequest, WorkerRetireBranchRequest, WorkerRetireBranchesRequest,
};

use super::signals::flush_deferred_runtime_callbacks;
use super::types::SignalHistory;

#[wasm_bindgen]
impl SignalHistory {
    pub fn worker_branch_basis(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let basis = self
            .core
            .borrow()
            .worker_branch_basis(branch_id)
            .map_err(JsValue::from)?;
        to_js(&basis).map_err(JsValue::from)
    }

    pub fn fork_branch(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerForkBranchRequest = from_js(request).map_err(JsValue::from)?;
        let receipt = self
            .core
            .borrow_mut()
            .fork_worker_branch(request)
            .map_err(JsValue::from);
        flush_deferred_runtime_callbacks();
        let receipt = receipt?;
        to_js(&receipt).map_err(JsValue::from)
    }

    pub fn apply_transaction_to_branch(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerApplyTransactionToBranchRequest =
            from_js(request).map_err(JsValue::from)?;
        let receipt = self
            .core
            .borrow_mut()
            .apply_transaction_to_worker_branch(request)
            .map_err(JsValue::from);
        flush_deferred_runtime_callbacks();
        let receipt = receipt?;
        to_js(&receipt).map_err(JsValue::from)
    }

    pub fn retire_branch(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerRetireBranchRequest = from_js(request).map_err(JsValue::from)?;
        let receipt = self
            .core
            .borrow_mut()
            .retire_worker_branch(request)
            .map_err(JsValue::from);
        flush_deferred_runtime_callbacks();
        let receipt = receipt?;
        to_js(&receipt).map_err(JsValue::from)
    }

    pub fn retire_branches(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerRetireBranchesRequest = from_js(request).map_err(JsValue::from)?;
        let receipt = self
            .core
            .borrow_mut()
            .retire_worker_branches(request)
            .map_err(JsValue::from);
        flush_deferred_runtime_callbacks();
        let receipt = receipt?;
        to_js(&receipt).map_err(JsValue::from)
    }

    pub fn closeout_effect_branch(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request: WorkerCloseoutEffectBranchRequest = from_js(request).map_err(JsValue::from)?;
        let receipt = self
            .core
            .borrow_mut()
            .closeout_worker_effect_branch(request)
            .map_err(JsValue::from);
        flush_deferred_runtime_callbacks();
        let receipt = receipt?;
        to_js(&receipt).map_err(JsValue::from)
    }
}
