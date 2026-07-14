mod certification;
mod model;
mod registry;

pub use certification::certify_declarative_product_boundary;
pub use model::{
    WorthQueryProductBoundaryCertificationBundle, WorthQueryProductBoundaryCertificationError,
    WorthQueryProductBoundaryEvidenceKind, WorthQueryProductBoundaryEvidenceRow,
    WorthQueryProductBoundaryHostileCase, WorthQueryProductBoundarySabotageCase,
};
pub use registry::worth_query_product_boundary_evidence_rows;

#[cfg(test)]
mod tests;
