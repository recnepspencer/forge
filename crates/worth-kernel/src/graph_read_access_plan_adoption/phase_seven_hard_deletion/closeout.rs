use std::path::{Path, PathBuf};

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed;

use super::capped_residue::WorthGraphReadAccessHardDeletionCappedResidueReport;
use super::closeout_digest::hard_deletion_closeout_digest;
use super::deletion_proof::WorthGraphReadAccessHardDeletionProofReport;
use super::errors::{
    WorthGraphReadAccessHardDeletionError, WorthGraphReadAccessHardDeletionErrorKind,
};
use super::phase_eight_seed::{
    WorthGraphReadAccessHardDeletionPhaseEightSeed,
    WorthGraphReadAccessHardDeletionPhaseEightSeedInput,
};
use super::source_firewall::{
    scan_workspace, WorthGraphReadAccessHardDeletionSourceFirewallReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionCloseout {
    phase_seven_seed_digest: String,
    deletion_proof_report: WorthGraphReadAccessHardDeletionProofReport,
    capped_residue_report: WorthGraphReadAccessHardDeletionCappedResidueReport,
    source_firewall_report: WorthGraphReadAccessHardDeletionSourceFirewallReport,
    phase_eight_seed: WorthGraphReadAccessHardDeletionPhaseEightSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_hard_deletion_closeout(
    seed: &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
) -> Result<WorthGraphReadAccessHardDeletionCloseout, WorthGraphReadAccessHardDeletionError> {
    closeout_for_workspace_root(seed, &workspace_root_from_manifest())
}

pub(crate) fn closeout_for_workspace_root(
    seed: &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    workspace_root: &Path,
) -> Result<WorthGraphReadAccessHardDeletionCloseout, WorthGraphReadAccessHardDeletionError> {
    reject_invalid_seed(seed)?;
    let deletion_proof_report =
        WorthGraphReadAccessHardDeletionProofReport::from_workspace_root(seed, workspace_root)?;
    let capped_residue_report =
        WorthGraphReadAccessHardDeletionCappedResidueReport::from_deletion_proof(
            &deletion_proof_report,
        )?;
    let source_firewall_report = scan_workspace(workspace_root).map_err(|_| {
        WorthGraphReadAccessHardDeletionError::new(
            WorthGraphReadAccessHardDeletionErrorKind::SourceFirewallViolation,
        )
    })?;
    let closeout_digest = hard_deletion_closeout_digest(
        seed,
        &deletion_proof_report,
        &capped_residue_report,
        &source_firewall_report,
    );
    let phase_eight_seed = WorthGraphReadAccessHardDeletionPhaseEightSeed::from_input(
        WorthGraphReadAccessHardDeletionPhaseEightSeedInput {
            phase_seven_closeout_digest: closeout_digest.clone(),
            source_seed: seed.clone(),
            deletion_proof_report: deletion_proof_report.clone(),
            capped_residue_report: capped_residue_report.clone(),
            source_firewall_report: source_firewall_report.clone(),
        },
    );
    Ok(WorthGraphReadAccessHardDeletionCloseout {
        phase_seven_seed_digest: seed.seed_digest().to_string(),
        deletion_proof_report,
        capped_residue_report,
        source_firewall_report,
        phase_eight_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessHardDeletionCloseout {
    pub fn phase_seven_seed_digest(&self) -> &str {
        &self.phase_seven_seed_digest
    }

    pub const fn deletion_proof_report(&self) -> &WorthGraphReadAccessHardDeletionProofReport {
        &self.deletion_proof_report
    }

    pub const fn capped_residue_report(
        &self,
    ) -> &WorthGraphReadAccessHardDeletionCappedResidueReport {
        &self.capped_residue_report
    }

    pub const fn source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessHardDeletionSourceFirewallReport {
        &self.source_firewall_report
    }

    pub const fn phase_eight_seed(&self) -> &WorthGraphReadAccessHardDeletionPhaseEightSeed {
        &self.phase_eight_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

fn reject_invalid_seed(
    seed: &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
) -> Result<(), WorthGraphReadAccessHardDeletionError> {
    if seed.claims_validator_selection() {
        return Err(WorthGraphReadAccessHardDeletionError::new(
            WorthGraphReadAccessHardDeletionErrorKind::SeedAlreadyClaimsValidatorSelection,
        ));
    }
    if seed.receipt_accounting_report().rows().is_empty() {
        return Err(WorthGraphReadAccessHardDeletionError::new(
            WorthGraphReadAccessHardDeletionErrorKind::MissingReceiptAccountingProof,
        ));
    }
    if seed.counter_accounting_report().rows().is_empty() {
        return Err(WorthGraphReadAccessHardDeletionError::new(
            WorthGraphReadAccessHardDeletionErrorKind::MissingCounterAccountingProof,
        ));
    }
    if seed.batch_accounting_report().rows().is_empty() {
        return Err(WorthGraphReadAccessHardDeletionError::new(
            WorthGraphReadAccessHardDeletionErrorKind::MissingBatchAccountingProof,
        ));
    }
    Ok(())
}

fn workspace_root_from_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel manifest should live under workspace/crates/worth-kernel")
        .to_path_buf()
}
