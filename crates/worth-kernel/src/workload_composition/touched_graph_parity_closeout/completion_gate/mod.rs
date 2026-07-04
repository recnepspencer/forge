mod current;
mod error;
mod gate;
mod validation;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_failure_guards;

pub use current::current_worth_touched_graph_roadmap_completion_gate;
pub use error::{
    WorthTouchedGraphRoadmapCompletionGateError,
    WorthTouchedGraphRoadmapCompletionGateErrorKind,
};
pub use gate::WorthTouchedGraphRoadmapCompletionGate;

pub(crate) use validation::validate_roadmap_completion_gate;
