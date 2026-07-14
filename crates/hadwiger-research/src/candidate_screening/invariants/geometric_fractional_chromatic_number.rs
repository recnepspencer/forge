use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::GeometricFractionalChromaticNumber,
        "geometric_fractional_chromatic_number",
        "Geometric fractional chromatic number",
        T::CertificateRequired,
        A::PointEmbedding,
        "Geometric fractional certificates add exact isometry/equality constraints to finite fractional coloring pressure.",
        "priority_if_retained_geometric_fractional_dual_reaches_target_lower_bound",
        "replayed rational dual weights plus exact subset-isometry witnesses",
    )
}
