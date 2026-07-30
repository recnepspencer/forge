use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use worth_store::physical_runtime::{
    PhysicalWorkCourtroomRunBinding, PhysicalWorkExecutionContext, PhysicalWorkFreshReopenEvidence,
    PhysicalWorkHostileProcessEvidence, PhysicalWorkHostileTruthCaseBinding,
    PhysicalWorkHostileTruthCaseEvidence, PhysicalWorkHostileTruthScenario,
    PhysicalWorkPlatformEvidence, PhysicalWorkRerunEvidence, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkScheduleSeed, PhysicalWorkWorkloadSeed,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    offline_protocol::{self, OfflineObservation},
    oracle::{self, OraclePayloads, TruthOracleInput},
    process_execution, reopen_protocol,
    timing::{CampaignTimings, ScenarioStage},
    world::CampaignWorld,
    writer_protocol::{self, CheckpointObservation, SeedObservation},
};

const CHILD_TIMEOUT: Duration = Duration::from_secs(30);
const CHECKPOINT_TIMEOUT: Duration = Duration::from_secs(15);

pub(super) fn run_all(
    world: &CampaignWorld,
    binaries: &BuiltCourtroomExecutables,
    rerun: &PhysicalWorkRerunEvidence,
    timings: &mut CampaignTimings,
) -> Result<Vec<PhysicalWorkHostileTruthCaseEvidence>, String> {
    let seed_payload = std::fs::read(world.seed_oracle())
        .map_err(|error| format!("cannot read seed oracle: {error}"))?;
    let mutation_payload = std::fs::read(world.mutation_oracle())
        .map_err(|error| format!("cannot read mutation oracle: {error}"))?;
    let payloads = OraclePayloads {
        seed: &seed_payload,
        mutation: &mutation_payload,
    };
    let mut cases = Vec::new();
    for scenario in PhysicalWorkHostileTruthScenario::ALL {
        let root = world.scenario_root(scenario.label())?;
        cases.push(
            CaseExecution {
                scenario,
                root,
                world,
                binaries,
                rerun,
                timings,
                payloads,
            }
            .run()?,
        );
    }
    Ok(cases)
}

struct CaseExecution<'campaign> {
    scenario: PhysicalWorkHostileTruthScenario,
    root: PathBuf,
    world: &'campaign CampaignWorld,
    binaries: &'campaign BuiltCourtroomExecutables,
    rerun: &'campaign PhysicalWorkRerunEvidence,
    timings: &'campaign mut CampaignTimings,
    payloads: OraclePayloads<'campaign>,
}

struct CaseObservations {
    seed: SeedObservation,
    baseline: OfflineObservation,
    checkpoint: CheckpointObservation,
    observed: OfflineObservation,
    reopener: PhysicalWorkFreshReopenEvidence,
    processes: PhysicalWorkHostileProcessEvidence,
}

impl CaseExecution<'_> {
    fn run(mut self) -> Result<PhysicalWorkHostileTruthCaseEvidence, String> {
        let observations = self.observe()?;
        let started = Instant::now();
        let evidence = self.finish(observations);
        self.timings
            .record_case_verification(self.scenario, started.elapsed());
        evidence
    }

    fn observe(&mut self) -> Result<CaseObservations, String> {
        let seed_process = self.run_seed()?;
        let seed = writer_protocol::parse_seed(&seed_process)?;
        let baseline_process =
            self.run_offline("baseline-observer", ScenarioStage::BaselineObserver)?;
        let baseline = offline_protocol::parse(&baseline_process)?;
        let (fault_process, checkpoint_marker) = self.run_fault()?;
        let checkpoint =
            writer_protocol::parse_checkpoint(&fault_process, &checkpoint_marker, self.scenario)?;
        let observer_process =
            self.run_offline("post-kill-observer", ScenarioStage::PostKillObserver)?;
        let observed = offline_protocol::parse(&observer_process)?;
        let reopen_process = self.run_reopen()?;
        let reopener = reopen_protocol::parse(&reopen_process)?;
        let processes = PhysicalWorkHostileProcessEvidence::new(
            seed_process.evidence("seed-writer")?,
            baseline_process.evidence("baseline-observer")?,
            fault_process.evidence("faulting-writer")?,
            observer_process.evidence("post-kill-observer")?,
            reopen_process.evidence("fresh-reopener")?,
        )
        .map_err(|denial| format!("scenario process evidence denied: {denial:?}"))?;
        Ok(CaseObservations {
            seed,
            baseline,
            checkpoint,
            observed,
            reopener,
            processes,
        })
    }

    fn finish(
        &self,
        observations: CaseObservations,
    ) -> Result<PhysicalWorkHostileTruthCaseEvidence, String> {
        let (comparison, oracle) = oracle::compare(TruthOracleInput {
            scenario: self.scenario,
            seed: &observations.seed,
            baseline: &observations.baseline,
            observed: &observations.observed,
            checkpoint: &observations.checkpoint,
            payloads: self.payloads,
        })?;
        let processes = observations.processes;
        let execution = PhysicalWorkExecutionContext::new(
            PhysicalWorkWorkloadSeed::new(self.world.seed()),
            PhysicalWorkScheduleSeed::new(self.world.seed()),
            observations.checkpoint.schedule(self.scenario),
            processes.ordered().map(Clone::clone),
        )
        .map_err(|denial| format!("scenario execution binding denied: {denial:?}"))?;
        let environment = PhysicalWorkRunEnvironmentEvidence::new(
            self.binaries.feature_graph().clone(),
            PhysicalWorkPlatformEvidence::current(),
            observations.seed.filesystem().clone(),
            self.rerun.clone(),
        );
        let run = PhysicalWorkCourtroomRunBinding::new(
            self.binaries.source().clone(),
            self.binaries.writer().binding().clone(),
            execution,
            environment,
        );
        let binding = PhysicalWorkHostileTruthCaseBinding::new(
            self.scenario,
            run,
            self.binaries.observer().binding().clone(),
            processes,
        );
        let evidence = binding.finish(
            comparison,
            observations.observed.lower_artifacts()?,
            observations.reopener,
            oracle,
        );
        self.require_accepted(evidence)
    }

    fn require_accepted(
        &self,
        evidence: PhysicalWorkHostileTruthCaseEvidence,
    ) -> Result<PhysicalWorkHostileTruthCaseEvidence, String> {
        if !evidence.verdict().accepted() {
            return Err(format!(
                "{} evidence rejected: {:?}",
                self.scenario.label(),
                evidence.verdict().findings()
            ));
        }
        Ok(evidence)
    }

    fn run_seed(&mut self) -> Result<process_execution::CapturedProcess, String> {
        let mut command = Command::new(self.binaries.writer().path());
        command
            .arg("write")
            .arg("--root")
            .arg(&self.root)
            .arg("--configuration")
            .arg(self.world.configuration())
            .arg("--oracle")
            .arg(self.world.seed_oracle())
            .arg("--scenario")
            .arg("seed-prior-truth");
        let output = process_execution::run_success(&mut command, CHILD_TIMEOUT, "seed writer")?;
        self.record_timing(ScenarioStage::Seed, output.elapsed());
        Ok(output)
    }

    fn run_offline(
        &mut self,
        phase: &str,
        stage: ScenarioStage,
    ) -> Result<process_execution::CapturedProcess, String> {
        let mut command = Command::new(self.binaries.observer().path());
        command.arg("hostile-physical-truth").arg(&self.root);
        let output = process_execution::run_success(&mut command, CHILD_TIMEOUT, phase)?;
        self.record_timing(stage, output.elapsed());
        Ok(output)
    }

    fn run_fault(&mut self) -> Result<(process_execution::CapturedProcess, String), String> {
        let mut command = Command::new(self.binaries.writer().path());
        if self.scenario == PhysicalWorkHostileTruthScenario::DuringShutdown {
            command
                .arg("shutdown")
                .arg("--root")
                .arg(&self.root)
                .arg("--configuration")
                .arg(self.world.configuration());
        } else {
            command
                .arg("write")
                .arg("--root")
                .arg(&self.root)
                .arg("--configuration")
                .arg(self.world.configuration())
                .arg("--oracle")
                .arg(self.world.mutation_oracle())
                .arg("--scenario")
                .arg(self.scenario.label());
        }
        let marker = format!("C5_1_COURTROOM_CHECKPOINT {} ", self.scenario.label());
        let output = process_execution::kill_at_stdout_marker(
            &mut command,
            CHECKPOINT_TIMEOUT,
            &marker,
            self.scenario.label(),
        )?;
        self.record_timing(ScenarioStage::Fault, output.0.elapsed());
        Ok(output)
    }

    fn run_reopen(&mut self) -> Result<process_execution::CapturedProcess, String> {
        let mut command = Command::new(self.binaries.writer().path());
        command
            .arg("reopen")
            .arg("--root")
            .arg(&self.root)
            .arg("--configuration")
            .arg(self.world.configuration());
        let output = process_execution::run_success(&mut command, CHILD_TIMEOUT, "fresh reopener")?;
        self.record_timing(ScenarioStage::FreshReopener, output.elapsed());
        Ok(output)
    }

    fn record_timing(&mut self, stage: ScenarioStage, elapsed: Duration) {
        self.timings.record_scenario(self.scenario, stage, elapsed);
    }
}
