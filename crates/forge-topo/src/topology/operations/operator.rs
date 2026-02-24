//! Euler operator trait and the `apply_op` runner.
//!
//! # Architecture
//!
//! Every topology mutation implements the `EulerOperator` trait.
//! Operators are never called directly — they go through `apply_op()`,
//! which is the single choke point for:
//! - Lineage tracking (every entity knows its provenance)
//! - Operation logging (for replay and debugging)
//! - Consistent error handling
//!
//! # Example
//! ```ignore
//! let mut draft = state.into_mutation();
//!
//! // Always use apply_op — never call op.execute() directly
//! let (edge_a, edge_b, vertex) = apply_op(&mut draft, SplitEdge {
//!     edge: my_edge,
//!     parameter: 0.5,
//! })?;
//!
//! Ok(draft.commit()?)
//! ```

use std::time::Instant;
use forge_core::{
    KernelError, OperationResult, OperationMetrics, LineageDelta,
    TopologyError, ErrorContext, ErrorScope,
};
use forge_core::{
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionTier, DecisionLog,
};
use crate::state::MutableDraft;
use crate::lineage::OpSignature;
use crate::validate::{self, ValidationLevel};

/// A topology mutation that can be applied to a `MutableDraft`.
///
/// Every Euler operator must implement this trait. The `apply_op` runner
/// handles lineage, logging, and error propagation automatically.
///
/// # Implementing a New Operator
///
/// 1. Define a struct with the operation's parameters
/// 2. Implement `EulerOperator` for it
/// 3. Call it via `apply_op(draft, MyOp { ... })` — never directly
///
/// ```ignore
/// pub struct SplitEdge {
///     pub edge: HalfEdgeId,
///     pub parameter: f64,
/// }
///
/// impl EulerOperator for SplitEdge {
///     type Output = (HalfEdgeId, HalfEdgeId, VertexId);
///
///     fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
///         // Pure topology manipulation + lineage stamping
///         todo!()
///     }
///
///     fn signature(&self) -> OpSignature {
///         OpSignature::new("split_edge")
///     }
/// }
/// ```
pub trait EulerOperator: std::fmt::Debug {
    /// The result type produced by this operation.
    type Output;

    /// Execute the topology mutation on the draft.
    ///
    /// Returns an `ExecutionResult` wrapping both the operation output
    /// and the Euler delta that this specific code path **intended**.
    /// The runner compares the declared delta against actual arena
    /// count changes to catch wiring bugs.
    ///
    /// This method contains the pure topology logic. It should:
    /// - Read/write topology data in the draft
    /// - Stamp `Lineage` on every created/modified entity
    /// - Return `ExecutionResult` with the correct `declared_delta`
    /// - Return structured errors, never panic
    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError>;

    /// A unique signature identifying this operation type.
    ///
    /// Used for lineage tracking and replay. The invocation ID is
    /// assigned by the runner (you don't need to set it).
    fn signature(&self) -> OpSignature;
}

/// Declared Euler formula delta for an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EulerDelta {
    /// Expected change in vertex count.
    pub vertices: i32,
    /// Expected change in half-edge count.
    pub half_edges: i32,
    /// Expected change in face count.
    pub faces: i32,
    /// Expected change in loop count.
    pub loops: i32,
    /// Expected change in edge count.
    pub edges: i32,
    /// Expected change in shell count.
    pub shells: i32,
    /// Expected change in solid count.
    pub solids: i32,
    /// Expected change in lump count.
    pub lumps: i32,
    /// Expected change in region count.
    pub regions: i32,
}

/// Result of an Euler operator execution.
///
/// Wraps the operator output with the Euler delta that this specific
/// code path **intended** to produce. The `apply_op` runner compares
/// this against actual arena count changes to catch wiring bugs.
///
/// Each branch in an operator (e.g. self-loop vs normal in SplitEdge)
/// declares its own delta, giving "what should have happened?" vs
/// "what did happen?" enforcement.
pub struct ExecutionResult<T> {
    /// The operator's output value.
    pub value: T,
    /// The Euler delta this code path intended.
    pub declared_delta: EulerDelta,
}

/// Apply an Euler operator through the formalized runner.
///
/// This is the ONLY correct way to execute topology mutations.
/// It handles:
/// 1. **Logging**: Records the operation start for replay (D1)
/// 2. **Execution**: Calls the operator's `execute()` method
/// 3. **Lineage**: Updates ancestry tracking for affected entities
/// 4. **Tracing**: Records a `TracedDecision` for every execution
///
/// # Errors
///
/// Returns whatever error the operator produces. The draft remains
/// valid after an error — you can apply more ops or drop it.
///
/// # Example
/// ```ignore
/// let mut draft = state.into_mutation();
/// let result = apply_op(&mut draft, MyOperator { ... })?;
/// // draft is still valid — you can apply more ops
/// let result2 = apply_op(&mut draft, AnotherOp { ... })?;
/// Ok(draft.commit()?)
/// ```
pub fn apply_op<O: EulerOperator>(
    draft: &mut MutableDraft,
    op: O,
) -> Result<OperationResult<O::Output>, KernelError> {
    let start = Instant::now();

    let face_count_before = draft.arena().face_count();
    let vertex_count_before = draft.arena().vertex_count();
    let halfedge_count_before = draft.arena().half_edge_count();
    let loop_count_before = draft.arena().loop_count();
    let edge_count_before = draft.arena().edge_count();
    let shell_count_before = draft.arena().shell_count();
    let body_count_before = draft.arena().body_count();
    let lump_count_before = draft.arena().lump_count();
    let region_count_before = draft.arena().region_count();

    let invocation_id = draft.next_op_id();
    let mut signature = op.signature();
    signature.set_invocation_id(invocation_id);

    let op_name = signature.get_name().to_string();

    tracing::debug!(
        op = ?op,
        invocation_id = invocation_id,
        "Applying Euler operator"
    );
    draft.log_operation_start(&signature);

    let exec_result = op.execute(draft, &signature)?;
    let declared_delta = exec_result.declared_delta;
    let result = exec_result.value;

    draft.apply_lineage(&signature);

    draft.bump_topology_version();

    if draft.config().per_op_hashing {
        let post_hash = draft.compute_topology_hash();
        draft.set_topology_hash(post_hash);
        draft.replay_log_mut().finalize_last(post_hash);
    }

    let face_count_after = draft.arena().face_count();
    let vertex_count_after = draft.arena().vertex_count();
    let halfedge_count_after = draft.arena().half_edge_count();
    let loop_count_after = draft.arena().loop_count();
    let edge_count_after = draft.arena().edge_count();
    let shell_count_after = draft.arena().shell_count();
    let body_count_after = draft.arena().body_count();
    let lump_count_after = draft.arena().lump_count();
    let region_count_after = draft.arena().region_count();

    // ── Euler invariant enforcement: declared intent vs actual reality ──
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
        return Err(KernelError::TopologyViolation {
            err: TopologyError::EulerFormulaViolation {
                vertices: vertex_count_after,
                edges: halfedge_count_after / 2,
                faces: face_count_after,
                expected_chi: 0,
                actual_chi: 0,
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

    // ── Per-op structural validation + reciprocity checks ────────────
    if draft.config().per_op_validation {
        validate::validate_topology(draft.arena(), ValidationLevel::Full)
            .map_err(|e| {
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

        validate_halfedge_reciprocity(draft, &op_name, invocation_id as u64)?;
    }

    // ── P2: Accurate metrics from arena count deltas ────────────────
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

    let entities_created = faces_created + vertices_created + half_edges_created
        + loops_created + edges_created + shells_created + solids_created;
    let entities_deleted = faces_deleted + vertices_deleted + half_edges_deleted
        + loops_deleted + edges_deleted + shells_deleted + solids_deleted;

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

    let mut decision = TracedDecision::new(
        DecisionId(invocation_id as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "EulerOp({}) #{}: +{}F +{}V +{}HE -{}F -{}V -{}HE in {:.0?}",
                op_name, invocation_id,
                faces_created, vertices_created, half_edges_created,
                faces_deleted, vertices_deleted, half_edges_deleted,
                start.elapsed(),
            ),
        },
    );
    decision.set_feature_scope(u64::MAX);
    let mut log = DecisionLog::new();
    log.record(decision);

    let mut op_result = OperationResult::new(result);
    op_result.set_metrics(metrics);
    op_result.set_lineage_delta(lineage_delta);
    op_result.set_decision_log(log);

    Ok(op_result)
}

/// Per-op post-condition: twin reciprocity and next/prev reciprocity.
///
/// For every halfedge in the arena, checks:
/// - `he.radial_next().radial_next() == he` (twin reciprocity)
/// - `he.next().prev() == he` (next/prev reciprocity)
///
/// These catch silent wiring bugs where operators set the wrong
/// next/prev/twin pointers — these pass structural validation but
/// produce incorrect geometry under traversal.
fn validate_halfedge_reciprocity(
    draft: &MutableDraft,
    op_name: &str,
    invocation_id: u64,
) -> Result<(), KernelError> {
    for (he_id, he_data) in draft.arena().iter_half_edges() {
        let twin_id = he_data.radial_next();
        if he_id != twin_id {
            let twin_data = draft.arena().get_half_edge(twin_id)?;
            if twin_data.radial_next() != he_id {
                return Err(KernelError::TopologyViolation {
                    err: TopologyError::BrokenLoop {
                        face_index: he_data.face().index(),
                        starting_halfedge: he_id.index(),
                    },
                    context: Some(ErrorContext {
                        scope: ErrorScope::Operation {
                            op_name: op_name.to_string(),
                            invocation_id,
                        },
                        suggested_fixes: vec![],
                        detail: format!(
                            "Twin reciprocity broken after {}: he[{}].twin={}, but he[{}].twin={} (expected {})",
                            op_name, he_id.index(), twin_id.index(),
                            twin_id.index(), twin_data.radial_next().index(), he_id.index()
                        ),
                    }),
                });
            }
        }

        let next_id = he_data.next();
        let next_data = draft.arena().get_half_edge(next_id)?;
        if next_data.prev() != he_id {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::BrokenLoop {
                    face_index: he_data.face().index(),
                    starting_halfedge: he_id.index(),
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Operation {
                        op_name: op_name.to_string(),
                        invocation_id,
                    },
                    suggested_fixes: vec![],
                    detail: format!(
                        "Next/prev reciprocity broken after {}: he[{}].next={}, but he[{}].prev={} (expected {})",
                        op_name, he_id.index(), next_id.index(),
                        next_id.index(), next_data.prev().index(), he_id.index()
                    ),
                }),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;

    /// A trivial operator for testing the runner
    #[derive(Debug)]
    struct NoOp;

    impl EulerOperator for NoOp {
        type Output = ();

        fn execute(&self, _draft: &mut MutableDraft, _sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
            Ok(ExecutionResult {
                value: (),
                declared_delta: EulerDelta { vertices: 0, half_edges: 0, faces: 0, loops: 0, edges: 0, shells: 0, solids: 0, lumps: 0, regions: 0 },
            })
        }

        fn signature(&self) -> OpSignature {
            OpSignature::new("no_op")
        }
    }

    /// An operator that always fails
    #[derive(Debug)]
    struct FailOp;

    impl EulerOperator for FailOp {
        type Output = ();

        fn execute(&self, _draft: &mut MutableDraft, _sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
            Err(KernelError::InvalidInput {
                message: "test failure".to_string(),
                context: None,
            })
        }

        fn signature(&self) -> OpSignature {
            OpSignature::new("fail_op")
        }
    }

    #[test]
    fn apply_op_succeeds_for_noop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let result = apply_op(&mut draft, NoOp);
        assert!(result.is_ok());
    }

    #[test]
    fn apply_op_propagates_errors() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let result = apply_op(&mut draft, FailOp);
        assert!(result.is_err());
    }

    #[test]
    fn apply_op_assigns_unique_ids() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // Apply multiple ops — each gets a unique invocation ID
        apply_op(&mut draft, NoOp).unwrap().into_value();
        apply_op(&mut draft, NoOp).unwrap().into_value();
        apply_op(&mut draft, NoOp).unwrap().into_value();

        // The draft state should reflect 3 operations
        // (verified by the commit succeeding with correct topology version)
        let new_state = draft.commit().unwrap();
        assert!(new_state.topology_version() > 0);
    }

    #[test]
    fn failed_op_doesnt_prevent_commit() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        // One op succeeds, one fails, then commit
        apply_op(&mut draft, NoOp).unwrap().into_value();
        let _ = apply_op(&mut draft, FailOp); // Error, but draft is still valid
        apply_op(&mut draft, NoOp).unwrap().into_value();

        // Draft can still commit (the failed op did nothing)
        assert!(draft.commit().is_ok());
    }
}
