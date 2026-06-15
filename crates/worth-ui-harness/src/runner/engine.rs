use worth_ui::facade::WorthUiApp;

use crate::evidence::{
    HarnessEvidenceBasis, HarnessEvidenceBundle, HarnessEvidenceLedger, HarnessFailureLocation,
    HarnessOperationReceipt,
};
use crate::honesty::HarnessHonestyPolicy;
use crate::scenario::{HarnessScenario, HarnessScenarioOperation, HarnessScenarioStep};

use super::{HarnessReplayRecord, HarnessRunDenial, HarnessRunReceipt};

pub struct HarnessRunner {
    app: WorthUiApp,
    honesty: HarnessHonestyPolicy,
}

impl HarnessRunner {
    pub fn for_app(app: WorthUiApp) -> Self {
        Self {
            app,
            honesty: HarnessHonestyPolicy,
        }
    }

    pub fn run(self, scenario: HarnessScenario) -> Result<HarnessRunReceipt, HarnessRunDenial> {
        let (scenario_id, steps) = scenario.into_parts();
        reject_empty_scenario(&scenario_id, &steps)?;
        let mut ledger = HarnessEvidenceLedger::empty();
        let mut operation_identities = Vec::with_capacity(steps.len());
        self.run_steps(
            scenario_id.clone(),
            steps,
            &mut ledger,
            &mut operation_identities,
        )?;
        let completed_steps = operation_identities.len();
        Ok(HarnessRunReceipt::new(
            scenario_id,
            ledger,
            operation_identities,
            completed_steps,
        ))
    }

    pub fn replay(
        self,
        expected: &HarnessReplayRecord,
        scenario: HarnessScenario,
    ) -> Result<HarnessRunReceipt, HarnessRunDenial> {
        let replay = self.run(scenario)?;
        expected
            .validate_replay(replay.replay_record())
            .map_err(|denial| HarnessRunDenial::ReplayMismatch { denial })?;
        Ok(replay)
    }

    fn run_steps(
        &self,
        scenario_id: crate::scenario::HarnessScenarioId,
        steps: Vec<HarnessScenarioStep>,
        ledger: &mut HarnessEvidenceLedger,
        operation_identities: &mut Vec<String>,
    ) -> Result<usize, HarnessRunDenial> {
        let mut completed_steps = 0;
        for (step_index, step) in steps.into_iter().enumerate() {
            let location = step_failure_location(&scenario_id, step_index, &step, None);
            let requirements = step.requirements().to_vec();
            let expectations = step.expectations().to_vec();
            let operation_identity = step.operation().identity_text().to_owned();
            let mut step_evidence = HarnessEvidenceBundle::empty();
            self.run_step(location.clone(), step, &mut step_evidence)?;
            step_evidence.record_operation_receipt();
            self.honesty
                .validate_step_evidence(&step_evidence, &requirements, &expectations)
                .map_err(|denial| {
                    let family = evidence_family_from_honesty(&denial);
                    HarnessRunDenial::localized_honesty(
                        location.clone().with_evidence_family(family),
                        denial,
                    )
                })?;
            ledger.push(HarnessOperationReceipt::new(
                step_index,
                location.step_label(),
                operation_identity.clone(),
                step_evidence,
            ));
            operation_identities.push(operation_identity);
            completed_steps += 1;
        }
        Ok(completed_steps)
    }

    fn run_step(
        &self,
        location: HarnessFailureLocation,
        step: HarnessScenarioStep,
        evidence: &mut HarnessEvidenceBundle,
    ) -> Result<(), HarnessRunDenial> {
        match step.into_operation() {
            HarnessScenarioOperation::LaunchRuntime { launch } => {
                let host = self
                    .app
                    .launch_runtime(launch)
                    .map_err(|_| HarnessRunDenial::RuntimeLaunchDenied { location })?;
                let basis = HarnessEvidenceBasis::from_active_observation(host.inspect_active());
                evidence.observe_runtime_launch(basis);
                Ok(())
            }
            HarnessScenarioOperation::ObserveVisibleFrame => {
                evidence.observe_visible_frame();
                Ok(())
            }
            HarnessScenarioOperation::AttemptAppLocalShellStateInjection => {
                Err(HarnessRunDenial::localized_honesty(
                    location,
                    self.honesty.reject_app_local_shell_state_injection(),
                ))
            }
        }
    }
}

fn reject_empty_scenario(
    scenario_id: &crate::scenario::HarnessScenarioId,
    steps: &[HarnessScenarioStep],
) -> Result<(), HarnessRunDenial> {
    if steps.is_empty() {
        Err(HarnessRunDenial::EmptyScenario {
            scenario_id: scenario_id.clone(),
        })
    } else {
        Ok(())
    }
}

fn step_failure_location(
    scenario_id: &crate::scenario::HarnessScenarioId,
    step_index: usize,
    step: &HarnessScenarioStep,
    evidence_family: Option<crate::evidence::HarnessEvidenceFamily>,
) -> HarnessFailureLocation {
    HarnessFailureLocation::new(
        scenario_id.clone(),
        step_index,
        step.label(),
        evidence_family,
    )
}

fn evidence_family_from_honesty(
    denial: &crate::honesty::HarnessHonestyDenial,
) -> Option<crate::evidence::HarnessEvidenceFamily> {
    match denial {
        crate::honesty::HarnessHonestyDenial::EvidenceValidation(validation) => match validation {
            crate::evidence::HarnessEvidenceValidationDenial::MissingRequiredEvidence {
                family,
            }
            | crate::evidence::HarnessEvidenceValidationDenial::ExpectedEvidenceMissing {
                family,
            } => Some(*family),
            crate::evidence::HarnessEvidenceValidationDenial::DigestExpectation(denial) => {
                Some(match denial {
                    crate::evidence::HarnessDigestExpectationDenial::MissingRunBasis { family }
                    | crate::evidence::HarnessDigestExpectationDenial::FixedDigestRejected {
                        family,
                        ..
                    }
                    | crate::evidence::HarnessDigestExpectationDenial::MissingDigestFamily {
                        family,
                    } => *family,
                })
            }
            crate::evidence::HarnessEvidenceValidationDenial::RuntimeEvidenceWithoutBasis
            | crate::evidence::HarnessEvidenceValidationDenial::StaleEvidenceBasis { .. } => {
                Some(crate::evidence::HarnessEvidenceFamily::RuntimeReceipt)
            }
        },
        crate::honesty::HarnessHonestyDenial::AppLocalShellStateInjection => None,
    }
}

trait HarnessFailureLocationFamily {
    fn with_evidence_family(
        self,
        evidence_family: Option<crate::evidence::HarnessEvidenceFamily>,
    ) -> Self;
}

impl HarnessFailureLocationFamily for HarnessFailureLocation {
    fn with_evidence_family(
        self,
        evidence_family: Option<crate::evidence::HarnessEvidenceFamily>,
    ) -> Self {
        HarnessFailureLocation::new(
            self.scenario_id().clone(),
            self.step_index(),
            self.step_label(),
            evidence_family,
        )
    }
}
