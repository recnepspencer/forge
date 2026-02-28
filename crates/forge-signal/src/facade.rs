//! Public API Boundary for forge-signal.
//! External components must depend ONLY on this module.

// Re-export Data constructs
pub use crate::data::aspect::{Aspect, AspectVersion};
pub use crate::data::dependency::DependencyEdge;
pub use crate::data::graph::SignalGraph;
pub use crate::data::handle::NodeId;
pub use crate::data::node::NodeState;

// Re-export Logic constructs
pub use crate::logic::context::EvaluationContext;
pub use crate::logic::evaluation::evaluate;
pub use crate::logic::invalidation::mark_dirty;
