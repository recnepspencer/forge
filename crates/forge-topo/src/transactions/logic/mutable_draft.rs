//! Mutable draft for transactional topology mutation (Doctrine D6).
//!
//! DOMAIN: MutableDraft wraps the arena for copy-on-write mutation.
//! Commit finalizes changes into a new TopologyState; drop rolls back.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::b_rep::TopologyArena;
use crate::handles::{
    BodyId, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId, VertexId,
};
use crate::transactions::compute_arena_topology_hash;
use crate::transactions::data::mutation_journal::MutationJournal;
use crate::provenance::{Lineage, LineageEvent, OpSignature};
use crate::provenance::LineageStore;
use crate::provenance::{ReplayEntry, ReplayLog};
use crate::provenance::ReidentificationLinkIndex;
use crate::provenance::{LineageRecorder, LineageMode, OperationLineageContext, FEATURE_ID_SYSTEM};
use crate::operations::operator::TopoOperator;
use crate::validators::validate::ValidationLevel;

use forge_core::{
    ErrorContext, ErrorScope, KernelError, LineageDelta, OperationMetrics, OperationResult,
    TopologyError,
};

use crate::transactions::data::draft_configuration::DraftConfig;
use crate::transactions::data::versioned_snapshot::TopologyState;

/// A mutable draft for topology changes (Doctrine D6).
///
/// # Transactional Safety
///
/// - **Commit**: Call `.commit()` to finalize changes → returns new `TopologyState`
/// - **Rollback**: Drop without committing → all changes are silently discarded
/// - **Auto-validation**: `.commit()` runs topology validation before returning
///
/// This mirrors your Angular `createOptimisticMutation` pattern:
/// try the operation, rollback on failure, commit on success.
pub struct MutableDraft {
    /// The epoch of the state we forked from
    pub(crate) base_epoch: u64,
    /// The epoch this draft will produce if committed
    pub(crate) next_epoch: u64,
    /// Current topology version (may be bumped during mutations)
    pub(crate) topology_version: u64,
    /// Current geometry version (may be bumped during mutations)
    pub(crate) geometry_version: u64,
    /// Counter for assigning unique operation IDs within this draft
    pub(crate) op_counter: u64,
    /// Whether commit() was called
    pub(crate) committed: bool,
    /// Replay log for this draft (Milestone 0.4)
    pub(crate) replay_log: ReplayLog,
    /// Current topology hash (for replay entry pre-hashes)
    pub(crate) topology_hash: u128,
    /// Draft configuration (per-op hashing, deterministic seed)
    pub(crate) config: DraftConfig,
    /// The mutable arena — cloned from the source state's Arc on begin_mutation
    pub(crate) arena: TopologyArena,
    /// Live lineage store tracking all entity provenance during this draft.
    pub(crate) lineage_store: LineageStore,
    /// Lineage events inherited from the prior committed state.
    pub(crate) prior_lineage_events: Vec<LineageEvent>,
    /// Per-operation mutation journal — records every insert/remove automatically.
    pub(crate) mutation_journal: MutationJournal,
    /// If true, a previous operation failed and this draft MUST NOT be used.
    pub(crate) poisoned: bool,
}

impl MutableDraft {
    /// The current topology hash of this draft.
    pub fn topology_hash(&self) -> u128 {
        self.topology_hash
    }



    /// Get the next unique operation ID for this draft.
    pub fn next_op_id(&mut self) -> u64 {
        self.op_counter += 1;
        self.op_counter
    }

    /// Log the start of an operation (called by `execute()` method).
    ///
    /// Records the current topology hash as `pre_hash` and computes a
    /// deterministic seed from the config's base seed + op counter.
    pub fn log_operation_start(&mut self, signature: &OpSignature, semantic_summary: String) {
        let seed = self.config.deterministic_seed.wrapping_add(self.op_counter);
        let entry = ReplayEntry::new(
            signature.clone(),
            Vec::new(),
            seed,
            self.topology_hash,
            semantic_summary,
        );
        self.replay_log.record(entry);
    }

    /// Debug-only guard: assert that every entity in the arena has lineage.
    ///
    /// Fires at the end of `execute()` to catch missing stamps immediately
    /// rather than six operations later.
    #[cfg(debug_assertions)]
    pub fn validate_lineage_coverage(&self, signature: &OpSignature) {
        let arena_count = self.arena.face_count()
            + self.arena.vertex_count()
            + self.arena.half_edge_count()
            + self.arena.edge_count()
            + self.arena.loop_count()
            + self.arena.shell_count()
            + self.arena.body_count()
            + self.arena.lump_count()
            + self.arena.region_count();
        let lineage_count = self.lineage_store.active_count();
        // Only assert if lineage store is non-empty (i.e., lineage wiring is active).
        // This avoids false positives during the migration period where not all
        // code paths stamp lineage yet.
        if lineage_count > 0 {
            debug_assert_eq!(
                arena_count, lineage_count,
                "Lineage coverage gap after {}: arena has {} entities, lineage tracks {}",
                signature, arena_count, lineage_count
            );
        }
    }

    /// The draft's configuration.
    pub fn config(&self) -> &DraftConfig {
        &self.config
    }

    /// Bump the topology version (call after connectivity changes).
    pub fn bump_topology_version(&mut self) {
        self.topology_version += 1;
    }

    /// Bump the geometry version (call after position-only changes).
    pub fn bump_geometry_version(&mut self) {
        self.geometry_version += 1;
    }



    /// The replay log recorded during this draft (Milestone 0.4).
    pub fn replay_log(&self) -> &ReplayLog {
        &self.replay_log
    }

    /// Mutable access to the replay log.
    pub fn replay_log_mut(&mut self) -> &mut ReplayLog {
        &mut self.replay_log
    }

    /// Set the current topology hash.
    pub(crate) fn set_topology_hash(&mut self, hash: u128) {
        self.topology_hash = hash;
    }

    /// Read-only access to the draft's arena.
    pub fn arena(&self) -> &TopologyArena {
        &self.arena
    }

    /// Mutable access to the draft's arena (for Euler operators).
    pub fn arena_mut(&mut self) -> &mut TopologyArena {
        &mut self.arena
    }

    /// Read-only access to the lineage store.
    pub fn lineage_store(&self) -> &LineageStore {
        &self.lineage_store
    }

    /// Mutable access to the lineage store.
    pub fn lineage_store_mut(&mut self) -> &mut LineageStore {
        &mut self.lineage_store
    }

    /// Disjoint mutable access to both the arena and the lineage store.
    ///
    /// Essential for Euler operators to pass the lineage store to arena hooks
    /// without violating borrow checker rules.
    pub fn unbundle_mut(&mut self) -> (&mut TopologyArena, &mut LineageStore) {
        (&mut self.arena, &mut self.lineage_store)
    }

    // ── Provenance Stamping API ─────────────────────────────────────────
    //
    // These methods are the production-grade API for Euler operators to
    // declare lineage. They handle the borrow-splitting internally so
    // operators don't have to juggle `lineage_store()` vs `lineage_store_mut()`.

    /// Stamp multiple child entities as derived from a single parent.
    ///
    /// Used by creation operators (SplitEdge, MakeEdgeFace, MakeEdgeVertex, etc.)
    /// to declare: "these new entities were born from this parent entity."
    ///
    /// # Panics (debug builds)
    ///
    /// Panics if the parent entity has no lineage in the store. After
    /// `build_halfedge_mesh` completes, every entity MUST have lineage.
    /// A missing parent indicates a wiring bug upstream, not a recoverable condition.
    pub fn stamp_children_of(
        &mut self,
        recorder: &mut LineageRecorder,
        parent: forge_core::EntityRef,
        children: &[forge_core::EntityRef],
    ) {
        let parent_lineage = self.lineage_store.get_lineage(&parent).cloned();
        debug_assert!(
            parent_lineage.is_some(),
            "stamp_children_of: parent {:?} has no lineage — wiring bug upstream",
            parent
        );
        if let Some(ref lineage) = parent_lineage {
            for &child in children {
                recorder.stamp_derived(&mut self.lineage_store, child, lineage);
            }
        }
    }

    /// Stamp multiple child entities as merged from multiple parents.
    ///
    /// Used by Boolean operations where a new entity derives from entities
    /// on two (or more) different bodies.
    ///
    /// # Panics (debug builds)
    ///
    /// Panics if any parent entity has no lineage in the store.
    pub fn stamp_merged_children_of(
        &mut self,
        recorder: &mut LineageRecorder,
        parents: &[forge_core::EntityRef],
        children: &[forge_core::EntityRef],
    ) {
        let parent_lineages: Vec<_> = parents.iter()
            .map(|p| {
                let lineage = self.lineage_store.get_lineage(p).cloned();
                debug_assert!(
                    lineage.is_some(),
                    "stamp_merged_children_of: parent {:?} has no lineage — wiring bug upstream",
                    p
                );
                lineage
            })
            .flatten()
            .collect();

        if parent_lineages.len() != parents.len() {
            return; // release-mode graceful degradation
        }

        let mode = LineageMode::Merged { parents: parent_lineages.into() };
        for &child in children {
            let context = OperationLineageContext {
                feature_id: recorder.feature_id(),
                op_name: recorder.op_name(),
                mode: mode.clone(),
            };
            let mut merge_recorder = LineageRecorder::new(context, recorder.invocation_id());
            merge_recorder.stamp(&mut self.lineage_store, child);
        }
    }

    /// Read-only access to the mutation journal for the current operation.
    pub fn mutation_journal(&self) -> &MutationJournal {
        &self.mutation_journal
    }

    /// Mutable access to the mutation journal (for testing and runner internals).
    pub(crate) fn mutation_journal_mut(&mut self) -> &mut MutationJournal {
        &mut self.mutation_journal
    }

    /// Take ownership of the lineage store, replacing it with an empty one.
    ///
    /// Use this to extract lineage data before commit (or on error paths
    /// when the draft will be dropped without committing).
    pub fn take_lineage_store(&mut self) -> LineageStore {
        std::mem::take(&mut self.lineage_store)
    }

    /// Finalize the mutation, producing a new `TopologyState`.
    ///
    /// This automatically runs topology validation (D4, D6).
    /// If validation fails, returns `KernelError::TopologyViolation`.
    ///
    /// # Errors
    ///
    /// Returns `KernelError::TopologyViolation` if the resulting topology
    /// violates any invariant (Euler formula, twin consistency, etc.).
    pub fn commit(mut self) -> Result<TopologyState, KernelError> {
        if self.poisoned {
            return Err(KernelError::InternalError {
                message: "Cannot commit a poisoned draft. A previous operation failed mid-transaction.".to_string(),
                context: None,
            });
        }
        self.committed = true;

        crate::validators::structural::validate_topology(&self.arena, self.config.validation_level)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        // Drain lineage from the single source of truth (LineageStore)
        // and append to the prior history.
        let new_events = self.lineage_store.drain_events();
        let mut all_events = std::mem::take(&mut self.prior_lineage_events);
        all_events.extend(new_events);
        let reid_index =
            ReidentificationLinkIndex::from_lineage_events(self.next_epoch, &all_events);

        Ok(TopologyState {
            epoch: self.next_epoch,
            topology_version: self.topology_version,
            geometry_version: self.geometry_version,
            topology_hash,
            arena: Arc::new(committed_arena),
            lineage_events: Arc::new(all_events),
            reidentification_link_index: Arc::new(reid_index),
        })
    }




    /// Compute the structural topology hash from the arena.
    pub(crate) fn compute_topology_hash(&self) -> u128 {
        compute_arena_topology_hash(&self.arena)
    }

    // ── Proxy CRUD Methods ───────────────────────────────────────────
    // Generated by `define_draft_proxies!` macro in `arena/crud_macro.rs`.
    // insert_*/remove_* for all 9 entity types are auto-generated there.

    /// Insert a pair of radial halfedges and wire their `radial_next` fields.
    pub fn insert_radial_pair(
        &mut self,
        data_a: crate::b_rep::HalfEdgeData,
        data_b: crate::b_rep::HalfEdgeData,
    ) -> (HalfEdgeId, HalfEdgeId) {
        let (a, b) = self.arena.insert_radial_pair(data_a, data_b);
        self.mutation_journal.record_creation(forge_core::EntityRef::from(a));
        self.mutation_journal.record_creation(forge_core::EntityRef::from(b));
        (a, b)
    }

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
        // Runs after every op unless suppressed. Cost-tier aware:
        //   Normal mode:  Cheap validators for MayBreak invariants only
        //   Debug override: ALL validators for ALL invariants
        //   Suppressed: skip everything (macro-op batch mode)
        if !self.config.suppress_per_op_validation {
            use crate::validators::invariant_id::{
                InvariantId, ValidatorCost, validator_for,
            };

            let max_cost = if self.config.validate_all_invariants_per_op {
                ValidatorCost::Expensive
            } else {
                ValidatorCost::Cheap
            };

            let invariants_to_check: Vec<InvariantId> = if self.config.validate_all_invariants_per_op {
                // Debug override: check ALL invariants
                InvariantId::ALL.to_vec()
            } else {
                // Normal: only MayBreak invariants from operator contract
                O::INVARIANT_CONTRACT.may_break().collect()
            };

            for id in invariants_to_check {
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
                        return Err(e.ensure_operation_context(
                            &op_name,
                            invocation_id as u64,
                            &format!("Invariant {:?} violated after {}", id, op_name),
                        ));
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

impl Drop for MutableDraft {
    fn drop(&mut self) {
        if !self.committed {
            tracing::warn!(
                base_epoch = self.base_epoch,
                ops_applied = self.op_counter,
                "MutableDraft dropped without commit. Topology rolled back."
            );
        }
    }
}

/// Manual `Debug` impl — `MutableDraft` is not `Clone` (forking a transaction is invalid).
impl std::fmt::Debug for MutableDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutableDraft")
            .field("base_epoch", &self.base_epoch)
            .field("next_epoch", &self.next_epoch)
            .field("committed", &self.committed)
            .field("ops_applied", &self.op_counter)
            .finish()
    }
}
