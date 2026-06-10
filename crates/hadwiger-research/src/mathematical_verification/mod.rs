mod algebraic_unit_distance;
mod exact_unit_distance;
mod finite_colorability;
mod hexagonal_plane_coloring;

pub use algebraic_unit_distance::{
    verify_algebraic_unit_distance_embedding_checked, AlgebraicGraphEmbedding,
    AlgebraicGraphEmbeddingBuilder, AlgebraicPoint2, AlgebraicScalar,
    AlgebraicUnitDistanceCertificate, HadwigerAlgebraicGeometryError, QuadraticFieldElement,
};
pub use exact_unit_distance::{
    verify_unit_distance_embedding_checked, ExactGraphEmbedding, ExactGraphEmbeddingBuilder,
    ExactPoint2, ExactRational, HadwigerExactGeometryError, UnitDistanceVerificationChecked,
};
pub use finite_colorability::{
    generate_k_colorability_certificate_with_varisat_checked, verify_k_colorability_checked,
    verify_k_colorability_with_certificate_checked, ColoringProofCertificate,
    ColoringProofCertificateFormat, ColoringRefutationReplayReport, HadwigerColorabilityError,
    KColorabilityVerificationChecked,
};
pub use hexagonal_plane_coloring::{
    verify_hexagonal_seven_coloring_checked, HadwigerPlaneColoringError,
    HexagonalSevenColoringConstruction, HexagonalSevenColoringVerificationChecked,
    WholePlaneColoringConstruction, WholePlaneColoringVerification,
};

pub(crate) use exact_unit_distance::{admitted_declaration_reference, checker_evidence};
