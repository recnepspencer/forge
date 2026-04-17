use forge_signal::facade::history::RuntimeSnapshot;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

use super::types::SignalHistory;

#[wasm_bindgen]
impl SignalHistory {
    pub fn replay_for(&self, id: String) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow_mut()
            .replay_for_id(&id)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn lineage_for(&self, id: String) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow_mut()
            .lineage_for_id(&id)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn snapshot(&self) -> Result<JsValue, JsValue> {
        let snapshot = self.core.borrow_mut().snapshot().map_err(JsValue::from)?;
        to_js(&snapshot).map_err(JsValue::from)
    }

    pub fn restore_snapshot(&self, snapshot: JsValue) -> Result<(), JsValue> {
        let snapshot: RuntimeSnapshotEnvelope = from_js(snapshot)?;
        self.core
            .borrow_mut()
            .restore_snapshot(snapshot)
            .map_err(JsValue::from)
    }

    pub fn current_branch(&self) -> Result<JsValue, JsValue> {
        let branch = self.core.borrow().current_branch();
        to_js(&branch).map_err(JsValue::from)
    }

    pub fn branches(&self) -> Result<JsValue, JsValue> {
        let branches = self.core.borrow().branches();
        to_js(&branches).map_err(JsValue::from)
    }

    pub fn create_branch(&self, name: String) -> Result<JsValue, JsValue> {
        let branch = self
            .core
            .borrow_mut()
            .create_branch(name)
            .map_err(JsValue::from)?;
        to_js(&branch).map_err(JsValue::from)
    }

    pub fn switch_branch(&self, branch_id: u64) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .switch_branch(branch_id)
            .map_err(JsValue::from)
    }

    pub fn replay_for_branch(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let replay = self
            .core
            .borrow_mut()
            .replay_for_branch(branch_id)
            .map_err(JsValue::from)?;
        to_js(&replay).map_err(JsValue::from)
    }

    pub fn branch_snapshot(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let snapshot = self
            .core
            .borrow_mut()
            .branch_snapshot(branch_id)
            .map_err(JsValue::from)?;
        to_js(&snapshot).map_err(JsValue::from)
    }

    pub fn branch_snapshot_id(&self, branch_id: u64) -> Result<u64, JsValue> {
        self.core
            .borrow_mut()
            .branch_snapshot_id(branch_id)
            .map_err(JsValue::from)
    }

    pub fn branch_snapshot_envelope(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let snapshot = self
            .core
            .borrow_mut()
            .branch_snapshot_envelope(branch_id)
            .map_err(JsValue::from)?;
        to_js(&snapshot).map_err(JsValue::from)
    }

    pub fn restore_branch_snapshot(
        &self,
        branch_id: u64,
        snapshot: JsValue,
    ) -> Result<(), JsValue> {
        let snapshot: RuntimeSnapshot = from_js(snapshot)?;
        self.core
            .borrow_mut()
            .restore_branch_snapshot(branch_id, snapshot)
            .map_err(JsValue::from)
    }

    pub fn restore_branch_snapshot_by_id(
        &self,
        branch_id: u64,
        snapshot_id: u64,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .restore_branch_snapshot_by_id(branch_id, snapshot_id)
            .map_err(JsValue::from)
    }

    pub fn merge_branches(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let result = self
            .core
            .borrow_mut()
            .merge_branches(source_branch_id, target_branch_id)
            .map_err(JsValue::from)?;
        to_js(&result).map_err(JsValue::from)
    }

    pub fn merge_branches_with_proof(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .merge_branches_with_proof(source_branch_id, target_branch_id)
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn plan_merge_branches(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let plan = self
            .core
            .borrow_mut()
            .plan_merge_branches(source_branch_id, target_branch_id)
            .map_err(JsValue::from)?;
        to_js(&plan).map_err(JsValue::from)
    }

    pub fn plan_merge_branches_with_proof(
        &self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .plan_merge_branches_with_proof(source_branch_id, target_branch_id)
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn plan_merge_policy_preview(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request = from_js(request)?;
        let plan = self
            .core
            .borrow_mut()
            .plan_merge_policy_preview(request)
            .map_err(JsValue::from)?;
        to_js(&plan).map_err(JsValue::from)
    }

    pub fn plan_merge_policy_preview_with_proof(
        &self,
        request: JsValue,
    ) -> Result<JsValue, JsValue> {
        let request = from_js(request)?;
        let envelope = self
            .core
            .borrow_mut()
            .plan_merge_policy_preview_with_proof(request)
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn merge_branches_policy_preview(&self, request: JsValue) -> Result<JsValue, JsValue> {
        let request = from_js(request)?;
        let result = self
            .core
            .borrow_mut()
            .merge_branches_policy_preview(request)
            .map_err(JsValue::from)?;
        to_js(&result).map_err(JsValue::from)
    }

    pub fn merge_branches_policy_preview_with_proof(
        &self,
        request: JsValue,
    ) -> Result<JsValue, JsValue> {
        let request = from_js(request)?;
        let envelope = self
            .core
            .borrow_mut()
            .merge_branches_policy_preview_with_proof(request)
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn branch_state_proof(&self, branch_id: u64) -> Result<JsValue, JsValue> {
        let proof = self
            .core
            .borrow()
            .branch_state_proof(branch_id)
            .map_err(JsValue::from)?;
        to_js(&proof).map_err(JsValue::from)
    }

    pub fn replay_parity_proof(
        &self,
        expected_branch_id: u64,
        replayed_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let proof = self
            .core
            .borrow()
            .replay_parity_proof(expected_branch_id, replayed_branch_id)
            .map_err(JsValue::from)?;
        to_js(&proof).map_err(JsValue::from)
    }

    pub fn replay_artifact_proof(
        &self,
        expected: JsValue,
        replayed_branch_id: u64,
    ) -> Result<JsValue, JsValue> {
        let expected = from_js(expected)?;
        let proof = self
            .core
            .borrow()
            .replay_artifact_proof(expected, replayed_branch_id)
            .map_err(JsValue::from)?;
        to_js(&proof).map_err(JsValue::from)
    }
}
