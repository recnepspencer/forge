use crate::graph_read_access_plan_adoption::WorthGraphReadAccessHardDeletionPhaseEightSeed;

use super::super::proof_exports::{
    WorthGraphReadAccessPlanAdoptionDeletionExport, WorthGraphReadAccessPlanAdoptionPostureExport,
    WorthGraphReadAccessPlanAdoptionReceiptExport, WorthGraphReadAccessPlanAdoptionResidueExport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
};
use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionCloseoutCounters {
    executed_receipt_count: usize,
    receipt_row_count: usize,
    admitted_plan_count: usize,
    admitted_plan_requires_receipt_count: usize,
    required_posture_count: usize,
    denied_posture_count: usize,
    carried_gap_count: usize,
    visible_non_executed_posture_count: usize,
    required_future_receipt_count: usize,
    no_receipt_posture_count: usize,
    accounted_counter_row_count: usize,
    explicit_counter_gap_count: usize,
    no_execution_counter_required_count: usize,
    caller_owned_graph_work_count: usize,
    batch_row_count: usize,
    deleted_path_count: usize,
    capped_residue_count: usize,
    uncapped_residue_count: usize,
    source_firewall_region_count: usize,
    source_firewall_source_count: usize,
    source_firewall_violation_count: usize,
    posture_projection_count: usize,
    cap_row_count: usize,
    counter_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionCloseoutCounters {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_seed_and_exports(
        seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
        receipts: &WorthGraphReadAccessPlanAdoptionReceiptExport,
        postures: &WorthGraphReadAccessPlanAdoptionPostureExport,
        deletion: &WorthGraphReadAccessPlanAdoptionDeletionExport,
        residue: &WorthGraphReadAccessPlanAdoptionResidueExport,
        source_firewall: &WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
    ) -> Self {
        let executed_receipt_count = receipts.executed_receipt_count();
        let receipt_row_count = receipts.report().rows().len();
        let admitted_plan_requires_receipt_count = receipts.admitted_plan_requires_receipt_count();
        let admitted_plan_count = executed_receipt_count + admitted_plan_requires_receipt_count;
        let required_posture_count = receipts.required_posture_count();
        let denied_posture_count = receipts.denied_posture_count();
        let carried_gap_count = receipts.carried_gap_count();
        let visible_non_executed_posture_count = receipts.visible_non_executed_posture_count();
        let required_future_receipt_count = seed
            .receipt_accounting_report()
            .required_future_receipt_count();
        let no_receipt_posture_count = seed.receipt_accounting_report().no_receipt_posture_count();
        let accounted_counter_row_count = seed
            .counter_accounting_report()
            .accounted_counter_row_count();
        let explicit_counter_gap_count = seed
            .counter_accounting_report()
            .explicit_counter_gap_count();
        let no_execution_counter_required_count = seed
            .counter_accounting_report()
            .no_execution_counter_required_count();
        let caller_owned_graph_work_count = seed
            .counter_accounting_report()
            .caller_owned_graph_work_count()
            + seed
                .batch_accounting_report()
                .caller_owned_graph_work_count();
        let batch_row_count = seed.batch_accounting_report().rows().len();
        let deleted_path_count = deletion.deleted_count();
        let capped_residue_count = residue.residue_count();
        let uncapped_residue_count = residue.uncapped_residue_count();
        let source_firewall_region_count = source_firewall.scanned_region_count();
        let source_firewall_source_count = source_firewall.scanned_source_count();
        let source_firewall_violation_count = source_firewall.violation_count();
        let posture_projection_count = postures.posture_projections().len();
        let cap_row_count = postures.cap_rows().len();
        let counter_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_closeout_counters_v1".to_string(),
            format!("executed_receipts:{executed_receipt_count}"),
            format!("receipt_rows:{receipt_row_count}"),
            format!("admitted_plans:{admitted_plan_count}"),
            format!("pending_plans:{admitted_plan_requires_receipt_count}"),
            format!("required_postures:{required_posture_count}"),
            format!("denied_postures:{denied_posture_count}"),
            format!("carried_gaps:{carried_gap_count}"),
            format!("visible_non_executed:{visible_non_executed_posture_count}"),
            format!("required_future:{required_future_receipt_count}"),
            format!("no_receipt:{no_receipt_posture_count}"),
            format!("accounted_counter_rows:{accounted_counter_row_count}"),
            format!("explicit_counter_gaps:{explicit_counter_gap_count}"),
            format!("no_execution_counter:{no_execution_counter_required_count}"),
            format!("caller_owned_graph_work:{caller_owned_graph_work_count}"),
            format!("batch_rows:{batch_row_count}"),
            format!("deleted:{deleted_path_count}"),
            format!("capped_residue:{capped_residue_count}"),
            format!("uncapped_residue:{uncapped_residue_count}"),
            format!("firewall_regions:{source_firewall_region_count}"),
            format!("firewall_sources:{source_firewall_source_count}"),
            format!("firewall_violations:{source_firewall_violation_count}"),
            format!("postures:{posture_projection_count}"),
            format!("cap_rows:{cap_row_count}"),
        ]);
        Self {
            executed_receipt_count,
            receipt_row_count,
            admitted_plan_count,
            admitted_plan_requires_receipt_count,
            required_posture_count,
            denied_posture_count,
            carried_gap_count,
            visible_non_executed_posture_count,
            required_future_receipt_count,
            no_receipt_posture_count,
            accounted_counter_row_count,
            explicit_counter_gap_count,
            no_execution_counter_required_count,
            caller_owned_graph_work_count,
            batch_row_count,
            deleted_path_count,
            capped_residue_count,
            uncapped_residue_count,
            source_firewall_region_count,
            source_firewall_source_count,
            source_firewall_violation_count,
            posture_projection_count,
            cap_row_count,
            counter_digest,
        }
    }

    pub const fn executed_receipt_count(&self) -> usize {
        self.executed_receipt_count
    }

    pub const fn receipt_row_count(&self) -> usize {
        self.receipt_row_count
    }

    pub const fn admitted_plan_count(&self) -> usize {
        self.admitted_plan_count
    }

    pub const fn admitted_plan_requires_receipt_count(&self) -> usize {
        self.admitted_plan_requires_receipt_count
    }

    pub const fn required_posture_count(&self) -> usize {
        self.required_posture_count
    }

    pub const fn denied_posture_count(&self) -> usize {
        self.denied_posture_count
    }

    pub const fn carried_gap_count(&self) -> usize {
        self.carried_gap_count
    }

    pub const fn visible_non_executed_posture_count(&self) -> usize {
        self.visible_non_executed_posture_count
    }

    pub const fn required_future_receipt_count(&self) -> usize {
        self.required_future_receipt_count
    }

    pub const fn no_receipt_posture_count(&self) -> usize {
        self.no_receipt_posture_count
    }

    pub const fn accounted_counter_row_count(&self) -> usize {
        self.accounted_counter_row_count
    }

    pub const fn explicit_counter_gap_count(&self) -> usize {
        self.explicit_counter_gap_count
    }

    pub const fn no_execution_counter_required_count(&self) -> usize {
        self.no_execution_counter_required_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub const fn batch_row_count(&self) -> usize {
        self.batch_row_count
    }

    pub const fn deleted_path_count(&self) -> usize {
        self.deleted_path_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub const fn uncapped_residue_count(&self) -> usize {
        self.uncapped_residue_count
    }

    pub const fn source_firewall_region_count(&self) -> usize {
        self.source_firewall_region_count
    }

    pub const fn source_firewall_source_count(&self) -> usize {
        self.source_firewall_source_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn posture_projection_count(&self) -> usize {
        self.posture_projection_count
    }

    pub const fn cap_row_count(&self) -> usize {
        self.cap_row_count
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}
