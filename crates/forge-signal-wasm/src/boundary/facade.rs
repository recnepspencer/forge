use forge_signal::facade::history::RuntimeSnapshot;
use forge_signal::facade::ChangedRegion;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

use crate::boundary::serde::{from_js, to_js};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeSpec, SourceSpec,
    TransactionOp,
};
use crate::runtime::adapters::RuntimeEnvelope;
use crate::runtime::core::{new_shared_core, SharedCore};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

#[wasm_bindgen(js_name = SignalApp)]
pub struct SignalApp {
    core: SharedCore,
}

#[wasm_bindgen(js_name = SignalRuntime)]
pub struct SignalRuntime {
    core: SharedCore,
}

#[wasm_bindgen(js_name = SignalDiagnostics)]
pub struct SignalDiagnostics {
    core: SharedCore,
}

#[wasm_bindgen(js_name = SignalHistory)]
pub struct SignalHistory {
    core: SharedCore,
}

#[wasm_bindgen(js_name = SignalSpecialist)]
pub struct SignalSpecialist {
    core: SharedCore,
}

#[wasm_bindgen(js_name = SignalAdapters)]
pub struct SignalAdapters {
    core: SharedCore,
}

#[wasm_bindgen]
impl SignalApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignalApp, JsValue> {
        Ok(Self {
            core: new_shared_core(RuntimePolicySpec::default()).map_err(JsValue::from)?,
        })
    }

    pub fn source(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: SourceSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source(spec)
            .map_err(JsValue::from)
    }

    pub fn recipe(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: RecipeSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_recipe(spec)
            .map_err(JsValue::from)
    }

    pub fn source_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedSourceFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source_family(spec)
            .map_err(JsValue::from)
    }

    pub fn recipe_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedRecipeFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_keyed_recipe_family(spec)
            .map_err(JsValue::from)
    }

    pub fn batch(&self, ops: JsValue) -> Result<JsValue, JsValue> {
        let ops: Vec<TransactionOp> = from_js(ops)?;
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn transaction_with_packed_grid_rgba(
        &self,
        prefix_ops: JsValue,
        family_id: String,
        width: u32,
        height: u32,
        rgba: JsValue,
        suffix_ops: JsValue,
    ) -> Result<JsValue, JsValue> {
        let mut ops: Vec<TransactionOp> = from_js(prefix_ops)?;
        let rgba = Uint8Array::new(&rgba).to_vec();
        ops.push(TransactionOp::SetPackedGridRgba {
            family_id,
            width,
            height,
            rgba,
        });
        let suffix_ops: Vec<TransactionOp> = from_js(suffix_ops)?;
        ops.extend(suffix_ops);
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read(&self, id: String) -> Result<JsValue, JsValue> {
        let value = self
            .core
            .borrow_mut()
            .read_value(&id)
            .map_err(JsValue::from)?;
        to_js(&value).map_err(JsValue::from)
    }

    pub fn read_many(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let values = self
            .core
            .borrow_mut()
            .read_values(ids)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed(&self, family_id: String, key: String) -> Result<JsValue, JsValue> {
        let value = self
            .core
            .borrow_mut()
            .read_keyed_value(&family_id, &key)
            .map_err(JsValue::from)?;
        to_js(&value).map_err(JsValue::from)
    }

    pub fn set_keyed(
        &self,
        family_id: String,
        key: String,
        value: JsValue,
    ) -> Result<JsValue, JsValue> {
        let value = from_js(value)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_value(&family_id, &key, value)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read_keyed_many(&self, family_id: String, keys: JsValue) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_values(&family_id, keys)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_many_packed_fields(
        &self,
        family_id: String,
        keys: JsValue,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_values_packed_fields(&family_id, keys, fields)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_grid_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_grid_packed_fields(&family_id, columns, rows, fields)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_rect_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        row: u32,
        start_column: u32,
        width: u32,
        height: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_rect_packed_fields(
                &family_id,
                columns,
                rows,
                row,
                start_column,
                width,
                height,
                fields,
            )
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn prewarm_keyed_grid(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .prewarm_keyed_grid(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn seed_keyed_grid_coords(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .seed_keyed_grid_coords(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn take_debug_events(&self) -> Result<JsValue, JsValue> {
        let events = self.core.borrow_mut().take_debug_events();
        to_js(&events).map_err(JsValue::from)
    }

    pub fn set_keyed_many(&self, family_id: String, values: JsValue) -> Result<JsValue, JsValue> {
        let values: Vec<KeyedSetValue> = from_js(values)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_values(&family_id, values)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn mark_changed_with_regions(
        &self,
        id: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_changed_with_regions(&id, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn mark_keyed_changed_with_regions(
        &self,
        family_id: String,
        key: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_keyed_changed_with_regions(&family_id, &key, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn diagnostics(&self) -> SignalDiagnostics {
        SignalDiagnostics {
            core: self.core.clone(),
        }
    }

    pub fn history(&self) -> SignalHistory {
        SignalHistory {
            core: self.core.clone(),
        }
    }

    pub fn specialist(&self) -> SignalSpecialist {
        SignalSpecialist {
            core: self.core.clone(),
        }
    }

    pub fn adapters(&self) -> SignalAdapters {
        SignalAdapters {
            core: self.core.clone(),
        }
    }
}

#[wasm_bindgen]
impl SignalRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SignalRuntime, JsValue> {
        Ok(Self {
            core: new_shared_core(RuntimePolicySpec::default()).map_err(JsValue::from)?,
        })
    }

    pub fn set_runtime_policy(&self, policy: JsValue) -> Result<(), JsValue> {
        let policy: RuntimePolicySpec = from_js(policy)?;
        self.core
            .borrow_mut()
            .set_runtime_policy(policy)
            .map_err(JsValue::from)
    }

    pub fn define_source(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: SourceSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source(spec)
            .map_err(JsValue::from)
    }

    pub fn define_recipe(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: RecipeSpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_recipe(spec)
            .map_err(JsValue::from)
    }

    pub fn define_source_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedSourceFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_source_family(spec)
            .map_err(JsValue::from)
    }

    pub fn define_recipe_family(&self, spec: JsValue) -> Result<(), JsValue> {
        let spec: KeyedRecipeFamilySpec = from_js(spec)?;
        self.core
            .borrow_mut()
            .define_keyed_recipe_family(spec)
            .map_err(JsValue::from)
    }

    pub fn transaction(&self, ops: JsValue) -> Result<JsValue, JsValue> {
        let ops: Vec<TransactionOp> = from_js(ops)?;
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn transaction_with_packed_grid_rgba(
        &self,
        prefix_ops: JsValue,
        family_id: String,
        width: u32,
        height: u32,
        rgba: JsValue,
        suffix_ops: JsValue,
    ) -> Result<JsValue, JsValue> {
        let mut ops: Vec<TransactionOp> = from_js(prefix_ops)?;
        let rgba = Uint8Array::new(&rgba).to_vec();
        ops.push(TransactionOp::SetPackedGridRgba {
            family_id,
            width,
            height,
            rgba,
        });
        let suffix_ops: Vec<TransactionOp> = from_js(suffix_ops)?;
        ops.extend(suffix_ops);
        let summary = self
            .core
            .borrow_mut()
            .apply_transaction(ops)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read(&self, id: String) -> Result<JsValue, JsValue> {
        let value = self
            .core
            .borrow_mut()
            .read_value(&id)
            .map_err(JsValue::from)?;
        to_js(&value).map_err(JsValue::from)
    }

    pub fn read_many(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let values = self
            .core
            .borrow_mut()
            .read_values(ids)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed(&self, family_id: String, key: String) -> Result<JsValue, JsValue> {
        let value = self
            .core
            .borrow_mut()
            .read_keyed_value(&family_id, &key)
            .map_err(JsValue::from)?;
        to_js(&value).map_err(JsValue::from)
    }

    pub fn set_keyed(
        &self,
        family_id: String,
        key: String,
        value: JsValue,
    ) -> Result<JsValue, JsValue> {
        let value = from_js(value)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_value(&family_id, &key, value)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read_keyed_many(&self, family_id: String, keys: JsValue) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_values(&family_id, keys)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_many_packed_fields(
        &self,
        family_id: String,
        keys: JsValue,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let keys: Vec<String> = from_js(keys)?;
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_values_packed_fields(&family_id, keys, fields)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_grid_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_grid_packed_fields(&family_id, columns, rows, fields)
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn read_keyed_rect_packed_fields(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
        row: u32,
        start_column: u32,
        width: u32,
        height: u32,
        fields: JsValue,
    ) -> Result<JsValue, JsValue> {
        let fields: Vec<String> = from_js(fields)?;
        let values = self
            .core
            .borrow_mut()
            .read_keyed_rect_packed_fields(
                &family_id,
                columns,
                rows,
                row,
                start_column,
                width,
                height,
                fields,
            )
            .map_err(JsValue::from)?;
        to_js(&values).map_err(JsValue::from)
    }

    pub fn prewarm_keyed_grid(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .prewarm_keyed_grid(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn seed_keyed_grid_coords(
        &self,
        family_id: String,
        columns: u32,
        rows: u32,
    ) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .seed_keyed_grid_coords(&family_id, columns, rows)
            .map_err(JsValue::from)
    }

    pub fn take_debug_events(&self) -> Result<JsValue, JsValue> {
        let events = self.core.borrow_mut().take_debug_events();
        to_js(&events).map_err(JsValue::from)
    }

    pub fn set_keyed_many(&self, family_id: String, values: JsValue) -> Result<JsValue, JsValue> {
        let values: Vec<KeyedSetValue> = from_js(values)?;
        let summary = self
            .core
            .borrow_mut()
            .set_keyed_values(&family_id, values)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn mark_changed_with_regions(
        &self,
        id: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_changed_with_regions(&id, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn mark_keyed_changed_with_regions(
        &self,
        family_id: String,
        key: String,
        changed_regions: JsValue,
    ) -> Result<JsValue, JsValue> {
        let changed_regions: Vec<ChangedRegion> = from_js(changed_regions)?;
        let summary = self
            .core
            .borrow_mut()
            .mark_keyed_changed_with_regions(&family_id, &key, changed_regions)
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn clear_keyed_family_cache(&self, family_id: String) -> Result<(), JsValue> {
        self.core
            .borrow_mut()
            .clear_keyed_family_cache(&family_id)
            .map_err(JsValue::from)
    }

    pub fn diagnostics(&self) -> SignalDiagnostics {
        SignalDiagnostics {
            core: self.core.clone(),
        }
    }

    pub fn history(&self) -> SignalHistory {
        SignalHistory {
            core: self.core.clone(),
        }
    }

    pub fn specialist(&self) -> SignalSpecialist {
        SignalSpecialist {
            core: self.core.clone(),
        }
    }

    pub fn adapters(&self) -> SignalAdapters {
        SignalAdapters {
            core: self.core.clone(),
        }
    }
}

#[wasm_bindgen]
impl SignalDiagnostics {
    pub fn why(&self, id: String) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().why(&id).map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn health(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().health().map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn summary_now(&self) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow()
            .diagnostics_summary_now()
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn history_now(&self) -> Result<JsValue, JsValue> {
        let history = self
            .core
            .borrow()
            .execution_history_now()
            .map_err(JsValue::from)?;
        to_js(&history).map_err(JsValue::from)
    }

    pub fn latest_flow(&self) -> Result<JsValue, JsValue> {
        let flow = self.core.borrow().latest_flow().map_err(JsValue::from)?;
        to_js(&flow).map_err(JsValue::from)
    }

    pub fn latest_failure(&self) -> Result<JsValue, JsValue> {
        let failure = self.core.borrow().latest_failure().map_err(JsValue::from)?;
        to_js(&failure).map_err(JsValue::from)
    }

    pub fn latest_rollback(&self) -> Result<JsValue, JsValue> {
        let rollback = self
            .core
            .borrow()
            .latest_rollback()
            .map_err(JsValue::from)?;
        to_js(&rollback).map_err(JsValue::from)
    }

    pub fn latest_frontier_execution(&self) -> Result<JsValue, JsValue> {
        let frontier = self
            .core
            .borrow()
            .latest_frontier_execution()
            .map_err(JsValue::from)?;
        to_js(&frontier).map_err(JsValue::from)
    }

    pub fn latest_invalidation_trace_records(&self) -> Result<JsValue, JsValue> {
        let records = self
            .core
            .borrow()
            .latest_invalidation_trace_records()
            .map_err(JsValue::from)?;
        to_js(&records).map_err(JsValue::from)
    }

    pub fn recent_history(&self) -> Result<JsValue, JsValue> {
        let history = self.core.borrow().recent_history().map_err(JsValue::from)?;
        to_js(&history).map_err(JsValue::from)
    }
}

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

#[wasm_bindgen]
impl SignalSpecialist {
    pub fn graph_summary(&self) -> Result<JsValue, JsValue> {
        let summary = self.core.borrow().graph_summary().map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn evaluate_dirty(&self) -> Result<JsValue, JsValue> {
        let summary = self
            .core
            .borrow_mut()
            .evaluate_dirty()
            .map_err(JsValue::from)?;
        to_js(&summary).map_err(JsValue::from)
    }

    pub fn read_versions(&self, ids: JsValue) -> Result<JsValue, JsValue> {
        let ids: Vec<String> = from_js(ids)?;
        let versions = self
            .core
            .borrow_mut()
            .read_versions(ids)
            .map_err(JsValue::from)?;
        to_js(&versions).map_err(JsValue::from)
    }
}

#[wasm_bindgen]
impl SignalAdapters {
    pub fn export_definitions(&self) -> Result<JsValue, JsValue> {
        let definitions = self
            .core
            .borrow()
            .export_definitions()
            .map_err(JsValue::from)?;
        to_js(&definitions).map_err(JsValue::from)
    }

    pub fn export_runtime_envelope(&self) -> Result<JsValue, JsValue> {
        let envelope = self
            .core
            .borrow_mut()
            .export_runtime_envelope()
            .map_err(JsValue::from)?;
        to_js(&envelope).map_err(JsValue::from)
    }

    pub fn runtime_proof_report(&self) -> Result<JsValue, JsValue> {
        let report = self.core.borrow().runtime_proof_report();
        to_js(&report).map_err(JsValue::from)
    }

    pub fn replace_runtime_envelope(&self, envelope: JsValue) -> Result<(), JsValue> {
        let envelope: RuntimeEnvelope = from_js(envelope)?;
        self.core
            .borrow_mut()
            .replace_runtime_envelope(envelope)
            .map_err(JsValue::from)
    }
}
