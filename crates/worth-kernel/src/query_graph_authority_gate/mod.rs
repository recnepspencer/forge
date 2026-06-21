mod closeout_bypass;
mod closeout_counters;
mod closeout_deletion_evidence;
mod closeout_doc;
mod closeout_facade;
mod closeout_report;
mod closeout_types;
mod gate_report_types;
mod registry;
mod report;
mod source_discovery;
mod touched_graph_certification;
mod touched_graph_facade_audit;
mod touched_graph_inventory;
mod touched_graph_static_authority;
mod touched_graph_types;
mod types;

pub use closeout_counters::WorthGraphAuthorityCloseoutCounters;
pub use closeout_report::{
    current_worth_graph_authority_closeout_report, WorthGraphAuthorityCloseoutReport,
    WorthGraphAuthorityCloseoutViolation,
};
pub use closeout_types::{
    WorthGraphAuthorityCloseoutBypassClass, WorthGraphAuthorityCloseoutBypassEvidence,
    WorthGraphAuthorityCloseoutDisposition, WorthGraphAuthorityCloseoutMatrixRow,
    WorthGraphAuthorityDeletionClassCloseoutEvidence, WorthGraphAuthorityPublicFacadeEvidence,
    WorthGraphAuthorityPublicFacadeProof,
};
pub use gate_report_types::{WorthGraphAuthorityGateCounters, WorthGraphAuthorityGateReport};
pub use registry::{
    current_worth_graph_authority_deletion_ledger, current_worth_graph_authority_discovery_records,
    current_worth_graph_authority_inventory, current_worth_lower_authority_promotion_guard_plan,
};
pub(crate) use report::certify_worth_graph_authority_gate;
pub use report::WorthGraphAuthorityGateViolation;
pub use source_discovery::{
    current_worth_graph_authority_audited_source_paths, worth_graph_authority_audited_source_roots,
};
pub use touched_graph_inventory::{
    current_worth_touched_graph_authority_inventory, current_worth_touched_graph_deletion_ledger,
};
pub use touched_graph_types::{
    WorthTouchedGraphAuthorityDeletionLedgerRow, WorthTouchedGraphAuthorityDisposition,
    WorthTouchedGraphAuthorityInventoryCategory, WorthTouchedGraphAuthorityInventoryRow,
};
pub use types::{
    WorthGraphAuthorityAction, WorthGraphAuthorityDeletionLedgerRow,
    WorthGraphAuthorityDeletionTarget, WorthGraphAuthorityDiscoveryRecord,
    WorthGraphAuthorityDiscoverySource, WorthGraphAuthorityInventoryRow, WorthGraphAuthorityOwner,
    WorthGraphAuthorityRootFamily, WorthGraphAuthorityRowClass, WorthGraphAuthoritySourceScope,
    WorthLowerAuthorityPromotionCase, WorthLowerAuthorityPromotionGuardPlan,
};

pub fn current_worth_graph_authority_gate_report(
) -> Result<WorthGraphAuthorityGateReport, WorthGraphAuthorityGateViolation> {
    if !crate::construction::graph_obligation_adoption::primitive_construction_graph_obligation_execution_closeout_passes()
    {
        return Err(WorthGraphAuthorityGateViolation::PrimitiveConstructionBirthExecutionNotCovered);
    }
    certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        current_worth_graph_authority_deletion_ledger(),
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
}

#[cfg(test)]
mod closeout_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod touched_graph_facade_tests;
#[cfg(test)]
mod touched_graph_tests;
