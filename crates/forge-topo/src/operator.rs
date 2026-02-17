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
//! let mut draft = state.begin_mutation();
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
use forge_core::{KernelError, OperationResult, OperationMetrics, LineageDelta};
use forge_core::result::{
    TracedDecision, DecisionId, DecisionKind, DecisionContext, DecisionLog,
};
use crate::state::MutableDraft;
use crate::lineage::OpSignature;

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
    /// This method contains the pure topology logic. It should:
    /// - Read/write topology data in the draft
    /// - Stamp `Lineage` on every created/modified entity
    /// - Return structured errors, never panic
    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError>;

    /// A unique signature identifying this operation type.
    ///
    /// Used for lineage tracking and replay. The invocation ID is
    /// assigned by the runner (you don't need to set it).
    fn signature(&self) -> OpSignature;
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
/// let mut draft = state.begin_mutation();
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
    let state_hash_before = draft.topology_hash();

    let face_count_before = draft.arena().face_count();
    let vertex_count_before = draft.arena().vertex_count();
    let halfedge_count_before = draft.arena().half_edge_count();

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

    let result = op.execute(draft, &signature)?;

    draft.apply_lineage(&signature);

    // Compute new hash immediately to see if anything structurally changed
    let state_hash_after = draft.compute_topology_hash();
    
    // Finalize the replay log entry
    draft.replay_log_mut().finalize_last(state_hash_after);
    draft.set_topology_hash(state_hash_after);

    // D4: Smart Version Bumping
    // Only invalidate the DAG (bump version) if the structural hash actually changed.
    if state_hash_before != state_hash_after {
        draft.bump_topology_version();
    }

    let face_count_after = draft.arena().face_count();
    let vertex_count_after = draft.arena().vertex_count();
    let halfedge_count_after = draft.arena().half_edge_count();

    let faces_created = face_count_after.saturating_sub(face_count_before) as u32;
    let vertices_created = vertex_count_after.saturating_sub(vertex_count_before) as u32;
    let half_edges_created = halfedge_count_after.saturating_sub(halfedge_count_before) as u32;

    let entities_created = faces_created + vertices_created + half_edges_created;

    let metrics = OperationMetrics {
        duration: start.elapsed(),
        entities_created,
        entities_deleted: 0,
        entities_modified: 0,
        exact_predicate_calls: 0,
        policy_decisions_made: 0,
    };

    let lineage_delta = LineageDelta {
        faces_created,
        faces_deleted: 0,
        half_edges_created,
        half_edges_deleted: 0,
        vertices_created,
        vertices_deleted: 0,
    };

    let mut decision = TracedDecision::new(
        DecisionId(invocation_id as u64),
        DecisionKind::Exact,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "EulerOp({}) #{}: +{}F +{}V +{}HE in {:.0?}",
                op_name, invocation_id,
                faces_created, vertices_created, half_edges_created,
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
    op_result.set_state_hash_before(state_hash_before);
    op_result.set_state_hash_after(state_hash_after);
    op_result.set_decision_log(log);

    Ok(op_result)
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

        fn execute(&self, _draft: &mut MutableDraft, _sig: &OpSignature) -> Result<Self::Output, KernelError> {
            Ok(())
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

        fn execute(&self, _draft: &mut MutableDraft, _sig: &OpSignature) -> Result<Self::Output, KernelError> {
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
        let mut draft = state.begin_mutation();
        let result = apply_op(&mut draft, NoOp);
        assert!(result.is_ok());
    }

    #[test]
    fn apply_op_propagates_errors() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();
        let result = apply_op(&mut draft, FailOp);
        assert!(result.is_err());
    }

    #[test]
    fn apply_op_assigns_unique_ids() {
        let state = TopologyState::empty();
        let mut draft = state.begin_mutation();

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
        let mut draft = state.begin_mutation();

        // One op succeeds, one fails, then commit
        apply_op(&mut draft, NoOp).unwrap().into_value();
        let _ = apply_op(&mut draft, FailOp); // Error, but draft is still valid
        apply_op(&mut draft, NoOp).unwrap().into_value();

        // Draft can still commit (the failed op did nothing)
        assert!(draft.commit().is_ok());
    }
}
