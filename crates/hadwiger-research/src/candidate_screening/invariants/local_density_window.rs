use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::LocalDensityWindow,
        "local_density_window",
        "Local density-window test",
        T::CertificateRequired,
        A::GeneratedPattern,
        "Local windows can violate 1-avoiding density even when global density looks plausible.",
        "a window density exceeds the selected finite-window bound",
        "window bound provenance and checked local-density estimate",
    )
}
