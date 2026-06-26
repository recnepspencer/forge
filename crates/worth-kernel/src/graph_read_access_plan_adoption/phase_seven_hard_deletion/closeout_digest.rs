use crate::graph_read_access_plan_adoption::WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed;

use super::capped_residue::WorthGraphReadAccessHardDeletionCappedResidueReport;
use super::deletion_proof::WorthGraphReadAccessHardDeletionProofReport;
use super::source_firewall::WorthGraphReadAccessHardDeletionSourceFirewallReport;
use super::stable_digest;

pub(crate) fn hard_deletion_closeout_digest(
    seed: &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    deletion_proof_report: &WorthGraphReadAccessHardDeletionProofReport,
    capped_residue_report: &WorthGraphReadAccessHardDeletionCappedResidueReport,
    source_firewall_report: &WorthGraphReadAccessHardDeletionSourceFirewallReport,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_hard_deletion_closeout_v1".to_string(),
        format!("phase_seven_seed:{}", seed.seed_digest()),
        format!("phase_six:{}", seed.phase_six_closeout_digest()),
        format!(
            "receipt:{}",
            seed.receipt_accounting_report().report_digest()
        ),
        format!(
            "counter:{}",
            seed.counter_accounting_report().report_digest()
        ),
        format!("batch:{}", seed.batch_accounting_report().report_digest()),
        format!("deletion:{}", deletion_proof_report.report_digest()),
        format!("residue:{}", capped_residue_report.report_digest()),
        format!("firewall:{}", source_firewall_report.report_digest()),
    ])
}
