//! Topology operator trait and supporting types.
//!
//! # Architecture
//!
//! Every topology mutation implements the `TopoOperator` trait.
//! Operators are never called directly — they go through
//! `MutableDraft::execute()`, which is the single choke point for:
//! - Lineage tracking (every entity knows its provenance)
//! - Operation logging (for replay and debugging)
//! - Euler delta verification (declared vs actual entity counts)
//! - Consistent error handling
//!
//! # Example
//! ```ignore
//! let mut draft = state.into_mutation();
//!
//! // Always use draft.execute() — never call op.execute() directly
//! let (edge_a, edge_b, vertex) = draft.execute(SplitEdge {
//!     edge: my_edge,
//!     parameter: 0.5,
//! })?.into_value();
//!
//! Ok(draft.commit()?)
//! ```

use crate::transactions::MutableDraft;
use forge_core::{
    ErrorContext, ErrorScope, KernelError, TopologyError,
};

/// A topology mutation that can be applied to a `MutableDraft`.
///
/// Every topology operator must implement this trait. The
/// `MutableDraft::execute()` runner handles lineage, logging,
/// Euler delta verification, and error propagation automatically.
///
/// # Implementing a New Operator
///
/// 1. Define a struct with the operation's parameters
/// 2. Implement `TopoOperator` for it
/// 3. Call it via `draft.execute(MyOp { ... })` — never directly
///
/// ```ignore
/// pub struct SplitEdge {
///     pub edge: HalfEdgeId,
///     pub parameter: f64,
/// }
///
/// impl TopoOperator for SplitEdge {
///     type Output = (HalfEdgeId, HalfEdgeId, VertexId);
///
///     fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
///         // Pure topology manipulation
///         panic!("example stub")
///     }
///
///     const NAME: &'static str = "split_edge";
/// }
/// ```
pub trait TopoOperator: std::fmt::Debug {
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
    /// - Return `ExecutionResult` with the correct `declared_delta`
    /// - Return structured errors, never panic
    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError>;

    /// A unique name identifying this operation type.
    ///
    /// Used for lineage tracking and replay. The invocation ID is
    /// assigned by the runner (you don't need to set it).
    const NAME: &'static str;

    /// Human-readable semantic summary of this operation with its parameters.
    ///
    /// Override this to provide a meaningful description for lineage chains.
    /// Default uses the Debug repr. When P3.3 semantic summarization is
    /// implemented, `MutableDraft::execute()` will record this alongside
    /// the raw Euler delta to produce < 200 token causal narratives.
    fn semantic_summary(&self) -> String {
        format!("{:?}", self)
    }
}

/// Declared Euler formula delta for an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

/// Result of a topology operator execution.
///
/// Wraps the operator output with the Euler delta that this specific
/// code path **intended** to produce. The `MutableDraft::execute()`
/// runner compares this against actual arena count changes to catch
/// wiring bugs.
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

/// Per-op post-condition: twin reciprocity and next/prev reciprocity.
///
/// For every halfedge in the arena, checks:
/// - `he.radial_next().radial_next() == he` (twin reciprocity)
/// - `he.next().prev() == he` (next/prev reciprocity)
///
/// These catch silent wiring bugs where operators set the wrong
/// next/prev/twin pointers — these pass structural validation but
/// produce incorrect geometry under traversal.
pub(crate) fn validate_halfedge_reciprocity(
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
    use crate::transactions::TopologyState;

    /// A trivial operator for testing the runner
    #[derive(Debug)]
    struct NoOp;

    impl TopoOperator for NoOp {
        type Output = ();

        fn execute(
            &self,
            _draft: &mut MutableDraft,
            _recorder: &mut crate::provenance::LineageRecorder,
        ) -> Result<ExecutionResult<Self::Output>, KernelError> {
            Ok(ExecutionResult {
                value: (),
                declared_delta: EulerDelta {
                    vertices: 0,
                    half_edges: 0,
                    faces: 0,
                    loops: 0,
                    edges: 0,
                    shells: 0,
                    solids: 0,
                    lumps: 0,
                    regions: 0,
                },
            })
        }

        const NAME: &'static str = "no_op";
    }

    /// An operator that always fails
    #[derive(Debug)]
    struct FailOp;

    impl TopoOperator for FailOp {
        type Output = ();

        fn execute(
            &self,
            _draft: &mut MutableDraft,
            _recorder: &mut crate::provenance::LineageRecorder,
        ) -> Result<ExecutionResult<Self::Output>, KernelError> {
            Err(KernelError::InvalidInput {
                message: "test failure".to_string(),
                context: None,
            })
        }

        const NAME: &'static str = "fail_op";
    }

    #[test]
    fn execute_succeeds_for_noop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let result = draft.execute(NoOp);
        assert!(result.is_ok());
    }

    #[test]
    fn execute_propagates_errors() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let result = draft.execute(FailOp);
        assert!(result.is_err());
    }

    #[test]
    fn execute_assigns_unique_ids() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        draft.execute(NoOp).unwrap().into_value();
        draft.execute(NoOp).unwrap().into_value();
        draft.execute(NoOp).unwrap().into_value();

        let new_state = draft.commit().unwrap();
        assert!(new_state.topology_version() > 0);
    }

    #[test]
    fn failed_op_doesnt_prevent_commit() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        draft.execute(NoOp).unwrap().into_value();
        let _ = draft.execute(FailOp);
        draft.execute(NoOp).unwrap().into_value();

        assert!(draft.commit().is_ok());
    }
}
