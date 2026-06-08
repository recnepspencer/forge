use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::ExactArithmeticIntervalCertificate,
        "exact_arithmetic_interval_certificate",
        "Exact arithmetic / interval certificate test",
        T::ExactCheckerReady,
        A::RegionGeometry,
        "Final conflict and safety claims must replay without trusting floating point.",
        "a claimed safe or conflict pair lacks an exact/interval certificate",
        "exact arithmetic or interval certificate for every relevant pair",
    )
}
