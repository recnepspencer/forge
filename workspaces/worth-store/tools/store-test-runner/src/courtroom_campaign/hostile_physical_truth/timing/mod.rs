mod identity;

use std::collections::BTreeSet;
use std::time::Duration;

use serde::Serialize;
use worth_store::physical_runtime::PhysicalWorkHostileTruthScenario;

use identity::{expected_before_report, expected_complete, TimingIdentity};
pub(super) use identity::{CampaignPhase, ScenarioStage};

const MUTATION_EVIDENCE_BUDGET_MS: u64 = 30_000;
const WORLD_CREATION_BUDGET_MS: u64 = 1_000;
const SOURCE_INVENTORY_BUDGET_MS: u64 = 5_000;
const PREBUILD_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const POSTBUILD_BINARY_BINDING_BUDGET_MS: u64 = 3_000;
const POSTBUILD_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const FINAL_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const EXECUTABLE_VERIFICATION_BUDGET_MS: u64 = 1_000;
const SCENARIO_STAGE_BUDGET_MS: u64 = 5_000;
const SCENARIO_TOTAL_BUDGET_MS: u64 = 15_000;
const REPORT_ENCODING_BUDGET_MS: u64 = 500;
const RUNNER_CONTROLLED_TOTAL_BUDGET_MS: u64 = 30_000;

#[derive(Debug, Clone, Serialize)]
pub(super) struct TimedCampaignPhase {
    #[serde(skip)]
    identity: TimingIdentity,
    name: Box<str>,
    elapsed_ms: u64,
}

impl TimedCampaignPhase {
    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) const fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms
    }
}

#[derive(Debug, Default, Serialize)]
pub(super) struct CampaignTimings {
    phases: Vec<TimedCampaignPhase>,
}

impl CampaignTimings {
    pub(super) const fn new() -> Self {
        Self { phases: Vec::new() }
    }

    pub(super) fn record_campaign(&mut self, phase: CampaignPhase, elapsed: Duration) {
        self.record(TimingIdentity::Campaign(phase), elapsed);
    }

    pub(super) fn record_scenario(
        &mut self,
        scenario: PhysicalWorkHostileTruthScenario,
        stage: ScenarioStage,
        elapsed: Duration,
    ) {
        self.record(TimingIdentity::Scenario { scenario, stage }, elapsed);
    }

    pub(super) fn record_case_verification(
        &mut self,
        scenario: PhysicalWorkHostileTruthScenario,
        elapsed: Duration,
    ) {
        self.record(TimingIdentity::CaseVerification(scenario), elapsed);
    }

    pub(super) fn phases(&self) -> &[TimedCampaignPhase] {
        &self.phases
    }

    pub(super) fn validate_runtime_budget(&self) -> Result<(), String> {
        self.require_exact(&expected_before_report())?;
        self.validate_stage_budgets()
    }

    pub(super) fn validate_complete_budget(&self) -> Result<(), String> {
        self.require_exact(&expected_complete())?;
        self.validate_stage_budgets()?;
        self.require_within(CampaignPhase::ReportEncoding, REPORT_ENCODING_BUDGET_MS)
    }

    fn validate_stage_budgets(&self) -> Result<(), String> {
        for (phase, budget) in [
            (CampaignPhase::MutationEvidence, MUTATION_EVIDENCE_BUDGET_MS),
            (CampaignPhase::World, WORLD_CREATION_BUDGET_MS),
            (CampaignPhase::SourceInventory, SOURCE_INVENTORY_BUDGET_MS),
            (
                CampaignPhase::PrebuildSourceBinding,
                PREBUILD_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                CampaignPhase::PostbuildBinaryBinding,
                POSTBUILD_BINARY_BINDING_BUDGET_MS,
            ),
            (
                CampaignPhase::PostbuildSourceBinding,
                POSTBUILD_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                CampaignPhase::FinalSourceBinding,
                FINAL_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                CampaignPhase::ExecutableVerification,
                EXECUTABLE_VERIFICATION_BUDGET_MS,
            ),
        ] {
            self.require_within(phase, budget)?;
        }
        self.validate_scenario_budgets()
    }

    pub(super) fn validate_completed_campaign(
        &self,
        completed_wall: Duration,
    ) -> Result<u64, String> {
        self.validate_complete_budget()?;
        let completed_ms = elapsed_ms(completed_wall);
        let build_ms = self
            .require_campaign(CampaignPhase::BinaryBuild)?
            .elapsed_ms();
        let runner_ms = completed_ms.checked_sub(build_ms).ok_or_else(|| {
            "Courtroom B cold-build timing exceeded completed campaign wall time".to_owned()
        })?;
        if runner_ms > RUNNER_CONTROLLED_TOTAL_BUDGET_MS {
            return Err(format!(
                "Courtroom B runner-controlled work took {runner_ms}ms; budget is \
                 {RUNNER_CONTROLLED_TOTAL_BUDGET_MS}ms"
            ));
        }
        Ok(runner_ms)
    }

    fn record(&mut self, identity: TimingIdentity, elapsed: Duration) {
        self.phases.push(TimedCampaignPhase {
            identity,
            name: identity.label(),
            elapsed_ms: elapsed_ms(elapsed),
        });
    }

    fn validate_scenario_budgets(&self) -> Result<(), String> {
        let bounded = self.phases.iter().filter(|phase| {
            matches!(
                phase.identity,
                TimingIdentity::Scenario { .. } | TimingIdentity::CaseVerification(_)
            )
        });
        for phase in bounded {
            require_within(phase, SCENARIO_STAGE_BUDGET_MS)?;
        }
        let scenario_stages = self
            .phases
            .iter()
            .filter(|phase| matches!(phase.identity, TimingIdentity::Scenario { .. }))
            .collect::<Vec<_>>();
        let total = scenario_stages
            .iter()
            .fold(0_u64, |sum, phase| sum.saturating_add(phase.elapsed_ms()));
        if total > SCENARIO_TOTAL_BUDGET_MS {
            return Err(format!(
                "Courtroom B scenario stages took {total}ms; budget is \
                 {SCENARIO_TOTAL_BUDGET_MS}ms"
            ));
        }
        Ok(())
    }

    fn require_exact(&self, expected: &[TimingIdentity]) -> Result<(), String> {
        let actual = self
            .phases
            .iter()
            .map(|phase| phase.identity)
            .collect::<BTreeSet<_>>();
        if actual.len() != self.phases.len() {
            return Err("Courtroom B timing evidence duplicated a phase".into());
        }
        let expected = expected.iter().copied().collect::<BTreeSet<_>>();
        if actual != expected {
            let missing = expected.difference(&actual).map(|phase| phase.label());
            let unexpected = actual.difference(&expected).map(|phase| phase.label());
            return Err(format!(
                "Courtroom B timing schema mismatch; missing [{}], unexpected [{}]",
                missing.collect::<Vec<_>>().join(", "),
                unexpected.collect::<Vec<_>>().join(", "),
            ));
        }
        Ok(())
    }

    fn require_campaign(&self, phase: CampaignPhase) -> Result<&TimedCampaignPhase, String> {
        let identity = TimingIdentity::Campaign(phase);
        self.phases
            .iter()
            .find(|candidate| candidate.identity == identity)
            .ok_or_else(|| format!("Courtroom B timing evidence omitted `{}`", phase.label()))
    }

    fn require_within(&self, phase: CampaignPhase, budget_ms: u64) -> Result<(), String> {
        require_within(self.require_campaign(phase)?, budget_ms)
    }
}

fn require_within(phase: &TimedCampaignPhase, budget_ms: u64) -> Result<(), String> {
    if phase.elapsed_ms() <= budget_ms {
        Ok(())
    } else {
        Err(format!(
            "Courtroom B phase `{}` took {}ms; budget is {budget_ms}ms",
            phase.name(),
            phase.elapsed_ms(),
        ))
    }
}

fn elapsed_ms(elapsed: Duration) -> u64 {
    elapsed.as_millis().try_into().unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests;
