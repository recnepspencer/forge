mod boundary_audit;
mod closeout;
mod counters;
mod error;
mod evidence_report;
mod requirement_row;
mod residue_audit;
mod row;
mod source_set;
mod support_pinning;
mod support_snapshot;

#[cfg(test)]
mod tests;

pub use closeout::{
    current_evidence_lookup_query_consumer_kit, EvidenceLookupQueryConsumerKitCloseout,
};
pub use counters::EvidenceLookupQueryConsumerKitCounters;
pub use error::{EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind};
pub(crate) use requirement_row::EvidenceLookupQuerySupportRequirementRow;
pub(crate) use residue_audit::{
    audit_evidence_lookup_query_consumer_residue_for_roots, residue_rows_from_report,
};
pub use row::{
    EvidenceLookupQueryConsumerKitBindingRow, EvidenceLookupQueryConsumerResidueRow,
    EvidenceLookupQuerySupportPinRow,
};
pub(crate) use source_set::evidence_lookup_query_consumer_kit_residue_roots;
pub(crate) use support_pinning::{
    derived_support_requirements, evidence_lookup_query_support_pinning_contract,
};
pub(crate) use support_snapshot::project_evidence_lookup_query_support_snapshot;
