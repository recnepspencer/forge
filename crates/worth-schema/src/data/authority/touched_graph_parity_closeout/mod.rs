mod architecture_claim;
mod coverage_row;
mod error;
mod family_kind;
mod readiness_input;
mod residue_classification;

#[cfg(test)]
mod tests;

pub(crate) use architecture_claim::admit_touched_graph_parity_readiness_claim;
pub use architecture_claim::{TouchedGraphParityArchitectureClaim, TouchedGraphParityClaimKind};
pub use coverage_row::{
    TouchedGraphParityCoverageContributor, TouchedGraphParityCoverageRow,
    TouchedGraphParityQuerySurfaceKind,
};
pub use error::{TouchedGraphParityReadinessError, TouchedGraphParityReadinessErrorKind};
pub use family_kind::TouchedGraphParityFamilyKind;
pub(crate) use readiness_input::admit_touched_graph_parity_readiness_input;
pub use readiness_input::TouchedGraphParityReadinessInput;
pub use residue_classification::TouchedGraphParityResidueClassification;
