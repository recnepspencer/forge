use std::path::{Path, PathBuf};

use super::capped_residue::WorthGraphReadDeclarationCappedResidueReport;
use super::deletion_ledger::WorthGraphReadDeclarationDeletionLedgerReport;
use super::errors::{
    WorthGraphReadDeclarationDeletionFirewallError,
    WorthGraphReadDeclarationDeletionFirewallErrorKind,
};
use super::phase_seven_seed::WorthGraphReadAccessDeclarationPhaseSevenSeed;
use super::source_firewall::WorthGraphReadDeclarationSourceFirewallReport;
use super::stable_identity_digest::stable_digest;
use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationPhaseSixSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationDeletionFirewallCloseout {
    deletion_ledger_report: WorthGraphReadDeclarationDeletionLedgerReport,
    capped_residue_report: WorthGraphReadDeclarationCappedResidueReport,
    source_firewall_report: WorthGraphReadDeclarationSourceFirewallReport,
    phase_seven_seed: WorthGraphReadAccessDeclarationPhaseSevenSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_declaration_deletion_firewall_closeout(
    seed: &WorthGraphReadAccessDeclarationPhaseSixSeed,
) -> Result<
    WorthGraphReadDeclarationDeletionFirewallCloseout,
    WorthGraphReadDeclarationDeletionFirewallError,
> {
    closeout_for_workspace_root(seed, &workspace_root_from_manifest())
}

pub(crate) fn closeout_for_workspace_root(
    seed: &WorthGraphReadAccessDeclarationPhaseSixSeed,
    workspace_root: &Path,
) -> Result<
    WorthGraphReadDeclarationDeletionFirewallCloseout,
    WorthGraphReadDeclarationDeletionFirewallError,
> {
    reject_execution_shaped_seed(seed)?;
    if seed.posture_records().is_empty() {
        return Err(error(
            WorthGraphReadDeclarationDeletionFirewallErrorKind::MissingAdmissionPostureRecord,
        ));
    }
    let deletion_ledger_report =
        WorthGraphReadDeclarationDeletionLedgerReport::from_deletion_items(
            seed.deletion_items(),
            workspace_root,
        )?;
    let source_firewall_report =
        WorthGraphReadDeclarationSourceFirewallReport::scan_workspace(workspace_root)?;
    let capped_residue_report = WorthGraphReadDeclarationCappedResidueReport::from_deletion_report(
        &deletion_ledger_report,
    )?;
    let closeout_digest = closeout_digest(
        seed,
        &deletion_ledger_report,
        &capped_residue_report,
        &source_firewall_report,
    );
    let phase_seven_seed = WorthGraphReadAccessDeclarationPhaseSevenSeed::new(
        seed.admission_closeout_digest(),
        closeout_digest.clone(),
        deletion_ledger_report.clone(),
        capped_residue_report.clone(),
        source_firewall_report.clone(),
        seed.posture_records().to_vec(),
        seed.admission_capability_gaps().to_vec(),
        seed.carried_requirement_derivation_gaps().to_vec(),
    );
    Ok(WorthGraphReadDeclarationDeletionFirewallCloseout {
        deletion_ledger_report,
        capped_residue_report,
        source_firewall_report,
        phase_seven_seed,
        closeout_digest,
    })
}

impl WorthGraphReadDeclarationDeletionFirewallCloseout {
    pub fn deletion_ledger_report(&self) -> &WorthGraphReadDeclarationDeletionLedgerReport {
        &self.deletion_ledger_report
    }

    pub fn capped_residue_report(&self) -> &WorthGraphReadDeclarationCappedResidueReport {
        &self.capped_residue_report
    }

    pub fn source_firewall_report(&self) -> &WorthGraphReadDeclarationSourceFirewallReport {
        &self.source_firewall_report
    }

    pub fn phase_seven_seed(&self) -> &WorthGraphReadAccessDeclarationPhaseSevenSeed {
        &self.phase_seven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}

fn reject_execution_shaped_seed(
    seed: &WorthGraphReadAccessDeclarationPhaseSixSeed,
) -> Result<(), WorthGraphReadDeclarationDeletionFirewallError> {
    if seed.claims_graph_read_execution() {
        return Err(error(
            WorthGraphReadDeclarationDeletionFirewallErrorKind::SeedClaimedExecutionAuthority,
        ));
    }
    if seed.claims_access_plan_consumption() {
        return Err(error(
            WorthGraphReadDeclarationDeletionFirewallErrorKind::SeedClaimedAccessPlanConsumption,
        ));
    }
    Ok(())
}

fn closeout_digest(
    seed: &WorthGraphReadAccessDeclarationPhaseSixSeed,
    deletion_ledger_report: &WorthGraphReadDeclarationDeletionLedgerReport,
    capped_residue_report: &WorthGraphReadDeclarationCappedResidueReport,
    source_firewall_report: &WorthGraphReadDeclarationSourceFirewallReport,
) -> String {
    stable_digest(&[
        "worth_graph_read_declaration_deletion_firewall_closeout_v1".to_string(),
        format!("admission_closeout:{}", seed.admission_closeout_digest()),
        format!("deletion_ledger:{}", deletion_ledger_report.report_digest()),
        format!("capped_residue:{}", capped_residue_report.report_digest()),
        format!("source_firewall:{}", source_firewall_report.report_digest()),
    ])
}

fn workspace_root_from_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel manifest should live under workspace/crates/worth-kernel")
        .to_path_buf()
}

const fn error(
    kind: WorthGraphReadDeclarationDeletionFirewallErrorKind,
) -> WorthGraphReadDeclarationDeletionFirewallError {
    WorthGraphReadDeclarationDeletionFirewallError::new(kind)
}
