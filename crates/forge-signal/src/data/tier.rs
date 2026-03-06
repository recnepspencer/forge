//! Evaluation tier classification for multilevel signal scheduling.

/// Signal runtime tier.
///
/// - `Entity`: static effect routing + checkpoint-barrier refresh
/// - `Feature`: dynamic dependency discovery + lazy pull
/// - `Analysis`: feature-like graph with async-capable scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationTier {
    Entity,
    Feature,
    Analysis,
}
