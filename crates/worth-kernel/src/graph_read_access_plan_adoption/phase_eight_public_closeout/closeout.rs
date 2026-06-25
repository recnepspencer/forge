use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessBatchAccountingReport, WorthGraphReadAccessBoundedExecutionContract,
    WorthGraphReadAccessCounterAccountingReport, WorthGraphReadAccessHardDeletionPhaseEightSeed,
    WorthGraphReadAccessSliceCutoverProof,
};

use super::closeout_counters::WorthGraphReadAccessPlanAdoptionCloseoutCounters;
use super::closeout_digest::plan_adoption_closeout_digest;
use super::errors::{
    WorthGraphReadAccessPlanAdoptionCloseoutError,
    WorthGraphReadAccessPlanAdoptionCloseoutErrorKind,
};
use super::milestone_nine_seed::{
    WorthGraphReadAccessPlanAdoptionMilestoneNineSeed,
    WorthGraphReadAccessPlanAdoptionMilestoneNineSeedInput,
};
use super::proof_exports::{
    WorthGraphReadAccessPlanAdoptionDeletionExport, WorthGraphReadAccessPlanAdoptionPostureExport,
    WorthGraphReadAccessPlanAdoptionReceiptExport, WorthGraphReadAccessPlanAdoptionResidueExport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionCloseout {
    phase_eight_seed_digest: String,
    receipts: WorthGraphReadAccessPlanAdoptionReceiptExport,
    postures: WorthGraphReadAccessPlanAdoptionPostureExport,
    counters: WorthGraphReadAccessPlanAdoptionCloseoutCounters,
    counter_accounting_report: WorthGraphReadAccessCounterAccountingReport,
    batch_accounting_report: WorthGraphReadAccessBatchAccountingReport,
    deletion: WorthGraphReadAccessPlanAdoptionDeletionExport,
    residue: WorthGraphReadAccessPlanAdoptionResidueExport,
    source_firewall: WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    phase_four_cutover_proof: WorthGraphReadAccessSliceCutoverProof,
    milestone_nine_seed: WorthGraphReadAccessPlanAdoptionMilestoneNineSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_plan_adoption_closeout(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<WorthGraphReadAccessPlanAdoptionCloseout, WorthGraphReadAccessPlanAdoptionCloseoutError>
{
    reject_invalid_seed(seed)?;

    let receipts = WorthGraphReadAccessPlanAdoptionReceiptExport::from_report(
        seed.receipt_accounting_report(),
    );
    reject_empty_receipts_or_postures(seed, &receipts)?;
    reject_empty_counter_proof(seed)?;
    reject_empty_batch_proof(seed)?;
    reject_batch_counter_receipt_association(seed)?;

    let postures = WorthGraphReadAccessPlanAdoptionPostureExport::from_parts(
        seed.posture_projections(),
        seed.cap_rows(),
    );
    let deletion =
        WorthGraphReadAccessPlanAdoptionDeletionExport::from_report(seed.deletion_proof_report());
    let residue =
        WorthGraphReadAccessPlanAdoptionResidueExport::from_report(seed.capped_residue_report());
    let source_firewall = WorthGraphReadAccessPlanAdoptionSourceFirewallExport::from_report(
        seed.source_firewall_report(),
    );
    reject_caller_owned_graph_work(seed)?;
    reject_unresolved_deletion(&deletion)?;
    reject_uncapped_residue(&residue)?;
    reject_source_firewall_violations(&source_firewall)?;

    let counters = WorthGraphReadAccessPlanAdoptionCloseoutCounters::from_seed_and_exports(
        seed,
        &receipts,
        &postures,
        &deletion,
        &residue,
        &source_firewall,
    );
    let closeout_digest = plan_adoption_closeout_digest(
        seed,
        &receipts,
        &postures,
        &deletion,
        &residue,
        &source_firewall,
        &counters,
    );
    let milestone_nine_seed = WorthGraphReadAccessPlanAdoptionMilestoneNineSeed::from_input(
        WorthGraphReadAccessPlanAdoptionMilestoneNineSeedInput {
            milestone_eight_closeout_digest: closeout_digest.clone(),
            receipt_export: receipts.clone(),
            posture_export: postures.clone(),
            closeout_counters: counters.clone(),
            counter_accounting_report: seed.counter_accounting_report().clone(),
            batch_accounting_report: seed.batch_accounting_report().clone(),
            deletion_export: deletion.clone(),
            residue_export: residue.clone(),
            source_firewall_export: source_firewall.clone(),
            bounded_execution_contract: seed.bounded_execution_contract().clone(),
            phase_four_cutover_proof: seed.phase_four_cutover_proof().clone(),
        },
    );

    Ok(WorthGraphReadAccessPlanAdoptionCloseout {
        phase_eight_seed_digest: seed.seed_digest().to_string(),
        receipts,
        postures,
        counters,
        counter_accounting_report: seed.counter_accounting_report().clone(),
        batch_accounting_report: seed.batch_accounting_report().clone(),
        deletion,
        residue,
        source_firewall,
        bounded_execution_contract: seed.bounded_execution_contract().clone(),
        phase_four_cutover_proof: seed.phase_four_cutover_proof().clone(),
        milestone_nine_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessPlanAdoptionCloseout {
    pub fn phase_eight_seed_digest(&self) -> &str {
        &self.phase_eight_seed_digest
    }

    pub const fn receipts(&self) -> &WorthGraphReadAccessPlanAdoptionReceiptExport {
        &self.receipts
    }

    pub const fn postures(&self) -> &WorthGraphReadAccessPlanAdoptionPostureExport {
        &self.postures
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessPlanAdoptionCloseoutCounters {
        &self.counters
    }

    pub const fn counter_accounting_report(&self) -> &WorthGraphReadAccessCounterAccountingReport {
        &self.counter_accounting_report
    }

    pub const fn batch_accounting_report(&self) -> &WorthGraphReadAccessBatchAccountingReport {
        &self.batch_accounting_report
    }

    pub const fn deletion(&self) -> &WorthGraphReadAccessPlanAdoptionDeletionExport {
        &self.deletion
    }

    pub const fn residue(&self) -> &WorthGraphReadAccessPlanAdoptionResidueExport {
        &self.residue
    }

    pub const fn source_firewall(&self) -> &WorthGraphReadAccessPlanAdoptionSourceFirewallExport {
        &self.source_firewall
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn phase_four_cutover_proof(&self) -> &WorthGraphReadAccessSliceCutoverProof {
        &self.phase_four_cutover_proof
    }

    pub const fn milestone_nine_seed(&self) -> &WorthGraphReadAccessPlanAdoptionMilestoneNineSeed {
        &self.milestone_nine_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

fn reject_invalid_seed(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if seed.claims_validator_selection() {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::SeedAlreadyClaimsValidatorSelection,
        ));
    }
    Ok(())
}

fn reject_empty_receipts_or_postures(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
    receipts: &WorthGraphReadAccessPlanAdoptionReceiptExport,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if seed.receipt_accounting_report().rows().is_empty()
        || !receipts.has_executed_receipts_or_visible_postures()
    {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingReceiptOrPostureProof,
        ));
    }
    Ok(())
}

fn reject_empty_counter_proof(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if seed.counter_accounting_report().rows().is_empty() {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingCounterProof,
        ));
    }
    Ok(())
}

fn reject_empty_batch_proof(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if seed.batch_accounting_report().rows().is_empty() {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::MissingBatchAccountingProof,
        ));
    }
    Ok(())
}

fn reject_batch_counter_receipt_association(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if !seed
        .batch_accounting_report()
        .per_read_association_preserved()
    {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::BatchCounterReceiptAssociationLost,
        ));
    }
    Ok(())
}

fn reject_caller_owned_graph_work(
    seed: &WorthGraphReadAccessHardDeletionPhaseEightSeed,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if seed
        .counter_accounting_report()
        .caller_owned_graph_work_count()
        > 0
        || seed
            .batch_accounting_report()
            .caller_owned_graph_work_count()
            > 0
    {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::CallerOwnedGraphWorkDetected,
        ));
    }
    Ok(())
}

fn reject_unresolved_deletion(
    deletion: &WorthGraphReadAccessPlanAdoptionDeletionExport,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if deletion.unresolved_count() > 0 {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::UnresolvedDeletionProof,
        ));
    }
    Ok(())
}

fn reject_uncapped_residue(
    residue: &WorthGraphReadAccessPlanAdoptionResidueExport,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if residue.uncapped_residue_count() > 0 {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::UncappedResidue,
        ));
    }
    Ok(())
}

fn reject_source_firewall_violations(
    source_firewall: &WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
) -> Result<(), WorthGraphReadAccessPlanAdoptionCloseoutError> {
    if source_firewall.violation_count() > 0 {
        return Err(WorthGraphReadAccessPlanAdoptionCloseoutError::new(
            WorthGraphReadAccessPlanAdoptionCloseoutErrorKind::SourceFirewallViolation,
        ));
    }
    Ok(())
}
