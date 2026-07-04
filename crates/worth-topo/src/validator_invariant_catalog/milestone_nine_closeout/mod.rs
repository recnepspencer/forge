mod authority_occurrence_inventory;
mod closeout;
mod counters;
mod current;
mod deletion_ledger;
mod denial;
mod milestone_ten_seed;
mod public_proof;
mod residue_report;
mod source_firewall;

pub use authority_occurrence_inventory::{
    WorthTopologyMilestoneNineAuthorityOccurrenceInventory,
    WorthTopologyMilestoneNineAuthorityOccurrenceInventoryRow,
    WorthTopologyMilestoneNineAuthorityOccurrenceStatus,
};
pub use closeout::WorthTopologyMilestoneNineCloseout;
pub use counters::WorthTopologyMilestoneNineCloseoutCounters;
pub use current::current_topology_validator_invariant_milestone_nine_closeout;
pub use deletion_ledger::{
    WorthTopologyMilestoneNineDeletionDisposition, WorthTopologyMilestoneNineDeletionLedgerReport,
    WorthTopologyMilestoneNineDeletionLedgerRow,
};
pub use denial::{
    WorthTopologyMilestoneNineCloseoutDenial, WorthTopologyMilestoneNineCloseoutDenialKind,
};
pub use milestone_ten_seed::WorthTopologyMilestoneTenSeed;
pub use public_proof::WorthTopologyMilestoneNinePublicProof;
pub use residue_report::{
    WorthTopologyMilestoneNineResidueAuditReport, WorthTopologyMilestoneNineResidueAuditRow,
    WorthTopologyMilestoneNineResidueStatus,
};
pub use source_firewall::WorthTopologyMilestoneNineSourceFirewallReport;

pub(crate) fn stable_digest(parts: &[String]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b"\0");
    }
    format!("{:x}", hasher.finalize())
}
