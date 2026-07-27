mod identity;

use std::collections::BTreeSet;
use std::time::Duration;

use serde::Serialize;

pub(super) use identity::BoundedResidencySiegePhase;
use identity::{expected_before_report, expected_complete};

const MUTATION_EVIDENCE_BUDGET_MS: u64 = 30_000;
const WORLD_BUDGET_MS: u64 = 1_000;
const SOURCE_INVENTORY_BUDGET_MS: u64 = 5_000;
const PREBUILD_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const POSTBUILD_BINARY_BINDING_BUDGET_MS: u64 = 3_000;
const POSTBUILD_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const CHILD_STAGE_BUDGET_MS: u64 = 5_000;
const FINAL_SOURCE_BINDING_BUDGET_MS: u64 = 2_000;
const EXECUTABLE_VERIFICATION_BUDGET_MS: u64 = 1_000;
const REPORT_ENCODING_BUDGET_MS: u64 = 500;
const RUNNER_CONTROLLED_TOTAL_BUDGET_MS: u64 = 30_000;

#[derive(Debug, Clone, Serialize)]
pub(super) struct TimedSiegePhase {
    #[serde(skip)]
    identity: BoundedResidencySiegePhase,
    name: &'static str,
    elapsed_ms: u64,
}

impl TimedSiegePhase {
    pub(super) const fn name(&self) -> &str {
        self.name
    }

    pub(super) const fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms
    }
}

#[derive(Debug, Default, Serialize)]
pub(super) struct BoundedResidencySiegeTimings {
    phases: Vec<TimedSiegePhase>,
}

impl BoundedResidencySiegeTimings {
    pub(super) const fn new() -> Self {
        Self { phases: Vec::new() }
    }

    pub(super) fn record(&mut self, phase: BoundedResidencySiegePhase, elapsed: Duration) {
        self.phases.push(TimedSiegePhase {
            identity: phase,
            name: phase.label(),
            elapsed_ms: elapsed_ms(elapsed),
        });
    }

    pub(super) fn phases(&self) -> &[TimedSiegePhase] {
        &self.phases
    }

    pub(super) fn validate_runtime_budget(&self) -> Result<(), String> {
        self.require_exact(&expected_before_report())?;
        self.validate_stage_budgets()
    }

    pub(super) fn validate_complete_budget(&self) -> Result<(), String> {
        self.require_exact(&expected_complete())?;
        self.validate_stage_budgets()?;
        self.require_within(
            BoundedResidencySiegePhase::ReportEncoding,
            REPORT_ENCODING_BUDGET_MS,
        )
    }

    fn validate_stage_budgets(&self) -> Result<(), String> {
        for (phase, budget) in [
            (
                BoundedResidencySiegePhase::MutationEvidence,
                MUTATION_EVIDENCE_BUDGET_MS,
            ),
            (BoundedResidencySiegePhase::World, WORLD_BUDGET_MS),
            (
                BoundedResidencySiegePhase::SourceInventory,
                SOURCE_INVENTORY_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::PrebuildSourceBinding,
                PREBUILD_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::PostbuildBinaryBinding,
                POSTBUILD_BINARY_BINDING_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::PostbuildSourceBinding,
                POSTBUILD_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::SiegeWriter,
                CHILD_STAGE_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::OfflineObserver,
                CHILD_STAGE_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::FreshReopener,
                CHILD_STAGE_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::FinalSourceBinding,
                FINAL_SOURCE_BINDING_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::ExecutableVerification,
                EXECUTABLE_VERIFICATION_BUDGET_MS,
            ),
        ] {
            self.require_within(phase, budget)?;
        }
        Ok(())
    }

    pub(super) fn validate_completed_campaign(
        &self,
        completed_wall: Duration,
    ) -> Result<u64, String> {
        self.validate_complete_budget()?;
        let completed_ms = elapsed_ms(completed_wall);
        let build_ms = self
            .require(BoundedResidencySiegePhase::BinaryBuild)?
            .elapsed_ms();
        let runner_ms = completed_ms.checked_sub(build_ms).ok_or_else(|| {
            "Courtroom C cold-build timing exceeded completed campaign wall time".to_owned()
        })?;
        if runner_ms > RUNNER_CONTROLLED_TOTAL_BUDGET_MS {
            return Err(format!(
                "Courtroom C runner-controlled work took {runner_ms}ms; budget is \
                 {RUNNER_CONTROLLED_TOTAL_BUDGET_MS}ms"
            ));
        }
        Ok(runner_ms)
    }

    fn require_exact(&self, expected: &[BoundedResidencySiegePhase]) -> Result<(), String> {
        let actual = self
            .phases
            .iter()
            .map(|phase| phase.identity)
            .collect::<BTreeSet<_>>();
        if actual.len() != self.phases.len() {
            return Err("Courtroom C timing evidence duplicated a phase".into());
        }
        let expected = expected.iter().copied().collect::<BTreeSet<_>>();
        if actual != expected {
            let missing = expected.difference(&actual).map(|phase| phase.label());
            let unexpected = actual.difference(&expected).map(|phase| phase.label());
            return Err(format!(
                "Courtroom C timing schema mismatch; missing [{}], unexpected [{}]",
                missing.collect::<Vec<_>>().join(", "),
                unexpected.collect::<Vec<_>>().join(", "),
            ));
        }
        Ok(())
    }

    fn require(&self, phase: BoundedResidencySiegePhase) -> Result<&TimedSiegePhase, String> {
        self.phases
            .iter()
            .find(|candidate| candidate.identity == phase)
            .ok_or_else(|| format!("Courtroom C timing evidence omitted `{}`", phase.label()))
    }

    fn require_within(
        &self,
        phase: BoundedResidencySiegePhase,
        budget_ms: u64,
    ) -> Result<(), String> {
        let timing = self.require(phase)?;
        if timing.elapsed_ms() <= budget_ms {
            Ok(())
        } else {
            Err(format!(
                "Courtroom C phase `{}` took {}ms; budget is {budget_ms}ms",
                timing.name(),
                timing.elapsed_ms()
            ))
        }
    }
}

fn elapsed_ms(elapsed: Duration) -> u64 {
    elapsed.as_millis().try_into().unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests;
