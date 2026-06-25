use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSpatialDensePhaseSixSeed;

use super::batch_accounting::{
    build_batch_accounting_report, WorthGraphReadAccessBatchAccountingReport,
};
use super::closeout_digest::execution_receipt_accounting_closeout_digest;
use super::counter_accounting::{
    build_counter_accounting_report, WorthGraphReadAccessCounterAccountingReport,
};
use super::errors::{
    WorthGraphReadAccessExecutionReceiptAccountingError,
    WorthGraphReadAccessExecutionReceiptAccountingErrorKind,
};
use super::phase_seven_seed::{
    WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeedInput,
};
use super::receipt_accounting::{
    build_receipt_accounting_report, WorthGraphReadAccessReceiptAccountingReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessExecutionReceiptAccountingCloseout {
    phase_six_seed_digest: String,
    receipt_accounting_report: WorthGraphReadAccessReceiptAccountingReport,
    counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    phase_seven_seed: WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_execution_receipt_accounting_closeout(
    seed: &WorthGraphReadAccessSpatialDensePhaseSixSeed,
) -> Result<
    WorthGraphReadAccessExecutionReceiptAccountingCloseout,
    WorthGraphReadAccessExecutionReceiptAccountingError,
> {
    reject_invalid_seed(seed)?;
    let receipt_accounting_report = build_receipt_accounting_report(seed);
    reject_empty_receipt_accounting(&receipt_accounting_report)?;
    let counter_accounting_report =
        build_counter_accounting_report(seed, &receipt_accounting_report);
    reject_caller_owned_graph_work(&counter_accounting_report)?;
    let batch_accounting_report =
        build_batch_accounting_report(&receipt_accounting_report, &counter_accounting_report);
    reject_batch_counter_receipt_association(&batch_accounting_report)?;
    let closeout_digest = execution_receipt_accounting_closeout_digest(
        seed,
        &receipt_accounting_report,
        &counter_accounting_report,
        &batch_accounting_report,
    );
    let phase_seven_seed = WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed::from_input(
        WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeedInput {
            phase_six_closeout_digest: closeout_digest.clone(),
            phase_five_closeout_digest: seed.phase_five_closeout_digest().to_string(),
            receipt_accounting_report: receipt_accounting_report.clone(),
            counter_accounting_report: counter_accounting_report.clone(),
            batch_accounting_report: batch_accounting_report.clone(),
            source_firewall_report: seed.source_firewall_report().clone(),
            bounded_execution_contract: seed.bounded_execution_contract().clone(),
            phase_four_cutover_proof: seed.phase_four_cutover_proof().clone(),
            posture_projections: seed.posture_projections().to_vec(),
            cap_rows: seed.cap_rows().to_vec(),
        },
    );

    Ok(WorthGraphReadAccessExecutionReceiptAccountingCloseout {
        phase_six_seed_digest: seed.seed_digest().to_string(),
        receipt_accounting_report,
        counter_accounting_report,
        batch_accounting_report,
        phase_seven_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessExecutionReceiptAccountingCloseout {
    pub fn phase_six_seed_digest(&self) -> &str {
        &self.phase_six_seed_digest
    }

    pub const fn receipt_accounting_report(&self) -> &WorthGraphReadAccessReceiptAccountingReport {
        &self.receipt_accounting_report
    }

    pub const fn counter_accounting_report(&self) -> &WorthGraphReadAccessCounterAccountingReport {
        &self.counter_accounting_report
    }

    pub const fn batch_accounting_report(&self) -> &WorthGraphReadAccessBatchAccountingReport {
        &self.batch_accounting_report
    }

    pub const fn phase_seven_seed(
        &self,
    ) -> &WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed {
        &self.phase_seven_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

fn reject_invalid_seed(
    seed: &WorthGraphReadAccessSpatialDensePhaseSixSeed,
) -> Result<(), WorthGraphReadAccessExecutionReceiptAccountingError> {
    if seed.claims_validator_selection() {
        return Err(WorthGraphReadAccessExecutionReceiptAccountingError::new(
            WorthGraphReadAccessExecutionReceiptAccountingErrorKind::SeedAlreadyClaimsValidatorSelection,
        ));
    }
    Ok(())
}

fn reject_empty_receipt_accounting(
    report: &WorthGraphReadAccessReceiptAccountingReport,
) -> Result<(), WorthGraphReadAccessExecutionReceiptAccountingError> {
    if report.rows().is_empty() {
        return Err(WorthGraphReadAccessExecutionReceiptAccountingError::new(
            WorthGraphReadAccessExecutionReceiptAccountingErrorKind::EmptyReceiptAccountingInput,
        ));
    }
    Ok(())
}

fn reject_caller_owned_graph_work(
    report: &WorthGraphReadAccessCounterAccountingReport,
) -> Result<(), WorthGraphReadAccessExecutionReceiptAccountingError> {
    if report.caller_owned_graph_work_count() > 0 {
        return Err(WorthGraphReadAccessExecutionReceiptAccountingError::new(
            WorthGraphReadAccessExecutionReceiptAccountingErrorKind::CallerOwnedGraphWorkDetected,
        ));
    }
    Ok(())
}

fn reject_batch_counter_receipt_association(
    report: &WorthGraphReadAccessBatchAccountingReport,
) -> Result<(), WorthGraphReadAccessExecutionReceiptAccountingError> {
    if !report.per_read_association_preserved() {
        return Err(WorthGraphReadAccessExecutionReceiptAccountingError::new(
            WorthGraphReadAccessExecutionReceiptAccountingErrorKind::BatchCounterReceiptAssociationLost,
        ));
    }
    Ok(())
}
