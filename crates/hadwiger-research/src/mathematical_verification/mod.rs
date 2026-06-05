mod exact_unit_distance;
mod finite_colorability;
mod hexagonal_plane_coloring;

pub use exact_unit_distance::{
    verify_unit_distance_embedding_checked, ExactGraphEmbedding, ExactGraphEmbeddingBuilder,
    ExactPoint2, ExactRational, HadwigerExactGeometryError, UnitDistanceVerificationChecked,
};
pub use finite_colorability::{
    verify_k_colorability_checked, HadwigerColorabilityError, KColorabilityVerificationChecked,
};
pub use hexagonal_plane_coloring::{
    verify_hexagonal_seven_coloring_checked, HadwigerPlaneColoringError,
    HexagonalSevenColoringConstruction, HexagonalSevenColoringVerificationChecked,
    WholePlaneColoringConstruction, WholePlaneColoringVerification,
};

pub(crate) use exact_unit_distance::{admitted_declaration_reference, checker_evidence};
