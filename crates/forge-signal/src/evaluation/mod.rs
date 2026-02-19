pub mod push;
pub mod pull;
pub mod context;

pub use context::EvaluationContext;
pub use push::mark_dirty;
pub use pull::evaluate;
