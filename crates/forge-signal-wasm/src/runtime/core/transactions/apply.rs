use forge_signal::facade::{specialist::EvaluationVerdict, SignalError};

use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::summaries::RunSummary;

use super::super::debug::{perf_now_ms, wasm_debug};
use super::super::keyed_families::set_rgba_signal_value;
use super::super::RuntimeCore;
use super::changes::SetChange;
use crate::recipe::model::TransactionOp;

impl RuntimeCore {
    pub fn mark_keyed_changed_with_regions(
        &mut self,
        family_id: &str,
        key: &str,
        changed_regions: Vec<forge_signal::facade::ChangedRegion>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let id = self.ensure_source_key(family_id, key, None)?;
        self.mark_changed_with_regions(&id, changed_regions)
    }

    pub fn mark_keyed_changed_on_aspects(
        &mut self,
        family_id: &str,
        key: &str,
        aspect_ids: Vec<crate::recipe::model::WasmAspectId>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let id = self.ensure_source_key(family_id, key, None)?;
        self.mark_changed_on_aspects(&id, aspect_ids)
    }

    pub fn apply_transaction(
        &mut self,
        ops: Vec<TransactionOp>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let started_at = perf_now_ms();
        wasm_debug(format!("[forge-signal-wasm] tx:start ops={}", ops.len()));
        let previous = self.lock_store()?.clone();
        let changes = self.collect_changes(&ops)?;
        wasm_debug(format!(
            "[forge-signal-wasm] tx:collect-done changes={} elapsed_ms={:.1}",
            changes.len(),
            perf_now_ms() - started_at
        ));
        let store = self.store.clone();
        let dense_grids = self.dense_grids.clone();
        let evaluator = self.evaluator();

        let result = self.runtime.transaction(&mut self.store, move |tx| {
            wasm_debug("[forge-signal-wasm] tx:apply-start");
            {
                let mut locked = store
                    .lock()
                    .map_err(|_| SignalError::internal("runtime store mutex poisoned"))?;
                for change in &changes {
                    match change {
                        SetChange::Source {
                            id,
                            value,
                            node,
                            changed_regions,
                            aspects,
                        } => {
                            let source = locked.sources.get_mut(id).ok_or_else(|| {
                                SignalError::invalid_input(format!("unknown source `{id}`"))
                            })?;
                            source.value = value.clone();
                            source.version =
                                super::super::aspects::bump_aspects(source.version, aspects);
                            if changed_regions.is_empty() {
                                for aspect in aspects {
                                    tx.mark_changed(*node, *aspect)?;
                                }
                            } else {
                                for aspect in aspects {
                                    tx.mark_changed_with_regions(*node, *aspect, changed_regions)?;
                                }
                            }
                        }
                        SetChange::DenseGridRgba {
                            family_id,
                            rgba,
                            aspects,
                        } => {
                            let family = dense_grids.get(family_id).ok_or_else(|| {
                                SignalError::invalid_input(format!(
                                    "unknown dense grid family `{family_id}`"
                                ))
                            })?;
                            wasm_debug(format!(
                                "[forge-signal-wasm] tx:dense-apply-start family={family_id} cells={}",
                                family.ids.len()
                            ));
                            for index in 0..family.ids.len() {
                                let offset = index * 4;
                                let source = locked.sources.get_mut(&family.ids[index]).ok_or_else(|| {
                                    SignalError::invalid_input(format!(
                                        "unknown dense source `{}`",
                                        family.ids[index]
                                    ))
                                })?;
                                set_rgba_signal_value(
                                    &mut source.value,
                                    rgba[offset],
                                    rgba[offset + 1],
                                    rgba[offset + 2],
                                    rgba[offset + 3],
                                );
                                source.version =
                                    super::super::aspects::bump_aspects(source.version, aspects);
                                for aspect in aspects {
                                    tx.mark_changed(family.nodes[index], *aspect)?;
                                }
                                if index > 0 && index % 10_000 == 0 {
                                    wasm_debug(format!(
                                        "[forge-signal-wasm] tx:dense-apply progress family={family_id} applied={index}"
                                    ));
                                }
                            }
                            wasm_debug(format!(
                                "[forge-signal-wasm] tx:dense-apply-done family={family_id}"
                            ));
                        }
                    }
                }
            }

            wasm_debug("[forge-signal-wasm] tx:evaluate-dirty-start");
            tx.evaluate_dirty(&evaluator)?;
            wasm_debug("[forge-signal-wasm] tx:evaluate-dirty-done");
            Ok(())
        });

        match result {
            Ok(result) => {
                self.apply_pending_callback_dependency_patches()?;
                let active_branch_id = self.runtime.current_branch().id.0;
                self.branch_states
                    .insert(active_branch_id, self.snapshot_branch_state());
                wasm_debug(format!(
                    "[forge-signal-wasm] tx:done touched={} evaluated={} elapsed_ms={:.1}",
                    result.touched_nodes,
                    result.evaluation_summary.nodes_evaluated,
                    perf_now_ms() - started_at
                ));
                Ok(RunSummary {
                    touched_nodes: result.touched_nodes,
                    nodes_evaluated: result.evaluation_summary.nodes_evaluated,
                    nodes_recomputed: result.evaluation_summary.nodes_recomputed,
                    nodes_suppressed: result.evaluation_summary.nodes_suppressed,
                    plans_built: result.evaluation_summary.plans_built,
                    stages_executed: result.evaluation_summary.stages_executed,
                    total_nanos: result.timing.total_nanos.to_string(),
                    evaluation_nanos: result.timing.evaluation_nanos.to_string(),
                    commit_nanos: result.timing.commit_nanos.to_string(),
                })
            }
            Err(err) => {
                wasm_debug(format!(
                    "[forge-signal-wasm] tx:error elapsed_ms={:.1} message={}",
                    perf_now_ms() - started_at,
                    err
                ));
                self.restore_store(previous)?;
                Err(ForgeSignalJsError::from(err))
            }
        }
    }

    pub fn evaluate_dirty(&mut self) -> Result<RunSummary, ForgeSignalJsError> {
        let evaluator = self.evaluator();
        let report = self
            .runtime
            .evaluate_dirty(&self.store, &evaluator)
            .map_err(ForgeSignalJsError::from)?;
        Ok(RunSummary {
            touched_nodes: report.task_count,
            nodes_evaluated: report.tasks_executed,
            nodes_recomputed: report
                .stages
                .iter()
                .flat_map(|stage| stage.task_records.iter())
                .filter(|record| matches!(record.verdict, Some(EvaluationVerdict::Recomputed)))
                .count() as u32,
            nodes_suppressed: report.tasks_with_suppressed_propagation,
            plans_built: 1,
            stages_executed: report.stage_count,
            total_nanos: (report.execution_snapshot_nanos
                + report.stage_precompute_nanos
                + report.stage_apply_nanos
                + report.semantic_finalize_nanos)
                .to_string(),
            evaluation_nanos: (report.stage_precompute_nanos + report.stage_apply_nanos)
                .to_string(),
            commit_nanos: report.semantic_finalize_nanos.to_string(),
        })
    }
}
