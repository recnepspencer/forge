pub mod data;
pub mod logic;
mod validated_mutation_footprint;

pub use validated_mutation_footprint::{
    ValidatedMutationFootprint, ValidatedMutationFootprintNotRequested,
    ValidatedMutationFootprintProjection, ValidatedMutationFootprintWork,
};
