//! Parallel-safe evaluation context for dependency tracking.
//!
//! DOMAIN: Explicit dependency discovery during feature evaluation.
//!
//! INVARIANTS:
//! - Context is passed by value, not stored in thread-locals (D8 safe)
//! - All upstream reads are recorded for graph wiring
//! - Each context tracks exactly one evaluating node
//!
//! DEPENDENCIES: `handles` (NodeId), `schema` (Aspect, DependencyEdge, AspectVersion),
//!               `graph` (SignalGraph)

use forge_core::KernelError;

use crate::graph::SignalGraph;
use crate::handles::NodeId;
use crate::schema::{Aspect, AspectVersion, DependencyEdge};

/// Explicit evaluation context for parallel-safe dependency tracking.
///
/// Replaces thread-local stacks for dependency discovery. Under Rayon
/// work-stealing, each task receives its own `EvaluationContext` by value,
/// so dependency recording is safe regardless of the executing OS thread.
///
/// # Usage
/// ```ignore
/// let mut ctx = EvaluationContext::new(my_node_id);
/// let upstream_ver = ctx.read(&graph, upstream_id, Aspect::Topology)?;
/// // ... use upstream data ...
/// let deps = ctx.finalize();
/// // Wire deps into the graph
/// ```
pub struct EvaluationContext {
    /// The node currently being evaluated.
    evaluating: NodeId,
    /// Dependencies discovered during this evaluation.
    discovered_deps: Vec<DependencyEdge>,
}

impl EvaluationContext {
    /// Create a new context for evaluating the given node.
    pub fn new(evaluating: NodeId) -> Self {
        Self {
            evaluating,
            discovered_deps: Vec::new(),
        }
    }

    /// The node this context is evaluating.
    pub fn evaluating(&self) -> NodeId {
        self.evaluating
    }

    /// Read an upstream signal's aspect version, recording the dependency.
    ///
    /// The dependency is recorded for later wiring into the graph via
    /// `finalize()`. This ensures dependency discovery works correctly
    /// under Rayon work-stealing (Doctrine D8).
    pub fn read(
        &mut self,
        graph: &SignalGraph,
        signal: NodeId,
        aspect: Aspect,
    ) -> Result<AspectVersion, KernelError> {
        let edge = DependencyEdge::new(signal, aspect);

        let already_recorded = self
            .discovered_deps
            .iter()
            .any(|d| d.source() == signal && d.aspect() == aspect);

        if !already_recorded {
            self.discovered_deps.push(edge);
        }

        let entry = graph.get_entry(signal)?;
        Ok(entry.get_aspect_version())
    }

    /// Consume the context and return all discovered dependencies.
    ///
    /// The caller uses these to wire edges in the `SignalGraph`.
    pub fn finalize(self) -> Vec<DependencyEdge> {
        self.discovered_deps
    }

    /// The number of dependencies discovered so far.
    pub fn discovered_count(&self) -> usize {
        self.discovered_deps.len()
    }
}
