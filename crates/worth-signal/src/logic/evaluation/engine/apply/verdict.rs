use crate::logic::evaluation::EvaluationVerdict;

/// Evaluated candidates remain provisional until the canonical output-commit
/// authority applies producer-side semantic equivalence exactly once.
pub(crate) const fn provisional_evaluated_verdict() -> EvaluationVerdict {
    EvaluationVerdict::Recomputed
}
