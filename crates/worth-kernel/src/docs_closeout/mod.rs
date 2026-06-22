mod error;
mod model;
mod reports;
mod scan;

pub use error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};
pub use model::doc_metadata::{WorthDocKind, WorthDocMetadata};
pub use model::invariant_evidence::WorthDocsInvariantEvidence;
pub use model::report_context::WorthDocsReportContext;
pub use reports::backfill_report::{
    current_worth_docs_backfill_report, worth_docs_backfill_report_for_root,
    WorthDocsBackfillReport, WorthDocsBackfillRow, WorthDocsBackfillStatus,
};
pub use reports::boundary_doc_coverage_matrix::{
    current_worth_boundary_doc_coverage_matrix, worth_boundary_doc_coverage_matrix_for_root,
    WorthBoundaryDocCoverageMatrix, WorthBoundaryDocCoverageRow, WorthBoundaryDocCoverageStatus,
};
pub use reports::crate_surface_report::{
    current_worth_crate_docs_surface_report, worth_crate_docs_surface_report_for_root,
    WorthCrateDocsSurfaceReport, WorthCrateDocsSurfaceRow, WorthCrateDocsSurfaceStatus,
};
pub use reports::feature_doc_coverage_matrix::{
    current_worth_feature_doc_coverage_matrix, worth_feature_doc_coverage_matrix_for_root,
    WorthFeatureDocCoverageMatrix, WorthFeatureDocCoverageRow, WorthFeatureDocCoverageStatus,
};
pub use scan::docs_graph::{
    current_worth_docs_graph, worth_docs_graph_for_root, WorthDocsGraph, WorthDocsGraphEdge,
    WorthDocsGraphEdgeKind, WorthDocsGraphUnresolvedLink,
};

#[cfg(test)]
mod tests;
