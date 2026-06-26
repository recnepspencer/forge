use super::access_posture::WorthGraphReadAccessPlanAdoptionPostureReport;
use super::counters::WorthGraphReadAccessPlanAdoptionPhaseTwoCounters;
use super::errors::WorthGraphReadAccessPlanAdoptionPhaseTwoError;
use super::read_family_adoption::WorthGraphReadAccessPlanAdoptionLedger;
use super::stable_digest;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseOneCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
    phase_one_closeout_digest: String,
    adoption_ledger: WorthGraphReadAccessPlanAdoptionLedger,
    posture_report: WorthGraphReadAccessPlanAdoptionPostureReport,
    counters: WorthGraphReadAccessPlanAdoptionPhaseTwoCounters,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_plan_adoption_phase_two_closeout(
    phase_one: &WorthGraphReadAccessPlanAdoptionPhaseOneCloseout,
) -> Result<
    WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout,
    WorthGraphReadAccessPlanAdoptionPhaseTwoError,
> {
    let adoption_ledger = WorthGraphReadAccessPlanAdoptionLedger::from_phase_one_closeout(
        phase_one.milestone_seven_closeout_digest(),
        phase_one.declaration_catalog_digest(),
        phase_one.read_family_identities(),
        phase_one.requirement_row_evidence(),
        phase_one.admission_capability_gaps(),
    )?;
    let posture_report = WorthGraphReadAccessPlanAdoptionPostureReport::from_adoption_attempts(
        adoption_ledger.attempts(),
    );
    let counters = WorthGraphReadAccessPlanAdoptionPhaseTwoCounters::from_products(
        &adoption_ledger,
        &posture_report,
    );
    let closeout_digest = phase_two_closeout_digest(phase_one, &adoption_ledger, &posture_report);
    Ok(WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
        phase_one_closeout_digest: phase_one.milestone_seven_closeout_digest().to_string(),
        adoption_ledger,
        posture_report,
        counters,
        closeout_digest,
    })
}

impl WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
    pub fn phase_one_closeout_digest(&self) -> &str {
        &self.phase_one_closeout_digest
    }

    #[cfg(test)]
    pub(crate) fn for_posture_matrix_tests(
        attempts: Vec<super::query_admission::WorthGraphReadAccessPlanAdoptionAttempt>,
        carried_capability_gaps: Vec<
            super::read_family_adoption::WorthGraphReadAccessPlanAdoptionCarriedGapRow,
        >,
    ) -> Self {
        let adoption_ledger =
            WorthGraphReadAccessPlanAdoptionLedger::from_attempts_for_posture_matrix_tests(
                attempts,
                carried_capability_gaps,
            );
        let posture_report =
            WorthGraphReadAccessPlanAdoptionPostureReport::from_adoption_attempts_for_tests(
                adoption_ledger.attempts(),
            );
        let counters = WorthGraphReadAccessPlanAdoptionPhaseTwoCounters::from_products(
            &adoption_ledger,
            &posture_report,
        );
        let closeout_digest = stable_digest(&[
            "worth_graph_read_access_plan_adoption_phase_two_closeout_v1".to_string(),
            "phase_one_closeout:phase-one-posture-matrix-test".to_string(),
            format!("adoption_ledger:{}", adoption_ledger.ledger_digest()),
            format!("posture_report:{}", posture_report.report_digest()),
        ]);
        Self {
            phase_one_closeout_digest: "phase-one-posture-matrix-test".to_string(),
            adoption_ledger,
            posture_report,
            counters,
            closeout_digest,
        }
    }

    pub const fn adoption_ledger(&self) -> &WorthGraphReadAccessPlanAdoptionLedger {
        &self.adoption_ledger
    }

    pub const fn posture_report(&self) -> &WorthGraphReadAccessPlanAdoptionPostureReport {
        &self.posture_report
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessPlanAdoptionPhaseTwoCounters {
        &self.counters
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_access_plan_admission_attempts(&self) -> bool {
        true
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts(&self) -> bool {
        false
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }

    pub const fn claims_milestone_nine_seed_export(&self) -> bool {
        false
    }
}

fn phase_two_closeout_digest(
    phase_one: &WorthGraphReadAccessPlanAdoptionPhaseOneCloseout,
    adoption_ledger: &WorthGraphReadAccessPlanAdoptionLedger,
    posture_report: &WorthGraphReadAccessPlanAdoptionPostureReport,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_plan_adoption_phase_two_closeout_v1".to_string(),
        format!(
            "phase_one_closeout:{}",
            phase_one.milestone_seven_closeout_digest()
        ),
        format!("adoption_ledger:{}", adoption_ledger.ledger_digest()),
        format!("posture_report:{}", posture_report.report_digest()),
    ])
}
