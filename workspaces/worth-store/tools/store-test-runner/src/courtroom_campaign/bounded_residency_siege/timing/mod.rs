mod identity;

use std::collections::BTreeSet;
use std::time::Duration;

use serde::Serialize;

use super::binary_binding::SourceClosureWorkload;

pub(super) use identity::BoundedResidencySiegePhase;
use identity::{expected_before_report, expected_complete};

const MUTATION_EVIDENCE_BUDGET_MS: u64 = 30_000;
const WORLD_BUDGET_MS: u64 = 1_000;
const SOURCE_INVENTORY_BUDGET_MS: u64 = 5_000;
const SOURCE_BINDING_WALL_BUDGET_MS: u64 = 10_000;
const SOURCE_BINDING_FILE_LIMIT: u64 = 6_000;
const SOURCE_BINDING_BYTE_LIMIT: u64 = 32 * 1024 * 1024;
const POSTBUILD_BINARY_BINDING_BUDGET_MS: u64 = 3_000;
const CHILD_STAGE_BUDGET_MS: u64 = 5_000;
const SERVING_STAGE_BUDGET_MS: u64 = 30_000;
const EXECUTABLE_VERIFICATION_BUDGET_MS: u64 = 1_000;
// The source-bound v9 report is about 20 MiB. Three seconds preserves a
// meaningful serialization ceiling without making ordinary scheduler variance
// at the former 2-second boundary a courtroom failure.
const REPORT_ENCODING_BUDGET_MS: u64 = 3_000;
const RUNNER_CONTROLLED_TOTAL_BUDGET_MS: u64 = 30_000;

#[derive(Debug, Clone, Serialize)]
pub(super) struct TimedSiegePhase {
    #[serde(skip)]
    identity: BoundedResidencySiegePhase,
    name: &'static str,
    elapsed_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    case_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_files: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_bytes: Option<u64>,
}

impl TimedSiegePhase {
    pub(super) const fn name(&self) -> &str {
        self.name
    }

    pub(super) const fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms
    }

    pub(super) const fn case_count(&self) -> Option<u64> {
        self.case_count
    }

    pub(super) const fn source_workload(&self) -> Option<SourceClosureWorkload> {
        match (self.source_files, self.source_bytes) {
            (Some(source_files), Some(source_bytes)) => {
                Some(SourceClosureWorkload::new(source_files, source_bytes))
            }
            _ => None,
        }
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
            case_count: None,
            source_files: None,
            source_bytes: None,
        });
    }

    pub(super) fn record_source_binding(
        &mut self,
        phase: BoundedResidencySiegePhase,
        elapsed: Duration,
        workload: SourceClosureWorkload,
    ) -> Result<(), String> {
        if !matches!(
            phase,
            BoundedResidencySiegePhase::PrebuildSourceBinding
                | BoundedResidencySiegePhase::PostbuildSourceBinding
                | BoundedResidencySiegePhase::FinalSourceBinding
        ) {
            return Err(format!(
                "Courtroom C source workload cannot bind timing phase `{}`",
                phase.label()
            ));
        }
        self.phases.push(TimedSiegePhase {
            identity: phase,
            name: phase.label(),
            elapsed_ms: elapsed_ms(elapsed),
            case_count: None,
            source_files: Some(workload.source_files()),
            source_bytes: Some(workload.source_bytes()),
        });
        Ok(())
    }

    pub(super) fn record_case_campaign(&mut self, elapsed: Duration, case_count: usize) {
        self.phases.push(TimedSiegePhase {
            identity: BoundedResidencySiegePhase::DurabilityTerminationCampaign,
            name: BoundedResidencySiegePhase::DurabilityTerminationCampaign.label(),
            elapsed_ms: elapsed_ms(elapsed),
            case_count: u64::try_from(case_count).ok(),
            source_files: None,
            source_bytes: None,
        });
    }

    pub(super) fn phases(&self) -> &[TimedSiegePhase] {
        &self.phases
    }

    pub(super) fn elapsed_ms(&self, phase: BoundedResidencySiegePhase) -> Result<u64, String> {
        self.require(phase).map(TimedSiegePhase::elapsed_ms)
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
        self.validate_source_binding_workload()?;
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
                SOURCE_BINDING_WALL_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::PostbuildBinaryBinding,
                POSTBUILD_BINARY_BINDING_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::PostbuildSourceBinding,
                SOURCE_BINDING_WALL_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::SiegeServing,
                SERVING_STAGE_BUDGET_MS,
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
                SOURCE_BINDING_WALL_BUDGET_MS,
            ),
            (
                BoundedResidencySiegePhase::ExecutableVerification,
                EXECUTABLE_VERIFICATION_BUDGET_MS,
            ),
        ] {
            self.require_within(phase, budget)?;
        }
        self.require_case_campaign_shape()?;
        Ok(())
    }

    fn validate_source_binding_workload(&self) -> Result<(), String> {
        let mut expected = None;
        for phase in [
            BoundedResidencySiegePhase::PrebuildSourceBinding,
            BoundedResidencySiegePhase::PostbuildSourceBinding,
            BoundedResidencySiegePhase::FinalSourceBinding,
        ] {
            let workload = self.require(phase)?.source_workload().ok_or_else(|| {
                format!(
                    "Courtroom C phase `{}` omitted source workload counters",
                    phase.label()
                )
            })?;
            if workload.source_files() == 0 || workload.source_files() > SOURCE_BINDING_FILE_LIMIT {
                return Err(format!(
                    "Courtroom C source closure carried {} files; admitted range is 1..={SOURCE_BINDING_FILE_LIMIT}",
                    workload.source_files()
                ));
            }
            if workload.source_bytes() > SOURCE_BINDING_BYTE_LIMIT {
                return Err(format!(
                    "Courtroom C source closure carried {} bytes; limit is {SOURCE_BINDING_BYTE_LIMIT}",
                    workload.source_bytes()
                ));
            }
            if expected.is_some_and(|expected| expected != workload) {
                return Err("Courtroom C source workload changed during the campaign".into());
            }
            expected = Some(workload);
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
        let producer_ms = self
            .require(BoundedResidencySiegePhase::SiegeProducer)?
            .elapsed_ms();
        let case_campaign_ms = self
            .require(BoundedResidencySiegePhase::DurabilityTerminationCampaign)?
            .elapsed_ms();
        let setup_ms = build_ms
            .checked_add(producer_ms)
            .and_then(|elapsed| elapsed.checked_add(case_campaign_ms))
            .ok_or_else(|| "Courtroom C setup timing overflowed u64".to_owned())?;
        let runner_ms = completed_ms.checked_sub(setup_ms).ok_or_else(|| {
            "Courtroom C setup timing exceeded completed campaign wall time".to_owned()
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

    fn require_case_campaign_shape(&self) -> Result<(), String> {
        let timing = self.require(BoundedResidencySiegePhase::DurabilityTerminationCampaign)?;
        timing
            .case_count()
            .filter(|count| (1..=8).contains(count))
            .ok_or_else(|| {
                "Courtroom C durability termination timing requires one through eight cases"
                    .to_owned()
            })?;
        Ok(())
    }
}

fn elapsed_ms(elapsed: Duration) -> u64 {
    elapsed.as_millis().try_into().unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests;
