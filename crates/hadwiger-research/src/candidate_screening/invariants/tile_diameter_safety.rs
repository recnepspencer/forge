use super::{definition_row, A, F, T};
use crate::candidate_screening::definitions::CandidateScreeningInvariantDefinition;

pub(crate) fn definition() -> CandidateScreeningInvariantDefinition {
    definition_row(
        F::TileDiameterSafety,
        "tile_diameter_safety",
        "Tile diameter safety test",
        T::CertificateRequired,
        A::RegionGeometry,
        "Every individual color region must avoid internal unit distances.",
        "tile diameter is at least 1 without an exact internal-distance clearance certificate",
        "exact diameter or internal-distance certificate",
    )
}
