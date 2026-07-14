mod conditioning;
mod exhaustion;
mod geometry_identity;
mod private_support;
mod schema;
mod support;
mod witnesses;

pub(crate) use conditioning::{
    conditioning_witness, conditioning_witness_with_normalization, feature_size_collapsed,
};
pub use exhaustion::{
    PrimitiveRealizationError, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionReport,
};
pub use schema::{
    build_direct_realization_report, PrimitiveConditioningWitness,
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass, PrimitiveSupportRealization,
};
pub use support::{
    realize_block_support, realize_prism_support, realize_pyramid_support,
    realize_tetrahedron_support, realize_tetrahedron_support_with_altitude_component,
};
pub use witnesses::{
    primitive_realization_exhaustion_witness_rows, PrimitiveRealizationExhaustionWitnessKind,
    PrimitiveRealizationExhaustionWitnessRow,
};

#[cfg(test)]
#[path = "../shape_realization_tests.rs"]
mod tests;
