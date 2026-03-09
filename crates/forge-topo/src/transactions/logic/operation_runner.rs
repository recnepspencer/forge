//! Operation execution runner for `MutableDraft`.
//!
//! DOMAIN: The `execute()` method is the single entry point for all topology
//! mutations. It handles: operator dispatch, Euler-delta verification,
//! journal cross-checks, contract-driven invariant validation, and
//! lineage/metrics bookkeeping.

use super::mutable_draft::MutableDraft;
use crate::identity::OperationCount;
use crate::operations::operator::TopoOperator;
use crate::provenance::OpSignature;
use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::OperationArtifacts;
use forge_signal::facade::{CheckpointBarrier, EventBus};

use forge_core::{KernelError, LineageDelta, OperationMetrics, OperationResult};

impl MutableDraft {
    /// Execute a topology operator through the formalized runner.
    ///
    /// This is the ONLY correct way to execute topology mutations.
    /// It handles:
    /// 1. **Logging**: Records the operation start for replay (D1)
    /// 2. **Execution**: Calls the operator's `execute()` method
    /// 3. **Euler delta verification**: Compares declared vs actual entity counts
    /// 4. **Lineage**: Updates ancestry tracking (stub — Phase 3)
    ///
    /// # Example
    /// ```ignore
    /// let mut draft = state.into_mutation();
    /// let result = draft.execute(MakeVertexFace { shell, point })?;
    /// let result2 = draft.execute(MakeEdgeVertex { face, point })?;
    /// Ok(draft.commit()?)
    /// ```
    pub fn execute<O: TopoOperator>(
        &mut self,
        op: O,
    ) -> Result<OperationResult<O::Output>, KernelError> {
        self.execute_with_event_bus::<O>(op)
    }

    fn execute_with_event_bus<O: TopoOperator>(
        &mut self,
        op: O,
    ) -> Result<OperationResult<O::Output>, KernelError> {
        let mut event_bus = std::mem::take(&mut self.event_bus);
        let result = self.execute_with_external_event_bus(op, &mut event_bus);
        self.event_bus = event_bus;
        result
    }

    fn execute_with_external_event_bus<O: TopoOperator>(
        &mut self,
        op: O,
        event_bus: &mut EventBus<TopoOperationEvent, TopoSubscriberDataId, MutableDraft>,
    ) -> Result<OperationResult<O::Output>, KernelError> {
        self.pending_operation_events.clear();
        if self.poisoned {
            return Err(KernelError::InternalError {
                message: "Draft was poisoned by a previously failed operation. Create a new MutableDraft from TopologyState.".to_string(),
                context: None,
            });
        }

        use crate::operations::operator::EulerDelta;
        use std::time::Instant;

        let start = Instant::now();

        let face_count_before = self.arena.face_count();
        let vertex_count_before = self.arena.vertex_count();
        let halfedge_count_before = self.arena.half_edge_count();
        let loop_count_before = self.arena.loop_count();
        let edge_count_before = self.arena.edge_count();
        let shell_count_before = self.arena.shell_count();
        let body_count_before = self.arena.body_count();
        let lump_count_before = self.arena.lump_count();
        let region_count_before = self.arena.region_count();

        let invocation_id = self.next_op_id();
        let mut signature = OpSignature::new(O::NAME);
        signature.set_invocation_id(invocation_id);
        let op_name = signature.get_name().to_string();
        let summary = op.semantic_summary();

        if let Err(err) = event_bus.begin(self) {
            self.poisoned = true;
            return Err(KernelError::InternalError {
                message: format!("Event bus begin failed: {err:?}"),
                context: None,
            });
        }

        self.emit_operation_event(TopoOperationEvent::OperationStarted {
            op_name: O::NAME,
            invocation_id,
            draft_id: self.draft_id(),
            schema_version: O::SCHEMA_VERSION,
            invariant_relation: O::INVARIANT_CONTRACT.relation,
            summary: summary.clone(),
        });

        tracing::debug!(
            op = ?op,
            invocation_id = invocation_id.get(),
            summary = %summary,
            "Applying topology operator"
        );
        // Reset the mutation journal before execution — ensures a clean per-op record.
        self.mutation_journal.reset();

        let mut recorder = crate::provenance::LineageRecorder::new(
            crate::provenance::OperationLineageContext {
                feature_id: crate::provenance::FEATURE_ID_SYSTEM,
                op_name: O::NAME,
                mode: crate::provenance::LineageMode::Root, // overwritten when operator calls stamp_derived
            },
            invocation_id,
        );

        let exec_result = match op.execute(self, &mut recorder) {
            Ok(result) => result,
            Err(e) => {
                let with_context =
                    e.ensure_operation_context(&op_name, invocation_id.get(), &format!("{:?}", op));
                self.emit_operation_event(TopoOperationEvent::OperationFailed {
                    invocation_id,
                    error_summary: with_context.to_string(),
                });
                self.emit_operation_event(TopoOperationEvent::DraftRolledBack {
                    draft_id: self.draft_id(),
                    ops_completed: OperationCount::new(self.op_counter.get()),
                });
                self.drain_pending_events_into(event_bus);
                event_bus.rollback(self);
                self.rollback_applied = true;
                self.poisoned = true;
                return Err(with_context);
            }
        };
        let declared_delta = exec_result.declared_delta;
        self.emit_operation_event(TopoOperationEvent::OperationCompleted {
            invocation_id,
            declared_delta,
        });

        let mutation_snapshot = self.mutation_journal.snapshot();

        // Apply cache refreshes scheduled for per-operation checkpoint.
        let cache_trace = self
            .arena_mut()
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)?;
        self.emit_operation_event(TopoOperationEvent::ReplayCacheTraceApplied {
            op_id: invocation_id,
            trace: cache_trace.into_iter().map(|t| t.encode()).collect(),
        });

        // ── Compute journal counts BEFORE draining ────────────────────
        // The journal records every insert/remove that happened during op.execute().
        // We snapshot the gross counts now, before drain_destroyed() empties the list.
        let created = self.mutation_journal.count_created();
        let deleted = self.mutation_journal.count_destroyed();

        // ── Auto-stamp deletion lineage from the journal ──────────────
        // The MutationJournal recorded every entity that was removed during
        // op.execute(). We stamp their deletion into the lineage store
        // automatically — operators never need to call stamp_deletions.
        let result = exec_result.value;

        self.emit_operation_event(TopoOperationEvent::TopologyChanged);

        // ── EulerDelta verification (net change — topological invariant) ──
        let face_count_after = self.arena.face_count();
        let vertex_count_after = self.arena.vertex_count();
        let halfedge_count_after = self.arena.half_edge_count();
        let loop_count_after = self.arena.loop_count();
        let edge_count_after = self.arena.edge_count();
        let shell_count_after = self.arena.shell_count();
        let body_count_after = self.arena.body_count();
        let lump_count_after = self.arena.lump_count();
        let region_count_after = self.arena.region_count();

        let actual_delta = EulerDelta {
            vertices: vertex_count_after as i32 - vertex_count_before as i32,
            half_edges: halfedge_count_after as i32 - halfedge_count_before as i32,
            faces: face_count_after as i32 - face_count_before as i32,
            loops: loop_count_after as i32 - loop_count_before as i32,
            edges: edge_count_after as i32 - edge_count_before as i32,
            shells: shell_count_after as i32 - shell_count_before as i32,
            solids: body_count_after as i32 - body_count_before as i32,
            lumps: lump_count_after as i32 - lump_count_before as i32,
            regions: region_count_after as i32 - region_count_before as i32,
        };
        // ── Journal–Arena cross-check (debug only) ────────────────────
        // Catches silently missing proxy hooks: if someone adds a new entity
        // type without updating the draft proxy macro, the journal's net delta
        // will disagree with the arena's objective net delta.
        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                created.vertices as i32 - deleted.vertices as i32,
                actual_delta.vertices,
                "Journal/arena vertex count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.faces as i32 - deleted.faces as i32,
                actual_delta.faces,
                "Journal/arena face count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.half_edges as i32 - deleted.half_edges as i32,
                actual_delta.half_edges,
                "Journal/arena half-edge count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.loops as i32 - deleted.loops as i32,
                actual_delta.loops,
                "Journal/arena loop count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.edges as i32 - deleted.edges as i32,
                actual_delta.edges,
                "Journal/arena edge count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.shells as i32 - deleted.shells as i32,
                actual_delta.shells,
                "Journal/arena shell count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.bodies as i32 - deleted.bodies as i32,
                actual_delta.solids,
                "Journal/arena body count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.lumps as i32 - deleted.lumps as i32,
                actual_delta.lumps,
                "Journal/arena lump count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.regions as i32 - deleted.regions as i32,
                actual_delta.regions,
                "Journal/arena region count mismatch — draft proxy hook may be missing"
            );
        }

        // ── Build LineageDelta + OperationMetrics from journal (gross counts) ──
        let lineage_delta = LineageDelta {
            faces_created: created.faces,
            faces_deleted: deleted.faces,
            half_edges_created: created.half_edges,
            half_edges_deleted: deleted.half_edges,
            vertices_created: created.vertices,
            vertices_deleted: deleted.vertices,
            loops_created: created.loops,
            loops_deleted: deleted.loops,
            edges_created: created.edges,
            edges_deleted: deleted.edges,
            shells_created: created.shells,
            shells_deleted: deleted.shells,
            solids_created: created.bodies,
            solids_deleted: deleted.bodies,
            lumps_created: created.lumps,
            lumps_deleted: deleted.lumps,
            regions_created: created.regions,
            regions_deleted: deleted.regions,
        };
        self.emit_operation_event(TopoOperationEvent::OperationArtifactsBuilt {
            created: created.clone(),
            destroyed: deleted.clone(),
            lineage_delta: lineage_delta.clone(),
        });

        self.drain_pending_events_into(event_bus);
        if let Err(err) = event_bus.flush(CheckpointBarrier::PerOperation, self) {
            self.poisoned = true;
            return Err(KernelError::InternalError {
                message: format!("Event bus checkpoint failed: {err}"),
                context: None,
            });
        }

        let operation_artifacts = event_bus
            .context()
            .committed::<OperationArtifacts>(TopoSubscriberDataId::OperationMetrics)
            .ok_or_else(|| KernelError::InternalError {
                message: "OperationMetrics operation output missing after PerOperation flush"
                    .to_string(),
                context: None,
            })?;

        let mut op_result = OperationResult::new(result);
        op_result.set_metrics(OperationMetrics {
            duration: start.elapsed(),
            entities_created: operation_artifacts.entities_created,
            entities_deleted: operation_artifacts.entities_deleted,
            entities_modified: 0,
            exact_predicate_calls: 0,
            policy_decisions_made: 0,
        });
        op_result.set_lineage_delta(operation_artifacts.lineage_delta.clone());
        op_result.set_mutation_snapshot(mutation_snapshot);

        Ok(op_result)
    }
}
