mod catalog;
mod catalog_additions;
mod closeout;
mod counters;
mod cut_line;
mod discovery;
mod error;
mod query_support_rows;
mod row;
mod row_builder;
mod scan_pattern;
mod source_firewall;

#[cfg(test)]
mod phase_eleven_consumer_sweep_tests;
#[cfg(test)]
mod repair_tests;
#[cfg(test)]
mod source_firewall_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use closeout::{current_conflict_batch_admission_inventory, ConflictBatchAdmissionInventory};
pub use counters::ConflictBatchAdmissionInventoryCounters;
pub use cut_line::ConflictBatchAdmissionCutLine;
pub use discovery::{
    ConflictBatchAdmissionDiscoveredSurface, ConflictBatchAdmissionDiscoveryReport,
    ConflictBatchAdmissionReconciliation,
};
pub use error::ConflictBatchAdmissionInventoryError;
pub use row::{
    ConflictBatchAdmissionAuthorityKind, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionOwner,
    ConflictBatchAdmissionQuerySurface, ConflictBatchAdmissionReplacementPhase,
    ConflictBatchAdmissionRowScope, ConflictBatchAdmissionSurfaceIdentity,
};
pub use scan_pattern::ConflictBatchAdmissionScanPattern;
pub use source_firewall::{
    ConflictBatchAdmissionSourceFirewallReport, ConflictBatchAdmissionSourceFirewallViolation,
};

#[cfg(test)]
pub(crate) use row_builder::ConflictBatchAdmissionInventoryRowBuilder;
