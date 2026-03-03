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
use crate::provenance::{Lineage, LineageEvent, OpSignature};
use crate::provenance::LineageStore;
use crate::provenance::{ReplayEntry, ReplayLog};
use crate::provenance::ReidentificationLinkIndex;
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
    /// Lineage events recorded during this draft
    pub(crate) lineage_log: Vec<LineageEvent>,
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
}

impl MutableDraft {
    /// The current topology hash of this draft.
    pub fn topology_hash(&self) -> u128 {
        self.topology_hash
    }

    /// Record a lineage event during mutation on the explicit/manual lineage channel.
    ///
    /// Forge currently has two lineage event sources in a draft:
    /// - `lineage_store`: live arena-driven provenance events emitted by topology mutations
    /// - `lineage_log`: explicit/manual events emitted by higher-level orchestration/tests
    ///
    /// Commit semantics must persist both channels into the committed chronology.
    /// This method exists for the explicit/manual channel.
    pub fn log_lineage_event(&mut self, event: LineageEvent) {
        self.lineage_log.push(event);
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

    /// Apply lineage tracking for the completed operation (called by `execute()` method).
    ///
    /// Currently a stub — expanded in Milestone 1.2 (Euler Lineage Tracking).
    pub fn apply_lineage(&mut self, _signature: &OpSignature) {}

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

    /// The lineage events recorded during this draft.
    pub fn lineage_log(&self) -> &[LineageEvent] {
        &self.lineage_log
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
        self.committed = true;

        crate::validators::structural::validate_topology(&self.arena, self.config.validation_level)?;

        let topology_hash = self.compute_topology_hash();
        let committed_arena = std::mem::take(&mut self.arena);

        // Drain lineage sources and append to the prior history.
        //
        // `lineage_log` is the explicit/manual lineage channel used by some
        // callers and tests; `lineage_store` is the live arena-driven lineage
        // event source. Both are part of the committed chronology.
        let mut new_events = std::mem::take(&mut self.lineage_log);
        new_events.extend(self.lineage_store.drain_events());
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
        self.arena.insert_radial_pair(data_a, data_b)
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

        tracing::debug!(
            op = ?op,
            invocation_id = invocation_id,
            summary = %summary,
            "Applying topology operator"
        );
        self.log_operation_start(&signature, summary);

        let exec_result = op.execute(self).map_err(|e| {
            // Only format the Debug repr on the error path — zero cost on success.
            e.ensure_operation_context(&op_name, invocation_id as u64, &format!("{:?}", op))
        })?;
        let declared_delta = exec_result.declared_delta;
        let result = exec_result.value;

        self.apply_lineage(&signature);

        self.bump_topology_version();

        if self.config.per_op_hashing {
            let post_hash = self.compute_topology_hash();
            self.set_topology_hash(post_hash);
            self.replay_log.finalize_last(post_hash);
        }

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

        if self.config.per_op_validation {
            crate::validators::structural::validate_topology(&self.arena, ValidationLevel::Full).map_err(|e| {
                KernelError::TopologyViolation {
                    err: TopologyError::BrokenLoop {
                        face_index: 0,
                        starting_halfedge: 0,
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.clone(),
                            invocation_id: invocation_id as u64,
                        },
                        suggested_fixes: vec![],
                        detail: format!("Per-op validation failed after {}: {}", op_name, e),
                    }),
                }
            })?;

            validate_halfedge_reciprocity(self, &op_name, invocation_id as u64)?;
        }

        let faces_created = face_count_after.saturating_sub(face_count_before) as u32;
        let vertices_created = vertex_count_after.saturating_sub(vertex_count_before) as u32;
        let half_edges_created = halfedge_count_after.saturating_sub(halfedge_count_before) as u32;
        let loops_created = loop_count_after.saturating_sub(loop_count_before) as u32;
        let edges_created = edge_count_after.saturating_sub(edge_count_before) as u32;
        let shells_created = shell_count_after.saturating_sub(shell_count_before) as u32;
        let solids_created = body_count_after.saturating_sub(body_count_before) as u32;

        let faces_deleted = face_count_before.saturating_sub(face_count_after) as u32;
        let vertices_deleted = vertex_count_before.saturating_sub(vertex_count_after) as u32;
        let half_edges_deleted = halfedge_count_before.saturating_sub(halfedge_count_after) as u32;
        let loops_deleted = loop_count_before.saturating_sub(loop_count_after) as u32;
        let edges_deleted = edge_count_before.saturating_sub(edge_count_after) as u32;
        let shells_deleted = shell_count_before.saturating_sub(shell_count_after) as u32;
        let solids_deleted = body_count_before.saturating_sub(body_count_after) as u32;

        let entities_created = faces_created
            + vertices_created
            + half_edges_created
            + loops_created
            + edges_created
            + shells_created
            + solids_created;
        let entities_deleted = faces_deleted
            + vertices_deleted
            + half_edges_deleted
            + loops_deleted
            + edges_deleted
            + shells_deleted
            + solids_deleted;

        let metrics = OperationMetrics {
            duration: start.elapsed(),
            entities_created,
            entities_deleted,
            entities_modified: 0,
            exact_predicate_calls: 0,
            policy_decisions_made: 0,
        };

        let lineage_delta = LineageDelta {
            faces_created,
            faces_deleted,
            half_edges_created,
            half_edges_deleted,
            vertices_created,
            vertices_deleted,
            loops_created,
            loops_deleted,
            edges_created,
            edges_deleted,
            shells_created,
            shells_deleted,
            solids_created,
            solids_deleted,
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
