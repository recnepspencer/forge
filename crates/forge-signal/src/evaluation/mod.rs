pub mod context;
pub mod pull;
pub mod push;

pub use context::EvaluationContext;
pub use pull::evaluate;
pub use push::mark_dirty;
