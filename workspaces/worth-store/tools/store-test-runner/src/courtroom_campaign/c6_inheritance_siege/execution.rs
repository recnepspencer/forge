use std::process::Command;
use std::time::Duration;

use worth_store::physical_runtime::{
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkFreshReopenEvidence,
    PhysicalWorkProcessEvidence,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    offline_protocol::{self, OfflineObservation},
    process_execution,
    protocol::{self, C6SiegeObservation},
    timing::C6SiegeTimings,
    world::C6SiegeWorld,
};

const CHILD_TIMEOUT: Duration = Duration::from_secs(30);

pub(super) struct C6SiegeObservations {
    pub(super) child: C6SiegeObservation,
    pub(super) offline: OfflineObservation,
    pub(super) reopen: PhysicalWorkFreshReopenEvidence,
    pub(super) filesystem: PhysicalWorkFilesystemProfileEvidence,
    pub(super) processes: Box<[PhysicalWorkProcessEvidence]>,
}

pub(super) fn observe(
    world: &C6SiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut C6SiegeTimings,
) -> Result<C6SiegeObservations, String> {
    let writer = run_writer(world, binaries, timings)?;
    let offline = run_offline(world, binaries, timings)?;
    let reopen = run_reopen(world, binaries, timings)?;
    Ok(C6SiegeObservations {
        child: writer.observation,
        offline: offline.observation,
        reopen: reopen.observation,
        filesystem: writer.filesystem,
        processes: vec![writer.process, offline.process, reopen.process].into_boxed_slice(),
    })
}

struct WriterObservation {
    observation: C6SiegeObservation,
    filesystem: PhysicalWorkFilesystemProfileEvidence,
    process: PhysicalWorkProcessEvidence,
}

struct ProcessObservation<T> {
    observation: T,
    process: PhysicalWorkProcessEvidence,
}

fn run_writer(
    world: &C6SiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut C6SiegeTimings,
) -> Result<WriterObservation, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("c6-pressure")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration())
        .arg("--oracle")
        .arg(world.oracle());
    let output =
        process_execution::run_success(&mut command, CHILD_TIMEOUT, "C.6 inheritance siege")?;
    timings.record(super::timing::SiegePhase::SiegeWriter, output.elapsed());
    Ok(WriterObservation {
        observation: protocol::parse(&output)?,
        filesystem: super::super::filesystem_profile_protocol::parse(output.stdout())?,
        process: output.evidence("siege-writer")?,
    })
}

fn run_offline(
    world: &C6SiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut C6SiegeTimings,
) -> Result<ProcessObservation<OfflineObservation>, String> {
    let mut command = Command::new(binaries.observer().path());
    command.arg("hostile-physical-truth").arg(world.store());
    let output =
        process_execution::run_success(&mut command, CHILD_TIMEOUT, "C.6 offline observer")?;
    timings.record(super::timing::SiegePhase::OfflineObserver, output.elapsed());
    Ok(ProcessObservation {
        observation: offline_protocol::parse(&output)?,
        process: output.evidence("offline-observer")?,
    })
}

fn run_reopen(
    world: &C6SiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut C6SiegeTimings,
) -> Result<ProcessObservation<PhysicalWorkFreshReopenEvidence>, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("reopen")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration());
    let output = process_execution::run_success(&mut command, CHILD_TIMEOUT, "C.6 fresh reopener")?;
    timings.record(super::timing::SiegePhase::FreshReopener, output.elapsed());
    Ok(ProcessObservation {
        observation: super::reopen_protocol::parse(&output)?,
        process: output.evidence("fresh-reopener")?,
    })
}
