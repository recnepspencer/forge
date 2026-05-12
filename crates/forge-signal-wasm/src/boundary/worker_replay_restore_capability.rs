use forge_signal::facade::history::{RuntimeBranch, RuntimeSnapshot};
use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::worker_host::{
    WorkerBranchTruthEnvelope, WorkerReplayCheckpointRetainedHistoryCertificationPackage,
    WorkerReplayCheckpointRetainedHistoryReport, WorkerReplayRestoreCapabilityCertificationPackage,
    WorkerReplayRestoreCapabilityReport,
};

use super::types::SignalWorkerRuntime;

#[wasm_bindgen]
impl SignalWorkerRuntime {
    #[wasm_bindgen(js_name = createWorkerBranch)]
    pub fn create_worker_branch(&self, name: String) -> Result<JsValue, JsValue> {
        let branch = self.create_worker_branch_for_test(name)?;
        to_js(&branch).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = switchWorkerBranch)]
    pub fn switch_worker_branch(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let branch = self.switch_worker_branch_for_test(branch_id)?;
        to_js(&branch).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = workerBranchSnapshot)]
    pub fn worker_branch_snapshot(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let snapshot = self.worker_branch_snapshot_for_test(branch_id)?;
        to_js(&snapshot).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = restoreBranchSnapshotWithCapabilityReport)]
    pub fn restore_branch_snapshot_with_capability_report(
        &self,
        branch_id: u64,
        snapshot: JsValue,
    ) -> Result<JsValue, JsValue> {
        let snapshot: RuntimeSnapshot = from_js(snapshot).map_err(JsValue::from)?;
        let report =
            self.restore_branch_snapshot_with_capability_report_for_test(branch_id, snapshot)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerReplayRestoreCapability)]
    pub fn certify_worker_replay_restore_capability(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_replay_restore_capability_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = recordWorkerReplayCheckpointRetainedHistory)]
    pub fn record_worker_replay_checkpoint_retained_history(
        &self,
        branch_id: u64,
        checkpoint: JsValue,
    ) -> Result<JsValue, JsValue> {
        let checkpoint: RuntimeSnapshot = from_js(checkpoint).map_err(JsValue::from)?;
        let report =
            self.record_worker_replay_checkpoint_retained_history_for_test(branch_id, checkpoint)?;
        to_js(&report).map_err(JsValue::from)
    }

    #[wasm_bindgen(js_name = certifyWorkerReplayCheckpointRetainedHistory)]
    pub fn certify_worker_replay_checkpoint_retained_history(&self) -> Result<JsValue, JsValue> {
        let package = self.certify_worker_replay_checkpoint_retained_history_for_test()?;
        to_js(&package).map_err(JsValue::from)
    }
}

impl SignalWorkerRuntime {
    pub(crate) fn create_worker_branch_for_test(
        &self,
        name: String,
    ) -> Result<RuntimeBranch, JsValue> {
        self.shell
            .borrow_mut()
            .create_branch(name)
            .map_err(JsValue::from)
    }

    pub(crate) fn switch_worker_branch_for_test(
        &self,
        branch_id: u64,
    ) -> Result<WorkerBranchTruthEnvelope, JsValue> {
        self.shell
            .borrow_mut()
            .switch_branch(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn worker_branch_snapshot_for_test(
        &self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, JsValue> {
        self.shell
            .borrow_mut()
            .branch_snapshot(branch_id)
            .map_err(JsValue::from)
    }

    pub(crate) fn restore_branch_snapshot_with_capability_report_for_test(
        &self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<WorkerReplayRestoreCapabilityReport, JsValue> {
        self.shell
            .borrow_mut()
            .restore_branch_snapshot_with_capability_report(branch_id, snapshot)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_replay_restore_capability_for_test(
        &self,
    ) -> Result<WorkerReplayRestoreCapabilityCertificationPackage, JsValue> {
        self.shell
            .borrow_mut()
            .certify_worker_replay_restore_capability()
            .map_err(JsValue::from)
    }

    pub(crate) fn record_worker_replay_checkpoint_retained_history_for_test(
        &self,
        branch_id: u64,
        checkpoint: RuntimeSnapshot,
    ) -> Result<WorkerReplayCheckpointRetainedHistoryReport, JsValue> {
        self.shell
            .borrow_mut()
            .record_worker_replay_checkpoint_retained_history(branch_id, checkpoint)
            .map_err(JsValue::from)
    }

    pub(crate) fn certify_worker_replay_checkpoint_retained_history_for_test(
        &self,
    ) -> Result<WorkerReplayCheckpointRetainedHistoryCertificationPackage, JsValue> {
        self.shell
            .borrow_mut()
            .certify_worker_replay_checkpoint_retained_history()
            .map_err(JsValue::from)
    }
}
