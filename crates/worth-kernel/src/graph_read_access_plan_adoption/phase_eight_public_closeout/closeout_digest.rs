use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionPhaseEightSeed;

use super::closeout_counters::WorthGraphReadAccessPlanAdoptionCloseoutCounters;
use super::proof_exports::{
    WorthGraphReadAccessPlanAdoptionDeletionExport, WorthGraphReadAccessPlanAdoptionPostureExport,
    WorthGraphReadAccessPlanAdoptionReceiptExport, WorthGraphReadAccessPlanAdoptionResidueExport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
};
use super::stable_digest;

pub(crate) fn plan_adoption_closeout_digest(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
    receipts: &WorthGraphReadAccessPlanAdoptionReceiptExport,
    postures: &WorthGraphReadAccessPlanAdoptionPostureExport,
    deletion: &WorthGraphReadAccessPlanAdoptionDeletionExport,
    residue: &WorthGraphReadAccessPlanAdoptionResidueExport,
    source_firewall: &WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
    counters: &WorthGraphReadAccessPlanAdoptionCloseoutCounters,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_plan_adoption_closeout_v1".to_string(),
        format!("phase_eight_seed:{}", seed.seed_digest()),
        format!("phase_seven:{}", seed.phase_seven_closeout_digest()),
        format!("phase_six:{}", seed.phase_six_closeout_digest()),
        format!("receipt_export:{}", receipts.export_digest()),
        format!("posture_export:{}", postures.export_digest()),
        format!("deletion_export:{}", deletion.export_digest()),
        format!("residue_export:{}", residue.export_digest()),
        format!("source_firewall_export:{}", source_firewall.export_digest()),
        format!("counter_export:{}", counters.counter_digest()),
        format!("batch:{}", seed.batch_accounting_report().report_digest()),
        format!(
            "bounded_execution:{}",
            seed.bounded_execution_contract().contract_digest()
        ),
        format!(
            "phase_four_cutover:{}",
            seed.phase_four_cutover_proof().cutover_digest()
        ),
    ])
}
