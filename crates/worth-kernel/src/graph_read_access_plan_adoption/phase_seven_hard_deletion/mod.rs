mod capped_residue;
mod closeout;
mod closeout_digest;
mod deletion_proof;
mod errors;
mod phase_eight_seed;
mod source_firewall;

#[cfg(test)]
mod tests;

pub use capped_residue::{
    WorthGraphReadAccessHardDeletionCappedResidueReport,
    WorthGraphReadAccessHardDeletionCappedResidueRow,
};
pub use closeout::{
    current_worth_graph_read_access_hard_deletion_closeout,
    WorthGraphReadAccessHardDeletionCloseout,
};
pub use deletion_proof::{
    WorthGraphReadAccessHardDeletionProofReport, WorthGraphReadAccessHardDeletionProofRow,
    WorthGraphReadAccessHardDeletionStatus,
};
pub use errors::{
    WorthGraphReadAccessHardDeletionError, WorthGraphReadAccessHardDeletionErrorKind,
};
pub use phase_eight_seed::WorthGraphReadAccessHardDeletionPhaseEightSeed;
pub use source_firewall::{
    WorthGraphReadAccessHardDeletionSourceFirewallRegionRow,
    WorthGraphReadAccessHardDeletionSourceFirewallReport,
    WorthGraphReadAccessHardDeletionSourceFirewallViolation,
};

pub(crate) fn stable_digest(parts: &[String]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b"\0");
    }
    format!("{:x}", hasher.finalize())
}
