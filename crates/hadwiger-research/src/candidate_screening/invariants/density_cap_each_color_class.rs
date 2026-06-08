use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::DensityCapEachColorClass,
        "density_cap_each_color_class",
        "Density cap for each color class",
        T::CertificateRequired,
        A::GeneratedPattern,
        "Each measurable 1-avoiding color class must respect the selected density upper bound.",
        "a color density exceeds the selected D_max",
        "named density theorem/bound and checked density estimate",
    )
}
