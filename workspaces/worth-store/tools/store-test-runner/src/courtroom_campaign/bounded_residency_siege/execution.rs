use std::num::NonZeroU32;
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
    protocol::{self, BoundedResidencySiegeObservation},
    schedule::SchedulePerturbationPlan,
    timing::BoundedResidencySiegeTimings,
    world::BoundedResidencySiegeWorld,
};

const PRODUCER_TIMEOUT: Duration = Duration::from_secs(300);
const SERVING_TIMEOUT: Duration = Duration::from_secs(30);
const LIGHTWEIGHT_CHILD_TIMEOUT: Duration = Duration::from_secs(10);

pub(super) struct BoundedResidencySiegeObservations {
    pub(super) producer: BoundedResidencyProducerObservation,
    pub(super) child: BoundedResidencySiegeObservation,
    pub(super) verifier: BoundedResidencyVerifierObservation,
    pub(super) offline: OfflineObservation,
    pub(super) reopen: PhysicalWorkFreshReopenEvidence,
    pub(super) filesystem: PhysicalWorkFilesystemProfileEvidence,
    pub(super) processes: Box<[PhysicalWorkProcessEvidence]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyProducerObservation {
    pub(super) process: NonZeroU32,
    pub(super) store: [u8; 16],
    pub(super) runtime: u64,
    pub(super) generation: u64,
    pub(super) records: u64,
    pub(super) payload_bytes: u64,
    pub(super) expectation_digest: [u8; 32],
    pub(super) peak_resident_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BoundedResidencyVerifierObservation {
    pub(super) records: u64,
    pub(super) payload_bytes: u64,
    pub(super) expectation_digest: [u8; 32],
    pub(super) seed: u64,
}

pub(super) fn observe(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    schedule: &SchedulePerturbationPlan,
    timings: &mut BoundedResidencySiegeTimings,
) -> Result<BoundedResidencySiegeObservations, String> {
    let producer = run_producer(world, binaries, timings)?;
    let writer = run_serving(world, binaries, schedule, timings)?;
    let offline = run_offline(world, binaries, timings)?;
    let reopen = run_reopen(world, binaries, timings)?;
    Ok(BoundedResidencySiegeObservations {
        producer: producer.observation,
        child: writer.observation,
        verifier: offline.verification,
        offline: offline.observation,
        reopen: reopen.observation,
        filesystem: writer.filesystem,
        processes: vec![
            producer.process,
            writer.process,
            offline.process,
            reopen.process,
        ]
        .into_boxed_slice(),
    })
}

#[cfg(test)]
pub(super) fn observe_serving_for_mutation(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
) -> Result<BoundedResidencySiegeObservation, String> {
    let mut timings = BoundedResidencySiegeTimings::new();
    let schedule = SchedulePerturbationPlan::canonical();
    run_producer(world, binaries, &mut timings)?;
    run_serving(world, binaries, &schedule, &mut timings).map(|writer| writer.observation)
}

struct WriterObservation {
    observation: BoundedResidencySiegeObservation,
    filesystem: PhysicalWorkFilesystemProfileEvidence,
    process: PhysicalWorkProcessEvidence,
}

struct ProcessObservation<T> {
    observation: T,
    process: PhysicalWorkProcessEvidence,
}

struct VerifierProcessObservation {
    observation: OfflineObservation,
    verification: BoundedResidencyVerifierObservation,
    process: PhysicalWorkProcessEvidence,
}

fn run_producer(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut BoundedResidencySiegeTimings,
) -> Result<ProcessObservation<BoundedResidencyProducerObservation>, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("bounded-residency-producer")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration());
    let output = process_execution::run_success(
        &mut command,
        PRODUCER_TIMEOUT,
        "bounded-residency producer",
    )?;
    timings.record(
        super::timing::BoundedResidencySiegePhase::SiegeProducer,
        output.elapsed(),
    );
    Ok(ProcessObservation {
        observation: parse_producer(&output)?,
        process: output.evidence("producer")?,
    })
}

fn run_serving(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    schedule: &SchedulePerturbationPlan,
    timings: &mut BoundedResidencySiegeTimings,
) -> Result<WriterObservation, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("bounded-residency-serving")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration())
        .arg("--schedule-plan")
        .arg(schedule.child_argument());
    let output =
        process_execution::run_success(&mut command, SERVING_TIMEOUT, "bounded-residency serving")?;
    timings.record(
        super::timing::BoundedResidencySiegePhase::SiegeServing,
        output.elapsed(),
    );
    Ok(WriterObservation {
        observation: protocol::parse(&output)?,
        filesystem: super::super::filesystem_profile_protocol::parse(output.stdout())?,
        process: output.evidence("serving")?,
    })
}

fn parse_producer(
    process: &process_execution::CapturedProcess,
) -> Result<BoundedResidencyProducerObservation, String> {
    let matching = process
        .stdout()
        .iter()
        .filter(|line| line.starts_with("BOUNDED_RESIDENCY_PRODUCER "))
        .collect::<Vec<_>>();
    let [line] = matching.as_slice() else {
        return Err(format!(
            "expected one bounded-residency producer marker, found {}",
            matching.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 9 {
        return Err(format!(
            "malformed bounded-residency producer marker `{line}`"
        ));
    }
    let reported = NonZeroU32::new(number(fields[1], "producer process")?)
        .ok_or_else(|| "producer process cannot be zero".to_owned())?;
    if reported != process.process() {
        return Err("bounded-residency producer reported a foreign process".to_owned());
    }
    Ok(BoundedResidencyProducerObservation {
        process: reported,
        store: fixed_hex(fields[2], "producer Store")?,
        runtime: number(fields[3], "producer runtime")?,
        generation: number(fields[4], "producer generation")?,
        records: number(fields[5], "producer records")?,
        payload_bytes: number(fields[6], "producer payload bytes")?,
        expectation_digest: fixed_hex(fields[7], "producer expectation digest")?,
        peak_resident_bytes: number(fields[8], "producer peak resident bytes")?,
    })
}

fn number<T: std::str::FromStr>(encoded: &str, label: &str) -> Result<T, String> {
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}

fn fixed_hex<const N: usize>(encoded: &str, label: &str) -> Result<[u8; N], String> {
    if encoded.len() != N * 2 || !encoded.is_ascii() {
        return Err(format!(
            "{label} must contain exactly {N} hexadecimal bytes"
        ));
    }
    let mut bytes = [0_u8; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| format!("{label} contains non-hexadecimal data"))?;
    }
    Ok(bytes)
}

fn run_offline(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut BoundedResidencySiegeTimings,
) -> Result<VerifierProcessObservation, String> {
    let mut command = Command::new(binaries.observer().path());
    command
        .arg("bounded-residency-verify")
        .arg(world.store())
        .arg(world.configuration());
    let output = process_execution::run_success(
        &mut command,
        LIGHTWEIGHT_CHILD_TIMEOUT,
        "C.6 offline observer",
    )?;
    timings.record(
        super::timing::BoundedResidencySiegePhase::OfflineObserver,
        output.elapsed(),
    );
    Ok(VerifierProcessObservation {
        observation: offline_protocol::parse(&output)?,
        verification: parse_verification(&output)?,
        process: output.evidence("offline-observer")?,
    })
}

fn parse_verification(
    process: &process_execution::CapturedProcess,
) -> Result<BoundedResidencyVerifierObservation, String> {
    let matching = process
        .stdout()
        .iter()
        .filter(|line| line.starts_with("BOUNDED_RESIDENCY_VERIFICATION "))
        .collect::<Vec<_>>();
    let [line] = matching.as_slice() else {
        return Err(format!(
            "expected one bounded-residency verifier marker, found {}",
            matching.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 6 || fields[1] != "accepted" {
        return Err(format!("bounded-residency verifier denied truth: `{line}`"));
    }
    Ok(BoundedResidencyVerifierObservation {
        records: number(fields[2], "verifier records")?,
        payload_bytes: number(fields[3], "verifier payload bytes")?,
        expectation_digest: fixed_hex(fields[4], "verifier expectation digest")?,
        seed: number(fields[5], "verifier seed")?,
    })
}

fn run_reopen(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    timings: &mut BoundedResidencySiegeTimings,
) -> Result<ProcessObservation<PhysicalWorkFreshReopenEvidence>, String> {
    let mut command = Command::new(binaries.writer().path());
    command
        .arg("reopen")
        .arg("--root")
        .arg(world.store())
        .arg("--configuration")
        .arg(world.configuration());
    let output = process_execution::run_success(
        &mut command,
        LIGHTWEIGHT_CHILD_TIMEOUT,
        "C.6 fresh reopener",
    )?;
    timings.record(
        super::timing::BoundedResidencySiegePhase::FreshReopener,
        output.elapsed(),
    );
    Ok(ProcessObservation {
        observation: super::reopen_protocol::parse(&output)?,
        process: output.evidence("fresh-reopener")?,
    })
}
