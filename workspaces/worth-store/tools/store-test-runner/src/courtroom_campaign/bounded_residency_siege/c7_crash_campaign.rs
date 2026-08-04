use std::ffi::OsString;
use std::process::Command;
use std::time::{Duration, Instant};

use worth_store::physical_runtime::{
    PhysicalWorkFreshReopenEvidence, PhysicalWorkProcessEvidence, PhysicalWorkRerunEvidence,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    offline_protocol, process_execution,
    schedule::{C7DurabilityCrashSeam, DurabilityCheckpointOrder},
    world::BoundedResidencySiegeWorld,
};

mod checkpoint;
mod process_accounting;
mod timing;

const CHILD_TIMEOUT: Duration = Duration::from_secs(300);
const CRASH_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy)]
enum C7CaseProcessRole {
    SeedProducer,
    BaselineObserver,
    ServingWriter,
    PostInterruptionObserver,
    FreshReopener,
}

impl C7CaseProcessRole {
    const ALL: [Self; 5] = [
        Self::SeedProducer,
        Self::BaselineObserver,
        Self::ServingWriter,
        Self::PostInterruptionObserver,
        Self::FreshReopener,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::SeedProducer => "seed-producer",
            Self::BaselineObserver => "baseline-observer",
            Self::ServingWriter => "serving-writer",
            Self::PostInterruptionObserver => "post-interruption-observer",
            Self::FreshReopener => "fresh-reopener",
        }
    }

    fn qualified(self, seam: C7DurabilityCrashSeam) -> String {
        format!("c7:{}:{}", seam.label(), self.label())
    }
}

pub(super) struct C7CrashCampaignEvidence {
    cases: Box<[C7CrashSeamEvidence]>,
    process_accounting: process_accounting::C7CrashProcessAccounting,
}

pub(super) struct C7CrashCampaignRequest<'a> {
    pub(super) target_root: Option<&'a std::path::Path>,
    pub(super) mutant_report: &'a std::path::Path,
    pub(super) report: &'a std::path::Path,
    pub(super) binaries: &'a BuiltCourtroomExecutables,
    pub(super) seams: &'a [C7DurabilityCrashSeam],
    pub(super) schedule_seed: u64,
    pub(super) checkpoint_order: DurabilityCheckpointOrder,
}

pub(super) struct C7CrashSeamEvidence {
    seam: C7DurabilityCrashSeam,
    checkpoint_order: DurabilityCheckpointOrder,
    baseline: offline_protocol::OfflineObservation,
    observed: offline_protocol::OfflineObservation,
    reopen: PhysicalWorkFreshReopenEvidence,
    checkpoint: Box<str>,
    rerun: PhysicalWorkRerunEvidence,
    timing: timing::C7CaseTiming,
}

struct ExecutedC7Case {
    evidence: C7CrashSeamEvidence,
    processes: [PhysicalWorkProcessEvidence; 5],
}

impl C7CrashCampaignEvidence {
    pub(super) fn cases(&self) -> &[C7CrashSeamEvidence] {
        &self.cases
    }

    pub(super) fn processes(&self) -> &[PhysicalWorkProcessEvidence] {
        self.process_accounting.processes()
    }
}

impl C7CrashSeamEvidence {
    pub(super) const fn seam(&self) -> C7DurabilityCrashSeam {
        self.seam
    }

    pub(super) const fn checkpoint_order(&self) -> DurabilityCheckpointOrder {
        self.checkpoint_order
    }

    pub(super) const fn baseline(&self) -> &offline_protocol::OfflineObservation {
        &self.baseline
    }

    pub(super) const fn observed(&self) -> &offline_protocol::OfflineObservation {
        &self.observed
    }

    pub(super) const fn reopen(&self) -> PhysicalWorkFreshReopenEvidence {
        self.reopen
    }

    pub(super) fn checkpoint(&self) -> &str {
        &self.checkpoint
    }

    pub(super) const fn rerun(&self) -> &PhysicalWorkRerunEvidence {
        &self.rerun
    }

    pub(super) fn timing(&self) -> impl serde::Serialize + '_ {
        &self.timing
    }
}

pub(super) fn execute(
    request: C7CrashCampaignRequest<'_>,
) -> Result<C7CrashCampaignEvidence, String> {
    let mut cases = Vec::with_capacity(request.seams.len());
    let mut processes = Vec::with_capacity(request.seams.len().saturating_mul(5));
    for seam in request.seams.iter().copied() {
        let executed = execute_case(&request, seam)?;
        processes.extend(executed.processes);
        cases.push(executed.evidence);
    }
    let process_accounting =
        process_accounting::C7CrashProcessAccounting::bind(request.seams, processes)?;
    Ok(C7CrashCampaignEvidence {
        cases: cases.into_boxed_slice(),
        process_accounting,
    })
}

fn execute_case(
    request: &C7CrashCampaignRequest<'_>,
    seam: C7DurabilityCrashSeam,
) -> Result<ExecutedC7Case, String> {
    let case_started = Instant::now();
    let started = Instant::now();
    let world = BoundedResidencySiegeWorld::create(request.target_root)?;
    let world_construction = started.elapsed();

    let started = Instant::now();
    let producer = run_producer(&world, request.binaries)?;
    let seed_producer = started.elapsed();

    let started = Instant::now();
    let baseline_process = run_offline(&world, request.binaries, "C7 baseline observer")?;
    let baseline = offline_protocol::parse(&baseline_process)?;
    let baseline_observer = started.elapsed();

    let started = Instant::now();
    let (serving, checkpoint) =
        run_crash(&world, request.binaries, seam, request.checkpoint_order)?;
    checkpoint::verify(&serving, &checkpoint, seam)?;
    let serving_writer = started.elapsed();

    let started = Instant::now();
    let observed_process = run_offline(&world, request.binaries, "C7 post-kill observer")?;
    let observed = offline_protocol::parse(&observed_process)?;
    let post_interruption_observer = started.elapsed();

    let started = Instant::now();
    let reopen_process = run_reopen(&world, request.binaries)?;
    let reopen = super::reopen_protocol::parse(&reopen_process)?;
    let fresh_reopener = started.elapsed();

    let started = Instant::now();
    let rerun =
        super::super::run_provenance::rerun(super::super::run_provenance::CourtroomRerunRequest {
            courtroom: "c",
            target_root: request.target_root,
            controlled_case_report: request.mutant_report,
            report: request.report,
            schedule_seed: Some(request.schedule_seed),
            termination_point: Some(seam.label()),
        })?;
    let processes = [
        producer.evidence(&C7CaseProcessRole::SeedProducer.qualified(seam))?,
        baseline_process.evidence(&C7CaseProcessRole::BaselineObserver.qualified(seam))?,
        serving.evidence(&C7CaseProcessRole::ServingWriter.qualified(seam))?,
        observed_process.evidence(&C7CaseProcessRole::PostInterruptionObserver.qualified(seam))?,
        reopen_process.evidence(&C7CaseProcessRole::FreshReopener.qualified(seam))?,
    ];
    let evidence_binding = started.elapsed();
    let timing = timing::C7CaseTiming::bind(
        timing::C7CaseStageDurations {
            world_construction,
            seed_producer,
            baseline_observer,
            serving_writer,
            post_interruption_observer,
            fresh_reopener,
            evidence_binding,
        },
        case_started.elapsed(),
    )?;
    Ok(ExecutedC7Case {
        evidence: C7CrashSeamEvidence {
            seam,
            checkpoint_order: request.checkpoint_order,
            baseline,
            observed,
            reopen,
            checkpoint: checkpoint.into_boxed_str(),
            rerun,
            timing,
        },
        processes,
    })
}

fn run_producer(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
) -> Result<process_execution::CapturedProcess, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("bounded-residency-producer")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration());
    process_execution::run_success(&mut command, CHILD_TIMEOUT, "C7 seed producer")
}

fn run_offline(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    label: &str,
) -> Result<process_execution::CapturedProcess, String> {
    let mut command = Command::new(binaries.observer().path());
    command.arg("hostile-physical-truth").arg(world.store());
    process_execution::run_success(&mut command, CHILD_TIMEOUT, label)
}

fn run_crash(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    seam: C7DurabilityCrashSeam,
    checkpoint_order: DurabilityCheckpointOrder,
) -> Result<(process_execution::CapturedProcess, String), String> {
    let mut command = Command::new(binaries.writer().path());
    command.args(crash_arguments(
        world.store(),
        world.configuration(),
        seam,
        checkpoint_order,
    ));
    let marker = format!("C7_COURTROOM_CRASH_CHECKPOINT {} ", seam.label());
    process_execution::kill_at_stdout_marker(&mut command, CRASH_TIMEOUT, &marker, seam.label())
}

fn crash_arguments(
    root: &std::path::Path,
    configuration: &std::path::Path,
    seam: C7DurabilityCrashSeam,
    checkpoint_order: DurabilityCheckpointOrder,
) -> [OsString; 9] {
    [
        "c7-crash".into(),
        "--root".into(),
        root.as_os_str().to_owned(),
        "--configuration".into(),
        configuration.as_os_str().to_owned(),
        "--crash-seam".into(),
        seam.label().into(),
        "--schedule-plan".into(),
        checkpoint_order.encoded().into(),
    ]
}

fn run_reopen(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
) -> Result<process_execution::CapturedProcess, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("reopen")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration());
    process_execution::run_success(&mut command, CHILD_TIMEOUT, "C7 fresh reopener")
}

#[cfg(test)]
mod tests;
