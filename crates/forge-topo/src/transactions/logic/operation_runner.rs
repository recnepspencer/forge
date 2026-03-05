//! Operation execution runner for `MutableDraft`.
//!
//! DOMAIN: The `execute()` method is the single entry point for all topology
//! mutations. It handles: operator dispatch, Euler-delta verification,
//! journal cross-checks, contract-driven invariant validation, and
//! lineage/metrics bookkeeping.

use super::mutable_draft::MutableDraft;
use crate::operations::operator::TopoOperator;
use crate::provenance::OpSignature;

use forge_core::{
    ErrorContext, ErrorScope, KernelError, LineageDelta, OperationMetrics, OperationResult,
    TopologyError,
};

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
        if self.poisoned {
            return Err(KernelError::InternalError {
                message: "Draft was poisoned by a previously failed operation. Create a new MutableDraft from TopologyState.".to_string(),
                context: None,
            });
        }

        use crate::operations::operator::{EulerDelta, validate_halfedge_reciprocity};
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
        let lineage_count_before = self.lineage_store.active_count();

        tracing::debug!(
            op = ?op,
            invocation_id = invocation_id,
            summary = %summary,
            "Applying topology operator"
        );
        self.log_operation_start(&signature, summary);

        // Reset the mutation journal before execution — ensures a clean per-op record.
        self.mutation_journal.reset();

        let mut recorder = crate::provenance::LineageRecorder::new(
            crate::provenance::OperationLineageContext {
                feature_id: crate::provenance::FEATURE_ID_SYSTEM,
                op_name: O::NAME,
                mode: crate::provenance::LineageMode::Root, // overwritten when operator calls stamp_derived
            },
            invocation_id as u64,
        );

        let exec_result = op.execute(self, &mut recorder).map_err(|e| {
            // Poison the draft immediately on first failure to prevent cascade corruption
            self.poisoned = true;
            // Only format the Debug repr on the error path — zero cost on success.
            e.ensure_operation_context(&op_name, invocation_id as u64, &format!("{:?}", op))
        })?;
        let declared_delta = exec_result.declared_delta;

        // ── Compute journal counts BEFORE draining ────────────────────
        // The journal records every insert/remove that happened during op.execute().
        // We snapshot the gross counts now, before drain_destroyed() empties the list.
        let created = self.mutation_journal.count_created();
        let deleted = self.mutation_journal.count_destroyed();

        // ── Auto-stamp deletion lineage from the journal ──────────────
        // The MutationJournal recorded every entity that was removed during
        // op.execute(). We stamp their deletion into the lineage store
        // automatically — operators never need to call stamp_deletions.
        let destroyed = self.mutation_journal.drain_destroyed();
        for entity in &destroyed {
            // stamp_deletion may fail if the entity had no lineage entry
            // (e.g. internal structural entities). Ignore gracefully.
            let _ = recorder.stamp_deletion(&mut self.lineage_store, *entity);
        }
        let result = exec_result.value;

        self.bump_topology_version();

        if self.config.per_op_hashing {
            let post_hash = self.compute_topology_hash();
            self.set_topology_hash(post_hash);
            self.replay_log.finalize_last(post_hash);
        }

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
        if actual_delta != declared_delta {
            let expected_vertices_after = vertex_count_before as i64 + declared_delta.vertices as i64;
            let expected_edges_after = edge_count_before as i64 + declared_delta.edges as i64;
            let expected_faces_after = face_count_before as i64 + declared_delta.faces as i64;
            let expected_chi = expected_vertices_after - expected_edges_after + expected_faces_after;
            let actual_chi =
                vertex_count_after as i64 - edge_count_after as i64 + face_count_after as i64;

            return Err(KernelError::TopologyViolation {
                err: TopologyError::EulerFormulaViolation {
                    vertices: vertex_count_after,
                    edges: edge_count_after,
                    faces: face_count_after,
                    expected_chi,
                    actual_chi,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Operation {
                        op_name: op_name.clone(),
                        invocation_id: invocation_id as u64,
                    },
                    suggested_fixes: vec![],
                    detail: format!(
                        "{} declared Euler delta V={} HE={} F={} L={} E={} S={} So={} but actual was V={} HE={} F={} L={} E={} S={} So={}",
                        op_name,
                        declared_delta.vertices, declared_delta.half_edges, declared_delta.faces, declared_delta.loops,
                        declared_delta.edges, declared_delta.shells, declared_delta.solids,
                        actual_delta.vertices, actual_delta.half_edges, actual_delta.faces, actual_delta.loops,
                        actual_delta.edges, actual_delta.shells, actual_delta.solids,
                    ),
                }),
            });
        }

        // ── Journal–Arena cross-check (debug only) ────────────────────
        // Catches silently missing proxy hooks: if someone adds a new entity
        // type without updating the draft proxy macro, the journal's net delta
        // will disagree with the arena's objective net delta.
        #[cfg(debug_assertions)]
        {
            debug_assert_eq!(
                created.vertices as i32 - deleted.vertices as i32, actual_delta.vertices,
                "Journal/arena vertex count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.faces as i32 - deleted.faces as i32, actual_delta.faces,
                "Journal/arena face count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.half_edges as i32 - deleted.half_edges as i32, actual_delta.half_edges,
                "Journal/arena half-edge count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.loops as i32 - deleted.loops as i32, actual_delta.loops,
                "Journal/arena loop count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.edges as i32 - deleted.edges as i32, actual_delta.edges,
                "Journal/arena edge count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.shells as i32 - deleted.shells as i32, actual_delta.shells,
                "Journal/arena shell count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.bodies as i32 - deleted.bodies as i32, actual_delta.solids,
                "Journal/arena body count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.lumps as i32 - deleted.lumps as i32, actual_delta.lumps,
                "Journal/arena lump count mismatch — draft proxy hook may be missing"
            );
            debug_assert_eq!(
                created.regions as i32 - deleted.regions as i32, actual_delta.regions,
                "Journal/arena region count mismatch — draft proxy hook may be missing"
            );
        }

        // ── Contract-driven invariant checking ──────────────────────
        // Runs after every op unless suppressed. Policy-aware:
        //   Normal mode:  Group policy + cost ceiling filter
        //   Debug override: ALL validators for ALL invariants
        //   Suppressed: skip everything (macro-op batch mode)
        if !self.config.suppress_per_op_validation {
            use crate::validators::invariant_id::{
                InvariantId, ValidatorCost, validator_for, InvariantRelation,
            };
            use forge_core::ValidationCheckpoint;

            let policy = &self.config.group_policy;
            let checkpoint = ValidationCheckpoint::PerOp;
            let max_cost = policy.max_cost_at(checkpoint);

            for &id in InvariantId::ALL {
                let should_run = if self.config.validate_all_invariants_per_op {
                    true
                } else {
                    (O::INVARIANT_CONTRACT.relation)(id) == InvariantRelation::MayBreak
                        && policy.should_run(id.group(), checkpoint)
                };

                if should_run {
                    let entry = validator_for(id);
                    if entry.cost <= max_cost {
                        let check_result = (entry.check)(&self.arena);
                        let passed = check_result.is_ok();

                        tracing::info!(
                            invariant = ?id,
                            operator = O::NAME,
                            invocation = invocation_id,
                            cost = ?entry.cost,
                            passed = passed,
                            "invariant_check"
                        );

                        if let Err(e) = check_result {
                            self.poisoned = true;

                            // Emit a structured tracing event at ERROR level.
                            // This is the ONLY place where validator failures are
                            // reported — it MUST include enough context to diagnose
                            // the root cause without patching in println! hacks.
                            tracing::error!(
                                invariant = ?id,
                                operator = O::NAME,
                                invocation = invocation_id,
                                error = %e,
                                error_debug = ?e,
                                "INVARIANT VIOLATION: {:?} failed after {} (invocation {})",
                                id, op_name, invocation_id,
                            );

                            // Force-stamp operation context. We use with_phase
                            // instead of ensure_operation_context because the
                            // latter is a no-op when context is already Some
                            // (e.g. StaleHandle errors from arena lookups carry
                            // their own Entity context, which loses the invariant
                            // name entirely).
                            return Err(e.ensure_operation_context(
                                &op_name,
                                invocation_id as u64,
                                &format!("Invariant {:?} violated after {}", id, op_name),
                            ).with_phase(&format!("invariant_check({:?})", id)));
                        }
                    }
                }
            }
        }

        // ── Build LineageDelta + OperationMetrics from journal (gross counts) ──
        let metrics = OperationMetrics {
            duration: start.elapsed(),
            entities_created: created.total(),
            entities_deleted: deleted.total(),
            entities_modified: 0,
            exact_predicate_calls: 0,
            policy_decisions_made: 0,
        };

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

        let mut op_result = OperationResult::new(result);
        op_result.set_metrics(metrics);
        op_result.set_lineage_delta(lineage_delta);

        Ok(op_result)
    }
}
